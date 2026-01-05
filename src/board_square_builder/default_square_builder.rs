use crate::board_square::BoardSquare;
use super::BoardSquareBuilder;
use crate::color::Color;
use crate::point::Point;
use crate::square::Square;

pub struct DefaultSquareBuilder;

impl BoardSquareBuilder for DefaultSquareBuilder {
    fn init() -> Self {
        Self {}
    }

    fn build(&self, point: &Point) -> Option<BoardSquare> {
        let color = if point.x().wrapping_add(**point.y()) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        };
        Some(BoardSquare::Square(Square::new(color, None)))
    }
}
