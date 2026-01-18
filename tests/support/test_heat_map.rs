use std::rc::Rc;
use tchess::heat_map::HeatMap;
use tchess::piece::Piece;
use tchess::point::Point;

pub struct TestHeatMap {
    map_bishop: Vec<Vec<i16>>,
    map_king: Vec<Vec<i16>>,
    map_knight: Vec<Vec<i16>>,
    map_pawn: Vec<Vec<i16>>,
    map_queen: Vec<Vec<i16>>,
    map_rook: Vec<Vec<i16>>,
}

impl TestHeatMap {
    pub fn init() -> Self {
        let mut map_bishop = vec![
            vec![300, 320, 320, 320, 320, 320, 320, 300],
            vec![305, 320, 320, 320, 320, 320, 320, 305],
            vec![310, 320, 320, 325, 325, 320, 320, 310],
            vec![310, 330, 330, 350, 350, 330, 330, 310],
            vec![325, 325, 330, 345, 345, 330, 325, 325],
            vec![325, 325, 325, 330, 330, 325, 325, 325],
            vec![310, 325, 325, 330, 330, 325, 325, 310],
            vec![300, 310, 310, 310, 310, 310, 310, 300],
        ];
        let mut map_king = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, -10, -10, 0, 0, 0],
            vec![0, 0, 20, -10, -10, 0, 20, 0],
        ];
        let mut map_knight = vec![
            vec![290, 300, 300, 300, 300, 300, 300, 290],
            vec![300, 305, 305, 305, 305, 305, 305, 300],
            vec![300, 305, 325, 325, 325, 325, 305, 300],
            vec![300, 305, 325, 325, 325, 325, 305, 300],
            vec![300, 305, 325, 325, 325, 325, 305, 300],
            vec![300, 305, 320, 325, 325, 325, 305, 300],
            vec![300, 305, 305, 305, 305, 305, 305, 300],
            vec![290, 310, 300, 300, 300, 300, 310, 290],
        ];
        let mut map_pawn = vec![
            vec![100, 100, 100, 100, 100, 100, 100, 100],
            vec![160, 160, 160, 160, 170, 160, 160, 160],
            vec![140, 140, 140, 150, 160, 140, 140, 140],
            vec![120, 120, 120, 140, 150, 120, 120, 120],
            vec![105, 105, 115, 130, 140, 110, 105, 105],
            vec![105, 100, 110, 120, 130, 105, 105, 105],
            vec![105, 105, 105,  70,  70, 105, 105, 105],
            vec![100, 100, 100, 100, 100, 100, 100, 100],
        ];
        let mut map_queen = vec![
            vec![870, 880, 890, 890, 890, 890, 880, 870],
            vec![880, 890, 895, 895, 895, 895, 890, 880],
            vec![890, 895, 910, 910, 910, 910, 895, 890],
            vec![890, 895, 910, 920, 920, 910, 895, 890],
            vec![890, 895, 910, 920, 920, 910, 895, 890],
            vec![890, 895, 895, 895, 895, 895, 895, 890],
            vec![880, 890, 895, 895, 895, 895, 890, 880],
            vec![870, 880, 890, 890, 890, 890, 880, 870],
        ];
        let mut map_rook = vec![
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![515, 515, 515, 520, 520, 515, 515, 515],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 510, 510, 510, 500, 500],
        ];
        map_bishop.reverse();
        map_king.reverse();
        map_knight.reverse();
        map_pawn.reverse();
        map_queen.reverse();
        map_rook.reverse();
        Self {
            map_bishop,
            map_king,
            map_knight,
            map_pawn,
            map_queen,
            map_rook, 
        }
    }
}

impl HeatMap for TestHeatMap {
    fn positional_value(&self, piece: &Rc<Piece>, position: &Point) -> i16 {
        let x = *position.x().value() as usize - 1;
        let y = *position.y().value() as usize - 1;

        match &**piece {
            Piece::Bishop(_) => self.map_bishop[y][x],
            Piece::King(_) => self.map_king[y][x],
            Piece::Knight(_) => self.map_knight[y][x],
            Piece::Pawn(_) => self.map_pawn[y][x],
            Piece::Queen(_) => self.map_queen[y][x],
            Piece::Rook(_) => self.map_rook[y][x],
        }
    }
}
