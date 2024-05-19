use self::{
    cell::Cell,
    config::Config,
    entities::{Action, Behavior, Direction},
    floor::FloorView,
    game::Dungeon,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

pub mod cell;
pub mod config;
pub mod entities;
pub mod floor;
pub mod game;
pub mod generator;

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
pub fn run_console(player: String) {
    let mut config = Config::default();
    config.game_seed = rand::random();

    let mut game = Dungeon::new_with(config);
    game.add_player(player, Box::new(ConsoleInput));

    while game.has_players() {
        game.compute_turn();
    }
}

/// Implementazione di una possibile interfaccia console.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleInput;
impl ConsoleInput {
    fn floor_as_string(floor: &FloorView) -> String {
        let view = 5;
        let size = (2 * view) * 3;
        let iter = floor.get_grid(view).flat_map(|iter| {
            iter.flat_map(|view| {
                if let Some(e) = view.entity {
                    return [' ', e.direction.as_char(), ' '].into_iter();
                }

                let ch = view.cell.as_char();
                match view.cell {
                    Cell::Wall => [ch, ch, ch],
                    _ => [' ', ch, ' '],
                }
                .into_iter()
            })
        });

        FloorView::box_of(size, iter).collect()
    }
}
#[typetag::serde]
impl Behavior for ConsoleInput {
    fn update(&self, floor: FloorView) {
        let mut term = console::Term::stdout();
        let _ = term.clear_screen();
        let _ = term.write_fmt(format_args!(
            "{}{}\n",
            Self::floor_as_string(&floor),
            floor.entity
        ));
    }
    fn you_died(&self, floor: FloorView) {
        let mut term = console::Term::stdout();
        let _ = term.clear_screen();
        let _ = term.write_fmt(format_args!("{}\nYOU DIED!\n", floor));
    }
    fn get_next_action(&self) -> Option<Action> {
        let mut term = console::Term::stdout();
        let _ = term.write("Insert your action [wasd or space for nothing]: ".as_bytes());

        loop {
            if let Ok(ch) = term.read_char() {
                match ch {
                    ' ' => return Some(Action::DoNothing),
                    'w' => return Some(Action::Move(Direction::Up)),
                    'a' => return Some(Action::Move(Direction::Left)),
                    's' => return Some(Action::Move(Direction::Down)),
                    'd' => return Some(Action::Move(Direction::Right)),
                    _ => (),
                }
            }
        }
    }
}
