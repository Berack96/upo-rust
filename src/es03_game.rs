
/** Es.3
 * Implementare una libreria che permetta di realizzare il seguente gioco.
 * Il Campo di gioco e' una matrice n x n di celle le celle sui 4 lati sono dei muri e all'interno le celle possono essere
 * - vuote
 * - contenere cibo (un intero positivo)
 * - contenere un veleno (un intero positivo)
 *
 * Un Giocatore si muove in questa matrice iniziando da una posizione casuale. Il giocatore ha
 * - Direzione in cui si muove: Su, Giu', Destra, Sinistra
 * - Posizione nella matrice
 * - una forza (un intero positivo)
 *
 * Quando si muove avanza di una posizione nella direzione in cui il giocatore si muove. Una Configurazione e'
 * un campo di gioco, e un giocatore in una posizione del campo per questa struttura implementate il trait Display
 *
 * Il gioco inizia con una configurazione in cui nella matrice ci sono m caselle con cibo e m con veleno (in posizioni casuali), un giocatore in una cella libera e un numero massimo di mosse.
 * Ad ogni iterazione: Si lancia una moneta (Testa o Croce) se
 * - Testa il giocatore si muove di una posizione nella direzione in cui si sta muovendo
 * - altrimenti sceglie casualmente una dell 4 direzioni e fa un passo in quella direzione.
 *
 * Se la cella in cui si finisce
 * contiene cibo, si aggiunge la quantita' di cibo alla forza
 * contiene veleno, si decrementa la quantita' di veleno dalla forza
 * e' un muro il giocatore rimbalza, cioe' resta nella stessa posizione ma cambia la sua direzione nella direzione opposta.
 *
 * Il gioco finisce quando
 * - il giocatore finisce la forza (cioe' questa diventerebbe un valore <=0) e in questo caso PERDE
 * - raggiunge il numero massimo di mosse nel qual caso VINCE
 *
 * Per n, m, le quantità iniziali dei vari elementi (elemento, cibo, forza) e il numero massimo di mosse usate variabili  che possano essere inserite dall'utente.
 * Se volete potete anche cambiare le regole del gioco.
 * Mettere main e definizioni in files separati (le definizioni in uno o più files) e scrivete i test in una directory a parte.
 */

pub mod floor;
pub mod game;
pub mod generator;
pub mod cell;
pub mod config;
pub mod entities;
