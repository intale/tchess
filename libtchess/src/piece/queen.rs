use crate::board::{Board, INVERT_COLORS};
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::piece::{PieceId, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector_points::VectorPoints;
use std::cell::Cell;
use crate::board_map::BoardMap;
use crate::dimension::Dimension;
use crate::piece_move::PieceMove;
use crate::strategy_point::StrategyPoint;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;

#[derive(Debug)]
pub struct Queen {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Cell<Point>,
    id: PieceId,
}

impl Queen {
    pub fn id(&self) -> &PieceId {
        &self.id
    }

    pub fn buffs(&self) -> &BuffsCollection {
        &self.buffs
    }

    pub fn debuffs(&self) -> &DebuffsCollection {
        &self.debuffs
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn current_position(&self) -> Point {
        self.current_position.get()
    }

    pub fn set_current_position(&self, point: Point) {
        self.current_position.set(point)
    }

    pub fn calculate_strategy_points<F: FnMut(StrategyPoint)>(&self, board_map: &BoardMap, dimension: &Dimension, mut consumer: F) {
        for direction in self.attack_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *dimension,
                direction,
            );
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    consumer(StrategyPoint::DeadEnd(point));
                    break;
                }
                if square.is_empty_square() || square.is_enemy_square(&self.color) {
                    consumer(StrategyPoint::Attack(point));
                }
                if square.is_ally_square(&self.color) {
                    consumer(StrategyPoint::Defense(point));
                }
                if !square.can_look_through(self.color()) {
                    break;
                }
            }
        }
    }

    pub fn calculate_moves<F: FnMut(PieceMove)>(&self, board_map: &BoardMap, dimension: &Dimension, mut consumer: F) {
        let pin = self.debuffs.pin();
        let available_directions =
            if pin.is_none() {
                Vector::diagonal_and_line_vectors()
            } else {
                let pin = pin.unwrap();
                Vector::diagonal_and_line_vectors()
                    .iter()
                    .filter(|&&vec| pin == vec || pin.inverse() == vec)
                    .map(|&vec| vec)
                    .collect::<Vec<_>>()
            };

        for direction in available_directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *dimension,
                direction,
            );
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    break;
                }

                let piece_move = PieceMove::Point(point);

                if square.is_empty_square() ||
                    square.is_capturable_enemy_square(&self.color) {
                    consumer(piece_move)
                }
                if !square.is_empty_square() {
                    break;
                }
            }
        }
    }

    pub fn attack_vectors(&self) -> Vec<Vector> {
        Vector::diagonal_and_line_vectors()
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        if let Some(vector) = DiagonalVector::calc_direction(point1, point2) {
            Some(Vector::Diagonal(vector))
        } else if let Some(vector) = LineVector::calc_direction(point1, point2) {
            Some(Vector::Line(vector))
        } else {
            None
        }
    }
}

impl PieceInit for Queen {
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
            id: PieceId(id),
        }
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
