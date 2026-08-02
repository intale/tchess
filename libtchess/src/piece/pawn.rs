use rustc_hash::FxHashSet;
use crate::board::{INVERT_COLORS};
use crate::board_map::BoardMap;
use crate::buff::Buff;
use crate::buffs_map::BuffsMap;
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::debuff::Debuff;
use crate::debuffs_map::DebuffsMap;
use crate::dimension::Dimension;
use crate::piece::{PieceId, PieceInit};
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::promote_piece::PromotePiece;
use crate::segment::Segment;
use crate::strategy_point::StrategyPoint;
use crate::strategy_segments::StrategySegments;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;

#[derive(Debug, Copy, Clone)]
pub struct Pawn {
    color: Color,
    current_position: Point,
    id: PieceId,
}

impl Pawn {
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
        cbuffs_map: &ColoredProperty<BuffsMap>,
        strategy_segments: &StrategySegments,
    ) -> Vec<Segment> {
        let mut segments: Vec<Segment> = vec![];
        let distance = |direction: &Vector, point: &Point| -> i32 {
            if &self.current_position == point {
                return 0;
            }
            let error_message = format!(
                "Logical error: distance between {} and {} using {:?} direction must be determined",
                self.current_position, point, direction
            );
            direction.distance(&self.current_position, point).expect(error_message.as_str())
        };
        // Attack/defense directions
        for direction in self.attack_vectors() {
            if let Some(nearest_seg) =
                strategy_segments.nearest_segment(&self.current_position, &direction) {
                if distance(&direction, nearest_seg.point1()) != 1 {
                    let mut vector_points =
                        VectorPoints::with_initial(self.current_position, *dimension, direction);
                    if let Some(point) = vector_points.next() {
                        segments.push(Segment::Attack(
                            self.current_position,
                            point,
                            direction,
                            self.id,
                        ));
                    }
                    continue
                }
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
                let mut vector_points =
                    VectorPoints::with_initial(self.current_position, *dimension, direction);
                if let Some(point) = vector_points.next() {
                    segments.push(Segment::Attack(
                        self.current_position,
                        point,
                        direction,
                        self.id,
                    ));
                }
            }
        }

        // Move directions
        for direction in self.move_vectors() {
            let nearest_seg =
                strategy_segments.nearest_segment(&self.current_position, &direction);
            let vector_points = VectorPoints::with_initial(self.current_position, *dimension, direction);
            if cbuffs_map[&self.color].has_additional_point(&self.id) {
                if let Some(nearest_seg) = nearest_seg {
                    match distance(&direction, nearest_seg.point1()) {
                        ..=2 => {
                            let segment =
                                match nearest_seg {
                                    Segment::DeadEnd(_, _) =>  {
                                        Segment::DeadEndMove(
                                            self.current_position,
                                            *nearest_seg.point1(),
                                            direction,
                                            self.id,
                                        )
                                    }
                                    _ => {
                                        Segment::BlockedMove(
                                            self.current_position,
                                            *nearest_seg.point1(),
                                            direction,
                                            self.id,
                                        )
                                    }
                                };
                            segments.push(segment);
                        }
                        _ => {
                            let segment = Segment::Move(
                                self.current_position,
                                vector_points.point_at_distance(2).unwrap(),
                                direction,
                                self.id,
                            );
                            segments.push(segment);
                        }
                    }
                } else {
                    let &distance = [distance(&direction, &vector_points.last_point()), 2].iter().min().unwrap();
                    if distance == 0 {
                        continue
                    }
                    let segment = Segment::Move(
                        self.current_position,
                        vector_points.point_at_distance(distance as u16).unwrap(),
                        direction,
                        self.id,
                    );
                    segments.push(segment);
                }
            } else {
                let &distance = [distance(&direction, &vector_points.last_point()), 1].iter().min().unwrap();
                if distance == 0 {
                    continue
                }
                let segment = Segment::Move(
                    self.current_position,
                    vector_points.point_at_distance(distance as u16).unwrap(),
                    direction,
                    self.id,
                );
                segments.push(segment);
            }
        }
        segments
    }

    pub fn calculate_moves<F: FnMut(PieceMove)>(
        &self,
        board_map: &BoardMap,
        cbuffs_map: &ColoredProperty<BuffsMap>,
        cdebuffs_map: &ColoredProperty<DebuffsMap>,
        dimension: &Dimension,
        mut consumer: F,
    ) {
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

        if let Some(debuff) = cdebuffs_map[&self.color].pin(&self.id) {
            let pin_vector =
                match debuff {
                    Debuff::Pin(v) => v,
                    _ => panic!("Logical error! Expected pin debuff, but got {:?}", debuff),
                };
            available_directions = available_directions
                .iter()
                .filter(|&vec| pin_vector == vec || &pin_vector.inverse() == vec)
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
                        if let Some(buff) = cbuffs_map[&self.color].en_passant(&self.id)
                        {
                            let (en_passant, enemy_piece_point) =
                                match buff {
                                    Buff::EnPassant(p1, p2) => (*p1, *p2),
                                    _ => panic!("Logical error! EnPassant buff is expected, but got: {:?}", buff),
                                };
                            if en_passant == point {
                                consumer(PieceMove::EnPassant(en_passant, enemy_piece_point));
                            }
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
                        if cbuffs_map[&self.color].has_additional_point(&self.id) && points_calculated < 2 {
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

    pub fn move_vectors(&self) -> Vec<Vector> {
        match self.color {
            Color::White => vec![Vector::Line(LineVector::Top)],
            Color::Black => vec![Vector::Line(LineVector::Bottom)],
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

impl PrettyPrint for Pawn {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♟' } else { '♙' }.to_string(),
            Color::Black => if INVERT_COLORS { '♙' } else { '♟' }.to_string(),
        }
    }
}
