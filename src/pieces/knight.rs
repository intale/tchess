use crate::board::{Board, INVERT_COLORS};
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::pieces::{PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector_points::VectorPoints;
use std::cell::Cell;
use crate::piece_move::PieceMove;

#[derive(Debug)]
pub struct Knight {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Cell<Point>,
    id: usize,
}

impl Knight {
    pub fn id(&self) -> usize {
        self.id
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

    pub fn attack_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for direction in Vector::jump_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_empty_square(&point) || board.is_enemy_square(&point, &self.color) {
                    points.push(point)
                }
                break;
            }
        }

        points
    }

    pub fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for direction in Vector::jump_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_ally_square(&point, &self.color) {
                    points.push(point)
                }
                break;
            }
        }

        points
    }

    pub fn moves(&self, board: &Board) -> Vec<PieceMove> {
        if self.debuffs.pin().is_some() {
            // Pinned knight has no legal moves as there is no other mechanics that pins using jump
            // vectors. Thus, there can't be any other non-pinned jump vector that would allow some
            // moves.
            return vec![];
        }

        let mut moves: Vec<PieceMove> = vec![];

        for direction in Vector::jump_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.dimension(),
                direction,
            );
            for point in vector_points {
                let piece_move = PieceMove::Point(point);
                if board.is_empty_square(&point) ||
                    board.is_capturable_enemy_square(&point, &self.color) {
                    moves.push(piece_move)
                }
                break;
            }
        }

        moves
    }
}

impl PieceInit for Knight {
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

impl PrettyPrint for Knight {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♞' } else { '♘' }.to_string(),
            Color::Black => if INVERT_COLORS { '♘' } else { '♞' }.to_string(),
        }
    }
}
