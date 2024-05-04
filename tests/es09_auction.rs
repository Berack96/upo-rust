use esercizi::es09_auction::{Auction, Product};
use std::collections::VecDeque;

#[test]
fn test_auction() {
    let products = VecDeque::from([
        Product::new("bau", 10.0, 1000.0),
        Product::new("woof", 321.0, 18554.0),
        Product::new("woof2", 31.0, 1854.0),
    ]);
    let mut auction = Auction::new(products, 0);

    auction.add_participant("th1".to_string(), 4520.0);
    auction.add_participant("th2".to_string(), 6500.0);
    auction.add_participant("th3".to_string(), 15020.0);
    auction.add_participant("th4".to_string(), 8520.0);

    auction.start();
}
