use super::{cell::Effect, floor::FloorPtr};
use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Position(pub usize, pub usize);

#[derive(Clone, Deserialize, Serialize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

impl Direction {
    pub fn invert(&mut self) {
        *self = match *self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
            _ => Direction::NONE,
        };
    }
    pub fn move_from(&self, pos: Position) -> Position {
        match *self {
            Direction::UP => Position(pos.0, pos.1 + 1),
            Direction::DOWN => Position(pos.0, pos.1 - 1),
            Direction::RIGHT => Position(pos.0 + 1, pos.1),
            Direction::LEFT => Position(pos.0 - 1, pos.1),
            Direction::NONE => pos,
        }
    }
    pub fn random(rng: &mut Pcg32) -> Self {
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

#[derive(Clone, Deserialize, Serialize)]
pub struct Entity {
    name: String,
    effects: VecDeque<Box<dyn Effect>>,
    decider: Box<dyn Decider>,
    floor: FloorPtr,
    pub buffer: Action,
    pub position: Position,
    pub direction: Direction,
    pub health: i32,
    attack: i32,
}

impl Entity {
    pub fn new(name: String, decider: Box<dyn Decider>, mut floor: FloorPtr) -> Self {
        let position = floor.get().get_entrance();
        Self {
            name,
            floor,
            decider,
            position,
            buffer: Action::DoNothing,
            effects: VecDeque::new(),
            direction: Direction::NONE,
            attack: 100,
            health: 100,
        }
    }
    pub fn add_effect(&mut self, effect: Box<dyn Effect>) {
        self.effects.push_back(effect);
    }
    pub fn get_floor(&self) -> FloorPtr {
        self.floor.clone()
    }
    pub fn set_floor(&mut self, floor: FloorPtr) {
        self.floor = floor;
        self.position = self.floor.get().get_entrance();
    }
    pub fn update(&mut self) {
        self.compute_action();
        self.compute_effects();
    }

    fn compute_effects(&mut self) {
        let total = self.effects.len(); // len could change
        for _ in 0..total {
            if let Some(effect) = self.effects.pop_front() {
                effect.apply_to(self);
            }
        }
    }
    fn compute_action(&mut self) {
        let action = self.decider.get_next_action();
        match self.buffer {
            Action::DoNothing => action.apply(self),
            _ => (),
        }
        self.buffer = Action::DoNothing;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Action {
    Move(Direction),
    // attack
    DoNothing,
}

impl Action {
    pub fn apply(self, entity: &mut Entity) {
        match self {
            Action::DoNothing => {}
            Action::Move(direction) => {
                let pos = direction.move_from(entity.position);
                entity.direction = direction;
                entity.position = pos;

                let mut floor = entity.floor.clone();
                let mut floor = floor.get();
                let cell = floor.get_cell(pos);
                cell.entity_over(entity);
            }
        }
    }
}

clone_trait_object!(Decider);
#[typetag::serde(tag = "type")]
pub trait Decider: DynClone {
    fn get_next_action(&self) -> Action;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Immovable;
#[typetag::serde]
impl Decider for Immovable {
    fn get_next_action(&self) -> Action {
        Action::DoNothing
    }
}
