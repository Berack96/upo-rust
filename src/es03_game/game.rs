use std::fmt::Display;

use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};

use super::{config::Config, entities::{Action, Entity}, floor::Floor, generator::Generator};

/**
 * Struttura del gioco generico che implementa un RogueLike.
 */
#[derive(Deserialize, Serialize)]
pub struct Rogue {
    floor: Floor,
    rng: Pcg32,
    config: Config,
}

impl Display for Rogue {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Rogue {
    pub fn new() -> Self {
        let config = Config::default();
        let mut rng = Pcg32::seed_from_u64(config.game_seed);
        let floor = Floor::new(Generator::new(rng.next_u64(), 0, &config));

        Self { rng, config, floor }
    }

    pub fn current_floor(&mut self) -> &mut Floor {
        &mut self.floor
    }

    pub fn build_new_floor(&mut self) {
        let level = self.floor.get_level();
        let floor_seed = self.rng.next_u64();

        let generator = Generator::new(floor_seed, level + 1, &self.config);
        self.floor = Floor::new(generator);
    }

    pub fn input_action(&self, entity: &Entity) -> Action {
        todo!()
    }

    pub fn compute_turn(&mut self) {
        todo!();
    }
}
