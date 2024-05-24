use super::{
    config::Config,
    entities::{Behavior, Entity},
    floor::Floor,
    generator::Generator,
};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

/// Rappresenta un Dungeon in stile RogueLike.\
/// In esso possiamo trovare dei piani generati casualmente
/// e dei giocatori che esplorano.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dungeon {
    floors: Vec<Floor>,
    config: Config,
    rng: Pcg32,
}

impl Dungeon {
    /// Crea una nuova istanza di un dungeon con le configurazioni di default
    pub fn new() -> Self {
        Self::new_with(Config::default())
    }

    /// Crea una nuova istanza di un dungeon con le configurazioni passate in input
    pub fn new_with(config: Config) -> Self {
        let mut game = Self {
            rng: Pcg32::seed_from_u64(config.game_seed),
            floors: vec![],
            config,
        };
        game.build_next_floor();
        game
    }

    /// Carica il dungeon da un file.\
    /// Il file deve essere formattato tramite json, altrimenti viene ritornato un errore.
    pub fn load(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let dungeon: Self = serde_json::from_reader(reader)?;
        Ok(dungeon)
    }

    /// Salva il dungeon corrente nel file indicato.\
    /// Il salvataggio viene fatto tramite serializzazione JSON in modo che sia facile da vedere.\
    /// Nel caso in cui ci siano problemi con I/O, viene ritornato un errore.
    pub fn save(&mut self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        let _ = serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Aggiunge un giocatore al Dungeon, esso avrà le statistiche di base assegnate
    /// ad esso tramite la configurazione indicata nel costruttore.\
    /// Il giocatore appena inserito si troverà al piano 0.
    pub fn add_player(&mut self, name: String, decider: Box<dyn Behavior>) {
        let stats = &self.config.player_stats;
        let player = Entity::new(name, stats.health, stats.attack, decider);
        self.floors[0].add_player(player);
    }

    /// Indica se nel dungeon ci sono dei giocatori.\
    /// Metodo utile, dato che nel caso in cui non ci siano, il dungen non verrà modificato
    /// siccome per calcolare il turno successivo ho bisogno di giocatori.
    pub fn has_players(&mut self) -> bool {
        self.floors.iter().any(|floor| floor.has_players())
    }

    /// Restituisce il piano indicato dal livello di profondità.\
    /// Nel caso il livello non esista, restituisce il piano con profondità maggiore.
    pub fn get_floor(&self, level: usize) -> &Floor {
        let floors = self.floors.len() - 1;
        let index = level.min(floors);
        &self.floors[index]
    }

    /// Funzione principale del dungeon.\
    /// In essa viene fatto fare l'update ai giocatori e ad ogni piano.
    /// In generale l'algoritmo è il seguente per ogni piano in cui si trova un giocatore:\
    /// - I giocatori fanno le loro mosse.\
    /// - Se un giocatore non è più in vita o non può indicare l'azione da fare, viene rimosso
    /// - Update di tutte le entità del piano
    /// - Modifica di piano di eventuali giocatori
    pub fn compute_turn(&mut self) {
        let moved = self.floors.iter_mut().fold(None, |moved, floor| {
            if floor.has_players() {
                let _ = floor.update_players(); //todo!() evantually return the dead players? idk
                floor.update_entities();
            }

            if let Some(player) = moved {
                floor.add_player(player);
            }

            floor.get_player_at_exit()
        });

        if let Some(player) = moved {
            self.build_next_floor();
            let len = self.floors.len();
            let floor = &mut self.floors[len - 1];
            floor.add_player(player);
        }
    }

    /// permette di costruire il piano successivo
    fn build_next_floor(&mut self) {
        let floor_seed = self.rng.next_u64();
        let floor_level = self.floors.len();
        let generator = Generator::new(floor_seed, floor_level, &self.config);
        let floor = generator.build_floor();
        self.floors.push(floor);
    }
}
