use crate::board::INVERT_COLORS;
use crate::board_map::BoardMap;
use crate::board_square::BoardSquare;
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::debuff::Debuff;
use crate::debuffs_map::DebuffsMap;
use crate::dimension::Dimension;
use crate::piece::PieceInit;
use crate::piece_id::PieceId;
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::strategy_point::StrategyPoint;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector_points::VectorPoints;

#[derive(Debug, Copy, Clone)]
pub struct Bishop {
    color: Color,
    current_position: Point,
    id: PieceId,
}

impl Bishop {
    pub fn id(&self) -> &PieceId {
        &self.id
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
        let bishop_color = self.bishop_color(board_map.board_square(&self.current_position()));
        let opposite_king_id = board_map.king_id(&self.color.inverse());
        for direction in self.attack_vectors() {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() || &bishop_color != square.color() {
                    consumer(StrategyPoint::DeadEnd(point));
                    break;
                }
                if square.is_empty_square() || square.is_enemy_square(&self.color) {
                    consumer(StrategyPoint::Attack(point));
                }
                if square.is_ally_square(&self.color) {
                    consumer(StrategyPoint::Defense(point));
                }
                if !square.can_look_through(&self.color, opposite_king_id) {
                    break;
                }
            }
        }
    }

    pub fn calculate_moves<F: FnMut(PieceMove)>(
        &self,
        board_map: &BoardMap,
        cdebuffs_map: &ColoredProperty<DebuffsMap>,
        dimension: &Dimension,
        mut consumer: F,
    ) {
        let debuff = cdebuffs_map[&self.color].pin(&self.id);
        let available_directions = if debuff.is_none() {
            Vector::diagonal_vectors()
        } else {
            let debuff = debuff.unwrap();
            let pin_vector =
                match debuff {
                    Debuff::Pin(v) => v,
                    _ => panic!("Logical error! Expected pin debuff, but got {:?}", debuff),
                };
            Vector::diagonal_vectors()
                .iter()
                .filter(|&vec| pin_vector == vec || &pin_vector.inverse() == vec)
                .map(|&vec| vec)
                .collect::<Vec<_>>()
        };
        let bishop_color = self.bishop_color(board_map.board_square(&self.current_position));
        let opposite_king_id = board_map.king_id(&self.color.inverse());

        for direction in available_directions {
            let vector_points =
                VectorPoints::without_initial(self.current_position, *dimension, direction);
            for point in vector_points {
                let square = board_map.board_square(&point);

                if square.is_void_square() || &bishop_color != square.color() {
                    break;
                }

                let piece_move = PieceMove::Point(point);

                if square.is_empty_square()
                    || square.is_capturable_enemy_square(&self.color, opposite_king_id)
                {
                    consumer(piece_move);
                }
                if !square.is_empty_square() {
                    break;
                }
            }
        }
    }

    fn bishop_color(&self, board_square: &BoardSquare) -> Color {
        match board_square {
            BoardSquare::Square(square) => *square.color(),
            BoardSquare::VoidSquare => {
                panic!(
                    "Logical error. Bishop {:#?} is placed on void square!",
                    self
                )
            }
        }
    }

    pub fn attack_vectors(&self) -> Vec<Vector> {
        Vector::diagonal_vectors()
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        if let Some(vector) = DiagonalVector::calc_direction(point1, point2) {
            Some(Vector::Diagonal(vector))
        } else {
            None
        }
    }
}

impl PieceInit for Bishop {
    fn from_parts(
        color: Color,
        current_position: Point,
        id: PieceId,
    ) -> Self {
        Self {
            color,
            current_position,
            id,
        }
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
