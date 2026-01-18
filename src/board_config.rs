use crate::castle_x_points::CastleXPoints;
use crate::color::Color;
use crate::dimension::Dimension;
use crate::heat_map::HeatMap;
use crate::player::Player;
use crate::squares_map::SquaresMap;
use crate::static_piece_weights::StaticPieceWeights;

pub struct BoardConfig {
    king_side_castle_x_points: CastleXPoints,
    queen_side_castle_x_points: CastleXPoints,
    heat_map: Box<dyn HeatMap>,
    squares_map: Box<dyn SquaresMap>,
    dimension: Dimension,
    static_piece_weights: StaticPieceWeights,
    white_side_player: Player,
    black_side_player: Player,
    evaluation_required: bool,
}

impl BoardConfig {
    pub fn new(
        king_side_castle_x_points: CastleXPoints,
        queen_side_castle_x_points: CastleXPoints,
        heat_map: Box<dyn HeatMap>,
        squares_map: Box<dyn SquaresMap>,
        dimension: Dimension,
        static_piece_weights: StaticPieceWeights,
        white_side_player: Player,
        black_side_player: Player,
    ) -> Self {
        let evaluation_required =
            white_side_player == Player::Computer ||
                black_side_player == Player::Computer;
        Self {
            king_side_castle_x_points,
            queen_side_castle_x_points,
            heat_map,
            squares_map,
            dimension,
            static_piece_weights,
            white_side_player,
            black_side_player,
            evaluation_required,
        }
    }

    pub fn static_piece_weights(&self) -> &StaticPieceWeights {
        &self.static_piece_weights
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
    
    pub fn player(&self, color: &Color) -> &Player {
        match color { 
            Color::White => &self.white_side_player,
            Color::Black => &self.black_side_player,
        }
    }

    pub fn is_evaluation_required(&self) -> bool {
        self.evaluation_required
    }
}
