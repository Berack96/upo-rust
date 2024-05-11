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
use std::fmt::Display;

/**
 * Struttura del gioco generico che implementa un RogueLike.
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct Rogue {
    floors: Vec<FloorPtr>,
    rng: Pcg32,
    config: Config,
    players: Vec<Entity>,
}

impl Display for Rogue {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Rogue {
    pub fn new() -> Self {
        Self::new_with(Config::default())
    }
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
    pub fn add_player(&mut self, name: String) {
        let floor = self.floors[0].clone();
        let decider = Box::new(Immovable);
        let entity = Entity::new(name, decider, floor);
        self.players.push(entity);
    }
    pub fn get_floor(&self, level: usize) -> FloorPtr {
        let floors = self.floors.len() - 1;
        let index = level.min(floors);
        self.floors[index].clone()
    }
    pub fn build_next_floor(&mut self) {
        let floor_seed = self.rng.next_u64();
        let floor_level = self.floors.len();
        let generator = Generator::new(floor_seed, floor_level, &self.config);
        let floor = generator.build_floor();
        self.floors.push(floor);
    }
    pub fn compute_turn(&mut self) {
        let mut update_floors = vec![false; self.floors.len()];
        let mut change_floors = vec![0; self.players.len()];

        self.players.iter_mut().enumerate().for_each(|(i, player)| {
            let mut floor = player.get_floor();
            let mut floor = floor.get();

            player.update();
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
