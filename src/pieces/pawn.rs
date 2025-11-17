use crate::board::{Board, INVERT_COLORS};
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::pieces::{PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector_points::VectorPoints;
use std::cell::Cell;
use crate::piece_move::PieceMove;
use crate::vector::line_vector::LineVector;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Cell<Point>,
    id: usize,
}

impl Pawn {
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

        let directions = match self.color {
            Color::White => {
                vec![
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    Vector::Diagonal(DiagonalVector::TopRight),
                ]
            }
            Color::Black => {
                vec![
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                ]
            }
        };
        for direction in directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_empty_cell(&point) || board.is_enemy_cell(&point, &self.color) {
                    points.push(point)
                }
                break;
            }
        }
        points
    }

    pub fn defensive_points(&self, board: &Board) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let directions = match self.color {
            Color::White => {
                vec![
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    Vector::Diagonal(DiagonalVector::TopRight),
                ]
            }
            Color::Black => {
                vec![
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                ]
            }
        };
        for direction in directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            for point in vector_points {
                if board.is_ally_cell(&point, &self.color) {
                    points.push(point)
                }
                break;
            }
        }
        points
    }

    pub fn moves(&self, board: &Board) -> Vec<PieceMove> {
        let pin = self.debuffs.pin();
        let mut available_directions = match self.color {
            Color::White => {
                vec![
                    Vector::Line(LineVector::Top),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    Vector::Diagonal(DiagonalVector::TopRight),
                ]
            }
            Color::Black => {
                vec![
                    Vector::Line(LineVector::Bottom),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                ]
            }
        };
        let mut moves: Vec<PieceMove> = vec![];

        if !pin.is_none() {
            let pin = pin.unwrap();
            available_directions = available_directions
                .iter()
                .filter(|&&vec| pin == vec || pin.inverse() == vec)
                .map(|&vec| vec)
                .collect::<Vec<_>>();
        }

        for direction in available_directions {
            let vector_points = VectorPoints::without_initial(
                self.current_position.get(),
                *board.get_dimension(),
                direction,
            );
            let mut points_calculated = 0;

            for point in vector_points {
                match direction {
                    Vector::Diagonal(_) => {
                        if let Some((en_passant, enemy_piece_point)) = self.buffs.en_passant()
                            && en_passant == point {
                            moves.push(PieceMove::EnPassant(en_passant, enemy_piece_point));
                        } else {
                            if board.is_capturable_enemy_cell(&point, &self.color) {
                                moves.push(PieceMove::Point(point))
                            }
                        }

                    },
                    Vector::Line(_) => {
                        let piece_move = if points_calculated == 1 {
                            PieceMove::LongMove(point)
                        } else {
                            PieceMove::Point(point)
                        };

                        if board.is_empty_cell(&point) {
                            moves.push(piece_move)
                        }
                        points_calculated += 1;
                        if self.buffs.has_additional_point() && points_calculated < 2 {
                            continue;
                        }
                    },
                    _ => (),
                }
                break;
            }
        }

        moves
    }
}

impl PieceInit for Pawn {
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

impl PrettyPrint for Pawn {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♟' } else { '♙' }.to_string(),
            Color::Black => if INVERT_COLORS { '♙' } else { '♟' }.to_string(),
        }
    }
}
