use std::rc::Rc;
use crate::color::Color;
use crate::piece::Piece;
pub use crate::square::Square;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub enum BoardSquare {
    Square(Square),
    VoidSquare
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
    pub fn can_look_through(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece) = square.get_piece() {
                    match &**piece {
                        Piece::King(_) => piece.color() != color,
                        _ => false
                    }
                } else {
                    // Empty square
                    true
                }
            },
            Self::VoidSquare => false,
        }
    }

    pub fn is_enemy_square(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece) = square.get_piece() {
                    !piece.is_ally(color)
                } else {
                    false
                }
            },
            Self::VoidSquare => false,
        }
    }

    pub fn is_capturable_enemy_square(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece) = square.get_piece() {
                    if piece.is_ally(color) {
                        return false
                    }
                    match &**piece {
                        Piece::King(_) => false,
                        _ => true,
                    }
                } else {
                    false
                }
            },
            Self::VoidSquare => false,
        }
    }

    pub fn is_ally_square(&self, color: &Color) -> bool {
        match self {
            Self::Square(square) => {
                if let Some(piece) = square.get_piece() {
                    piece.is_ally(color)
                } else {
                    false
                }
            },
            Self::VoidSquare => false,
        }
    }

    pub fn is_empty_square(&self) -> bool {
        match self {
            Self::Square(square) => {
                square.get_piece().is_none()
            },
            Self::VoidSquare => false,
        }
    }

    pub fn is_void_square(&self) -> bool {
        match self {
            Self::Square(_) => false,
            Self::VoidSquare => true,
        }
    }

    pub fn get_piece(&self) -> Option<&Rc<Piece>> {
        match self {
            Self::Square(square) => square.get_piece(),
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
