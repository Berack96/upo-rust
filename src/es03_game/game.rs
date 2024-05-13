use super::{
    cell::Cell,
    config::Config,
    entities::{Behavior, Entity},
    floor::{FloorPtr, FloorView},
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
    pub fn add_player(&mut self, name: String, decider: Box<dyn Behavior>) {
        let floor = self.floors[0].clone();
        let stats = &self.config.player_stats;
        let entity = Entity::new(name, stats.health, stats.attack, decider, floor);
        self.players.push(entity);
    }

    /// Indica se nel dungeon ci sono dei giocatori.\
    /// Metodo utile, dato che nel caso in cui non ci siano, il dungen non verrà modificato
    /// siccome per calcolare il turno successivo ho bisogno di giocatori.
    pub fn has_players(&self) -> bool {
        !self.get_players().is_empty()
    }

    /// Restiutuisce la lista dei giocatori che in questo momento stanno giocando.
    pub fn get_players(&self) -> &Vec<Entity> {
        &self.players
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
        if !self.has_players() {
            return;
        }

        let mut update = UpdateDungeon::compute(self);
        update.update_floors();
        update.remove_eventual_players();
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

/// Serve al dungeon per fare tutti i vari update.\
/// È stata creata una struttura e una sua implementazione apposta dato che per gli update la logica
/// è sia complessa che contorta. In questo modo si riesce a capire meglio che cosa viene fatto
/// utilizzando delle funzioni apposta.
struct UpdateDungeon<'a> {
    dungeon: &'a mut Dungeon,
    players: Vec<bool>,
    update_floors: Vec<bool>,
    change_floors: Vec<usize>,
}

impl<'a> UpdateDungeon<'a> {
    /// Crea un update del dungeon.\
    /// Con questo metodo si inizializza la struttura e per farlo viene chiamata la funzione
    /// update_display per ogni player che il dungeon ha attivo.\
    /// Dopo questo metodo, la struttura che ne risulta ha salvato alcuni parametri che possono
    /// diventare obsoleti nel caso in cui i metodo vangano chiamati dopo troppo tempo.
    fn compute(dungeon: &'a mut Dungeon) -> Self {
        let mut update_floors = vec![false; dungeon.floors.len()];
        let mut change_floors = vec![0; dungeon.players.len()];

        let players: Vec<bool> = (0..dungeon.players.len())
            .into_iter()
            .map(|i| {
                let player = &mut dungeon.players[i];
                let floor = FloorView::new(&player);
                let value = player.update_display(floor);

                let mut floor = player.get_floor();
                let mut floor = floor.get();
                update_floors[floor.get_level()] = true;
                if let Cell::Exit = floor.get_cell(player.position) {
                    change_floors[i] = floor.get_level() + 1;
                }

                value
            })
            .collect();
        Self {
            dungeon,
            players,
            update_floors,
            change_floors,
        }
    }

    /// Permette di fare l'update dei tutti i piani che hanno giocatori attivi.\
    /// I giocatori attivi vengono calcolati appena viene creata l'istanza, quindi
    /// questo metodo può diventare obsoleto nel caso in cui venga chiamato dopo troppo tempo
    /// dall'inizializzazione della struttura.
    fn update_floors(&mut self) -> &mut Self {
        self.update_floors
            .iter()
            .enumerate()
            .filter_map(|(i, b)| (*b).then(|| i))
            .for_each(|i| self.dungeon.floors[i].get().update_entities());
        self.change_floors
            .iter()
            .enumerate()
            .filter(|(_, f)| **f != 0)
            .for_each(|(player, floor)| {
                let floor = self.dungeon.get_floor_or_build(*floor);
                let player = &mut self.dungeon.players[player];
                player.set_floor(floor);
            });
        self
    }

    /// Permette di rimuovare eventuali giocatori che non servono più.\
    /// Questo metodo prende l'ownership della struttura dato che deve essere chiamato per ultimo
    /// siccome può modificare la lunghezza del vettore di players, invalidando quindi
    /// tutti i parametri creati precedentemente.
    fn remove_eventual_players(self) {
        let mut players = self.players.iter();
        self.dungeon.players.retain(|_| *players.next().unwrap());
    }
}
