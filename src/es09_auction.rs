#![allow(unused)]

use std::{
    collections::HashMap,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
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
    owner: Option<String>,
}

#[derive(Clone, Debug)]
enum AuctionRequest {
    NewProduct(String, f32),
    UpdatedPrice(f32),
    YouWon,
    EndProduct(Arc<Product>),
    AuctionOver,
}

#[derive(Clone, Debug)]
enum AuctionResponse {
    NotInterested,
    WantForPrice(String, f32),
}

#[derive(Debug)]
struct Auctioneer {
    products: Vec<Product>,
    sender: HashMap<String, Sender<AuctionRequest>>,
    recive: Receiver<AuctionResponse>,
}

#[derive(Debug)]
struct Participant {
    name: String,
    money: f32,
    products_won: Vec<Product>,
    sender: Sender<AuctionResponse>,
    recive: Receiver<AuctionRequest>,
}

#[derive(Debug)]
struct Auction {
    auctioneer: Auctioneer,
    participants: Vec<Participant>,
    sender: Sender<AuctionResponse>,
}

impl Participant {
    pub fn get_next_move(&self, price:f32) {
        let diff = 10.0_f32.min(price - self.money);
        if diff > 0.0 {

        }
    }
}

impl Auctioneer {
    fn send(&self, message: AuctionRequest) {
        for (_, channel) in &self.sender {
            channel.send(message.clone());
        }
    }

    pub fn new_product(&self, name: String, price:f32) {
        self.send(AuctionRequest::NewProduct(name, price));
    }
    pub fn update_price(&self, price:f32) {
        self.send(AuctionRequest::UpdatedPrice(price));
    }
    pub fn end_product(&self, product: Arc<Product>) {
        self.send(AuctionRequest::EndProduct(product));
    }
    pub fn end_auction(&self) {
        self.send(AuctionRequest::AuctionOver);
    }
    pub fn wait_for_responses(&self, timeout: u64) -> Option<(String, f32)> {
        let mut price = None;
        loop {
            let timeout = Duration::from_millis(timeout);
            match self.recive.recv_timeout(timeout) {
                Ok(AuctionResponse::WantForPrice(name, proposed)) => {
                    if let Some((n, p)) = &price {
                        if proposed > *p {
                            price = Some((name, proposed))
                        }
                    } else {
                        price = Some((name, proposed))
                    }
                }
                Ok(AuctionResponse::NotInterested) => (),
                _ => break,
            };
        }

        price
    }
}

impl Auction {
    pub fn new(products: Vec<Product>) -> Self {
        let channel_participant = mpsc::channel();
        let auctioneer = Auctioneer {
            products,
            sender: HashMap::new(),
            recive: channel_participant.1,
        };

        Self {
            auctioneer,
            participants: vec![],
            sender: channel_participant.0,
        }
    }

    pub fn add_participant(&mut self, name: String, money: f32) {
        let channel = mpsc::channel();
        let participant = Participant {
            name: name.clone(),
            money,
            products_won: vec![],
            sender: self.sender.clone(),
            recive: channel.1,
        };

        self.auctioneer.sender.insert(name, channel.0);
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

            while let Ok(result) = part.recive.recv() {
                match result {
                    AuctionRequest::NewProduct(_, price) => (),
                    AuctionRequest::UpdatedPrice(_) => todo!(),
                    AuctionRequest::EndProduct(_) => todo!(),
                    AuctionRequest::YouWon => todo!(),
                    AuctionRequest::AuctionOver => return
                }
            }
        });
    }

    fn start_auctioneer(auctioneer: Auctioneer) {
        thread::spawn(|| {
            let mut auct = auctioneer;
            while let Some(mut product) = auct.products.pop() {
                auct.new_product(product.name.clone(), product.price);

                while let Some(response) = auct.wait_for_responses(300) {
                    let (name, price) = response;
                    auct.update_price(price);
                    product.owner = Some(name);
                    product.price = price;
                }

                if product.price < product.reserve {
                    product.owner = None
                }

                auct.end_product(Arc::new(product));
            }

            auct.end_auction();
        });
    }
}
