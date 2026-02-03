use libtchess::heat_map::HeatMap;
use libtchess::piece::Piece;
use libtchess::point::Point;

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
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![325, 325, 330, 345, 300, 300, 300, 300],
            vec![325, 325, 325, 330, 300, 300, 300, 300],
            vec![310, 325, 325, 330, 300, 300, 300, 300],
            vec![300, 310, 310, 310, 300, 300, 300, 300],
        ];
        let mut map_king = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, -10, 0, 0, 0, 0],
            vec![0, 0, 20, -10, 0, 0, 0, 0],
        ];
        let mut map_knight = vec![
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
            vec![300, 300, 300, 300, 300, 300, 300, 300],
        ];
        let mut map_pawn = vec![
            vec![100, 100, 100, 100, 100, 100, 100, 100],
            vec![100, 100, 100, 100, 100, 100, 100, 100],
            vec![100, 100, 100, 100, 100, 100, 100, 100],
            vec![100, 100, 100, 100, 100, 100, 100, 100],
            vec![105, 105, 115, 130, 100, 100, 100, 100],
            vec![105, 100, 110, 120, 100, 100, 100, 100],
            vec![105, 105, 105,  70, 100, 100, 100, 100],
            vec![100, 100, 100, 100, 100, 100, 100, 100],
        ];
        let mut map_queen = vec![
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
            vec![800, 800, 800, 800, 800, 800, 800, 800],
        ];
        let mut map_rook = vec![
            vec![500, 500, 500, 500, 500, 500, 500, 500],
            vec![500, 500, 500, 500, 500, 500, 500, 500],
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
    fn positional_value(&self, piece: &Piece, position: &Point) -> i16 {
        let x = *position.x().value() as usize - 1;
        let y = *position.y().value() as usize - 1;

        match piece {
            Piece::Bishop(_) => self.map_bishop[y][x],
            Piece::King(_) => self.map_king[y][x],
            Piece::Knight(_) => self.map_knight[y][x],
            Piece::Pawn(_) => self.map_pawn[y][x],
            Piece::Queen(_) => self.map_queen[y][x],
            Piece::Rook(_) => self.map_rook[y][x],
            Piece::UnknownPiece(_) => panic!("Can't evaluate an unknown piece."),
        }
    }
}
