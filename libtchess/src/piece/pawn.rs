use crate::board::{INVERT_COLORS};
use crate::board_map::BoardMap;
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::dimension::Dimension;
use crate::piece::{PieceId, PieceInit};
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::promote_piece::PromotePiece;
use crate::strategy_point::StrategyPoint;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;

#[derive(Debug, Clone)]
pub struct Pawn {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Point,
    id: PieceId,
}

impl Pawn {
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
        // Attack/defense directions
        for direction in self.attack_vectors() {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

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
        // Move direction
        let direction = match self.color {
            Color::White => Vector::Line(LineVector::Top),
            Color::Black => Vector::Line(LineVector::Bottom),
        };
        let vector_points =
            VectorPoints::without_initial(self.current_position, *dimension, direction);
        let mut points_calculated: u8 = 0;
        for point in vector_points {
            let square = board_map.board_square(&point);

            if square.is_void_square() {
                consumer(StrategyPoint::DeadEnd(point));
                break;
            }
            if square.is_empty_square() {
                consumer(StrategyPoint::Move(point));
            } else {
                consumer(StrategyPoint::BlockedMove(point));
                break;
            }
            points_calculated += 1;
            if self.buffs.has_additional_point() && points_calculated == 1 {
                continue;
            }
            break;
        }
    }

    pub fn calculate_moves<F: FnMut(PieceMove)>(
        &self,
        board_map: &BoardMap,
        dimension: &Dimension,
        mut consumer: F,
    ) {
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

        if !pin.is_none() {
            let pin = pin.unwrap();
            available_directions = available_directions
                .iter()
                .filter(|&&vec| pin == vec || pin.inverse() == vec)
                .map(|&vec| vec)
                .collect::<Vec<_>>();
        }

        let pre_promote_position = match self.color {
            Color::White => {
                &(dimension.max_point().y().value() - 1) == self.current_position.y().value()
            }
            Color::Black => {
                &(dimension.min_point().y().value() + 1) == self.current_position.y().value()
            }
        };
        for direction in available_directions {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            let mut points_calculated = 0;
            let opposite_king_id = board_map.king_id(&self.color.inverse());

            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    break;
                }
                match direction {
                    Vector::Diagonal(_) => {
                        if let Some((en_passant, enemy_piece_point)) = self.buffs.en_passant()
                            && en_passant == point
                        {
                            consumer(PieceMove::EnPassant(en_passant, enemy_piece_point));
                        } else {
                            if square.is_capturable_enemy_square(&self.color, opposite_king_id) {
                                if pre_promote_position {
                                    for variant in PromotePiece::all_variants() {
                                        consumer(PieceMove::Promote(point, variant))
                                    }
                                } else {
                                    consumer(PieceMove::Point(point))
                                }
                            }
                        }
                    }
                    Vector::Line(_) => {
                        if square.is_empty_square() {
                            if pre_promote_position {
                                for variant in PromotePiece::all_variants() {
                                    consumer(PieceMove::Promote(point, variant))
                                }
                                break;
                            }
                            if points_calculated == 1 {
                                consumer(PieceMove::LongMove(point))
                            } else {
                                consumer(PieceMove::Point(point))
                            }
                        }
                        points_calculated += 1;
                        if self.buffs.has_additional_point() && points_calculated < 2 {
                            continue;
                        }
                    }
                    _ => (),
                }
                break;
            }
        }
    }

    pub fn attack_vectors(&self) -> Vec<Vector> {
        match self.color {
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
        }
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        self.attack_vectors().into_iter().find(|v| {
            let mut vector_points = VectorPoints::with_initial(
                self.current_position,
                Dimension::new(*point1, *point2),
                *v,
            );
            // Pawns can't attack more than one square far - no matter how much moves they have.
            // Thus, we additionally limit the result by distance of 1 square.
            vector_points.next().is_some()
        })
    }
}

impl PieceInit for Pawn {
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

impl PrettyPrint for Pawn {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♟' } else { '♙' }.to_string(),
            Color::Black => if INVERT_COLORS { '♙' } else { '♟' }.to_string(),
        }
    }
}
