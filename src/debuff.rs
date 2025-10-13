use std::cell::RefCell;
use std::ops::Deref;
use crate::buff::Buff;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Debuff {
    Captured,
    Check,
    Checkmate,
    Pin(Vector),
}

#[derive(Debug)]
pub struct DebuffsCollection {
    debuffs: RefCell<Vec<Debuff>>,
}

impl DebuffsCollection {
    pub fn new(debuffs: Vec<Debuff>) -> Self {
        Self { debuffs: RefCell::new(debuffs) }
    }

    pub fn add(&self, debuff: Debuff) {
        self.debuffs.borrow_mut().push(debuff)
    }

    pub fn to_vec(&self) -> Vec<Debuff> {
        self.debuffs.borrow()[..].iter().map(|debuff| debuff.clone()).collect::<Vec<_>>()
    }
}
