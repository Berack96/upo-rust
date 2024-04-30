#![allow(dead_code)]

use std::collections::HashMap;

/** Es.1
 * Scrivere una funzione che prende in input due riferimenti a stringhe e ritorna
 * true se le stringhe con anagramma una dell’altra e false altrimenti.
 */
pub fn anagrammi(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        false
    } else {
        let mut map = HashMap::new();

        str1.chars().for_each(|x| *map.entry(x).or_insert(0) += 1);
        str2.chars().for_each(|x| *map.entry(x).or_insert(0) -= 1);
        map.iter().all(|(_, x)| *x == 0)
    }
}
