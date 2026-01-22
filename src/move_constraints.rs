use std::collections::BTreeSet;
use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::move_score::MoveScore;
use crate::moves_map::{MovesSetT, PieceToMovesMapT};
use crate::piece::Piece;
use crate::piece_move::PieceMove;

// Moves map of pieces, used in situations when the king is in check.
#[derive(Debug)]
pub struct MoveConstraints {
    scores: BTreeSet<MoveScore>,
    score_to_piece_moves: FxHashMap<MoveScore, PieceToMovesMapT>,
    constraints: PieceToMovesMapT,
    has_constraints: bool,
}

impl MoveConstraints {
    pub fn empty() -> Self {
        let scores = BTreeSet::default();
        let score_to_piece_moves = FxHashMap::default();
        let constraints = FxHashMap::default();
        Self {
            scores,
            score_to_piece_moves,
            constraints,
            has_constraints: false
        }
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
        self.score_to_piece_moves.clear();
        self.scores.clear();
    }

    pub fn moves_of(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.constraints.get(piece)
    }
    
    fn constraint_moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.constraints.contains_key(piece) {
            self.constraints.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.constraints.get_mut(piece).unwrap()
    }

    fn s2p_moves_mut(&mut self, score: MoveScore, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.score_to_piece_moves.contains_key(&score) {
            self.score_to_piece_moves.insert(score, FxHashMap::default());
        }
        let pieces_hashmap = self.score_to_piece_moves.get_mut(&score).unwrap();
        if !pieces_hashmap.contains_key(piece) {
            pieces_hashmap.insert(Rc::clone(piece), FxHashSet::default());
        }
        pieces_hashmap.get_mut(piece).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn add(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove, score: MoveScore) -> bool {
        self.constraint_moves_mut(piece).insert(*piece_move) 
            && self.s2p_moves_mut(score, piece).insert(*piece_move) 
            && self.scores.insert(score)
    }

    pub fn move_scores(&self) -> &BTreeSet<MoveScore> {
        &self.scores
    }

    pub fn moves_by_score(&self, piece: &Rc<Piece>, score: &MoveScore) -> Option<&MovesSetT> {
        if let Some(pieces) = &self.score_to_piece_moves.get(score) {
            return pieces.get(piece)
        }
        None
    }
}
