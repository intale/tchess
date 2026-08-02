use rustc_hash::FxHashSet;
use crate::board::{INVERT_COLORS};
use crate::board_map::BoardMap;
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::debuff::Debuff;
use crate::debuffs_map::DebuffsMap;
use crate::dimension::Dimension;
use crate::piece::{PieceId, PieceInit};
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::segment::Segment;
use crate::strategy_point::StrategyPoint;
use crate::strategy_segments::StrategySegments;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;

#[derive(Debug, Copy, Clone)]
pub struct Rook {
    color: Color,
    current_position: Point,
    id: PieceId,
}

impl Rook {
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

    pub fn calculate_strategy_points(
        &self,
        dimension: &Dimension,
        strategy_segments: &StrategySegments,
    ) -> Vec<Segment> {
        let mut segments: Vec<Segment> = vec![];
        for direction in self.attack_vectors() {
            if let Some(nearest_seg) =
                strategy_segments.nearest_segment(&self.current_position, &direction) {
                let segment = match nearest_seg {
                    Segment::DeadEnd(seg_point, _) => Segment::DeadEndAttack(
                        self.current_position,
                        *seg_point,
                        direction,
                        self.id,
                    ),
                    _ => {
                        if nearest_seg.piece_id().color() == self.color {
                            Segment::Defense(
                                self.current_position,
                                *nearest_seg.point1(),
                                direction,
                                self.id,
                            )
                        } else {
                            Segment::Attack(
                                self.current_position,
                                *nearest_seg.point1(),
                                direction,
                                self.id,
                            )
                        }
                    },
                };
                segments.push(segment);
            } else {
                let vector_points =
                    VectorPoints::without_initial(self.current_position, *dimension, direction);
                let last_point = vector_points.last_point();
                if last_point != self.current_position {
                    segments.push(Segment::Attack(
                        self.current_position,
                        last_point,
                        direction,
                        self.id,
                    ));
                }
            }
        }
        segments
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
            Vector::line_vectors()
        } else {
            let debuff = debuff.unwrap();
            let pin_vector =
                match debuff {
                    Debuff::Pin(v) => v,
                    _ => panic!("Logical error! Expected pin debuff, but got {:?}", debuff),
                };
            Vector::line_vectors()
                .iter()
                .filter(|&vec| pin_vector == vec || &pin_vector.inverse() == vec)
                .map(|&vec| vec)
                .collect::<Vec<_>>()
        };
        let opposite_king_id = board_map.king_id(&self.color.inverse());

        for direction in available_directions {
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
                if !square.is_empty_square() {
                    break;
                }
            }
        }
    }

    pub fn attack_vectors(&self) -> Vec<Vector> {
        Vector::line_vectors()
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        if let Some(vector) = LineVector::calc_direction(point1, point2) {
            Some(Vector::Line(vector))
        } else {
            None
        }
    }
}

impl PieceInit for Rook {
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

impl PrettyPrint for Rook {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♜' } else { '♖' }.to_string(),
            Color::Black => if INVERT_COLORS { '♖' } else { '♜' }.to_string(),
        }
    }
}
