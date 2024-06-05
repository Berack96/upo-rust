use esercizi::es09_auction::{Auction, Product, Strategy};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct UpMax;
impl Strategy for UpMax {
    fn updated_price(&mut self, total_money: f32, price: f32) -> Option<f32> {
        if price <= total_money {
            Some(total_money)
        } else {
            None
        }
    }
}

#[test]
fn test_auction() {
    let products = VecDeque::from([
        Product::new("bau", 10.0, 1000.0),
        Product::new("woof", 321.0, 18554.0),
        Product::new("woof2", 31.0, 1854.0),
    ]);
    let mut auction = Auction::new(products);

    let name1 = "th1".to_string();
    let name2 = "th2".to_string();
    let name3 = "th3".to_string();
    let name4 = "th4".to_string();

    auction.add_participant(name1.clone(), 4520.0, Box::new(UpMax));
    auction.add_participant(name2.clone(), 6500.0, Box::new(UpMax));
    auction.add_participant(name3.clone(), 15020.0, Box::new(UpMax));
    auction.add_participant(name4.clone(), 8520.0, Box::new(UpMax));

    let mut results = auction.start();
    assert_eq!(results.len(), 3);

    let auction = results.pop_front();
    assert!(matches!(auction, Some(_)));
    let (product, winner) = auction.unwrap();
    assert_eq!(product.name, "bau".to_string());
    assert_eq!(product.price, 15020.0);
    assert_eq!(product.reserve, 1000.0);
    assert!(matches!(winner, Some(name) if name == name3));

    let auction = results.pop_front();
    assert!(matches!(auction, Some(_)));
    let (product, winner) = auction.unwrap();
    assert_eq!(product.name, "woof".to_string());
    assert_eq!(product.price, 8520.0);
    assert_eq!(product.reserve, 18554.0);
    assert_eq!(winner, None);

    let auction = results.pop_front();
    assert!(matches!(auction, Some(_)));
    let (product, winner) = auction.unwrap();
    assert_eq!(product.name, "woof2".to_string());
    assert_eq!(product.price, 8520.0);
    assert_eq!(product.reserve, 1854.0);
    assert!(matches!(winner, Some(name) if name == name4));

    let auction = results.pop_front();
    assert!(matches!(auction, None));
}
