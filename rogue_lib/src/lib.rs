use self::{
    cell::Cell,
    config::Config,
    entities::{Action, Behavior, Direction, Entity},
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
pub fn run_console(player: String, seed: u64) {
    let mut config = Config::default();
    config.game_seed = seed;

    let mut game = Dungeon::new_with(config);
    game.add_player(player, Box::new(ConsoleInput));

    while game.has_players() {
        let _ = game.save("save.json");
        game.compute_turn();
    }
}

/// todo!() add docs
pub fn box_of(
    size: usize,
    title: String,
    iter: impl Iterator<Item = String>,
) -> impl Iterator<Item = String> {
    assert!(
        size >= title.len(),
        "Title must not exceed the size of the box!"
    );

    let len = (size - title.len()) / 2;
    let correction = if 2 * len + title.len() < size { 1 } else { 0 };

    std::iter::once("╔".to_string())
        .chain(std::iter::repeat("═".to_string()).take(len + 1))
        .chain(std::iter::once(title))
        .chain(std::iter::repeat("═".to_string()).take(len + 1 + correction))
        .chain(std::iter::once("╗\n".to_string()))
        .chain(iter.map(|string| {
            std::iter::once("║ ".to_string())
                .chain(std::iter::once(string))
                .chain(std::iter::once(" ║\n".to_string()))
                .collect()
        }))
        .chain(std::iter::once("╚".to_string()))
        .chain(std::iter::repeat("═".to_string()).take(size + 2))
        .chain(std::iter::once("╝\n".to_string()))
}

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_EFFECT: &str = "\x1b[95m";
const COLOR_ENEMY: &str = "\x1b[38;5;1m";
const COLOR_PLAYER: &str = "\x1b[38;5;166m";
const COLOR_PLAYER_HEALTH: &str = "\x1b[31m";

/// Implementazione di una possibile interfaccia console.\
/// Ha fin troppi metodi per far vedere in modo carino il gioco, ma comunque la parte importante
/// è l'implementazione del Behavior.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleInput;
impl ConsoleInput {
    /// todo!() add docs
    fn print_floor(&self, floor: FloorView, other: String) {
        let mut term = console::Term::stdout();
        let _ = term.clear_screen();
        let _ = term.write_fmt(format_args!(
            "{}{}\n{other}",
            Self::floor_as_string(&floor),
            Self::entity_as_string(floor.entity),
        ));
    }
    /// todo!() add docs
    fn entity_as_string(entity: &Entity) -> String {
        let times = 20;
        let health_bar = (entity.get_health() * times) / entity.get_health_max();

        let filled = "■".repeat(health_bar as usize);
        let empty = " ".repeat((times - health_bar) as usize);
        format!(
            "{}: [{COLOR_PLAYER_HEALTH}{filled}{empty}{COLOR_RESET}] {:4}/{:4}",
            entity.get_name(),
            entity.get_health(),
            entity.get_health_max()
        )
    }
    /// todo!() add docs
    fn floor_as_string(floor: &FloorView) -> String {
        let view = 5;
        let size = (2 * view) * 3;
        let iter = floor.get_grid(view).map(|iter| {
            iter.map(|view| {
                if let Some(e) = view.entity {
                    let color = if floor.entity.position == e.position {
                        COLOR_PLAYER
                    } else {
                        COLOR_ENEMY
                    };
                    return format!("{} {} {COLOR_RESET}", color, e.direction.as_char());
                }

                let cell = view.cell.as_char();
                match view.cell {
                    Cell::Special(_) => format!("{COLOR_EFFECT} {cell} {COLOR_RESET}"),
                    Cell::Wall => format!("{cell}{cell}{cell}"),
                    _ => format!(" {cell} "),
                }
            })
            .collect()
        });

        let title = format!(" Floor lv.{:2} ", floor.floor.get_level());
        box_of(size, title, iter).collect()
    }
}
#[typetag::serde]
impl Behavior for ConsoleInput {
    fn update(&mut self, floor: FloorView) {
        self.print_floor(floor, "".to_string());
    }
    fn on_death(&mut self, floor: FloorView) {
        self.print_floor(floor, "YOU DIED!".to_string());
    }
    fn get_next_action(&mut self, entity: &Entity) -> Option<Action> {
        let prompt = "Insert your action [? for help]: ";
        let mut term = console::Term::stdout();
        let _ = term.write(prompt.as_bytes());

        loop {
            if let Ok(ch) = term.read_char() {
                match ch {
                    ' ' => return Some(Action::Attack(entity.direction)),
                    'z' => return Some(Action::DoNothing),
                    'w' => return Some(Action::Move(Direction::Up)),
                    'a' => return Some(Action::Move(Direction::Left)),
                    's' => return Some(Action::Move(Direction::Down)),
                    'd' => return Some(Action::Move(Direction::Right)),
                    'q' => return None,
                    '?' => {
                        let _ = term.write_line("");
                        let _ = term.write_line("[wasd]  => for movement");
                        let _ = term.write_line("[space] => for attacking the enemy in front");
                        let _ = term.write_line("[z]     => for doing nothing");
                        let _ = term.write_line("[q]     => for exit the game");
                        let _ = term.write("Press ANY button to continue...".as_bytes());
                        let _ = term.read_char();
                        let _ = term.clear_line();
                        let _ = term.clear_last_lines(4); // this number is from the previous message (4 total lines of help)
                        let _ = term.move_cursor_up(1);
                        let _ = term.move_cursor_right(prompt.len());
                    }
                    _ => (),
                }
            }
        }
    }
}
