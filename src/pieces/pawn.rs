use std::cell::Cell;
use crate::board::{Board, INVERT_COLORS};
use crate::buff::Buff;
use crate::color::Color;
use crate::debuff::Debuff;
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::Vector;
use crate::vector_points::{VectorPoints};

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    current_position: Cell<Point>,
    id: usize,
}

impl Pawn {
    pub fn add_debuff(&mut self, debuff: Debuff) {
        self.debuffs.push(debuff);
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

impl PieceInit for Pawn {
    fn from_parts(color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                  current_position: Point, id: usize) -> Self {
        Self { color, buffs, debuffs, current_position: Cell::new(current_position), id }
    }
}

impl AttackPoints for Pawn {
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let directions = match self.color {
            Color::White => {
                vec![
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    Vector::Diagonal(DiagonalVector::TopRight),
                ]
            },
            Color::Black => {
                vec![
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                ]
            },
        };
        for direction in directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(), *board.get_dimension(), direction
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

impl DefensivePoints for Pawn {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let directions = match self.color {
            Color::White => {
                vec![
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    Vector::Diagonal(DiagonalVector::TopRight),
                ]
            },
            Color::Black => {
                vec![
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                ]
            },
        };
        for direction in directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(), *board.get_dimension(), direction
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

impl Positioning for Pawn {
    fn get_current_position(&self) -> Point {
        self.current_position.get()
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
