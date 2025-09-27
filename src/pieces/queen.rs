use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::directions::diagonal_direction::DiagonalDirection;
use crate::directions::Direction;
use crate::directions::line_direction::LineDirection;
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vectors::diagonal_vector::DiagonalVector;
use crate::vectors::line_vector::LineVector;
use crate::vectors::Vector;

#[derive(Debug)]
pub struct Queen {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    current_position: Point,
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

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>,
                  current_position: Point, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, current_position, initial_position }
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
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color)
        };
        let terminator = |point: &Point| {
            !board.is_empty_cell(&point)
        };

        let diagonal_vector = Vector::Diagonal(DiagonalVector::new(self.current_position, *board.get_dimension()));
        let line_vector = Vector::Line(LineVector::new(self.current_position, *board.get_dimension()));

        for direction in Direction::diagonal_directions() {
            points.append(&mut diagonal_vector.calc_points(direction, validator, terminator));
        }
        for direction in Direction::line_directions() {
            points.append(&mut line_vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl DefensivePoints for Queen {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_ally_cell(&point, &self.color)
        };
        let terminator = |point: &Point| {
            !board.is_empty_cell(&point)
        };

        let diagonal_vector = Vector::Diagonal(DiagonalVector::new(self.current_position, *board.get_dimension()));
        let line_vector = Vector::Line(LineVector::new(self.current_position, *board.get_dimension()));

        for direction in Direction::diagonal_directions() {
            points.append(&mut diagonal_vector.calc_points(direction, validator, terminator));
        }
        for direction in Direction::line_directions() {
            points.append(&mut line_vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl Positioning for Queen {
    fn get_current_position(&self) -> &Point {
        &self.current_position
    }

    fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}
