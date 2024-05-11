use super::{
    cell::Cell,
    entities::{Entity, Position},
};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

/// Tupla creata per poter implementare qualche metodo sulla struttura Rc<RefCell<Floor>>\
/// In questo modo ho incapsulato i borrow e la creazione di questo oggetto per una
/// migliore lettura del codice (hopefully).
#[derive(Clone, Deserialize, Serialize)]
pub struct FloorPtr(Rc<RefCell<Floor>>);
impl FloorPtr {
    /// Crea un nuovo puntatore al piano indicato.\
    /// Il piano viene creato a partire dai parametri passati in input, che sono tutte cose
    /// necessarie ad esso.
    pub fn new(level: usize, rng: Pcg32, entities: Vec<Entity>, grid: Vec<Vec<Cell>>) -> Self {
        Self(Rc::new(RefCell::new(Floor::new(
            level, rng, entities, grid,
        ))))
    }

    /// Permette di prendere il valore puntato al piano.
    pub fn get(&mut self) -> RefMut<Floor> {
        self.0.borrow_mut()
    }
}

/// Indica un piano del dungeon, in essa si possono trovare le celle in cui si
/// cammina e le entità che abitano il piano.\
/// Per poter accedere a questa struttura è necessario utilizzare FloorPtr e fare get()
#[derive(Clone, Deserialize, Serialize)]
pub struct Floor {
    level: usize,
    grid: Vec<Vec<Cell>>,
    entities: Vec<Entity>,
    rng: Pcg32,
}

impl Floor {
    /// Crea un piano secondo i parametri indicati
    fn new(level: usize, rng: Pcg32, entities: Vec<Entity>, grid: Vec<Vec<Cell>>) -> Self {
        Self {
            level,
            grid,
            entities,
            rng,
        }
    }

    /// Restituisce il livello di profondità del piano
    pub fn get_level(&self) -> usize {
        self.level
    }

    /// Restituisce il generatore di numeri casuali utilizzato per qualunque cosa
    /// inerente al piano (generazione di entità, applicazione di effetti...)
    pub fn get_rng(&mut self) -> &mut Pcg32 {
        &mut self.rng
    }

    /// Restituisce la cella nella posizione indicata.\
    /// Con essa si può fare cio che si vuole, e quindi anche modificarla.
    pub fn get_cell(&mut self, pos: Position) -> &mut Cell {
        &mut self.grid[pos.0][pos.1]
    }

    /// Restituisce la posizione dell'entrata del piano.\
    /// Utile come spawn per quando i giocatori arrivano al piano.
    pub fn get_entrance(&mut self) -> Position {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(x, vec)| {
                vec.iter().enumerate().find_map(|(y, cell)| {
                    if let Cell::Entance = cell {
                        Some(Position(x, y))
                    } else {
                        None
                    }
                })
            })
            .expect("Entrance of the floor should be inside the grid!")
    }

    /// Fa l'update di tutte le entità e rimuove eventualmente quelle non più in vita
    pub fn update_entities(&mut self) {
        Entity::update_from_vec(&mut self.entities);
    }
}
