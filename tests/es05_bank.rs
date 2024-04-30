use esercizi::es05_bank::BankAccount;

#[test]
fn test_bank() {
    let name = "Nome".to_string();
    let mut account = BankAccount::new(name.clone(), 1000.0, 10000.0, 0.02);
    assert_eq!(account.balance(), 0.0);
    assert_eq!(account.state(), "Red");

    account.deposit(1001.0);
    assert_eq!(account.balance(), 1001.0);
    assert_eq!(account.state(), "Silver");

    account.pay_interest();
    assert_eq!(account.balance(), 1001.0);
    assert_eq!(account.state(), "Silver");

    account.withdraw(100.0);
    assert_eq!(account.balance(), 901.0);
    assert_eq!(account.state(), "Red");

    account.withdraw(1.0);
    assert_eq!(account.balance(), 901.0);
    assert_eq!(account.state(), "Red");

    account.deposit(100000.0);
    assert_eq!(account.balance(), 100901.0);
    assert_eq!(account.state(), "Gold");

    account.pay_interest();
    assert_eq!(account.balance(), 98882.98);
    assert_eq!(account.state(), "Gold");

    account.withdraw(90882.98);
    assert_eq!(account.balance(), 8000.0);
    assert_eq!(account.state(), "Silver");
}
