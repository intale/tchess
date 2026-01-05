use crate::piece::Piece;
use crate::point::Point;
use std::rc::Rc;

// Piece Square Tables (PST) implementation
pub trait HeatMap {
    fn positional_value(&self, piece: &Rc<Piece>, position: &Point) -> i16;
}
