use std::{collections::VecDeque, fmt::Display};

use rand::Rng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};

use super::{
    cell::{Cell, Effect},
    game::Rogue,
};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match *self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
            _ => Direction::NONE,
        }
    }
    pub fn move_from(&self, pos: (usize, usize)) -> (usize, usize) {
        match *self {
            Direction::UP => (pos.0, pos.1 + 1),
            Direction::DOWN => (pos.0, pos.1 - 1),
            Direction::RIGHT => (pos.0 + 1, pos.1),
            Direction::LEFT => (pos.0 - 1, pos.1),
            Direction::NONE => (pos.0, pos.1),
        }
    }
    pub fn generate_random(rng: &mut Pcg32) -> Self {
        match rng.gen_range(0..4) {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::LEFT,
            _ => Direction::RIGHT,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::UP => '▲',
            Self::DOWN => '▼',
            Self::LEFT => '◄',
            Self::RIGHT => '►',
            Self::NONE => '■',
        };

        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Action {
    Move(Direction),
    // attack
    DoNothing,
}

#[derive(Deserialize, Serialize)]
pub struct Entity {
    name: String,
    floor: usize,
    effected_by: VecDeque<Effect>,
    player: bool,
    pub position: (usize, usize),
    pub direction: Direction,
    pub health: i32,
    pub attack: i32,
}

impl Entity {
    pub fn new(name: String) -> Self {
        Self {
            name,
            floor: 0,
            effected_by: VecDeque::new(),
            position: (0, 0),
            player: false,
            direction: Direction::UP,
            attack: 100,
            health: 100,
        }
    }
    pub fn new_player(name: String) -> Self {
        let mut player = Self::new(name);
        player.player = true;
        player
    }
    pub fn add_effect(&mut self, effect: Effect) {
        self.effected_by.push_back(effect);
    }
    pub fn compute_effects(&mut self, game: &mut Rogue) {
        let total = self.effected_by.len(); // len could change
        for _ in 0..total {
            if let Some(effect) = self.effected_by.pop_front() {
                effect.effect(game, self);
            }
        }
    }
    pub fn do_action(&mut self, game: &mut Rogue, action: Action) {
        match action {
            Action::Move(direction) => {
                self.direction = direction;
                self.do_action_move(game);
            }
            _ => todo!(),
        }
    }

    fn do_action_move(&mut self, game: &mut Rogue) {
        let direction = self.direction;
        let pos = direction.move_from(self.position);
        let floor = game.current_floor();

        match floor.get_cell(pos) {
            Cell::Empty => {
                self.position = pos;
            }
            Cell::Special(effect) => {
                self.position = pos;
                self.add_effect(effect)
            }
            Cell::Staricase => {
                if self.player {
                    game.build_new_floor()
                }
            }
            Cell::Wall => self.direction = direction.invert(),
        }
    }
}
