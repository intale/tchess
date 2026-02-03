use crate::board::INVERT_COLORS;
use crate::board_config::BoardConfig;
use crate::board_map::BoardMap;
use crate::buff::{Buff, BuffsCollection};
use crate::castle_points::{CastlePoints, CastleSide};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::dimension::Dimension;
use crate::piece::{Piece, PieceInit};
use crate::piece_id::PieceId;
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::strategy_point::StrategyPoint;
use crate::strategy_points::StrategyPoints;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;

#[derive(Debug)]
pub struct King {
    color: Color,
    buffs: BuffsCollection,
    debuffs: DebuffsCollection,
    current_position: Point,
    id: PieceId,
}

impl King {
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
        board_config: &BoardConfig,
        opposite_strategy_points: &StrategyPoints,
        mut consumer: F,
    ) {
        let available_directions = Vector::diagonal_and_line_vectors();

        for direction in available_directions {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() {
                    break;
                }
                if square.is_empty_square() && !opposite_strategy_points.is_under_attack(&point) {
                    consumer(PieceMove::Point(point));
                    break;
                }
                if square.is_enemy_square(&self.color)
                    && !opposite_strategy_points.is_under_enemy_defense(&point)
                {
                    consumer(PieceMove::Point(point));
                }
                break;
            }
        }

        if !self.debuffs.has_check() && self.buffs.has_castle() {
            self.castle_moves(
                board_map,
                dimension,
                board_config,
                opposite_strategy_points,
                consumer,
            );
        }
    }

    fn castle_moves<F: FnMut(PieceMove)>(
        &self,
        board_map: &BoardMap,
        dimension: &Dimension,
        board_config: &BoardConfig,
        opposite_strategy_points: &StrategyPoints,
        mut consumer: F,
    ) {
        let current_position = self.current_position;

        for (king_point, rook_point, side) in self.castle_points(board_config) {
            let mut king_path_is_safe = false;
            let mut rook_path_is_safe = false;
            let mut ally_rook: Option<&Piece> = None;
            if current_position == king_point {
                // King is already on its position - no need to calculate its path.
                king_path_is_safe = true;
            } else {
                let direction = LineVector::calc_direction(&current_position, &king_point)
                    .unwrap_or_else(|| {
                        panic!(
                            "King at {:#?} is not on the same line with its castle point {:#?}",
                            current_position, king_point
                        );
                    });
                let direction = Vector::Line(direction);
                let points = VectorPoints::without_initial(current_position, *dimension, direction);

                for point in points {
                    let square = board_map.board_square(&point);

                    if square.is_void_square() {
                        break;
                    }
                    if opposite_strategy_points.is_under_attack(&point) {
                        break;
                    }
                    if let Some(piece_id) = square.get_piece_id() {
                        let piece = board_map.find_piece_by_id(piece_id);
                        if piece.is_enemy(self.color()) {
                            break;
                        }

                        match piece {
                            Piece::Rook(_) => {
                                if direction == side.direction() && ally_rook.is_none() {
                                    // King meets a rook when looking up its path to its castle
                                    // point. In this case we can say that rook's path is safe,
                                    // too
                                    rook_path_is_safe = true;
                                    ally_rook = Some(piece);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    if point == king_point {
                        king_path_is_safe = true;
                        break;
                    }
                }
            }

            if !king_path_is_safe {
                continue;
            }

            // Find rook on the board. The rook to castle with should be to the left for queen
            // side castle and to the right for king side castle. There should not be any
            // pieces between the king and the rook.
            if ally_rook.is_none() {
                let points =
                    VectorPoints::without_initial(current_position, *dimension, side.direction());

                for point in points {
                    if let Some(piece) = board_map.piece_at(&point) {
                        if piece.is_enemy(self.color()) {
                            break;
                        }
                        match piece {
                            Piece::Rook(_) => {
                                ally_rook = Some(piece);
                                break;
                            }
                            _ => break,
                        }
                    }
                }
            }

            if let Some(rook) = ally_rook {
                // Because during castle the king and the rook swap their places - it is
                // important to make sure rook is not pinned. If it is pinned then it means
                // castle will result in check which would be illegal move. Such kind of pin
                // is possible in chess 960. We may skip further checks in this case.
                if !(rook.buffs().has_castle() && rook.debuffs().pin().is_none()) {
                    continue;
                }

                // Rook was placed outside of king's path to king's castle point. Thus, we
                // have to make sure the rook's path to its castle point is safe as well.
                if !rook_path_is_safe {
                    let direction =
                        LineVector::calc_direction(&rook.current_position(), &rook_point);
                    if let Some(direction) = direction {
                        let points = VectorPoints::without_initial(
                            *rook.current_position(),
                            *dimension,
                            Vector::Line(direction),
                        );

                        for point in points {
                            let square = board_map.board_square(&point);

                            if square.is_void_square() {
                                break;
                            }
                            if let Some(piece_id) = square.get_piece_id() {
                                let piece = board_map.find_piece_by_id(piece_id);
                                if piece.is_enemy(self.color()) {
                                    break;
                                }
                                match piece {
                                    Piece::King(_) => continue,
                                    _ => break,
                                }
                            }
                            if point == rook_point {
                                rook_path_is_safe = true;
                                break;
                            }
                        }
                    }
                }
                if rook_path_is_safe {
                    consumer(PieceMove::Castle(CastlePoints::new(
                        king_point,
                        rook_point,
                        self.current_position,
                        *rook.current_position(),
                    )))
                }
            }
        }
    }

    fn castle_points(&self, config: &BoardConfig) -> [(Point, Point, CastleSide); 2] {
        let king_side_points = config.king_side_castle_x_points();
        let queen_side_points = config.queen_side_castle_x_points();
        [
            (
                Point::new(
                    *king_side_points.king_x(),
                    *self.current_position().y().value(),
                ),
                Point::new(
                    *king_side_points.rook_x(),
                    *self.current_position().y().value(),
                ),
                CastleSide::King,
            ),
            (
                Point::new(
                    *queen_side_points.king_x(),
                    *self.current_position().y().value(),
                ),
                Point::new(
                    *queen_side_points.rook_x(),
                    *self.current_position().y().value(),
                ),
                CastleSide::Queen,
            ),
        ]
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

impl PieceInit for King {
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

impl PrettyPrint for King {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♚' } else { '♔' }.to_string(),
            Color::Black => if INVERT_COLORS { '♔' } else { '♚' }
                .to_string()
                .to_string(),
        }
    }
}
