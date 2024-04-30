use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};

use super::cell::Cell;
use super::entities::Entity;
use super::game::Rogue;
use super::generator::Generator;

#[derive(Deserialize, Serialize)]
pub struct Floor {
    level: usize,
    grid: Vec<Vec<Cell>>,
    entities: Vec<Entity>,
    rng: Pcg32,
}

impl Floor {
    pub fn new(generator: Generator) -> Self {
        Self {
            entities: vec![],
            level: generator.level,
            grid: generator.build_empty_matrix(),
            rng: generator.rng,
        }
    }
    pub fn get_rng(&mut self) -> &mut Pcg32 {
        &mut self.rng
    }
    pub fn get_level(&self) -> usize {
        self.level
    }
    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        self.grid[pos.0][pos.1]
    }

    pub fn compute_entities(&mut self, game: &mut Rogue) {
        for entity in &mut self.entities {
            entity.compute_effects(game);
            entity.do_action(game, game.input_action(entity));
        }
    }
}
