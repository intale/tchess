use std::collections::BTreeSet;
use crate::evaluated_move::EvaluatedMove;
use crate::scoped_evaluated_move::ScopedEvaluatedMove;

pub struct EvaluatedMoveConstraints {
    constraints: BTreeSet<ScopedEvaluatedMove>,
    has_constraints: bool,
}

impl EvaluatedMoveConstraints {
    pub fn empty() -> Self {
        Self { constraints: BTreeSet::default(), has_constraints: false }
    }

    pub fn add(&mut self, evaluated_move: EvaluatedMove, piece_id: &usize) {
        self.constraints.insert(ScopedEvaluatedMove(evaluated_move, *piece_id));
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

    pub fn collection(&self) -> &BTreeSet<ScopedEvaluatedMove> {
        &self.constraints
    }
}
