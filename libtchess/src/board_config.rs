use crate::castle_x_points::CastleXPoints;
use crate::color::Color;
use crate::dimension::Dimension;
use crate::heat_map::HeatMap;
use crate::player::Player;
use crate::squares_map::SquaresMap;

#[derive(Clone)]
pub struct BoardConfig<HT, SM> {
    king_side_castle_x_points: CastleXPoints,
    queen_side_castle_x_points: CastleXPoints,
    heat_map: HT,
    squares_map: SM,
    dimension: Dimension,
    white_side_player: Player,
    black_side_player: Player,
    evaluation_required: bool,
}

impl<HT, SM> BoardConfig<HT, SM>
where
    HT: HeatMap,
    SM: SquaresMap,
{
    pub fn new(
        king_side_castle_x_points: CastleXPoints,
        queen_side_castle_x_points: CastleXPoints,
        heat_map: HT,
        squares_map: SM,
        dimension: Dimension,
        white_side_player: Player,
        black_side_player: Player,
    ) -> BoardConfig<HT, SM> {
        let evaluation_required =
            white_side_player == Player::Computer || black_side_player == Player::Computer;
        Self {
            king_side_castle_x_points,
            queen_side_castle_x_points,
            heat_map,
            squares_map,
            dimension,
            white_side_player,
            black_side_player,
            evaluation_required,
        }
    }

    pub fn king_side_castle_x_points(&self) -> &CastleXPoints {
        &self.king_side_castle_x_points
    }

    pub fn queen_side_castle_x_points(&self) -> &CastleXPoints {
        &self.queen_side_castle_x_points
    }

    pub fn heat_map(&self) -> &HT {
        &self.heat_map
    }

    pub fn squares_map(&self) -> &SM {
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
