use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Buff {
    Castle,
    EnPassant(Point, Point),
    AdditionalPoint, // A pawn buff to allow going one additional point further
}
