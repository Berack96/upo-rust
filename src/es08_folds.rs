#![allow(unused)]
/**
 * Scrivere due funzioni che contano il numero delle vocali presenti in una stringa
 * (sia che siano minuscole che maiuscole).
 * - Entrambe le funzione prendono in input una stringa ma una ritorna una tupla struttura e l'altra una struttura
 *      fn num_vocali_tuple(s:&String) -> TuplaVocali
 *      fn num_vocali_struct(s:&String) -> NumVocali
 * - Le definizioni delle strutture e il test  che le funzioni devono passare sono le seguenti
 *      struct NumVocali{ a: i32, e: i32, i: i32, o: i32, u: i32, }
 *      struct TuplaVocali(i32,i32,i32,i32,i32);
 *  fn testFolds(){
 *      let a=String::from("Ciao Paola come stai? Ok. Tu John come stai? Ok");
 *      assert_eq!(TuplaVocali(5, 2, 3, 7, 1),num_vocali_tuple(&a));
 *      assert_eq!(NumVocali{a:5,e:2,i:3,o:7,u:1},num_vocali_struct(&a));
 * }
 *
 * - Nell'implementare entrambe le funzioni DOVETE USARE il metodo fold di Iterator documentato qua:
 *      https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
 * - La prima funzione avra’ con accumulatore una TuplaVocali e la seconda una struttura NumVocali.
 */

#[derive(Debug, PartialEq, Default)]
pub struct NumVocali {
    a: i32,
    e: i32,
    i: i32,
    o: i32,
    u: i32,
}
impl NumVocali {
    pub fn new(a: i32, e: i32, i: i32, o: i32, u: i32) -> Self {
        Self { a, e, i, o, u }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct TuplaVocali(i32, i32, i32, i32, i32);
impl TuplaVocali {
    pub fn new(a: i32, e: i32, i: i32, o: i32, u: i32) -> Self {
        Self(a, e, i, o, u)
    }
}

pub fn num_vocali_tuple(s: &String) -> TuplaVocali {
    s.chars().fold(TuplaVocali::default(), |mut vocali, c| {
        match c.to_ascii_lowercase() {
            'a' => vocali.0 += 1,
            'e' => vocali.1 += 1,
            'i' => vocali.2 += 1,
            'o' => vocali.3 += 1,
            'u' => vocali.4 += 1,
            _ => (),
        };
        vocali
    })
}
pub fn num_vocali_struct(s: &String) -> NumVocali {
    s.chars().fold(NumVocali::default(), |mut vocali, c| {
        match c.to_ascii_lowercase() {
            'a' => vocali.a += 1,
            'e' => vocali.e += 1,
            'i' => vocali.i += 1,
            'o' => vocali.o += 1,
            'u' => vocali.u += 1,
            _ => (),
        };
        vocali
    })
}
