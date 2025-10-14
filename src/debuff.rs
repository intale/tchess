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

    pub fn to_ref_cell(&self) -> RefCell<Vec<Debuff>> {
        self.debuffs.to_owned()
    }
}
