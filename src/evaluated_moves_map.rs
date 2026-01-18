use std::collections::{BTreeSet};
use rustc_hash::FxHashMap;
use crate::evaluated_move::EvaluatedMove;
use crate::scoped_evaluated_move::ScopedEvaluatedMove;

pub struct EvaluatedMovesMap {
    moves: BTreeSet<ScopedEvaluatedMove>,
    piece_to_moves: FxHashMap<usize, Vec<EvaluatedMove>>
}

impl EvaluatedMovesMap {
    pub fn empty() -> Self {
        Self {
            moves: BTreeSet::default(),
            piece_to_moves: FxHashMap::default(),
        }
    }

    pub fn add(&mut self, evaluated_move: EvaluatedMove, piece_id: &usize) {
        self.moves.insert(ScopedEvaluatedMove(evaluated_move, *piece_id));
        if !self.piece_to_moves.contains_key(piece_id) {
            self.piece_to_moves.insert(*piece_id, vec![]);
        }
        let piece_moves = self.piece_to_moves.get_mut(piece_id).unwrap();
        piece_moves.push(evaluated_move);
    }

    pub fn remove_all_for(&mut self, piece_id: &usize) {
        if let Some(piece_moves) = self.piece_to_moves.remove(piece_id) {
            for piece_move in piece_moves {
                self.moves.remove(&ScopedEvaluatedMove(piece_move, *piece_id));
            }
        }
    }

    pub fn collection(&self) -> &BTreeSet<ScopedEvaluatedMove> {
        &self.moves
    }
}

