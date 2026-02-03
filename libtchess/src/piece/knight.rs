use crate::board::{INVERT_COLORS};
use crate::board_map::BoardMap;
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::dimension::Dimension;
use crate::piece::{PieceId, PieceInit};
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::strategy_point::StrategyPoint;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::jump_vector::JumpVector;
use crate::vector_points::VectorPoints;

#[derive(Debug, Clone)]
pub struct Knight {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Point,
    id: PieceId,
}

impl Knight {
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

    pub fn current_position(&self) -> &Point {
        &self.current_position
    }

    pub fn set_current_position(&mut self, point: Point) {
        self.current_position = point;
    }

    pub fn calculate_strategy_points<F: FnMut(StrategyPoint)>(
        &self,
        board_map: &BoardMap,
        dimension: &Dimension,
        mut consumer: F,
    ) {
        for direction in self.attack_vectors() {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    consumer(StrategyPoint::DeadEnd(point));
                    break;
                }
                if square.is_empty_square() || square.is_enemy_square(&self.color) {
                    consumer(StrategyPoint::Attack(point));
                    break;
                }
                if square.is_ally_square(&self.color) {
                    consumer(StrategyPoint::Defense(point));
                }
                break;
            }
        }
    }

    pub fn calculate_moves<F: FnMut(PieceMove)>(
        &self,
        board_map: &BoardMap,
        dimension: &Dimension,
        mut consumer: F,
    ) {
        if self.debuffs.pin().is_some() {
            // Pinned knight has no legal moves as there is no other mechanics that pins using jump
            // vectors. Thus, there can't be any other non-pinned jump vector that would allow some
            // moves.
            return;
        }
        let opposite_king_id = board_map.king_id(&self.color.inverse());

        for direction in Vector::jump_vectors() {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    break;
                }

                let piece_move = PieceMove::Point(point);

                if square.is_empty_square()
                    || square.is_capturable_enemy_square(&self.color, opposite_king_id)
                {
                    consumer(piece_move)
                }
                break;
            }
        }
    }

    pub fn attack_vectors(&self) -> Vec<Vector> {
        Vector::jump_vectors()
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        if let Some(vector) = JumpVector::calc_direction(point1, point2) {
            Some(Vector::Jump(vector))
        } else {
            None
        }
    }
}

impl PieceInit for Knight {
    fn from_parts(
        color: Color,
        buffs: Vec<Buff>,
        debuffs: Vec<Debuff>,
        current_position: Point,
        id: PieceId,
    ) -> Self {
        Self {
            color,
            buffs: BuffsCollection::new(buffs),
            debuffs: DebuffsCollection::new(debuffs),
            current_position,
            id,
        }
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
