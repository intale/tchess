use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::pieces::{AttackPoints, Piece, PieceColor, PieceInit};
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
        let current_x = current_point.get_x().get_value();
        let current_y = current_point.get_y().get_value();
        let mut points: Vec<Point> = vec![];

        match self.color {
            Color::White => {
                // White pawn can only attack (x - 1, y + 1) and (x + 1, y + 1) points
                let point = Point::new(current_x - 1, current_y + 1);
                if Self::is_attackable(&point, &board, &self.color) {
                    points.push(point)
                }
                let point = Point::new(current_x + 1, current_y + 1);
                if Self::is_attackable(&point, &board, &self.color) {
                    points.push(point)
                }
            },
            Color::Black => {
                // Black pawn can only attack (x - 1, y - 1) and (x + 1, y - 1) points
                let point = Point::new(current_x - 1, current_y - 1);
                if Self::is_attackable(&point, &board, &self.color) {
                    points.push(point)
                }
                let point = Point::new(current_x + 1, current_y - 1);
                if Self::is_attackable(&point, &board, &self.color) {
                    points.push(point)
                }
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
