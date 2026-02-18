use im_rc::{HashMap};
use rustc_hash::FxBuildHasher;
use crate::buff::Buff;
use crate::piece_id::PieceId;

#[derive(Clone, Hash, Eq, PartialEq)]
enum BuffRepr {
    Castle,
    EnPassant,
    AdditionalPoint,
}

impl BuffRepr {
    fn from_buff(buff: &Buff) -> Self {
        match buff {
            Buff::Castle => Self::Castle,
            Buff::EnPassant(_, _) => Self::EnPassant,
            Buff::AdditionalPoint => Self::AdditionalPoint,
        }
    }
}

#[derive(Clone)]
pub struct BuffsMap {
    piece_to_buffs: HashMap<PieceId, HashMap<BuffRepr, Buff, FxBuildHasher>, FxBuildHasher>,
}

impl BuffsMap {
    pub fn empty() -> Self {
        let piece_to_buffs = HashMap::default();
        Self { piece_to_buffs }
    }

    fn buffs_mut(&mut self, piece_id: &PieceId) -> &mut HashMap<BuffRepr, Buff, FxBuildHasher> {
        if !self.piece_to_buffs.contains_key(piece_id) {
            self.piece_to_buffs.insert(*piece_id, HashMap::default());
        }
        self.piece_to_buffs.get_mut(piece_id).unwrap()
    }
    
    pub fn add(&mut self, piece_id: &PieceId, buff: Buff) {
        let buffs_mut = self.buffs_mut(piece_id);
        buffs_mut.insert(BuffRepr::from_buff(&buff), buff);
    }

    pub fn add_from_vec(&mut self, piece_id: &PieceId, buffs: Vec<Buff>) {
        for buff in buffs.into_iter() {
            self.add(piece_id, buff)
        }
    }

    pub fn en_passant(&self, piece_id: &PieceId) -> Option<&Buff> {
        if let Some(buffs) = self.piece_to_buffs.get(piece_id) {
            buffs.get(&BuffRepr::EnPassant)
        } else {
            None
        }
    }

    pub fn remove_en_passant(&mut self, piece_id: &PieceId) {
        self.piece_to_buffs.get_mut(piece_id).expect("EnPassant should be there").remove(&BuffRepr::EnPassant);
    }

    pub fn has_castle(&self, piece_id: &PieceId) -> bool {
        if let Some(buffs) = self.piece_to_buffs.get(piece_id) {
            buffs.contains_key(&BuffRepr::Castle)
        } else {
            false
        }
    }

    pub fn has_en_passant(&self, piece_id: &PieceId) -> bool {
        if let Some(buffs) = self.piece_to_buffs.get(piece_id) {
            buffs.contains_key(&BuffRepr::EnPassant)
        } else {
            false
        }
    }

    pub fn has_additional_point(&self, piece_id: &PieceId) -> bool {
        if let Some(buffs) = self.piece_to_buffs.get(piece_id) {
            buffs.contains_key(&BuffRepr::AdditionalPoint)
        } else {
            false
        }
    }

    pub fn remove_castle(&mut self, piece_id: &PieceId) {
        if let Some(buffs) = self.piece_to_buffs.get_mut(piece_id) {
            buffs.remove(&BuffRepr::Castle);
        }
    }

    pub fn remove_additional_point(&mut self, piece_id: &PieceId) {
        if let Some(buffs) = self.piece_to_buffs.get_mut(piece_id) {
            buffs.remove(&BuffRepr::AdditionalPoint);
        }
    }
}
