use crate::board::{Board, INVERT_COLORS};
use crate::buff::Buff;
use crate::color::Color;
use crate::debuff::Debuff;
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector_points::{VectorPoints};

#[derive(Debug)]
pub struct King {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    current_position: Point,
    id: usize,
}

impl King {
    pub fn add_debuff(&mut self, debuff: Debuff) {
        self.debuffs.push(debuff)
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn buffs(&self) -> &Vec<Buff> {
        &self.buffs
    }

    pub fn debuffs(&self) -> &Vec<Debuff> {
        &self.debuffs
    }
}

impl PieceInit for King {
    fn from_parts(color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                  current_position: Point, id: usize) -> Self {
        Self { color, buffs, debuffs, current_position, id }
    }
}

impl PieceColor for King {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl AttackPoints for King {
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for direction in Vector::diagonal_and_line_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position, *board.get_dimension(), direction
            );
            for point in vector_points {
                if board.is_empty_cell(&point) || board.is_enemy_cell(&point, &self.color) {
                    points.push(point)
                }
                break
            }
        }

        points
    }
}

impl DefensivePoints for King {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for direction in Vector::diagonal_and_line_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position, *board.get_dimension(), direction
            );
            for point in vector_points {
                if board.is_ally_cell(&point, &self.color) {
                    points.push(point)
                }
                break
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

impl Positioning for King {
    fn get_current_position(&self) -> &Point {
        &self.current_position
    }
}
