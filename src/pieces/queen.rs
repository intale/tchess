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
pub struct Queen {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Cell<Point>,
    id: usize,
}

impl Queen {
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

        for direction in Vector::diagonal_and_line_vectors() {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_empty_cell(&point) || board.is_enemy_cell(&point, &self.color) {
                    points.push(point)
                }
                if !board.can_look_through(&point, self.color()) {
                    break;
                }
            }
        }

        points
    }

    pub fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        for direction in Vector::diagonal_and_line_vectors() {
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

    pub fn moves(&self, board: &Board) -> Vec<PieceMove> {
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
        let mut moves: Vec<PieceMove> = vec![];

        for direction in available_directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                let piece_move = PieceMove::Point(point);
                if (board.is_empty_cell(&point) || board.is_capturable_enemy_cell(&point, &self.color))
                    && board.matches_constraints(&piece_move, self.color()) {
                    moves.push(piece_move)
                }
                if !board.is_empty_cell(&point) {
                    break;
                }
            }
        }

        moves
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
            id,
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
