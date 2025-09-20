use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::diagonal_vector::{DiagonalDirection, DiagonalVector};
use crate::pieces::{AttackPoints, DefensivePoints, Piece, PieceColor, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
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

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, initial_position }
    }
}

impl AttackPoints for Pawn {
    fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let x = current_point.get_x().get_value();
        let y = current_point.get_y().get_value();
        let (max_x, max_y) = board.get_dimension().to_i16_tuple();

        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let vector = DiagonalVector { x, y, max_x, max_y };

        match self.color {
            Color::White => {
                points.append(&mut vector.calc_points(DiagonalDirection::TopLeft, validator, terminator));
                points.append(&mut vector.calc_points(DiagonalDirection::TopRight, validator, terminator));
            },
            Color::Black => {
                points.append(&mut vector.calc_points(DiagonalDirection::BottomLeft, validator, terminator));
                points.append(&mut vector.calc_points(DiagonalDirection::BottomRight, validator, terminator));
            },
        }
        points
    }
}

impl DefensivePoints for Pawn {
    fn defensive_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let x = current_point.get_x().get_value();
        let y = current_point.get_y().get_value();
        let (max_x, max_y) = board.get_dimension().to_i16_tuple();

        let mut points: Vec<Point> = vec![];

        let validator = |point: &Point| {
            board.is_ally_cell(&point, &self.color)
        };
        let terminator = |_point: &Point| {
            true
        };

        let vector = DiagonalVector { x, y, max_x, max_y };

        match self.color {
            Color::White => {
                points.append(&mut vector.calc_points(DiagonalDirection::TopLeft, validator, terminator));
                points.append(&mut vector.calc_points(DiagonalDirection::TopRight, validator, terminator));
            },
            Color::Black => {
                points.append(&mut vector.calc_points(DiagonalDirection::BottomLeft, validator, terminator));
                points.append(&mut vector.calc_points(DiagonalDirection::BottomRight, validator, terminator));
            },
        }
        points
    }
}

impl Pawn {
    pub fn get_initial_position(&self) -> &Point {
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
