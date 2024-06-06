# upo-rust
### Repository per gli esercizi di Rust
Qui si possono trovare tutti gil esercizi assegnati.\
In particolare ogni esercizio ha una parte di codice per la soluzione e una parte di test.\
Per far partire tutti i test bisogna usare il comando:
```console
$ cargo test --workspace
```
Questo perchè alcuni test sono in packages interni e quindi non verrebbero fatti partire nel caso in cui si esegua solamente il comando cargo test.\
Nel caso si voglia far partire l'unico possibile eseguibile (ovvero il gioco dell'esercizio 3) utilizzare il comando:
```console
$ cargo run -p rogue_lib
```

### Esercizi
- [Esercizio 1: Anagrammi](https://github.com/Berack96/upo-rust/blob/main/src/es01_anagram.rs)\
In questo esercizio bisogna controllare se due stringhe sono l'una l'anagramma dell'altra.
- [Esercizio 2 Razionali](https://github.com/Berack96/upo-rust/blob/main/src/es02_rational.rs)\
In questo esercizio bisogna creare un'implementazione dei numeri razionali.
- [Esercizio 3: Rogue](https://github.com/Berack96/upo-rust/blob/main/rogue_lib/src/lib.rs)\
In questo esercizio è stato implementato un gioco ispirato a Rogue. Ho seguito il testo dell'esercizio ma ho anche implementato altre funzionalità utilizzando anche dei Traits in modo da poter controllare i comportamenti delle Entità, l'interfaccia del giocatore ed eventuali effetti che si possono trovare nei vari piani del gioco.\
Il gioco si trova in un package interno ed ha un main semplice che lo esegue.\
Il lib espone le varie interfacce e implementa una versione del gioco tramite console.
- [Esercizio 4: Razionali con Trait](https://github.com/Berack96/upo-rust/blob/main/src/es04_rational_traits.rs)\
In questo esercizio sono stati implementati i trait Add e Mul per i razionali (Esercizio 2).
- [Esercizio 5: Banca](https://github.com/Berack96/upo-rust/blob/main/src/es05_bank.rs)\
In questo esercizio è stata implementato un account di una banca che può eseguire delle operazioni in base allo stato in cui si trova il suo conto.
- [Esercizio 6: Lista (con Copy)](https://github.com/Berack96/upo-rust/blob/main/src/es06_list.rs)\
In questo esercizio è stata implementata una lista doppiamente linkata che accetta un elemento che implementa il trait Copy.
- [Esercizio 7: Lista Generica](https://github.com/Berack96/upo-rust/blob/main/src/es07_list_generic.rs)\
In questo esercizio è stata implementata una lista doppiamente linkata che accetta un qualsiasi elemento.
- [Esercizio 8: Folds](https://github.com/Berack96/upo-rust/blob/main/src/es08_folds.rs)\
In questo esercizio sono state implementate due funzioni che permettono la conta di quante vocali si trovano in una frase.
- [Esercizio 9: Asta](https://github.com/Berack96/upo-rust/blob/main/src/es09_auction.rs)\
In questo esercizio è stata implementata un'asta in cui il banditore, data una lista di prodotti e dati dei partecipanti (ognuno avente il proprio thread), coordina la vendita dei prodotti tramite invio di messaggi.
