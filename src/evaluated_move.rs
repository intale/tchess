use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::move_score::MoveScore;
use crate::piece_move::PieceMove;
use crate::point::Point;

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub struct EvaluatedMove {
    piece_move: PieceMove,
    score: MoveScore,
}

impl EvaluatedMove {
    pub fn new(score: MoveScore, piece_move: PieceMove) -> Self {
        Self {
            score,
            piece_move,
        }
    }

    pub fn destination(&self) -> Option<Point> {
        self.piece_move.destination()
    }

    pub fn score(&self) -> &MoveScore {
        &self.score
    }

    pub fn piece_move(&self) -> &PieceMove {
        &self.piece_move
    }
}

impl PartialOrd for EvaluatedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EvaluatedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(other.score()).then(self.piece_move().cmp(&other.piece_move()))
    }
}

impl Display for EvaluatedMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.piece_move.fmt(f).expect("Successful write is expected.");
        write!(f, "<{}>", self.score)
    }
}
