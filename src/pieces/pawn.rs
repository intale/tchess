use crate::board::{Board, BoardDimension, INVERT_COLORS};
use crate::cell::Cell;
use crate::color::Color;
use crate::pieces::{MovePiece, Piece, PieceColor, PieceConstraints, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
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

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>) -> Self {
        Self {
            color,
            buffs,
            debuffs,
        }
    }
}

impl Pawn {
    pub fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let points = self.possible_attack_points(
            current_point.get_x().get_value(),
            current_point.get_y().get_value(),
        );

        points
            .into_iter()
            .filter(|point| {
                board.is_in_boundaries(point)
                    && (board.is_empty_cell(point) || board.is_enemy_cell(point, &self.color))
            })
            .collect()
    }

    // Pawn can only attack (x - 1, y + 1) and (x + 1, y + 1) points
    pub fn possible_attack_points(&self, x: i16, y: i16) -> Vec<Point> {
        match self.color {
            Color::White => vec![Point::new(x - 1, y + 1), Point::new(x + 1, y + 1)],
            Color::Black => vec![Point::new(x - 1, y - 1), Point::new(x + 1, y - 1)],
        }
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
