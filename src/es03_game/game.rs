use super::{
    cell::Cell,
    config::Config,
    entities::{Entity, Immovable},
    floor::FloorPtr,
    generator::Generator,
};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};

/// Rappresenta un Dungeon in stile RogueLike.\
/// In esso possiamo trovare dei piani generati casualmente
/// e dei giocatori che esplorano.
#[derive(Clone, Deserialize, Serialize)]
pub struct Dungeon {
    floors: Vec<FloorPtr>,
    rng: Pcg32,
    config: Config,
    players: Vec<Entity>,
}

impl Dungeon {
    /// Crea una nuova istanza di un dungeon con le configurazioni i default
    pub fn new() -> Self {
        Self::new_with(Config::default())
    }

    /// Crea una nuova istanza di un dungeon con le configurazioni passate in input
    pub fn new_with(config: Config) -> Self {
        let mut game = Self {
            rng: Pcg32::seed_from_u64(config.game_seed),
            floors: vec![],
            players: vec![],
            config,
        };
        game.build_next_floor();
        game
    }

    /// Aggiunge un giocatore al Dungeon, esso avrà le statistiche di base assegnate
    /// ad esso tramite la configurazione indicata nel costruttore.\
    /// Il giocatore appena inserito si troverà al piano 0.
    pub fn add_player(&mut self, name: String) {
        let floor = self.floors[0].clone();
        let decider = Box::new(Immovable);
        let stats = &self.config.player_stats;
        let entity = Entity::new(name, stats.health, stats.attack, decider, floor);
        self.players.push(entity);
    }

    /// Restituisce il piano indicato dal livello di profondità.\
    /// Nel caso il livello non esista, restituisce il piano con profondità maggiore.
    pub fn get_floor(&self, level: usize) -> FloorPtr {
        let floors = self.floors.len() - 1;
        let index = level.min(floors);
        self.floors[index].clone()
    }

    /// Funzione principale del dungeon.\
    /// In essa viene fatto fare l'update ai giocatori e ad ogni piano.
    /// In generale l'algoritmo è il seguente:\
    /// - I giocatori fanno le loro mosse.\
    /// - Se un giocatore non è più in vita o non può indicare l'azione da fare, viene rimosso
    /// - Update di tutti i piani in cui c'è almeno un giocatore
    /// - Modifica di piano di eventuali giocatori
    pub fn compute_turn(&mut self) {
        let mut update_floors = vec![false; self.floors.len()];
        let mut change_floors = vec![0; self.players.len()];

        Entity::update_from_vec(&mut self.players);
        self.players.iter_mut().enumerate().for_each(|(i, player)| {
            let mut floor = player.get_floor();
            let mut floor = floor.get();
            update_floors[floor.get_level()] = true;
            if let Cell::Exit = floor.get_cell(player.position) {
                change_floors[i] = floor.get_level() + 1;
            }
        });

        update_floors
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if *b { Some(i) } else { None })
            .for_each(|i| self.floors[i].get().update_entities());
        change_floors
            .iter()
            .enumerate()
            .filter(|(_, f)| **f != 0)
            .for_each(|(player, floor)| {
                let floor = self.get_floor_or_build(*floor);
                let player = &mut self.players[player];
                player.set_floor(floor);
            });
    }

    /// permette di costruire il piano successivo
    fn build_next_floor(&mut self) {
        let floor_seed = self.rng.next_u64();
        let floor_level = self.floors.len();
        let generator = Generator::new(floor_seed, floor_level, &self.config);
        let floor = generator.build_floor();
        self.floors.push(floor);
    }
    /// restituisce il piano indicato o ne crea uno nuovo se il livello è troppo profondo
    fn get_floor_or_build(&mut self, level: usize) -> FloorPtr {
        let mut level = level;
        if level > self.floors.len() {
            level = self.floors.len();
        }
        if level == self.floors.len() {
            self.build_next_floor()
        }

        self.get_floor(level)
    }
}
