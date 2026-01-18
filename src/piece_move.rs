use std::cmp::{PartialEq};
use std::fmt::{Display, Formatter};
use crate::castle_points::{CastlePoints, CastleSide};
use crate::point::Point;
use crate::promote_piece::PromotePiece;

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum PieceMove {
    Point(Point),
    // first point is where a pawn should land and second point is enemy pawn position
    EnPassant(Point, Point),
    Castle(CastlePoints),
    // This move is related to pawns. More specifically - when a pawn is moved by two points instead
    // of one. We later use it to calculate EnPassant moves for the opposite color.
    LongMove(Point),
    Promote(Point, PromotePiece),
    // This variant indicates a move not possible to complete.
    UnreachablePoint,
}

impl PieceMove {
    pub fn destination(&self) -> Option<Point> {
        match self {
            Self::Point(point) |
            Self::EnPassant(point, _) |
            Self::LongMove(point) | 
            Self::Promote(point, _) => { Some(*point) },
            Self::Castle(castle_points) => {
                Some(*castle_points.king_point())
            },
            Self::UnreachablePoint => { None },
        }
    }
}

impl Display for PieceMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point(point) => write!(f, "PieceMove::Move{}", point),
            Self::EnPassant(new_pos, enemy_pos) => {
                write!(f, "PieceMove::EnPassant({}, {})", new_pos, enemy_pos)
            },
            Self::Castle(castle_points) => {
                let side =
                    match castle_points.side() {
                        CastleSide::Queen => "Queen",
                        CastleSide::King => "King",
                    };
                write!(f, "PieceMove::Castle({} side)", side)
            },
            Self::LongMove(point) => write!(f, "PieceMove::LongMove{}", point),
            Self::Promote(point, promote_piece) => {
                write!(f, "PieceMove::Promote({} on {})", promote_piece.name(), point)
            },
            Self::UnreachablePoint => write!(f, "PieceMove::UnreachablePoint"),
        }
    }
}