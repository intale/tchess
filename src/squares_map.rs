use crate::board_square::BoardSquare;
use crate::point::Point;

pub trait SquaresMap {
    fn square(&self, point: &Point) -> Option<BoardSquare>;
}
