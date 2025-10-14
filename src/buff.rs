use std::cell::RefCell;
use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Buff {
    Castle,
    EnPassant(Point),
    LevelUp
}

#[derive(Debug)]
pub struct BuffsCollection {
    buffs: RefCell<Vec<Buff>>,
}

impl BuffsCollection {
    pub fn new(buffs: Vec<Buff>) -> Self {
        Self { buffs: RefCell::new(buffs) }
    }

    pub fn add(&self, buff: Buff) {
        self.buffs.borrow_mut().push(buff)
    }

    pub fn to_ref_cell(&self) -> RefCell<Vec<Buff>> {
        self.buffs.to_owned()
    }
}
