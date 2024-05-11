use super::{
    cell::Cell,
    config::{Config, ConfigEffect, ConfigEntity},
    floor::{Floor, FloorPtr},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::ops::Range;

/// Generatore del gioco che può creare dei piani del dungeon.
/// Idealmente questo generatore si comporta come il pattern Factory.
/// Per far si che funzioni ha bisongo di un seed per la generazione del piano
/// verrà utilizzato poi dal piano stesso per eventuali altri calcoli.
/// Inoltre ad esso viene passato una struttura di config che permette
/// di scegliere meglio come poter generare il piano.
pub struct Generator {
    pub rng: Pcg32,
    pub level: usize,
    size_rooms: Range<usize>,
    effects_total: usize,
    effects: Vec<ConfigEffect>,
    entities_total: usize,
    entities: Vec<ConfigEntity>,
    size: usize,
    grid: Vec<Vec<Cell>>, // enemies vec configuration?
}

impl Generator {
    /// Costruttore standard di un generatore
    /// il generatore creato avrà come entità quelle registrate nella configurazione
    pub fn new(floor_seed: u64, floor_level: usize, config: &Config) -> Self {
        let mut rand_pcg = Pcg32::seed_from_u64(floor_seed);
        let floor_size = rand_pcg.gen_range(config.floor_size.clone());
        Self {
            rng: rand_pcg,
            level: floor_level,
            size: floor_size,
            size_rooms: config.room_size.clone(),
            effects_total: config.effects_total,
            effects: Self::vec_filter(&config.effects, |val| {
                val.floors.contains(&floor_level)
            }),
            entities_total: config.entities_total,
            entities: Self::vec_filter(&config.entities, |val| {
                val.floors.contains(&floor_level)
            }),
            grid: Self::grid_with_only(floor_size, Cell::Wall),
        }
    }

    /// Questo metodo è il più semplice per la generazione del piano.\
    /// Crea una enorme stanza con dei muri attorno e piazza tutti gli effetti.
    pub fn build_floor(mut self) -> FloorPtr {
        for x in 1..(self.size - 1) {
            for y in 1..(self.size - 1) {
                self.grid[x][y] = Cell::Empty;
            }
        }

        self.rand_place_cell(Cell::Entance);
        self.rand_place_effects();
        FloorPtr::new(self.level, self.rng, vec![], self.grid)
    }

    /// TODO
    pub fn build_floor_catacomb(mut self) -> Floor {
        // init to WALLS
        // reserve some cells for rooms ??
        todo!()
    }

    /// TODO
    fn build_rooms(&mut self) {
        todo!()
    }
    /// crea una stanza in un punto casuale del piano e restituisce il suo boundingbox
    fn rand_build_room(&mut self) -> (usize, usize, usize, usize) {
        loop {
            let (x, y) = self.rand_2d(0..self.size);
            let size = self.rand_2d(self.size_rooms.clone());
            let (x_up, y_up) = (x + size.0, y + size.1);

            if x_up < self.size && y_up < self.size {
                for x in x..x_up {
                    for y in y..y_up {
                        self.grid[x][y] = Cell::Empty
                    }
                }
                return (x, y, x_up, y_up);
            }
        }
    }
    /// piazza gli effetti della confgurazione in modo casuale su tutto il piano.\
    /// essi vengono piazzati solamente sulle celle Empty
    fn rand_place_effects(&mut self) {
        for _ in 0..self.effects_total {
            let index = self.rng.gen_range(0..self.effects.len());
            let effect = self.effects[index].effect.clone();
            let cell = Cell::Special(effect);
            self.rand_place_cell(cell);
        }
    }
    /// piazza una cella in un punto casuale del piano.\
    /// il metodo contiuna a provare a piazzare la cella finche non trova una cella Empty.
    fn rand_place_cell(&mut self, cell: Cell) -> (usize, usize) {
        loop {
            let (x, y) = self.rand_2d(0..self.size);
            if let Cell::Empty = self.grid[x][y] {
                self.grid[x][y] = cell;
                return (x, y);
            }
        }
    }
    /// genera una tupla di due valori randomici nel range passato in input
    fn rand_2d(&mut self, range: Range<usize>) -> (usize, usize) {
        let x = self.rng.gen_range(range.clone());
        let y = self.rng.gen_range(range);
        (x, y)
    }
    /// crea una vista del vettore passato in input dopo aver applicato la funzione di filtro
    fn vec_filter<T: Clone>(original: &Vec<T>, filter: impl Fn(&T) -> bool) -> Vec<T> {
        original
            .clone()
            .into_iter()
            .filter_map(|val| if filter(&val) { Some(val) } else { None })
            .collect()
    }
    /// crea un campo con solamente la cella specificata clonata su tutto di esso
    fn grid_with_only(size: usize, cell: Cell) -> Vec<Vec<Cell>> {
        vec![vec![cell; size]; size]
    }
}
