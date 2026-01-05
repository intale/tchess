use crate::dimension::Dimension;
use crate::heat_map::HeatMap;
use crate::squares_map::SquaresMap;

pub struct KingCastleXPoint(pub i16);
pub struct RookCastleXPoint(pub i16);

pub struct CastleXPoints(pub KingCastleXPoint, pub RookCastleXPoint);

impl CastleXPoints {
    pub fn king_x(&self) -> &i16 {
        &self.0.0
    }

    pub fn rook_x(&self) -> &i16 {
        &self.1.0
    }
}

pub struct BoardConfig {
    king_side_castle_x_points: CastleXPoints,
    queen_side_castle_x_points: CastleXPoints,
    heat_map: Box<dyn HeatMap>,
    squares_map: Box<dyn SquaresMap>,
    dimension: Dimension,
}

impl BoardConfig {
    pub fn new(
        king_side_castle_x_points: CastleXPoints,
        queen_side_castle_x_points: CastleXPoints,
        heat_map: Box<dyn HeatMap>,
        squares_map: Box<dyn SquaresMap>,
        dimension: Dimension,
    ) -> Self {
        Self {
            king_side_castle_x_points,
            queen_side_castle_x_points,
            heat_map,
            squares_map,
            dimension,
        }
    }

    pub fn king_side_castle_x_points(&self) -> &CastleXPoints {
        &self.king_side_castle_x_points
    }

    pub fn queen_side_castle_x_points(&self) -> &CastleXPoints {
        &self.queen_side_castle_x_points
    }

    pub fn heat_map(&self) -> &Box<dyn HeatMap> {
        &self.heat_map
    }

    pub fn squares_map(&self) -> &Box<dyn SquaresMap> {
        &self.squares_map
    }

    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }
}
