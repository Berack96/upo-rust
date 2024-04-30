use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;

use super::cell::Cell;

use super::config::Config;
use super::config::ConfigEffect;

pub struct Generator<'a> {
    pub rng: Pcg32,
    pub level: usize,
    size: usize,
    effects: Vec<&'a ConfigEffect>,
    effects_total: usize,
    // enemies vec configuration?
}

impl<'a> Generator<'a> {
    pub fn new(floor_seed: u64, floor_level: usize, config: &'a Config) -> Self {
        let mut rand_pcg = Pcg32::seed_from_u64(floor_seed);
        let range = config.floor_size_range.0..config.floor_size_range.1;
        let floor_size = rand_pcg.gen_range(range);

        let effects_list = &config.effects;
        let effects_list = effects_list.into_iter();
        let effects_list = effects_list.filter_map(|val| {
            if floor_level >= val.first_floor && floor_level <= val.last_floor {
                Some(val)
            } else {
                None
            }
        });
        let effects_list = effects_list.collect();

        Self {
            rng: rand_pcg,
            level: floor_level,
            size: floor_size,
            effects_total: config.effects_total,
            effects: effects_list,
        }
    }

    pub fn build_empty_matrix(&self) -> Vec<Vec<Cell>> {
        self.build_matrix_with(Cell::Empty)
    }
    pub fn build_labyrinth(&mut self) -> Vec<Vec<Cell>> {
        todo!()
    }
    pub fn build_rooms(&mut self) -> Vec<Vec<Cell>> {
        todo!()
    }

    fn place_staircase(&mut self, grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        todo!()
    }
    fn build_matrix_with(&self, cell: Cell) -> Vec<Vec<Cell>> {
        vec![vec![cell; self.size]; self.size]
    }
}
