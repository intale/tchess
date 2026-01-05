use std::rc::Rc;
use tchess::heat_map::HeatMap;
use tchess::piece::Piece;
use tchess::point::Point;

pub struct TestHeatMap;

impl TestHeatMap {
    pub(crate) fn empty() -> Self {
        Self {}
    }
}

impl HeatMap for TestHeatMap {
    fn positional_value(&self, piece: &Rc<Piece>, _position: &Point) -> i16 {
        match &**piece {
            Piece::Bishop(_) => 300,
            Piece::King(_) => 0,
            Piece::Knight(_) => 300,
            Piece::Pawn(_) => 100,
            Piece::Queen(_) => 1000,
            Piece::Rook(_) => 500,
        }
    }
}
