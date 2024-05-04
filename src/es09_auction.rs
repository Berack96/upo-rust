#![allow(unused)]

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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
#[derive(Clone, Debug)]
pub struct Product {
    name: String,
    price: f32,
    reserve: f32,
}

#[derive(Clone, Debug)]
enum AuctionRequest {
    AuctionStart(String, f32),
    AuctionOver(Arc<Product>),
    UpdatedPrice(f32),
    YouAreWinning(f32),
    YouWon(Product),
    Stop,
}

#[derive(Clone, Debug)]
enum AuctionResponse {
    NotInterested(String),
    WantForPrice(String, f32),
}

type SendRequest = Sender<AuctionRequest>;
type ReciveRequest = Receiver<AuctionRequest>;
type SendResponse = Sender<AuctionResponse>;
type ReciveResponse = Receiver<AuctionResponse>;

#[derive(Debug)]
struct Auctioneer {
    products: VecDeque<Product>,
    currents: HashSet<String>,
    participants: HashMap<String, SendRequest>,
    recive: ReciveResponse,
}

#[derive(Debug)]
struct Participant {
    name: String,
    money: f32,
    rng: Arc<Mutex<Pcg32>>,
    products_won: Vec<Product>,
    sender: SendResponse,
    recive: ReciveRequest,
}

#[derive(Debug)]
pub struct Auction {
    auctioneer: Auctioneer,
    participants: Vec<Participant>,
    sender: SendResponse,
    rng: Arc<Mutex<Pcg32>>,
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
    pub fn auction_loop(&mut self) {
        while let Ok(result) = self.recive.recv() {
            match result {
                AuctionRequest::AuctionStart(_, price) => self.updated_price(price),
                AuctionRequest::UpdatedPrice(price) => self.updated_price(price),
                AuctionRequest::YouAreWinning(price) => self.winning_for(price),
                AuctionRequest::YouWon(prod) => self.won_product(prod),
                AuctionRequest::AuctionOver(prod) => println!(
                    "Participant {:?} money:{:?}, won: {:?}",
                    self.name, self.money, self.products_won
                ),
                AuctionRequest::Stop => return,
            }
        }
    }

    fn updated_price(&self, price: f32) {
        let name = self.name.clone();
        let response = if price <= self.money {
            let up = self.rng.lock().unwrap().gen_range(price..self.money);
            AuctionResponse::WantForPrice(name, up)
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
    pub fn auction_loop(&mut self) {
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

            self.end_product(winner, product);
        }

        self.end_auction();
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
                println!("Response -> {:?}", response);
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
            println!("Sending to {:?} -> {:?}", recipient, message);
            channel.send(message);
        }
    }
}

impl Auction {
    pub fn new(products: VecDeque<Product>, seed: u64) -> Self {
        let channel_participant = mpsc::channel();
        let auctioneer = Auctioneer {
            products,
            currents: HashSet::new(),
            participants: HashMap::new(),
            recive: channel_participant.1,
        };

        Self {
            auctioneer,
            rng: Arc::new(Mutex::new(Pcg32::seed_from_u64(seed))),
            participants: vec![],
            sender: channel_participant.0,
        }
    }

    pub fn add_participant(&mut self, name: String, money: f32) {
        let channel = mpsc::channel();
        let participant = Participant {
            name: name.clone(),
            money,
            rng: self.rng.clone(),
            products_won: vec![],
            sender: self.sender.clone(),
            recive: channel.1,
        };

        self.auctioneer.participants.insert(name, channel.0);
        self.participants.push(participant);
    }

    pub fn start(mut self) {
        while let Some(participant) = self.participants.pop() {
            Self::start_participant(participant);
        }
        Self::start_auctioneer(self.auctioneer);
    }

    fn start_participant(participant: Participant) {
        thread::spawn(|| {
            let mut part = participant;
            part.auction_loop();
        });
    }

    fn start_auctioneer(auctioneer: Auctioneer) {
        let mut auct = auctioneer;
        auct.auction_loop();
    }
}
