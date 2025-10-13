use std::cell::RefCell;
use crate::debuff::Debuff;
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

    pub fn to_vec(&self) -> Vec<Buff> {
        self.buffs.borrow()[..].iter().map(|buff| buff.clone()).collect::<Vec<_>>()
    }
}
