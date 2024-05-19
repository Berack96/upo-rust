use super::{
    cell::Cell,
    entities::{Entity, Position},
};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

/// Indica un piano del dungeon, in essa si possono trovare le celle in cui si
/// cammina e le entità che abitano il piano.\
/// Per poter accedere a questa struttura è necessario utilizzare FloorPtr e fare get()
#[derive(Clone, Debug, Deserialize, Serialize)]
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

    /// Restituisce la grandezza di un lato del piano.\
    /// Per avere la quantità di celle basterà prendere il valore ed elevarlo a 2.
    pub fn get_size(&self) -> usize {
        self.grid.len()
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
    /// Con essa si può fare cio che si vuole, e quindi anche modificarla.\
    /// Nel caso in cui la posizione non sia all'interno del piano, essa viene modificata
    /// facendola rientrare nei limiti di esso.\
    /// Es. pos(2,3) ma il piano è di max 2 allora diventa -> pos(2,2)
    pub fn get_cell(&mut self, pos: &Position) -> &mut Cell {
        let len = self.grid.len() - 1;
        let x = pos.0.min(len);
        let y = pos.1.min(len);
        &mut self.grid[x][y]
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
        let mut next_floor = vec![];
        for _ in 0..self.players.len() {
            let mut player = self.players.pop_front().unwrap();
            if player.update(self) {
                self.players.push_back(player);
            } else {
                next_floor.push(player);
            }
        }
        next_floor
    }

    /// Ritorna un eventuale giocatore che si trova sopra la cella di uscita del piano.\
    /// Nel caso in cui non ci siano giocatori sopra, questo metodo ritornerà None.
    pub fn get_player_at_exit(&mut self) -> Option<Entity> {
        let index = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(i, player)| {
                let pos = &player.position;
                match &self.grid[pos.0][pos.1] {
                    Cell::Exit => Some(i),
                    _ => None,
                }
            })
            .next();

        if let Some(i) = index {
            self.players.remove(i)
        } else {
            None
        }
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

    /// Crea una view del piano con l'entità partecipante all'update.
    pub fn get_limited_view_floor<'a>(&'a self, entity: &'a Entity) -> FloorView<'a> {
        FloorView::new(self, entity)
    }

    /// Ritorna un iteratore a tutte le entità del piano.\
    /// Le entità del piano si dividono in giocatori e entità, e questo iteratore le ritorna tutte,
    /// passando prima dai giocatori e poi da tutto il resto.
    pub fn get_all_entities<'a>(&'a self) -> impl Iterator<Item = &Entity> + 'a {
        self.players.iter().chain(self.entities.iter())
    }

    /// Controlla che nella posizione indicata non ci siano altre entità e restituisce il numero di collisioni trovate.\
    /// Questo metodo controlla TUTTE le entità e i giocatori del piano, quindi si svolge in O(n)
    fn collisions(&self, pos: &Position) -> usize {
        self.get_all_entities()
            .filter(|entity| entity.position == *pos)
            .fold(0, |count, _| count + 1)
    }
}

/// Struttura di mezzo tra un piano e il gioco vero e proprio.\
/// Utilizzata per la comunicazione con le entità per poter aggiornare quello che vedono.\
/// Infatti internamente ha solo alcuni pezzi del gioco per non far mostrare tutto.\
pub struct FloorView<'a> {
    pub entity: &'a Entity,
    pub floor: &'a Floor,
}

/// todo!() add docs
pub struct CellView<'a> {
    pub position: Position,
    pub entity: Option<&'a Entity>,
    pub cell: &'a Cell,
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

    /// Ritorna un iteratore contenente gli iteratori di ogni riga del piano.
    pub fn get_grid(&self, view: usize) -> impl Iterator<Item = impl Iterator<Item = CellView>> {
        let grid = &self.floor.grid;
        let entities = self
            .floor
            .get_all_entities()
            .chain(std::iter::once(self.entity))
            .map(|entity| (&entity.position, entity))
            .collect::<HashMap<_, _>>();
        let entities = std::rc::Rc::new(std::cell::RefCell::new(entities));

        let temp_x = self.entity.position.0.saturating_sub(view);
        let temp_y = self.entity.position.1.saturating_sub(view);
        let size_x = temp_x.saturating_add(2 * view).min(grid.len());
        let size_y = temp_y.saturating_add(2 * view).min(grid.len());
        let view_x = size_x.saturating_sub(2 * view);
        let view_y = size_y.saturating_sub(2 * view);

        (view_y..size_y).rev().map(move |y| {
            let entities = entities.clone();
            (view_x..size_x)
                .map(move |x| Position(x, y))
                .map(move |position| {
                    let cell = &grid[position.0][position.1];
                    let entity = entities.borrow_mut().remove(&position);
                    CellView {
                        position,
                        entity,
                        cell,
                    }
                })
        })
    }

    /// Rappresentazione del piano come matrice di char
    pub fn as_char_grid(&self) -> Vec<Vec<char>> {
        self.get_grid(self.floor.grid.len())
            .map(|iter| {
                iter.flat_map(|view| {
                    if let Some(e) = view.entity {
                        return [' ', e.direction.as_char(), ' '];
                    }

                    let ch = view.cell.as_char();
                    match view.cell {
                        Cell::Wall => [ch, ch, ch],
                        _ => [' ', ch, ' '],
                    }
                })
                .chain(std::iter::once('\n'))
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Display for FloorView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid: String = self
            .as_char_grid()
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect();

        write!(f, "{}\n{}", grid, self.entity)
    }
}
