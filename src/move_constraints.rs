use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::moves_map::{MovesSetT, PieceToMovesMapT};
use crate::piece::Piece;
use crate::piece_move::PieceMove;

// Moves map of pieces, except the king when the king is in check
#[derive(Debug)]
pub struct MoveConstraints {
    constraints: PieceToMovesMapT,
    has_constraints: bool,
}

impl MoveConstraints {
    pub fn empty() -> Self {
        let constraints = FxHashMap::default();
        Self { constraints, has_constraints: false }
    }

    pub fn is_enabled(&self) -> bool {
        self.has_constraints
    }

    pub fn enable(&mut self) {
        self.has_constraints = true;
    }

    pub fn clear(&mut self) {
        self.has_constraints = false;
        self.constraints.clear();
    }

    pub fn moves_of(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.constraints.get(piece)
    }
    
    fn moves_of_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.constraints.contains_key(piece) {
            self.constraints.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.constraints.get_mut(piece).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn add_move(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove) {
        self.moves_of_mut(piece).insert(*piece_move);
    }
}
