use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::directions::Direction;
use crate::directions::jump_direction::JumpDirection;
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vectors::jump_vector::JumpVector;
use crate::vectors::Vector;

#[derive(Debug)]
pub struct Knight {
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

impl PieceInit for Knight {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>,
                  current_position: Point, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, current_position, initial_position }
    }
}

impl PieceColor for Knight {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl AttackPoints for Knight {
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let validator = |point: &Point| {
            board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let mut points: Vec<Point> = vec![];

        let vector = Vector::Jump(JumpVector::new(self.current_position, *board.get_dimension()));
        for direction in Direction::jump_directions() {
            points.append(&mut vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl DefensivePoints for Knight {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let validator = |point: &Point| {
            board.is_ally_cell(point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let mut points: Vec<Point> = vec![];

        let vector = Vector::Jump(JumpVector::new(self.current_position, *board.get_dimension()));
        for direction in Direction::jump_directions() {
            points.append(&mut vector.calc_points(direction, validator, terminator));
        }

        points
    }
}

impl PrettyPrint for Knight {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♞' } else { '♘' }.to_string(),
            Color::Black => if INVERT_COLORS { '♘' } else { '♞' }.to_string(),
        }
    }
}

impl Positioning for Knight {
    fn get_current_position(&self) -> &Point {
        &self.current_position
    }

    fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}
