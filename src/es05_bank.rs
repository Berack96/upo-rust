#![allow(dead_code)]

use std::{fmt::Debug, rc::Rc};

use self::states::AccountState;

/**
 * Dovete implementare un ContoBancario che ha come informazioni
 * - nome del cliente (una stringa)
 * - ammontare del saldo
 * - limite inferiore
 * - limite superiore
 * - interesse
 *
 * Le operazioni che si vogliono fare sono:
 * - deposita che aggiunge al saldo la quantità depositata
 * - preleva che rimuove dal saldo la quantità prelevata
 * - paga gli interessi che aggiunge al saldo il saldo per l'interesse
 *
 * Queste operazioni hanno comportamenti diversi a seconda dello stato del conto, che può essere
 * - Rosso (se il saldo e' minore del limite inferiore)
 * - Argento (se il saldo e' minore del limite superiore e maggiore di quello inferiore)
 * - Oro (se il saldo e' maggiore del limite superiore)
 *
 * In particolare:
 * - prelevare da un conto Rosso non fa niente e cosi' pure paga interessi
 * - paga interessi in un conto Argento non fa niente
 *
 *
 * L'implementazione deve essere fatta usando il Pattern State.
 * Dovete implementare un crate lib (quindi senza main)
 * Mettere i test nella directory tests
 */
#[derive(Debug)]
pub struct BankAccount {
    name: String,
    balance: f32,
    limit_hi: f32,
    limit_lo: f32,
    interest: f32,
    state: Rc<dyn AccountState>,
}

impl BankAccount {
    pub fn new(name: String, limit_lo: f32, limit_hi: f32, interest: f32) -> Self {
        assert!(
            limit_hi >= 0.0 || limit_lo >= 0.0,
            "The limits must have a positive value!"
        );
        assert!(
            interest >= 0.0 && interest <= 1.0,
            "The interest must have a value between 0 and 1!"
        );

        Self {
            name,
            balance: 0.0,
            limit_hi,
            limit_lo,
            interest,
            state: Rc::new(states::Red),
        }
    }

    pub fn balance(&self) -> f32 {
        self.balance
    }

    pub fn state(&self) -> String {
        format!("{:?}", self.state.as_ref())
    }

    pub fn deposit(&mut self, amount: f32) {
        if amount > 0.0 {
            self.balance += amount;
            self.set_state();
        }
    }

    pub fn withdraw(&mut self, amount: f32) {
        if amount > 0.0 {
            let state = Rc::clone(&mut self.state);
            state.withdraw(self, amount);
            self.set_state();
        }
    }

    pub fn pay_interest(&mut self) {
        let state = Rc::clone(&mut self.state);
        state.pay_interest(self);
        self.set_state();
    }

    fn set_state(&mut self) {
        self.state = if self.balance < self.limit_lo {
            Rc::new(states::Red)
        } else if self.balance > self.limit_hi {
            Rc::new(states::Gold)
        } else {
            Rc::new(states::Silver)
        };
    }
}

/*********************************************
 * Trait for States and implementations of it
 */
pub mod states {
    use std::fmt::Debug;

    use super::BankAccount;

    pub trait AccountState: Debug {
        fn withdraw(&self, account: &mut BankAccount, amount: f32);
        fn pay_interest(&self, account: &mut BankAccount);
    }

    #[derive(Debug)]
    pub struct Red;
    impl AccountState for Red {
        fn withdraw(&self, _account: &mut BankAccount, _amount: f32) {}
        fn pay_interest(&self, _account: &mut BankAccount) {}
    }

    #[derive(Debug)]
    pub struct Silver;
    impl AccountState for Silver {
        fn withdraw(&self, account: &mut BankAccount, amount: f32) {
            account.balance -= amount;
        }
        fn pay_interest(&self, _account: &mut BankAccount) {}
    }

    #[derive(Debug)]
    pub struct Gold;
    impl AccountState for Gold {
        fn withdraw(&self, account: &mut BankAccount, amount: f32) {
            account.balance -= amount;
        }
        fn pay_interest(&self, account: &mut BankAccount) {
            account.balance -= account.balance * account.interest;
        }
    }
}
