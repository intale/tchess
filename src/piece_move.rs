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
}
