use super::{
    cell::Cell,
    entities::{Entity, Position},
};
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Clone, Deserialize, Serialize)]
pub struct FloorPtr(Rc<RefCell<Floor>>);
impl FloorPtr {
    pub fn new(level: usize, rng: Pcg32, entities: Vec<Entity>, grid: Vec<Vec<Cell>>) -> Self {
        Self(Rc::new(RefCell::new(Floor::new(
            level, rng, entities, grid,
        ))))
    }
    pub fn get(&mut self) -> RefMut<Floor> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Floor {
    level: usize,
    grid: Vec<Vec<Cell>>,
    entities: Vec<Entity>,
    rng: Pcg32,
}

impl Floor {
    fn new(level: usize, rng: Pcg32, entities: Vec<Entity>, grid: Vec<Vec<Cell>>) -> Self {
        Self {
            level,
            grid,
            entities,
            rng,
        }
    }

    pub fn get_level(&self) -> usize {
        self.level
    }
    pub fn get_rng(&mut self) -> &mut Pcg32 {
        &mut self.rng
    }
    pub fn get_cell(&mut self, pos: Position) -> &mut Cell {
        &mut self.grid[pos.0][pos.1]
    }
    pub fn get_entrance(&mut self) -> Position {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(x, vec)| {
                vec.iter().enumerate().find_map(|(y, cell)| {
                    if let Cell::Entance = cell {
                        Some(Position(x, y))
                    } else {
                        None
                    }
                })
            })
            .expect("Entrance of the floor should be inside the grid!")
    }

    pub fn update_entities(&mut self) {
        for entity in &mut self.entities {
            entity.update();
        }
    }
}
