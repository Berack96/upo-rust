use super::{
    cell::{Cell, Effect},
    floor::{Floor, FloorView},
};
use dyn_clone::{clone_trait_object, DynClone};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display, mem};

/// Tupla nominata Position in modo che nel codice sia più chiaro a cosa serve.\
/// È molto più facile capire a colpo d'occhio Position rispetto a (usize, usize)\
/// I due valori sono la posizione sull'asse X e sull'asse Y\
/// Il punto (0,0) si trova in basso a sinista.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Position(pub usize, pub usize);

/// Indica la direzione dove una entità sta guardando.\
/// È possibile anche non guardare in nessuna direzione tramite None.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
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
    /// Calcola e modifica la posizione in base a dove si stà guardando.\
    /// Il valore ritornato sarà la posizione modificata che è stata passata in input.\
    /// La posizione viene modificata come se si stesse avanzando di una
    /// unità di spazio.\
    /// Es. (0,0) Up -> aumento la y di uno (0,1)
    pub fn move_from<'a>(&self, pos: &'a mut Position) -> &'a mut Position {
        match *self {
            Direction::Up => pos.1 += 1,
            Direction::Down => pos.1 -= if pos.1 == 0 { 0 } else { 1 },
            Direction::Right => pos.0 += 1,
            Direction::Left => pos.0 -= if pos.0 == 0 { 0 } else { 1 },
            Direction::None => (),
        };
        pos
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entity {
    name: String,
    effects: VecDeque<Box<dyn Effect>>,
    behavior: Option<Box<dyn Behavior>>,
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
    pub fn new(name: String, health: i32, attack: i32, behavior: Box<dyn Behavior>) -> Self {
        Self {
            name,
            behavior: Some(behavior),
            position: Position(0, 0),
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

    /// Restituisce il valore della vita massima dell'entità.\
    pub fn get_health_max(&self) -> i32 {
        self.health_max
    }

    /// Restituisce il valore del nome dell'entità.\
    pub fn get_name(&self) -> &String {
        &self.name
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

    /// Permette all'entità di mostrare il piano in cui si trova e di fare una mossa.\
    /// Il piano viene mostrato tramite il behavior dell'entità e successivamente viene chiesto di fare un'azione.\
    /// Dopodichè vengono calcolati tutti gli effetti che devono essere applicati all'entità.\
    /// Nel caso in cui l'entità non sia più in vita questo metodo ritornerà None
    /// e l' entità smetterà di esistere.\
    /// Nel caso in cui l'entità non riesca a fare l'update viene ritornato None.\
    /// Cio significa che l'entità verrà rimossa dal gioco.
    pub fn update(mut self, floor: &mut Floor) -> Option<Self> {
        let mut behavior = mem::take(&mut self.behavior).unwrap();

        if !self.is_alive() {
            return self.die(behavior, floor);
        }

        behavior.update(floor.get_limited_view_floor(&self));
        let action = self.compute_action(&mut behavior, floor);
        if action.is_none() {
            return None;
        }

        if !self.is_alive() {
            return self.die(behavior, floor);
        }

        self.compute_effects(floor);
        if !self.is_alive() {
            return self.die(behavior, floor);
        }

        self.behavior = Some(behavior);
        Some(self)
    }

    /// metodo usato per la rimozione dell' entità e del suo behavior
    fn die(self, mut behavior: Box<dyn Behavior>, floor: &Floor) -> Option<Self> {
        let view = floor.get_limited_view_floor(&self);
        behavior.on_death(view);
        None
    }

    /// calcola gli effetti e li applica all'entità.
    fn compute_effects(&mut self, floor: &mut Floor) {
        let total = self.effects.len(); // len could change
        for _ in 0..total {
            if let Some(effect) = self.effects.pop_front() {
                effect.apply_to(self, floor);
            }
        }
    }
    /// prende una decisione e applica l'azione da fare
    /// L'azione compiuta viene restituita, altrimenti None
    fn compute_action(&mut self, behavior: &mut Box<dyn Behavior>, floor: &mut Floor) -> Option<Action> {
        let action = behavior.get_next_action()?;
        let action = match self.buffer {
            Action::DoNothing => action,
            _ => mem::replace(&mut self.buffer, Action::DoNothing),
        };

        let result = Some(action.clone());
        action.apply(self, floor);
        result
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let times = 20;
        let health_bar = (self.health * times) / self.health_max;

        let filled = "■".repeat(health_bar as usize);
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
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub enum Action {
    Move(Direction),
    //Attack(Direction),
    #[default]
    DoNothing,
}

impl Action {
    /// Applica l'azione all'entità passata.\
    /// Dopo la chiamata di questa funzione l'azione non sarà più disponibile.\
    /// Per ogni tipo di azione l'entità viene modificata opportunamente.\
    /// \
    /// Es. Move(Up) sposterà l'entità da una posizione (x,y) -> (x,y+1)\
    /// e applicherà qualunque effetto che si trovi sulla cella di destinazione
    pub fn apply(self, entity: &mut Entity, floor: &mut Floor) {
        match self {
            Action::DoNothing => {}
            Action::Move(direction) => {
                direction.move_from(&mut entity.position);
                entity.direction = direction;

                let cell = floor.get_cell_mut(&entity.position);
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
pub trait Behavior: DynClone + core::fmt::Debug {
    /// In questo metodo viene passata una struttura che contiene una rappresentazione del
    /// piano semplice, avente solo delle informazioni parziali.\
    /// Questo serve a mostrare eventualmente delle possibili informazioni all'utente
    /// o di registrare dei valori per l'algoritmo di generazione delle azioni.\
    /// Non è necessario implementarla.
    fn update(&mut self, _view: FloorView) {}
    /// Funzione che viene richiamata quando l'entità muore.\
    /// I parametri servono a far vedere un'ultima volta i dati del piano corrente all'entità
    /// in modo che possa eventualmente fare ulteriori calcoli.\
    /// Non è necessario implementarla.
    fn on_death(&mut self, _view: FloorView) {}
    /// Genera una azione che poi verrà usata per l'entità associata.\
    /// L'azione può essere generata in qualunque modo: casuale, sempre la stessa,
    /// tramite interazione con console, o tramite una connessione ad un client.\
    /// \
    /// Nel caso in cui venga restituito None come valore, l'entità verrà rimossa dal gioco.\
    /// Questo viene fatto in modo che si possa avere una possibilità di rimozione del giocatore,
    /// ma anche una possibilità che alcune entità rare possano sparire.
    fn get_next_action(&mut self) -> Option<Action>;
}
clone_trait_object!(Behavior);

/// Semplice implementazione di un possibile comportamento di una entità.\
/// In questo caso l'entità resterà immobile nel punto in cui si trova per sempre.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Immovable;
#[typetag::serde]
impl Behavior for Immovable {
    fn get_next_action(&mut self) -> Option<Action> {
        Some(Action::DoNothing)
    }
}

/// Semplice implementazione di un possibile comportamento di una entità.\
/// In questo caso l'entità si mouverà in maniera casuale evitando le caselle speciali.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomMovement {
    action: Action,
    rng: Pcg32,
}
impl RandomMovement {
    pub fn new() -> Self {
        Self {
            action: Action::DoNothing,
            rng: Pcg32::seed_from_u64(0),
        }
    }
}
#[typetag::serde]
impl Behavior for RandomMovement {
    fn update(&mut self, view: FloorView) {
        let dir = Direction::random(&mut self.rng);
        let mut pos = view.entity.position.clone();
        dir.move_from(&mut pos);
        if let Cell::Empty = view.floor.get_cell(&pos) {
            self.action = Action::Move(dir);
        }
    }
    fn get_next_action(&mut self) -> Option<Action> {
        Some(mem::take(&mut self.action))
    }
}
