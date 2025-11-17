use std::cmp::PartialEq;
use crate::castle_points::CastlePoints;
use crate::point::Point;

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum PieceMove {
    Point(Point),
    EnPassant(Point, Point),
    Castle(CastlePoints),
    // This move is relate to pawns. More specifically - when a pawn is moved by two points instead
    // of one.
    LongMove(Point),
    // This variant indicates a move not possible to complete.
    UnreachablePoint,
}

impl PieceMove {
    pub fn destination(&self) -> Option<Point> {
        match self {
            Self::Point(point) |
            Self::EnPassant(point, _) |
            Self::LongMove(point) => {
                Some(*point)
            },
            Self::Castle(castle_points) => {
                Some(*castle_points.king_point())
            },
            Self::UnreachablePoint => { None },
        }
    }
}
