use tchess::board_square::{BoardSquare, Square};
use tchess::board_square_builder::BoardSquareBuilder;
use tchess::color::Color;
use tchess::point::Point;

pub struct TestSquareBuilder {
    map: Vec<Vec<Option<Color>>>,
    pub pov: Color,
}

impl TestSquareBuilder {
    pub fn set_pov(&mut self, color: &Color) {
        self.pov = *color;
    }

    pub fn set_map(&mut self, map: Vec<Vec<Option<Color>>>) {
        match self.pov {
            Color::White => {
                let mut map = map.clone();
                map.reverse();
                self.map = map;
            },
            Color::Black => {
                let map = map.clone().into_iter().map(|mut row| {
                    row.reverse();
                    row
                }).collect::<Vec<_>>();
                self.map = map;
            },
        }
    }
}

impl BoardSquareBuilder for TestSquareBuilder {
    fn init() -> Self {
        Self { map: vec![], pov: Color::White }
    }

    fn build(&self, point: &Point) -> Option<BoardSquare> {
        let color = self.map[(*point.y().value() - 1) as usize][(*point.x().value() - 1) as usize];
        match color {
            Some(c) => Some(BoardSquare::Square(Square::new(c, None))),
            None => None,
        }
    }
}
