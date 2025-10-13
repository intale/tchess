use crate::pieces::Piece;
use crate::point::Point;

#[derive(Debug, PartialEq)]
pub enum Buff {
    Castle,
    EnPassant(Point),
    LevelUp(Piece)
}
