use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::pieces::{AttackPoints, PieceColor, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct King {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    initial_position: Point,
}

#[derive(Debug)]
pub enum Buff {
    Castle
}

#[derive(Debug)]
pub enum Debuff {
    Check,
    Checkmate,
}

impl PieceInit for King {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, initial_position }
    }
}

impl PieceColor for King {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl AttackPoints for King {
    fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let current_x = current_point.get_x().get_value();
        let current_y = current_point.get_y().get_value();

        let mut points: Vec<Point> = vec![];

        let variants = [
            (current_x + 1, current_y + 1),
            (current_x - 1, current_y - 1),
            (current_x + 1, current_y - 1),
            (current_x - 1, current_y + 1),
            (current_x + 1, current_y),
            (current_x - 1, current_y),
            (current_x, current_y + 1),
            (current_x, current_y - 1),
        ];
        for (x, y) in variants {
            let point = Point::new(x, y);
            if Self::is_attackable(&point, &board, &self.color) {
                points.push(point)
            }
        }
        points
    }
}

impl PrettyPrint for King {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♚' } else { '♔' }.to_string(),
            Color::Black => if INVERT_COLORS { '♔' } else { '♚' }.to_string().to_string(),
        }
    }
}

impl King {
    pub fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}
