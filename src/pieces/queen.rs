use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::diagonal_vector::{DiagonalDirection, DiagonalVector};
use crate::line_vector::{LineDirection, LineVector};
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Queen {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    initial_position: Point,
}

#[derive(Debug)]
pub enum Buff {}

#[derive(Debug)]
pub enum Debuff {
    Captured,
}

impl PieceInit for Queen {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, initial_position }
    }
}

impl PieceColor for Queen {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl PrettyPrint for Queen {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♛' } else { '♕' }.to_string(),
            Color::Black => if INVERT_COLORS { '♕' } else { '♛' }.to_string(),
        }
    }
}

impl AttackPoints for Queen {
    fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let x = current_point.get_x().get_value();
        let y = current_point.get_y().get_value();
        let (max_x, max_y) = board.get_dimension().to_i16_tuple();

        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color)
        };
        let terminator = |point: &Point| {
            !board.is_empty_cell(&point)
        };

        let diagonal_vector = DiagonalVector { x, y, max_x, max_y };
        let line_vector = LineVector { x, y, max_x, max_y };

        for direction in DiagonalDirection::all_variants() {
            points.append(&mut diagonal_vector.calc_points(direction, validator, terminator));
        }
        for direction in LineDirection::all_variants() {
            points.append(&mut line_vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl DefensivePoints for Queen {
    fn defensive_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let x = current_point.get_x().get_value();
        let y = current_point.get_y().get_value();
        let (max_x, max_y) = board.get_dimension().to_i16_tuple();

        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_ally_cell(&point, &self.color)
        };
        let terminator = |point: &Point| {
            !board.is_empty_cell(&point)
        };

        let diagonal_vector = DiagonalVector { x, y, max_x, max_y };
        let line_vector = LineVector { x, y, max_x, max_y };

        for direction in DiagonalDirection::all_variants() {
            points.append(&mut diagonal_vector.calc_points(direction, validator, terminator));
        }
        for direction in LineDirection::all_variants() {
            points.append(&mut line_vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl Queen {
    pub fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}
