use libtchess::heat_map::HeatMap;
use libtchess::piece::Piece;
use libtchess::point::Point;

pub struct ClassicHeatMap;

impl ClassicHeatMap {
    pub fn empty() -> Self {
        Self {}
    }
}

impl HeatMap for ClassicHeatMap {
    // The actual implementation will be presented later. For now just put stub values here.
    fn positional_value(&self, piece: &Piece, _position: &Point) -> i16 {
        match piece {
            Piece::Bishop(_) => 300,
            Piece::King(_) => 0,
            Piece::Knight(_) => 300,
            Piece::Pawn(_) => 100,
            Piece::Queen(_) => 1000,
            Piece::Rook(_) => 500,
            Piece::UnknownPiece(_) => panic!("Unknown piece can't be evaluated!"),
        }
    }
}
