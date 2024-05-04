#![allow(dead_code)]

use super::es02_rational::Rational;
use std::ops::Add;
use std::ops::Mul;

/** Es.4
 * Per la struttura dei numeri razionali implementare i traits Add e Mul sia per fare
 * somma e moltiplicazione di numeri razionali che per fare somma e moltiplicazione di un numero razionale e un intero (i32).
 * Aggiungere ai test che avete fatto altri test per queste implementazioni.
 */
impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut temp = rhs.clone();
        temp.multiplication(&self);
        temp
    }
}

impl Mul<i32> for Rational {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        let mut temp = Rational::from(rhs);
        temp.multiplication(&self);
        temp
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut temp = rhs.clone();
        temp.addition(&self);
        temp
    }
}

impl Add<i32> for Rational {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        let mut temp = Rational::from(rhs);
        temp.addition(&self);
        temp
    }
}
