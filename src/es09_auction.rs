#![allow(unused)]

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

/**
 * Implementare il protocollo English Auction.
 * Un banditore attende  n partecipanti ad un’asta per un prodotto che ha un prezzo iniziale ed un prezzo di
 * riserva al di sotto del quale non puo’ essere venduto.
 * Dopo aver ricevuto un messaggio da gli n partecipanti, invia loro la descrizione del prodotto che viene
 * bandito e il suo prezzo minimo.
 * I partecipanti possono rispondere: dicendo che non vogliono partecipare all’asta oppure che offrono un valore
 * che deve essere maggiore o uguale al prezzo minimo.
 *
 * Il banditore se riceve da un partecipante il messaggio di uscita dall’asta lo elimina dalle successive
 * interazioni, se invece riceve un messaggio con un prezzo, invia a tutti i partecipanti ancora interessati un
 * nuovo prezzo (maggiore di quello che ha ricevuto da un partecipante).
 * Il protocollo va avanti così fino a che il banditore non ha ricevuto da tutti i partecipanti il messaggio che non
 * vogliono partecipare.
 *
 * A questo punto se il prezzo e’ maggiore del prezzo di riserva informa il partecipante che ha ottenuto il
 * prodotto che e’ il vincitore e gli altri che l’asta e’ finita.
 *
 * Infine ill banditore mette in una struttura condivisa dai partecipanti l’indicazione del prodotto venduto a quale
 * cifra e’ statto venduto e a quale partecipante e’ stato assegnato.
 * I partecipanti leggono questa informazione.
 */

/// Struttura che indica un prodotto da bandire all'asta.\
/// Il Prodotto ha un nome, un prezzo di partenza e una riserva che se non
/// raggiunta implica il fallimento dell'asta per questo oggeto.
#[derive(Clone, Debug)]
pub struct Product {
    pub name: String,
    pub price: f32,
    pub reserve: f32,
}

/// Richieste che vengono inviate dal Banditore a tutti i Partecipanti.\
/// Possono essere di vario tipo e indicano un cambiamento di stato dell'asta corrente.
#[derive(Clone, Debug)]
enum AuctionRequest {
    AuctionStart(String, f32),
    AuctionOver(Arc<Product>),
    UpdatedPrice(f32),
    YouAreWinning(f32),
    YouWon(Product),
    Stop,
}

/// Risposte che vengono inviate dai Partecipanti dell'asta al Banditore.\
/// Qui si può indicare solamente se si vuole continuare o meno all'asta.
#[derive(Clone, Debug)]
enum AuctionResponse {
    NotInterested(String),
    WantForPrice(String, f32),
}

type SendRequest = Sender<AuctionRequest>;
type ReciveRequest = Receiver<AuctionRequest>;
type SendResponse = Sender<AuctionResponse>;
type ReciveResponse = Receiver<AuctionResponse>;

/// Asta che comprende più prodotti.\
/// La creazione di un'asta avrà sempre un Banditore, ma per i partecipandi bisogna
/// utilizzare l'apposita funzione per l'aggiunta.
#[derive(Debug)]
pub struct Auction {
    auctioneer: Auctioneer,
    participants: Vec<Participant>,
    sender: SendResponse,
}

/// Banditore di un'asta che modera i partecipanti, li aggiorna sul cambiamento del prezzo
/// e alla fine decide il vincitore del'asta nel caso in cui non ci siano altri partecipanti.
#[derive(Debug)]
struct Auctioneer {
    products: VecDeque<Product>,
    currents: HashSet<String>,
    participants: HashMap<String, SendRequest>,
    recive: ReciveResponse,
    log: bool,
}

/// Partecipante ad un'asta che ha un tot di soldi a disposizione.\
/// Ogni partecipante deve avere una strategia che permette di scegliere cosa fare nel caso
/// in cui il prezzo di un oggetto aumenta.
#[derive(Debug)]
struct Participant {
    name: String,
    money: f32,
    strategy: Box<dyn Strategy>,
    products_won: Vec<Product>,
    sender: SendResponse,
    recive: ReciveRequest,
}

/// Trait che indica una possibile Strategia per un partecipante.\
/// Il trait è stato introdotto nel caso in cui si voglia modificare il comportamento di un partecipante.
/// Per essere implementeto il trait ha bisogno di una sola funzione, dato che la funzione di
/// start_auction può essere ignorata.
pub trait Strategy: Send + Debug {
    /// Funzione utilizzata per poter indicare alla strategia che è iniziata una nuova asta.\
    /// Questo è utile nel caso in cui si voglia creare una strategia che tenga conto dello
    /// stato in cui si trova l'asta.\
    /// Nel caso in cui interessa fare solo una strategia stateless allora si può ignorare questa funzione.
    fn start_auction(&mut self, product: String) {}
    /// Questa funzione ritorna un nuovo valore nel caso in cui, per la strategia, si voglia continuare
    /// a provare a vincere l'asta.\
    /// Nel caso in cui ci si voglia ritirare, allora il valore di ritorno dovrà essere None.
    /// Nel caso in cui il valore di ritorno supera total_money allora si verrà ritirati dall'asta.
    fn updated_price(&mut self, total_money: f32, price: f32) -> Option<f32>;
}

impl Product {
    pub fn new(name: &str, price: f32, reserve: f32) -> Self {
        Self {
            name: name.to_string(),
            price,
            reserve,
        }
    }
}

impl Participant {
    /// Funzione utilizzata per il loop dell'asta e che deve essere fatta partire
    /// PRIMA di aver fatto partire il loop per il banditore.\
    /// Questo perchè altrimenti si può incorrere in problemi quali il partecipante
    /// non abilitato all'asta.
    pub fn auction_loop(&mut self, log: bool) {
        while let Ok(result) = self.recive.recv() {
            match result {
                AuctionRequest::AuctionStart(product, price) => {
                    self.strategy.start_auction(product);
                    self.updated_price(price)
                }
                AuctionRequest::AuctionOver(prod) => {
                    if log {
                        println!(
                            "Participant {:?} money:{:?}, won: {:?}",
                            self.name, self.money, self.products_won
                        )
                    }
                }
                AuctionRequest::UpdatedPrice(price) => self.updated_price(price),
                AuctionRequest::YouAreWinning(price) => self.winning_for(price),
                AuctionRequest::YouWon(prod) => self.won_product(prod),
                AuctionRequest::Stop => return,
            }
        }
    }

    fn updated_price(&mut self, price: f32) {
        let name = self.name.clone();
        let up_price = self.strategy.updated_price(self.money, price);
        let response = if matches!(up_price, Some(up) if up <= self.money) {
            AuctionResponse::WantForPrice(name, up_price.unwrap())
        } else {
            AuctionResponse::NotInterested(name)
        };

        self.sender.send(response);
    }
    fn winning_for(&self, price: f32) {
        self.sender
            .send(AuctionResponse::WantForPrice(self.name.clone(), price));
    }
    fn won_product(&mut self, product: Product) {
        self.money -= product.price;
        self.products_won.push(product);
    }
}

impl Auctioneer {
    /// Funzione utilizzata per il loop dell'asta e che deve essere fatta partire
    /// DOPO aver fatto partire tutti i loop dei partecipanti su dei thread diversi da questo.\
    /// Questo per evitare che si possano avere situazioni di deadlock o semplicemente problemi
    /// nella comunicazione con i thread dei partecipanti.
    pub fn auction_loop(&mut self) -> VecDeque<(Product, Option<String>)> {
        let mut results = VecDeque::new();
        while let Some(mut product) = self.products.pop_front() {
            self.new_product(product.name.clone(), product.price);
            let mut winner = None;

            loop {
                let response = self.wait_for_responses();
                if response.len() > 1 {
                    let (name, price) = response
                        .iter()
                        .reduce(|acc, curr| if curr.1 > acc.1 { curr } else { acc })
                        .unwrap();

                    winner = Some(name.clone());
                    product.price = *price;
                    self.update_price(name.clone(), *price);
                } else {
                    break;
                }
            }

            if product.price < product.reserve {
                winner = None
            }

            results.push_back((product.clone(), winner.clone()));
            self.end_product(winner, product);
        }

        self.end_auction();
        results
    }

    fn new_product(&mut self, name: String, price: f32) {
        let iter = self.participants.keys().map(|part| part.clone());
        self.currents.clear();
        self.currents.extend(iter);
        self.send(false, AuctionRequest::AuctionStart(name, price));
    }
    fn update_price(&mut self, owner: String, price: f32) {
        self.currents.remove(&owner);
        self.send_only(&owner, AuctionRequest::YouAreWinning(price));
        self.send(false, AuctionRequest::UpdatedPrice(price));
        self.currents.insert(owner);
    }
    fn end_product(&mut self, winner: Option<String>, product: Product) {
        if let Some(winner) = winner {
            self.currents.remove(&winner);
            self.send_only(&winner, AuctionRequest::YouWon(product.clone()));
        }
        self.send(true, AuctionRequest::AuctionOver(Arc::new(product)));
    }
    fn end_auction(&self) {
        self.send(true, AuctionRequest::Stop);
    }
    fn wait_for_responses(&mut self) -> Vec<(String, f32)> {
        let mut responses = vec![];
        let mut count = 0;
        let total = self.currents.len();
        while count < total {
            if let Some(response) = self.recive() {
                responses.push(response);
            }
            count += 1;
        }

        responses
    }

    fn recive(&mut self) -> Option<(String, f32)> {
        match self.recive.recv() {
            Ok(response) => {
                if self.log {
                    println!("Response -> {:?}", response)
                };

                match response {
                    AuctionResponse::NotInterested(name) => {
                        self.currents.remove(&name);
                        None
                    }
                    AuctionResponse::WantForPrice(name, price) => Some((name, price)),
                }
            }
            Err(err) => panic!("Channel with the partecipants is interrupted!\n{}", err),
        }
    }
    fn send(&self, all: bool, message: AuctionRequest) {
        let recipients: Vec<&String> = if all {
            self.participants.iter().map(|(k, v)| k).collect()
        } else {
            self.currents.iter().map(|x| x).collect()
        };

        let mut count = 0;
        for recipient in recipients {
            self.send_only(recipient, message.clone());
            count += 1;
        }
    }
    fn send_only(&self, recipient: &String, message: AuctionRequest) {
        if let Some(channel) = self.participants.get(recipient) {
            if self.log {
                println!("Sending to {:?} -> {:?}", recipient, message)
            };

            channel.send(message);
        }
    }
}

impl Auction {
    /// Crea un'asta con i prodotti inseriti.\
    /// Nel caso in cui ci siano due prodotti con lo stesso nome allora verrà fatto partire un PANIC.
    pub fn new(products: VecDeque<Product>) -> Self {
        let channel_participant = mpsc::channel();
        let auctioneer = Auctioneer {
            products,
            currents: HashSet::new(),
            participants: HashMap::new(),
            recive: channel_participant.1,
            log: false,
        };

        Self {
            auctioneer,
            participants: vec![],
            sender: channel_participant.0,
        }
    }

    /// Abilita la possibilità di vedere lo scambio dei messaggi tra il banditore d'asta e i vari partecipanti
    pub fn enable_log(&mut self) {
        self.auctioneer.log = true
    }

    /// Aggiunge un partecipante all'asta.\
    /// Il partecipante deve avere un nome univoco rispetto agli altri altrimenti verrà segnalato con un PANIC
    pub fn add_participant(&mut self, name: String, money: f32, strategy: Box<dyn Strategy>) {
        let channel = mpsc::channel();
        let participant = Participant {
            name: name.clone(),
            money,
            strategy,
            products_won: vec![],
            sender: self.sender.clone(),
            recive: channel.1,
        };

        self.auctioneer.participants.insert(name, channel.0);
        self.participants.push(participant);
    }

    /// Inizia l'asta e consuma la struttura.\
    /// Ogni partecipante avrà il proprio thread che verrà fatto partire PRIMA del banditore d'asta.\
    /// Alla fine dell'asta si riceverà in output un vettore con tutti i prodotti e il vincitore nel caso ci sia.\
    /// Questa chiamata è bloccante, ovvero aspetta finchè l'asta non sarà finita.
    pub fn start(mut self) -> VecDeque<(Product, Option<String>)> {
        while let Some(mut participant) = self.participants.pop() {
            let log = self.auctioneer.log;
            thread::spawn(move || participant.auction_loop(log));
        }

        self.auctioneer.auction_loop()
    }
}
