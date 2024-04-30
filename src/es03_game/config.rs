
use super::cell::{self, Effect};
use serde::{Deserialize, Serialize};

/**
 * Struttura Config usata per definire il gioco, ha alcune cose utili
 * TODO sarebbe bello poterle prendere da file.
 */

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub game_seed: u64,
    pub floor_size_range: (usize, usize),
    pub effects: Vec<ConfigEffect>,
    pub effects_total: usize,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct ConfigEffect {
    pub effect: Effect,
    pub first_floor: usize,
    pub last_floor: usize,
    pub priority: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_seed: 0,
            floor_size_range: (20, 30),
            effects: vec![ConfigEffect { effect: cell::POISON, first_floor: 0, last_floor: 255, priority: 1 }],
            effects_total: 45,
        }
    }
}
