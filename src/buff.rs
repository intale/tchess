use crate::pieces::Piece;
use crate::point::Point;

#[derive(Debug)]
pub enum Buff {
    Castle,
    EnPassant(Point),
    LevelUp(Piece)
}
