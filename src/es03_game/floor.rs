use super::{
    cell::Cell,
    entities::{Entity, Position},
};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

/// Indica un piano del dungeon, in essa si possono trovare le celle in cui si
/// cammina e le entità che abitano il piano.\
/// Per poter accedere a questa struttura è necessario utilizzare FloorPtr e fare get()
#[derive(Clone, Deserialize, Serialize)]
pub struct Floor {
    level: usize,
    grid: Vec<Vec<Cell>>,
    players: VecDeque<Entity>,
    entities: VecDeque<Entity>,
    rng: Pcg32,
}

impl Floor {
    /// Crea un nuovo piano al livello indicato.\
    /// Il piano viene creato a partire dai parametri passati in input, che sono tutte cose necessarie ad esso.
    pub fn new(level: usize, rng: Pcg32, entities: Vec<Entity>, grid: Vec<Vec<Cell>>) -> Self {
        Self {
            level,
            rng,
            players: VecDeque::new(),
            entities: VecDeque::from(entities),
            grid,
        }
    }

    /// Aggiunge un giocatore al piano e lo inserisce all'entrata.
    pub fn add_player(&mut self, mut player: Entity) {
        // todo!() check collision with other entities
        player.position = self.get_entrance();
        self.players.push_back(player);
    }

    /// Indica se il piano ha almeno un giocatore in vita o meno
    pub fn has_players(&self) -> bool {
        self.players.iter().any(|player| player.is_alive())
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

    /// Fa l'update di tutti i giocatori e rimuove eventualmente quelli non più in vita, restituendoli dentro un vec
    pub fn update_players(&mut self) -> Vec<Entity> {
        let mut remove = vec![];
        for _ in 0..self.players.len() {
            let mut player = self.players.pop_front().unwrap();
            if player.update(self) {
                self.players.push_back(player);
            } else {
                remove.push(player);
            }
        }
        remove
    }

    /// Fa l'update di tutte le entità e rimuove eventualmente quelle non più in vita
    pub fn update_entities(&mut self) {
        for _ in 0..self.entities.len() {
            let mut entity = self.entities.pop_front().unwrap();
            if entity.update(self) {
                self.entities.push_back(entity);
            }
        }
    }

    /// Crea una view del piano.\
    pub fn get_limited_view_floor<'a>(&'a self, entity: &'a Entity) -> FloorView<'a> {
        FloorView::new(self, entity)
    }
}

/// Struttura di mezzo tra un piano e il gioco vero e proprio.\
/// Utilizzata per la comunicazione con le entità per poter aggiornare quello che vedono.\
/// Infatti internamente ha solo alcuni pezzi del gioco per non far mostrare tutto.\
pub struct FloorView<'a> {
    pub entity: &'a Entity,
    pub floor: &'a Floor,
}

impl<'a> FloorView<'a> {
    /// Crea una vista del gioco corrente secondo la visione dell'entità passata in intput.\
    /// Il SimpleFloor risultante avrà il piano, entità, livello e giocatori che si trovano
    /// in questo momento sul piano dell'entità passata in input.
    pub fn new(floor: &'a Floor, entity: &'a Entity) -> Self {
        Self {
            entity: &entity,
            floor: &floor,
        }
    }

    /// Rappresentazione del piano come matrice di char
    pub fn as_char_grid(&self) -> Vec<Vec<char>> {
        let grid = &self.floor.grid;
        let size = grid.len();
        let mut grid: Vec<Vec<char>> = (0..size)
            .map(|y| {
                let row = (0..size).map(|x| Some(&grid[x][y]));
                let mut row: Vec<_> = row
                    .clone()
                    .zip(row.skip(1).chain(std::iter::once(None)))
                    .flat_map(Self::increase_x_dimension)
                    .collect();
                row.push('\n');
                row
            })
            .collect();

        let pos = &self.entity.position;
        grid[pos.1][pos.0 * 2] = self.entity.direction.as_char();
        grid
    }

    fn increase_x_dimension(tuple: (Option<&Cell>, Option<&Cell>)) -> Vec<char> {
        let (a, b) = tuple;
        let a = a.unwrap();
        if let Some(b) = b {
            let one_is_wall = matches!(b, Cell::Wall) || matches!(a, Cell::Wall);
            let c = if one_is_wall { Cell::Wall } else { Cell::Empty };
            vec![a.as_char(), c.as_char()]
        } else {
            vec![a.as_char()]
        }
    }
}

impl<'a> Display for FloorView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid: String = self
            .as_char_grid()
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect();

        write!(f, "{}\n{}", grid, self.entity)
    }
}
