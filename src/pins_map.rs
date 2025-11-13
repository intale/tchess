use std::collections::hash_map::Keys;
use std::rc::Rc;
use rustc_hash::{FxHashMap};
use crate::pieces::Piece;

type PinT = FxHashMap<Rc<Piece>, Rc<Piece>>;
pub struct PinsMap {
    pinned_by_to_pinned: PinT,
    pinned_to_pinned_by: PinT,
}

impl PinsMap {
    pub fn empty() -> Self {
        Self {
            pinned_by_to_pinned: FxHashMap::default(),
            pinned_to_pinned_by: FxHashMap::default(),
        }
    }

    pub fn add_association(&mut self, pinned: &Rc<Piece>, pinned_by: &Rc<Piece>) {
        self.pinned_by_to_pinned.insert(Rc::clone(pinned_by), Rc::clone(pinned));
        self.pinned_to_pinned_by.insert(Rc::clone(pinned), Rc::clone(pinned_by));
    }

    pub fn remove_pinned(&mut self, pinned: &Rc<Piece>) {
        if let Some(pinned_by) = self.pinned_to_pinned_by.remove(pinned) {
            self.pinned_by_to_pinned.remove(&pinned_by);
        }
    }

    pub fn remove_pinned_by(&mut self, pinned_by: &Rc<Piece>) {
        if let Some(pinned) = self.pinned_by_to_pinned.remove(pinned_by) {
            self.pinned_to_pinned_by.remove(&pinned);
        }
    }

    pub fn pinned(&self, pinned_by: &Rc<Piece>) -> Option<&Rc<Piece>> {
        self.pinned_by_to_pinned.get(pinned_by)
    }

    pub fn pinned_by(&self, pinned: &Rc<Piece>) -> Option<&Rc<Piece>> {
        self.pinned_to_pinned_by.get(pinned)
    }

    pub fn clear_all(&mut self) {
        self.pinned_by_to_pinned.clear();
        self.pinned_to_pinned_by.clear();
    }

    pub fn pinned_keys(&self) -> Keys<'_, Rc<Piece>, Rc<Piece>> {
        self.pinned_to_pinned_by.keys()
    }

    pub fn all_pinned(&self) -> Vec<&Rc<Piece>> {
        self.pinned_keys().collect()
    }

    pub fn all_pinned_by(&self) -> Vec<&Rc<Piece>> {
        self.pinned_by_to_pinned.keys().collect()
    }
}
