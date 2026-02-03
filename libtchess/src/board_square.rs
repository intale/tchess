use crate::color::Color;
use crate::piece_id::PieceId;
use crate::square::Square;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, Clone)]
pub enum BoardSquare {
    Square(Square),
    VoidSquare,
}

impl BoardSquare {
    // This method is used by x-ray capable pieces to calculate their attack points. This allows
    // properly calculate king's move points when it gets checked. So, for example if ally king is
    // on b2 and enemy bishop is on d4 - ally king is not able to move to a1 because a1 is on the
    // bishop's attack diagonal.
    // 4 ▓▓▓ ░░░ ▓▓▓ ░♗░
    // 3 ░░░ ▓▓▓ ░░░ ▓▓▓
    // 2 ▓▓▓ ░♚░ ▓▓▓ ░░░
    // 1 ░░░ ▓▓▓ ░░░ ▓▓▓
    //    a   b   c   d
    pub fn can_look_through(&self, color: &Color, opposite_king_id: Option<&PieceId>) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece_id) = square.get_piece_id() {
                    &piece_id.color() != color && Some(piece_id) == opposite_king_id
                } else {
                    // Empty square
                    true
                }
            }
            Self::VoidSquare => false,
        }
    }

    pub fn is_enemy_square(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece_id) = square.get_piece_id() {
                    &piece_id.color() != color
                } else {
                    false
                }
            }
            Self::VoidSquare => false,
        }
    }

    pub fn is_capturable_enemy_square(
        &self,
        color: &Color,
        opposite_king_id: Option<&PieceId>,
    ) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece_id) = square.get_piece_id() {
                    &piece_id.color() != color && Some(piece_id) != opposite_king_id
                } else {
                    false
                }
            }
            Self::VoidSquare => false,
        }
    }

    pub fn is_ally_square(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece_id) = square.get_piece_id() {
                    &piece_id.color() == color
                } else {
                    false
                }
            }
            Self::VoidSquare => false,
        }
    }

    pub fn is_empty_square(&self) -> bool {
        match self {
            Self::Square(square) => square.get_piece_id().is_none(),
            Self::VoidSquare => false,
        }
    }

    pub fn is_void_square(&self) -> bool {
        match self {
            Self::Square(_) => false,
            Self::VoidSquare => true,
        }
    }

    pub fn get_piece_id(&self) -> Option<&PieceId> {
        match self {
            Self::Square(square) => square.get_piece_id(),
            Self::VoidSquare => None,
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Self::Square(square) => square.color(),
            Self::VoidSquare => panic!("Void square is colorless."),
        }
    }
}

impl PrettyPrint for BoardSquare {
    fn pp(&self) -> String {
        match self {
            Self::Square(square) => square.pp(),
            Self::VoidSquare => " ¤ ".to_string(),
        }
    }
}
