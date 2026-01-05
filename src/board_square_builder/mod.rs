pub mod default_square_builder;

use crate::board_square::BoardSquare;
use crate::point::Point;

pub trait BoardSquareBuilder {
    fn init() -> Self;
    fn build(&self, point: &Point) -> Option<BoardSquare>;
}
