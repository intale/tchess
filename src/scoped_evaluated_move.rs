use std::fmt::{Display, Formatter};
use crate::evaluated_move::EvaluatedMove;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct ScopedEvaluatedMove(pub EvaluatedMove, pub usize);

impl Display for ScopedEvaluatedMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f).expect("Successful write is expected.");
        write!(f, "#{}", self.1)
    }
}
