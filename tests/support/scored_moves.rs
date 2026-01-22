use std::fmt::{Display, Formatter};
use tchess::move_score::MoveScore;
use tchess::piece_move::PieceMove;
use tchess::point::Point;

#[derive(Debug)]
pub struct ScoredMoves {
    piece_name: String,
    position: Point,
    score: MoveScore,
    moves: Vec<PieceMove>,
}

impl ScoredMoves {
    pub fn new(piece_name: &str, position: Point, score: MoveScore, moves: Vec<PieceMove>) -> Self {
        Self {
            piece_name: piece_name.to_string(),
            position,
            score,
            moves
        }
    }
}

impl Display for ScoredMoves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "({}({}), {} {:?})", self.piece_name, self.position, self.score, self.moves)
    }
}

impl PartialEq for ScoredMoves {
    fn eq(&self, other: &Self) -> bool {
        self.piece_name == other.piece_name
            && self.position == other.position
            && self.score == other.score
            && self.moves.iter().all(|m| other.moves.contains(m))
            && self.moves.len() == other.moves.iter().len()
    }
}
