use libtchess::board_square::{BoardSquare};
use libtchess::color::Color;
use libtchess::point::Point;
use libtchess::square::Square;
use libtchess::squares_map::SquaresMap;

pub struct ClassicSquaresMap;

impl ClassicSquaresMap {
    pub fn init() -> Self {
        Self {}
    }
}

impl SquaresMap for ClassicSquaresMap {
    fn square(&self, point: &Point) -> Option<BoardSquare> {
        let color = if point.x().value().wrapping_add(*point.y().value()) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        };
        Some(BoardSquare::Square(Square::new(color, None)))
    }
}
