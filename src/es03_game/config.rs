use super::{
    cell::{Confusion, Effect, InstantDamage},
    entities::Decider,
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Struttura di configurazione per la creazione di un dungeon.\
/// Ogni elemento indica un parametro per la generazione di un piano o di una entitità.\
/// Esiste una implementazione di default di questa struttura che genera un dungeon
/// molto semplice e standard.
#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub game_seed: u64,
    pub room_size: Range<usize>,
    pub floor_size: Range<usize>,
    pub effects_total: usize,
    pub effects: Vec<ConfigEffect>,
    pub entities_total: usize,
    pub entities: Vec<ConfigEntity>,
    pub player_stats: ConfigPlayer,
}

/// Un effetto che si può trovare per terra nel dungeon.\
/// La priorità indica quanto verrà spawnato l'effetto in media.\
/// \
/// Es. effetto A priorità 1 ed effetto B con priorità 2\
/// Se in Config mettiamo 15 effetti per piano, allora avremo
/// in media 10 A e 5 B per ogni piano.
#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigEffect {
    pub floors: Range<usize>,
    pub effect: Box<dyn Effect>,
    pub priority: usize,
}

/// Valori di base per le statistiche di un giocatore.\
/// Esse verranno utilizzate quando un giocatore verrà creato.
#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigPlayer {
    pub health: i32,
    pub attack: i32,
}

/// Una entità che si può trovare in un piano nel dungeon.\
/// La priorità indica quanto verrà spawnata l'entità in media.\
/// \
/// Es. entità A priorità 1 ed entità B con priorità 2\
/// Se in Config mettiamo 15 entità per piano, allora avremo
/// in media 10 A e 5 B per ogni piano.
#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigEntity {
    pub floors: Range<usize>,
    pub name: String,
    pub decider: Box<dyn Decider>,
    pub health: i32,
    pub attack: i32,
    pub priority: usize,
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
            player_stats: ConfigPlayer {
                health: 1000,
                attack: 100,
            }
        }
    }
}
