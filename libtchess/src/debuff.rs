use std::cell::RefCell;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Debuff {
    Captured,
    Check,
    Checkmate,
    Pin(Vector),
}

#[derive(Debug)]
struct DebuffsList {
    pub captured: Option<Debuff>,
    pub check: Option<Debuff>,
    pub checkmate: Option<Debuff>,
    pub pin: Option<Debuff>,
}

impl DebuffsList {
    pub fn empty() -> Self {
        Self {
            captured: None,
            check: None,
            checkmate: None,
            pin: None,
        }
    }

    pub fn add(&mut self, debuff: Debuff) {
        match debuff {
            Debuff::Captured => { self.captured = Some(debuff) },
            Debuff::Check => { self.check = Some(debuff) },
            Debuff::Checkmate => { self.checkmate = Some(debuff) },
            Debuff::Pin(_) => { self.pin = Some(debuff) },
        }
    }
}

#[derive(Debug)]
pub struct DebuffsCollection {
    debuffs: RefCell<DebuffsList>,
}

impl DebuffsCollection {
    pub fn new(debuffs: Vec<Debuff>) -> Self {
        let mut list = DebuffsList::empty();
        for debuff in debuffs {
            list.add(debuff)
        }
        Self { debuffs: RefCell::new(list) }
    }

    pub fn add(&self, debuff: Debuff) {
        self.debuffs.borrow_mut().add(debuff)
    }

    pub fn pin(&self) -> Option<Vector> {
        match self.debuffs.borrow().pin {
            Some(debuff) => {
                match debuff {
                    Debuff::Pin(v) => Some(v),

                    _ => panic!("Invalid pin debuff {:?}!", debuff)
                }
            },
            None => None
        }
    }

    pub fn remove_pin(&self) {
        self.debuffs.borrow_mut().pin = None;
    }

    pub fn remove_check(&self) {
        self.debuffs.borrow_mut().check = None;
    }

    pub fn has_check(&self) -> bool {
        self.debuffs.borrow().check.is_some()
    }

    pub fn to_vec(&self) -> Vec<Debuff> {
        let list = self.debuffs.borrow();
        [list.captured, list.check, list.checkmate, list.pin]
            .iter()
            .filter(|debuff| debuff.is_some())
            .map(|debuff| debuff.unwrap())
            .collect::<Vec<_>>()
    }
}
