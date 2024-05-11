use super::entities::{Action, Direction, Entity};
use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Cell {
    Entance,
    Exit,
    Special(Box<dyn Effect>),
    Wall,
    Empty,
}

impl Cell {
    pub fn entity_over(&mut self, entity: &mut Entity) {
        match self {
            Cell::Special(effect) => {
                entity.add_effect(effect.clone());
                if !effect.is_persistent() {
                    *self = Cell::Empty
                }
            }
            Cell::Wall => {
                entity.direction.invert();
                entity.position = entity.direction.move_from(entity.position);
            }
            _ => (),
        }
    }
}

#[typetag::serde(tag = "type")]
pub trait Effect: DynClone {
    fn is_persistent(&self) -> bool;
    fn apply_to(&self, entity: &mut Entity);
}
clone_trait_object!(Effect);

#[derive(Clone, Serialize, Deserialize)]
pub struct InstantDamage(pub i32);
#[typetag::serde]
impl Effect for InstantDamage {
    fn is_persistent(&self) -> bool {
        false
    }
    fn apply_to(&self, entity: &mut Entity) {
        entity.health += self.0;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Confusion(pub u8);
#[typetag::serde]
impl Effect for Confusion {
    fn is_persistent(&self) -> bool {
        true
    }
    fn apply_to(&self, entity: &mut Entity) {
        if self.0 > 0 {
            let mut floor = entity.get_floor();
            let mut floor = floor.get();
            let rng = floor.get_rng();
            let coin_flip = rng.gen_range(0..=1);
            if coin_flip == 1 {
                let random_direction = Direction::random(rng);
                entity.buffer = Action::Move(random_direction);
            }
            entity.add_effect(Box::new(Self(self.0 - 1)));
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TurnBasedDamage {
    time: u8,
    damage: i32,
}
#[typetag::serde]
impl Effect for TurnBasedDamage {
    fn is_persistent(&self) -> bool {
        false
    }
    fn apply_to(&self, entity: &mut Entity) {
        if self.time > 0 {
            entity.health += self.damage;
            entity.add_effect(Box::new(Self {
                time: self.time - 1,
                damage: self.damage,
            }));
        }
    }
}
