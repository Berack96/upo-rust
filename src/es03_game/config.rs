use super::{
    cell::{Confusion, Effect, InstantDamage},
    entities::Decider,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/**
 * Struttura Config usata per definire il gioco, ha alcune cose utili
 * TODO sarebbe bello poterle prendere da file.
 */

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub game_seed: u64,
    pub room_size: Range<usize>,
    pub floor_size: Range<usize>,
    pub effects_total: usize,
    pub effects: Vec<ConfigEffect>,
    pub entities_total: usize,
    pub entities: Vec<ConfigEntity>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigEffect {
    pub floors: Range<usize>,
    pub effect: Box<dyn Effect>,
    pub priority: usize,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigEntity {
    pub floors: Range<usize>,
    pub name: String,
    pub decider: Box<dyn Decider>,
    pub health: i32,
    pub attack: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_seed: 0,
            room_size: 5..10,
            floor_size: 30..40,
            effects: vec![
                ConfigEffect {
                    effect: Box::new(InstantDamage(20)),
                    floors: 0..255,
                    priority: 1,
                },
                ConfigEffect {
                    effect: Box::new(InstantDamage(-20)),
                    floors: 0..255,
                    priority: 1,
                },
                ConfigEffect {
                    effect: Box::new(Confusion(10)),
                    floors: 0..255,
                    priority: 1,
                },
            ],
            effects_total: 45,
            entities: vec![],
            entities_total: 0,
        }
    }
}
