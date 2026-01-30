use libtchess::squares_map::SquaresMap;
use libtchess::board_square::{BoardSquare, Square};
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::point::Point;

pub struct TestSquaresMap {
    map: Vec<Vec<Option<Color>>>,
}

impl TestSquaresMap {
    #[allow(unused)]
    pub fn from_chars(chars_map: Vec<Vec<char>>, pov: &Color) -> Self {
        let mut map = vec![];
        for row in chars_map {
            let mut squares_row = vec![];
            for square_notation in row {
                let color = match square_notation {
                    '▓' => Some(Color::White),
                    '░' => Some(Color::Black),
                    '¤' => None,
                    _ => panic!("Unhandled square notation: {}", square_notation),
                };
                squares_row.push(color);
            }
            map.push(squares_row);
        }

        match pov {
            Color::White => {
                map.reverse();
            }
            Color::Black => {
                map = map
                    .into_iter()
                    .map(|mut row| {
                        row.reverse();
                        row
                    })
                    .collect::<Vec<_>>();
            }
        }
        Self { map }
    }

    pub fn from_dimension(dimension: &Dimension) -> Self {
        let mut map = vec![];
        for y in dimension.get_rows_range() {
            let mut squares_row = vec![];
            for x in dimension.get_columns_range() {
                let color = if x.wrapping_add(y) % 2 == 0 {
                    Color::Black
                } else {
                    Color::White
                };
                squares_row.push(Some(color));
            }
            map.push(squares_row);
        }
        Self { map }
    }
}

impl SquaresMap for TestSquaresMap {
    fn square(&self, point: &Point) -> Option<BoardSquare> {
        let color = self.map[(*point.y().value() - 1) as usize][(*point.x().value() - 1) as usize];
        match color {
            Some(c) => Some(BoardSquare::Square(Square::new(c, None))),
            None => None,
        }
    }
}
