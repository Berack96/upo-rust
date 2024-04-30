#![allow(dead_code)]

/** Es.2
 * Si definisca un modulo razionali che contiene una implementazione dei numeri razionali,
 * cioe’ una struttura Razionali con i due campi interi num e denum (per numeratore e denominatore)
 * i metodi di somma, prodotto, e riduzione ai minimi termini (dividere num e denum per il massimo comun divisore)
 * le funzioni new che ritorna un Razionale (con 2 parametri) e int_to_raz che prende un intero
 * e ritorna il numero razionale che ha quell’intero come numeratore e 1 come denominatore.
 */

#[derive(Debug, PartialEq, Clone)]
pub struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        assert!(den != 0, "Cannot divide by zero!");

        let mut numero = Rational { num, den };
        numero.reduce();
        numero
    }
    pub fn from(int: i32) -> Self {
        Rational { num: int, den: 1 }
    }

    pub fn multiplication(&mut self, other: &Rational) -> &mut Self {
        self.num *= other.num;
        self.den *= other.den;
        self.reduce()
    }
    pub fn addition(&mut self, other: &Rational) -> &mut Self {
        self.num = (self.num * other.den) + (other.num * self.den);
        self.den *= other.den;
        self.reduce()
    }
    fn reduce(&mut self) -> &mut Self {
        if self.den < 0 {
            self.num *= -1;
            self.den *= -1;
        }

        let mcd = Self::mcd(self.num, self.den);
        self.num /= mcd;
        self.den /= mcd;
        self
    }
    fn mcd(a: i32, b: i32) -> i32 {
        // mcd Euclideo https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
        if b == 0 {
            a
        } else {
            Self::mcd(b, a % b)
        }
    }
}
