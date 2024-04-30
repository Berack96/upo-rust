use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{entities::{Direction, Entity}, game::Rogue};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Cell {
    Staricase,
    Special(Effect),
    Wall,
    Empty,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Effect {
    InstantDamage(i8),
    TurnBasedDamage(u8, i8),
    //DeBuff(i8, i8),
    Confusion(u8),
    //Custom(i16),
}

const DELTA: i32 = 5;
impl Effect {
    pub fn effect(&self, game: &mut Rogue, entity: &mut Entity) {
        let floor = game.current_floor();
        let rng = floor.get_rng();

        match *self {
            Effect::InstantDamage(damage) => {
                let damage = damage as i32;
                let damage = damage + rng.gen_range(-DELTA..DELTA);
                entity.health += damage;
            }
            Effect::Confusion(time) => {
                if time > 0 {
                    entity.add_effect(Effect::Confusion(time - 1));
                    let coin_flip = rng.gen_range(0..=1);
                    if coin_flip == 1 {
                        let random_direction = Direction::generate_random(rng);
                        entity.direction = random_direction;
                    }
                }
            }
            _ => todo!()
        }
    }
}

pub const POISON: Effect = Effect::InstantDamage(20);
pub const FOOD: Effect = Effect::InstantDamage(-20);
pub const CONFUSION: Effect = Effect::Confusion(u8::MAX);
