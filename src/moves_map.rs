use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_move::PieceMove;
use crate::pieces::Piece;

type MovesSetT = FxHashSet<PieceMove>;
type MovesMapT = FxHashMap<Rc<Piece>, MovesSetT>;

pub struct MovesMap {
    moves: MovesMapT,
    constraints: MovesSetT,
}

impl MovesMap {
    pub fn empty() -> Self {
        let moves = FxHashMap::default();
        let constraints = FxHashSet::default();
        Self { moves, constraints }
    }

    fn moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.moves.contains_key(piece) {
            self.moves.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.moves.get_mut(piece).unwrap()
    }

    pub fn moves(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.moves.get(piece)
    }

    pub fn add(&mut self, piece: &Rc<Piece>, piece_move: PieceMove) -> bool {
        self.moves_mut(piece).insert(piece_move)
    }

    pub fn clear(&mut self, piece: &Rc<Piece>) -> Option<MovesSetT> {
        self.moves.remove(piece)
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear()
    }

    pub fn add_constraint(&mut self, piece_move: PieceMove) -> bool {
        self.constraints.insert(piece_move)
    }

    pub fn matches_constraints(&self, piece_move: &PieceMove) -> bool {
        if self.constraints.is_empty() {
            return true;
        }
        self.constraints.contains(piece_move)
    }
}
