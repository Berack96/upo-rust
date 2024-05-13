use super::{
    cell::Cell,
    entities::{Entity, Position},
};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    fmt::Display,
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
        Self(Rc::new(RefCell::new(Floor {
            level,
            rng,
            players: vec![],
            entities,
            grid,
        })))
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
    players: Vec<Entity>,
    entities: Vec<Entity>,
    rng: Pcg32,
}

impl Floor {
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
        let to_remove: Vec<bool> = self
            .entities
            .iter_mut()
            .map(|entity| entity.update())
            .collect();
        let mut to_remove = to_remove.iter();
        self.entities.retain(|_| *to_remove.next().unwrap());
    }
}

/// Struttura di mezzo tra un piano e il gioco vero e proprio.\
/// Utilizzata per la comunicazione con le entità per poter aggiornare quello che vedono.\
/// Infatti internamente ha solo alcuni pezzi del gioco per non far mostrare tutto.\
pub struct FloorView {
    pub level: usize,
    pub entity: Entity,
    pub players: Vec<Entity>,
    pub entities: Vec<Entity>,
    pub grid: Vec<Vec<Cell>>,
}

impl FloorView {
    /// Crea una vista del gioco corrente secondo la visione dell entità passata in intput.\
    /// Il SimpleFloor risultante avrà il piano, entità, livello e giocatori che si trovano
    /// in questo momento sul piano dell'entità passata in input.
    pub fn new(entity: &Entity) -> Self {
        let mut floor = entity.get_floor();
        let floor = floor.get();

        let level = floor.level;
        let grid = floor.grid.clone();
        let entities: Vec<Entity> = floor
            .entities
            .iter()
            .filter_map(|e| (e.position != entity.position).then_some(e.clone()))
            .collect();
        let players: Vec<Entity> = floor
            .players
            .iter()
            .filter_map(|p| (p.position != entity.position).then_some(p.clone()))
            .collect();

        Self {
            level,
            entity: entity.clone(),
            players,
            entities,
            grid,
        }
    }

    /// Rappresentazione del piano come matrice di char
    pub fn as_char_grid(&self) -> Vec<Vec<char>> {
        let size = self.grid.len();
        let mut grid: Vec<Vec<char>> = (0..size)
            .map(|y| {
                let row = (0..size).map(|x| Some(&self.grid[x][y]));
                let mut row: Vec<_> = row
                    .clone()
                    .zip(row.skip(1).chain(std::iter::once(None)))
                    .flat_map(|(a, b)| {
                        let a = a.unwrap();
                        if let Some(b) = b {
                            let one_is_wall = matches!(b, Cell::Wall) || matches!(a, Cell::Wall);
                            let c = if one_is_wall { Cell::Wall } else { Cell::Empty };
                            vec![a.as_char(), c.as_char()]
                        } else {
                            vec![a.as_char()]
                        }
                    })
                    .collect();
                row.push('\n');
                row
            })
            .collect();

        let pos = &self.entity.position;
        grid[pos.1][pos.0 * 2] = self.entity.direction.as_char();
        grid
    }
}

impl Display for FloorView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid: String = self
            .as_char_grid()
            .iter()
            .rev()
            .map(|row| {
                let a: String = row.iter().collect();
                a
            })
            .collect();

        write!(f, "{}\n{}", grid, self.entity)
    }
}
