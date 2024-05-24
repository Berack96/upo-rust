use super::{
    cell::{Confusion, Effect, InstantDamage},
    entities::{Behavior, RandomMovement},
};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Struttura di configurazione per la creazione di un dungeon.\
/// Ogni elemento indica un parametro per la generazione di un piano o di una entitità.\
/// Esiste una implementazione di default di questa struttura che genera un dungeon standard.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub game_seed: u64,
    pub maze_generation: ConfigMaze,
    pub effects_total: usize,
    pub effects: Vec<ConfigEffect>,
    pub entities_total: usize,
    pub entities: Vec<ConfigEntity>,
    pub player_stats: ConfigPlayer,
}

/// Configura la generazione del labirinto all'interno del generatore.\
/// I parametri principali servono ad indicare quanto grande è il piano e quanto grandi sono le stanze.\
/// *room_placing_attempts* indica quanti tentativi il generatore deve fare prima di smettere di creare stanze.\
/// *straight_percentage* indica da 0 a 100 quanta percentuale c'è che un corridioio, quando viene generato
/// rimanga dritto o viri.\
/// *dead_ends* indica quanti corridoi che non portano a nulla devono esserci alla fine della generazione.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigMaze {
    pub floor_size: Range<usize>,
    pub room_size: Range<usize>,
    pub room_placing_attempts: u32,
    pub straight_percentage: u32,
    pub dead_ends: u32,
}

/// Un effetto che si può trovare per terra nel dungeon.\
/// La priorità indica quanto verrà spawnato l'effetto in media.\
/// \
/// Es. effetto A priorità 1 ed effetto B con priorità 2\
/// Se in Config mettiamo 15 effetti per piano, allora avremo
/// in media 10 A e 5 B per ogni piano.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigEffect {
    pub floors: Range<usize>,
    pub effect: Box<dyn Effect>,
    pub priority: u32,
}

/// Valori di base per le statistiche di un giocatore.\
/// Esse verranno utilizzate quando un giocatore verrà creato.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigEntity {
    pub floors: Range<usize>,
    pub name: String,
    pub behavior: Box<dyn Behavior>,
    pub health: i32,
    pub attack: i32,
    pub priority: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_seed: 0,
            maze_generation: ConfigMaze {
                floor_size: 30..40,
                room_size: 5..10,
                room_placing_attempts: 10,
                straight_percentage: 90,
                dead_ends: 0,
            },
            effects: vec![
                ConfigEffect {
                    effect: Box::new(InstantDamage(20)),
                    floors: 0..255,
                    priority: 1,
                },
                ConfigEffect {
                    effect: Box::new(InstantDamage(-10)),
                    floors: 0..255,
                    priority: 1,
                },
                ConfigEffect {
                    effect: Box::new(Confusion(10)),
                    floors: 0..255,
                    priority: 10,
                },
            ],
            effects_total: 45,
            entities: vec![ConfigEntity {
                floors: 0..255,
                name: "Basic enemy".to_string(),
                behavior: Box::new(RandomMovement::new()),
                health: 30,
                attack: 10,
                priority: 1,
            }],
            entities_total: 10,
            player_stats: ConfigPlayer {
                health: 100,
                attack: 10,
            },
        }
    }
}
