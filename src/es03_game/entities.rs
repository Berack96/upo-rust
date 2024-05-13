use super::{
    cell::Effect,
    floor::{FloorPtr, FloorView},
};
use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display, io::Write, mem};

/// Tupla nominata Position in modo che nel codice sia più chiaro a cosa serve.\
/// È molto più facile capire a colpo d'occhio Position rispetto a (usize, usize)\
/// I due valori sono la posizione sull'asse X e sull'asse Y\
/// Il punto (0,0) si trova in basso a sinista.
#[derive(PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct Position(pub usize, pub usize);

/// Indica la direzione dove una entità sta guardando.\
/// È possibile anche non guardare in nessuna direzione tramite None.
#[derive(PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    /// Inverte la direzione attuale. (es. dx -> sx)\
    /// Questo metodo modifica la direzione inplace.
    pub fn invert(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            _ => Direction::None,
        };
    }
    /// Calcola la nuova posizione in base a dove si stà guardando.\
    /// La posizione viene modificata come se si stesse avanzando di una
    /// unità di spazio.\
    /// Es. (0,0) Up -> aumento la y di uno (0,1)
    pub fn move_from(&self, pos: Position) -> Position {
        match *self {
            Direction::Up => Position(pos.0, pos.1 + 1),
            Direction::Down => Position(pos.0, pos.1 - 1),
            Direction::Right => Position(pos.0 + 1, pos.1),
            Direction::Left => Position(pos.0 - 1, pos.1),
            Direction::None => pos,
        }
    }
    /// Restituisce una direzione casuale a partire da un generatore.\
    /// La direzione viene generata con una distribuzione uniforme, ovvero non
    /// c'è una direzione preferita o con più probabilità.
    pub fn random(rng: &mut Pcg32) -> Self {
        match rng.gen_range(0..5) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::None,
        }
    }
    /// Restituisce la rappresentazione della direzione in formato char, in questo modo
    /// può essere utilizzata per vedere il valore e mostrarlo a terminale.
    pub fn as_char(&self) -> char {
        match self {
            Self::Up => '▲',
            Self::Down => '▼',
            Self::Left => '◄',
            Self::Right => '►',
            Self::None => '■',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

/// Rappresenta una entità all'interno del dungeon.
#[derive(Clone, Deserialize, Serialize)]
pub struct Entity {
    name: String,
    effects: VecDeque<Box<dyn Effect>>,
    behavior: Box<dyn Behavior>,
    floor: FloorPtr,
    pub buffer: Action,
    pub position: Position,
    pub direction: Direction,
    health_max: i32,
    health: i32,
    attack: i32,
}

impl Entity {
    /// Costruttore che crea una nuova entità a partire dal suo nome, vita, danno di attacco,
    /// il decisore che permette di muoversi (giocatore o IA) e il piano in cui si trova.\
    /// La posizione sarà all'entrata del piano (o in una cella vicina nel caso ci siano altre entità sopra),
    /// non avrà effetti, azioni o una direzione in particolare.
    pub fn new(
        name: String,
        health: i32,
        attack: i32,
        behavior: Box<dyn Behavior>,
        mut floor: FloorPtr,
    ) -> Self {
        let position = floor.get().get_entrance();
        Self {
            name,
            floor,
            behavior,
            position,
            attack,
            health,
            health_max: health,
            buffer: Action::DoNothing,
            effects: VecDeque::new(),
            direction: Direction::None,
        }
    }

    /// Aggiunge l'effetto passato in input all'entità.\
    /// Questo non viene calcolato immediatamente, ma solo quando si chiama la
    /// funzione update.\
    /// È stato fatto in questo modo dato che ci possono essere effetti che ne aggiungono altri
    /// e quindi si farebbe una ricorsione infinita.
    pub fn add_effect(&mut self, effect: Box<dyn Effect>) {
        self.effects.push_back(effect);
    }

    /// Indica se l'entità è considerata ancora in gioco o meno.\
    /// Per far si che l'entità non sia più in gioco bisobna far arrivare la vita a 0.
    /// Nota: una entità con vita negativa è considerata "viva"
    pub fn is_alive(&self) -> bool {
        self.health != 0
    }

    /// Restituisce il valore della vita dell'entità.\
    pub fn get_health(&self) -> i32 {
        self.health
    }

    /// Applica il valore inserito come danno alla vita.\
    /// Nel caso in cui il danno sia negativo allora verrà interpretato come cura.\
    /// Nel caso in cui la vita sia negativa la logica sarà inversa.\
    /// Il danno/cura non potrà comunque superare lo 0 o la vita massima.
    pub fn apply_damage(&mut self, damage: i32) {
        let health = self.health - damage;
        self.health = if self.health_max > 0 {
            health.min(self.health_max).max(0)
        } else {
            health.max(self.health_max).min(0)
        };
    }

    /// Restituisce il piano in cui si trova l'entità in questo momento.
    pub fn get_floor(&self) -> FloorPtr {
        self.floor.clone()
    }

    /// Modifica il piano dell'entità e la mette all'entrata di quello nuovo.
    pub fn set_floor(&mut self, floor: FloorPtr) {
        self.floor = floor;
        self.position = self.floor.get().get_entrance();
    }

    /// Permette all'entità di fare un'azione e successivamente calcola
    /// tutti gli effetti che devono essere applicati ad essa.\
    /// Nel caso in cui l'entità non sia più in vita questo metodo ritornerà false
    /// e non permetterà all'entità di fare un update.\
    /// Nel caso in cui l'entità non riesca a fare l'update viene ritornato false.\
    /// Cio significa che l'entità verrà rimossa dal gioco.
    pub fn update(&mut self) -> bool {
        if self.is_alive() && matches!(self.compute_action(), Some(_)) {
            self.compute_effects();
            return true;
        }
        false
    }

    /// Permette all'entità di mostrare il piano in cui si trova e di fare una mossa.\
    /// Ha la stessa funzionalità di update() ma prima mostra il piano dell'entità.\
    /// Il piano viene mostrato tramite il behavior dell'entità.
    pub fn update_display(&mut self, floor: FloorView) -> bool {
        self.behavior.update(floor);
        self.update()
    }

    /// calcola gli effetti e li applica all'entità.
    fn compute_effects(&mut self) {
        let total = self.effects.len(); // len could change
        for _ in 0..total {
            if let Some(effect) = self.effects.pop_front() {
                effect.apply_to(self);
            }
        }
    }
    /// prende una decisione e applica l'azione da fare
    /// L'azione compiuta viene restituita, altrimenti None
    fn compute_action(&mut self) -> Option<Action> {
        let action = self.behavior.get_next_action()?;
        let action = match self.buffer {
            Action::DoNothing => action,
            _ => mem::replace(&mut self.buffer, Action::DoNothing),
        };

        let result = Some(action.clone());
        action.apply(self);
        result
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let times = 20;
        let health_bar = (self.health * times) / self.health_max;

        let filled = "█".repeat(health_bar as usize);
        let empty = " ".repeat((times - health_bar) as usize);
        let health_bar = format!("[{}{}]", filled, empty);

        write!(
            f,
            "{}: {} {}{:4}/{:4}",
            self.name, self.direction, health_bar, self.health, self.health_max
        )
    }
}

/// Azione che una qualsiasi entità può fare.
/// L'azione DoNothing permette all'entità di saltare il turno nel caso in cui sia utile.
#[derive(Clone, Deserialize, Serialize)]
pub enum Action {
    Move(Direction),
    //Attack(Direction),
    DoNothing,
}

impl Action {
    /// Applica l'azione all'entità passata.\
    /// Dopo la chiamata di questa funzione l'azione non sarà più disponibile.\
    /// Per ogni tipo di azione l'entità viene modificata opportunamente.\
    /// \
    /// Es. Move(Up) sposterà l'entità da una posizione (x,y) -> (x,y+1)\
    /// e applicherà qualunque effetto che si trovi sulla cella di destinazione
    pub fn apply(self, entity: &mut Entity) {
        match self {
            Action::DoNothing => {}
            Action::Move(direction) => {
                let pos = direction.move_from(entity.position);
                entity.direction = direction;
                entity.position = pos;

                let mut floor = entity.floor.clone();
                let mut floor = floor.get();
                let cell = floor.get_cell(pos);
                cell.entity_over(entity);
            }
        }
    }
}

/// Questo trait è molto importante per le entità perchè è responsabile del loro comportamento.\
/// Con questo trait si possono creare diversi comportamenti semplicemente implementandolo
/// e utilizzandolo come parametro nella generazione di una entità.\
/// \
/// Il trait è taggato con typetag in modo che possa essere utilizzato
/// nella serializzazione e deserializzazione di serde.
/// Esso permette di trasformare le implementazioni di questo trait in una
/// spiecie di Enum senza il bisogno di farlo manualmente.\
/// Quello che viene richiesto è che, nell'implementazione di una
/// struttura concreta di questo trait, venga messo sopra impl X for Behavior:\
/// #\[typetag::serde\]\
/// \
/// In questo modo si possono creare molteplici comoprtamenti che implementano
/// questo trait senza il bisogno di avere un Enum con essi
#[typetag::serde(tag = "type")]
pub trait Behavior: DynClone {
    /// In questo metodo viene passata una struttura che contiene una rappresentazione del
    /// piano semplice, avente solo delle informazioni parziali.\
    /// Questo serve a mostrare eventualmente delle possibili informazioni all'utente
    /// o di registrare dei valori per l'algoritmo di generazione delle azioni.
    fn update(&self, floor: FloorView);
    /// Genera una azione che poi verrà usata per l'entità associata.\
    /// L'azione può essere generata in qualunque modo: casuale, sempre la stessa,
    /// tramite interazione con console, o tramite una connessione ad un client.\
    /// \
    /// Nel caso in cui venga restituito None come valore, l'entità verrà rimossa dal gioco.\
    /// Questo viene fatto in modo che si possa avere una possibilità di rimozione del giocatore,
    /// ma anche una possibilità che alcune entità rare possano sparire.
    fn get_next_action(&self) -> Option<Action>;
}
clone_trait_object!(Behavior);

/// Semplice implementazione di un possibile comportamento di una entità.\
/// In questo caso l'entità resterà immobile nel punto in cui si trova per sempre.
#[derive(Clone, Serialize, Deserialize)]
pub struct Immovable;
#[typetag::serde]
impl Behavior for Immovable {
    fn update(&self, _floor: FloorView) {}
    fn get_next_action(&self) -> Option<Action> {
        Some(Action::DoNothing)
    }
}

/// Semplice implementazione di una possibile interfaccia console.
#[derive(Clone, Serialize, Deserialize)]
pub struct ConsoleInput;
#[typetag::serde]
impl Behavior for ConsoleInput {
    fn update(&self, floor: FloorView) {
        let mut term = console::Term::stdout();
        let _ = term.clear_screen();
        let _ = term.write_fmt(format_args!("{}\n", floor));
    }
    fn get_next_action(&self) -> Option<Action> {
        let mut term = console::Term::stdout();
        let _ = term.write("Insert your action [wasd or enter for nothing]: ".as_bytes());

        loop {
            if let Ok(ch) = term.read_char() {
                match ch {
                    '\n' => return Some(Action::DoNothing),
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
