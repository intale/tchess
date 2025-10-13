use crate::board::{Board, INVERT_COLORS};
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::pieces::{AttackPoints, DefensivePoints, PieceColor, PieceInit, Positioning};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector_points::VectorPoints;
use std::cell::Cell;

#[derive(Debug)]
pub struct Bishop {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Cell<Point>,
    id: usize,
}

impl Bishop {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn buffs(&self) -> &BuffsCollection {
        &self.buffs
    }

    pub fn debuffs(&self) -> &DebuffsCollection {
        &self.debuffs
    }
}

impl PieceInit for Bishop {
    fn from_parts(
        color: Color,
        buffs: Vec<Buff>,
        debuffs: Vec<Debuff>,
        current_position: Point,
        id: usize,
    ) -> Self {
        Self {
            color,
            buffs: BuffsCollection::new(buffs),
            debuffs: DebuffsCollection::new(debuffs),
            current_position: Cell::new(current_position),
            id,
        }
    }
}

impl PieceColor for Bishop {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl AttackPoints for Bishop {
    fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for direction in Vector::diagonal_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_empty_cell(&point) || board.is_enemy_cell(&point, &self.color) {
                    points.push(point)
                }
                if !board.is_empty_cell(&point) {
                    break;
                }
            }
        }

        points
    }
}

impl DefensivePoints for Bishop {
    fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for direction in Vector::diagonal_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_ally_cell(&point, &self.color) {
                    points.push(point)
                }
                if !board.is_empty_cell(&point) {
                    break;
                }
            }
        }

        points
    }
}

impl PrettyPrint for Bishop {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♝' } else { '♗' }.to_string(),
            Color::Black => if INVERT_COLORS { '♗' } else { '♝' }.to_string(),
        }
    }
}

impl Positioning for Bishop {
    fn get_current_position(&self) -> Point {
        self.current_position.get()
    }
}
