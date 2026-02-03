use crate::piece::Piece;
use crate::point::Point;


// Piece Square Tables (PST) implementation
pub trait HeatMap {
    fn positional_value(&self, piece: &Piece, position: &Point) -> i16;
}
