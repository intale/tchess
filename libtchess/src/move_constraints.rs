use std::collections::BTreeSet;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_id::PieceId;
use crate::move_score::MoveScore;
use crate::moves_map::{MovesSetT, PieceToMovesMapT};
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

    pub fn moves_of(&self, piece_id: &PieceId) -> Option<&MovesSetT> {
        self.constraints.get(piece_id)
    }
    
    fn constraint_moves_mut(&mut self, piece_id: &PieceId) -> &mut MovesSetT {
        if !self.constraints.contains_key(piece_id) {
            self.constraints.insert(*piece_id, FxHashSet::default());
        }
        self.constraints.get_mut(piece_id).unwrap()
    }

    fn s2p_moves_mut(&mut self, score: MoveScore, piece_id: &PieceId) -> &mut MovesSetT {
        if !self.score_to_piece_moves.contains_key(&score) {
            self.score_to_piece_moves.insert(score, FxHashMap::default());
        }
        let pieces_hashmap = self.score_to_piece_moves.get_mut(&score).unwrap();
        if !pieces_hashmap.contains_key(piece_id) {
            pieces_hashmap.insert(*piece_id, FxHashSet::default());
        }
        pieces_hashmap.get_mut(piece_id).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    pub fn add(&mut self, piece_id: &PieceId, piece_move: &PieceMove, score: MoveScore) -> bool {
        self.constraint_moves_mut(piece_id).insert(*piece_move) 
            && self.s2p_moves_mut(score, piece_id).insert(*piece_move) 
            && self.scores.insert(score)
    }

    pub fn move_scores(&self) -> &BTreeSet<MoveScore> {
        &self.scores
    }

    pub fn moves_by_score(&self, score: &MoveScore) -> Option<&PieceToMovesMapT> {
        self.score_to_piece_moves.get(score)
    }
}
