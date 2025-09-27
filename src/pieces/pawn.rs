use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::directions::diagonal_direction::DiagonalDirection;
use crate::directions::Direction;
use crate::pieces::{AttackPoints, DefensivePoints, Piece, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vectors::diagonal_vector::DiagonalVector;
use crate::vectors::Vector;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    current_position: Point,
    initial_position: Point,
}

#[derive(Debug)]
pub enum Buff {
    EnPassant(Point),
    LevelUp(Piece),
}

#[derive(Debug)]
pub enum Debuff {
    Captured,
}

impl PieceInit for Pawn {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>,
                  current_position: Point, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, current_position, initial_position }
    }
}

impl AttackPoints for Pawn {
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let vector = Vector::Diagonal(DiagonalVector::new(self.current_position, *board.get_dimension()));

        match self.color {
            Color::White => {
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::TopLeft), validator, terminator));
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::TopRight), validator, terminator));
            },
            Color::Black => {
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::BottomLeft), validator, terminator));
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::BottomRight), validator, terminator));
            },
        }
        points
    }
}

impl DefensivePoints for Pawn {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_ally_cell(&point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let vector = Vector::Diagonal(DiagonalVector::new(self.current_position, *board.get_dimension()));

        match self.color {
            Color::White => {
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::TopLeft), validator, terminator));
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::TopRight), validator, terminator));
            },
            Color::Black => {
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::BottomLeft), validator, terminator));
                points.append(&mut vector.calc_points(Direction::Diagonal(DiagonalDirection::BottomRight), validator, terminator));
            },
        }
        points
    }
}

impl Positioning for Pawn {
    fn get_current_position(&self) -> &Point {
        &self.current_position
    }

    fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}

impl PieceColor for Pawn {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl PrettyPrint for Pawn {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♟' } else { '♙' }.to_string(),
            Color::Black => if INVERT_COLORS { '♙' } else { '♟' }.to_string(),
        }
    }
}
