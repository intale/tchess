use libtchess::move_score::MoveScore;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct PieceMovesByScore {
    pub piece_name: String,
    pub current_position: Point,
    pub score: MoveScore,
    pub moves: Vec<PieceMove>,
}

impl PieceMovesByScore {
    pub fn new(
        piece_name: &str,
        current_position: Point,
        score: MoveScore,
        moves: Vec<PieceMove>,
    ) -> Self {
        Self {
            piece_name: piece_name.to_string(),
            current_position,
            score,
            moves,
        }
    }
}

impl Display for PieceMovesByScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::write!(
            f,
            "({}({}), {} {:?})",
            self.piece_name,
            self.current_position,
            self.score,
            self.moves
        )
    }
}

impl PartialEq for PieceMovesByScore {
    fn eq(&self, other: &Self) -> bool {
        self.piece_name == other.piece_name
            && self.current_position == other.current_position
            && self.score == other.score
            && self.moves.iter().all(|m| other.moves.contains(m))
            && self.moves.len() == other.moves.iter().len()
    }
}
