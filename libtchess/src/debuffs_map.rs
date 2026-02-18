use im_rc::{HashMap};
use rustc_hash::FxBuildHasher;
use crate::debuff::Debuff;
use crate::piece_id::PieceId;

#[derive(Clone, Hash, Eq, PartialEq)]
enum DebuffRepr {
    Captured,
    Check,
    Checkmate,
    Pin,
}

impl DebuffRepr {
    fn from_debuff(debuff: &Debuff) -> Self {
        match debuff {
            Debuff::Captured => Self::Captured,
            Debuff::Check => Self::Check,
            Debuff::Checkmate => Self::Checkmate,
            Debuff::Pin(_) => Self::Pin,
        }
    }
}

#[derive(Clone)]
pub struct DebuffsMap {
    piece_to_debuffs: HashMap<PieceId, HashMap<DebuffRepr, Debuff, FxBuildHasher>, FxBuildHasher>,
}

impl DebuffsMap {
    pub fn empty() -> Self {
        let piece_to_debuffs = HashMap::default();
        Self { piece_to_debuffs }
    }

    fn debuffs_mut(&mut self, piece_id: &PieceId) -> &mut HashMap<DebuffRepr, Debuff, FxBuildHasher> {
        if !self.piece_to_debuffs.contains_key(piece_id) {
            self.piece_to_debuffs.insert(*piece_id, HashMap::default());
        }
        self.piece_to_debuffs.get_mut(piece_id).unwrap()
    }

    pub fn add(&mut self, piece_id: &PieceId, debuff: Debuff) {
        let debuffs_mut = self.debuffs_mut(piece_id);
        debuffs_mut.insert(DebuffRepr::from_debuff(&debuff), debuff);
    }

    pub fn add_from_vec(&mut self, piece_id: &PieceId, debuffs: Vec<Debuff>) {
        for debuff in debuffs.into_iter() {
            self.add(piece_id, debuff)
        }
    }

    pub fn remove_check(&mut self, piece_id: &PieceId) {
        if let Some(debuffs) = self.piece_to_debuffs.get_mut(piece_id) {
            debuffs.remove(&DebuffRepr::Check);
        }
    }

    pub fn remove_pin(&mut self, piece_id: &PieceId) {
        if let Some(debuffs) = self.piece_to_debuffs.get_mut(piece_id) {
            debuffs.remove(&DebuffRepr::Pin);
        }
    }

    pub fn has_check(&self, piece_id: &PieceId) -> bool {
        if let Some(debuffs) = self.piece_to_debuffs.get(piece_id) {
            debuffs.contains_key(&DebuffRepr::Check)
        } else {
            false
        }
    }

    pub fn has_pin(&self, piece_id: &PieceId) -> bool {
        if let Some(debuffs) = self.piece_to_debuffs.get(piece_id) {
            debuffs.contains_key(&DebuffRepr::Pin)
        } else {
            false
        }
    }

    pub fn pin(&self, piece_id: &PieceId) -> Option<&Debuff> {
        if let Some(debuffs) = self.piece_to_debuffs.get(piece_id) {
            debuffs.get(&DebuffRepr::Pin)
        } else {
            None
        }
    }
}
