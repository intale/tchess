use crate::dimension::Dimension;
use crate::piece::Piece;
use crate::piece_id::PieceId;
use crate::point::Point;
use crate::segment::Segment;
use crate::strategy_point::StrategyPoint;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::ops::Bound;

struct ChangesSlice {
    piece_to_points: Vec<(PieceId, FxHashSet<StrategyPoint>)>,
}

struct AddedPoints(FxHashSet<StrategyPoint>);
struct RemovedPoints(FxHashSet<StrategyPoint>);

#[derive(Debug, Clone)]
enum Event {
    PieceChanged(FxHashSet<StrategyPoint>),
    PieceRemoved(FxHashSet<StrategyPoint>),
}

impl Event {
    pub fn add_point(&mut self, strategy_point: StrategyPoint) {
        match self {
            Self::PieceChanged(points) => points.insert(strategy_point),
            Self::PieceRemoved(points) => points.insert(strategy_point),
        };
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum EventRepr {
    PieceChanged,
    PieceRemoved,
}

#[derive(Debug, Copy, Clone)]
enum SegmentRepr {
    Horizontal(Segment),
    Vertical(Segment),
    DiagonalPos(Segment),
    DiagonalNeg(Segment),
    Jump(Segment),
}

impl SegmentRepr {
    pub fn from(seg: Segment) -> Self {
        match seg.vector() {
            Vector::Line(line_vec) => match line_vec {
                LineVector::Left | LineVector::Right => Self::Horizontal(seg),
                LineVector::Top | LineVector::Bottom => Self::Vertical(seg),
            },
            Vector::Diagonal(diag_vec) => match diag_vec {
                DiagonalVector::TopRight | DiagonalVector::BottomLeft => Self::DiagonalPos(seg),
                DiagonalVector::TopLeft | DiagonalVector::BottomRight => Self::DiagonalNeg(seg),
            },
            Vector::Jump(_) => Self::Jump(seg),
        }
    }

    pub fn seg(&self) -> &Segment {
        match self {
            Self::Horizontal(seg) => seg,
            Self::Vertical(seg) => seg,
            Self::DiagonalPos(seg) => seg,
            Self::DiagonalNeg(seg) => seg,
            Self::Jump(seg) => seg,
        }
    }
}

impl PartialOrd for SegmentRepr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SegmentRepr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Horizontal(seg), Self::Horizontal(other_seg)) => seg
                .point1()
                .x()
                .cmp(other_seg.point1().x())
                .then(seg.vector().cmp(other_seg.vector())),
            (Self::Vertical(seg), Self::Vertical(other_seg)) => seg
                .point1()
                .y()
                .cmp(other_seg.point1().y())
                .then(seg.vector().cmp(other_seg.vector())),
            (Self::DiagonalPos(seg), Self::DiagonalPos(other_seg)) => seg
                .point1()
                .cmp(other_seg.point1())
                .then(seg.vector().cmp(other_seg.vector())),
            (Self::DiagonalNeg(seg), Self::DiagonalNeg(other_seg)) => other_seg
                .point1()
                .x()
                .cmp(seg.point1().x())
                .then(seg.point1().y().cmp(other_seg.point1().y()))
                .then(seg.vector().cmp(other_seg.vector())),
            (Self::Jump(seg), Self::Jump(other_seg)) => seg
                .point1()
                .cmp(other_seg.point1())
                .then(seg.vector().cmp(other_seg.vector())),
            _ => panic!("Don't know how to compare {:?} with {:?}", self, other),
        }
    }
}

impl Hash for SegmentRepr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Jump(seg) => match seg {
                Segment::Partial(_, _) | Segment::DeadEnd(_, _) => seg.point1().hash(state),
                _ => seg.point2().hash(state),
            },
            _ => panic!("Don't know how to hash {:?}", self),
        }
    }
}

impl PartialEq for SegmentRepr {
    fn eq(&self, other: &Self) -> bool {
        let cmp_non_jump = |seg: &Segment, other_seg: &Segment| -> bool {
            match (seg, other_seg) {
                (Segment::DeadEnd(_, _), _) | (_, Segment::DeadEnd(_, _)) => {
                    seg.point1() == other_seg.point1()
                }
                (Segment::Partial(_, _), _) | (_, Segment::Partial(_, _)) => {
                    seg.point1() == other_seg.point1()
                }
                _ => seg.point1() == other_seg.point1() && seg.point2() == other_seg.point2(),
            }
        };
        match (self, other) {
            (Self::DiagonalPos(seg), Self::DiagonalPos(other_seg)) => cmp_non_jump(seg, other_seg),
            (Self::DiagonalNeg(seg), Self::DiagonalNeg(other_seg)) => cmp_non_jump(seg, other_seg),
            (Self::Vertical(seg), Self::Vertical(other_seg)) => cmp_non_jump(seg, other_seg),
            (Self::Horizontal(seg), Self::Horizontal(other_seg)) => cmp_non_jump(seg, other_seg),
            (Self::Jump(seg), Self::Jump(other_seg)) => match (seg, other_seg) {
                (Segment::DeadEnd(_, _) | Segment::Partial(_, _), _) => {
                    seg.point1() == other_seg.point2()
                }
                (_, Segment::DeadEnd(_, _) | Segment::Partial(_, _)) => {
                    seg.point2() == other_seg.point1()
                }
                _ => seg.point2() == seg.point2(),
            },
            _ => panic!("Can't compare {:?} with {:?}", self, other),
        }
    }
}

impl Eq for SegmentRepr {}

#[derive(Debug, Clone)]
pub struct StrategySegments {
    horizontal_segments: FxHashMap<Point, BTreeSet<SegmentRepr>>,
    vertical_segments: FxHashMap<Point, BTreeSet<SegmentRepr>>,
    diagonal_pos_segments: FxHashMap<Point, BTreeSet<SegmentRepr>>,
    diagonal_neg_segments: FxHashMap<Point, BTreeSet<SegmentRepr>>,
    jump_segments: FxHashMap<Point, FxHashSet<SegmentRepr>>,

    track_changes: bool,
    current_changes: Option<FxHashMap<PieceId, Vec<Event>>>,
    historical_changes: Vec<FxHashMap<PieceId, Vec<Event>>>,
    dimension: Dimension,
}

pub struct SegmentsIter<'a> {
    strategy_points: &'a StrategySegments,
    point: Point,
    diagonal_and_line_vectors: Vec<Vector>,
    jump_vectors: Vec<Vector>,
    diagonal_and_line_vectors_pos: usize,
    jump_vectors_pos: usize,
}

impl<'a> SegmentsIter<'a> {
    fn new(
        strategy_points: &'a StrategySegments,
        point: Point,
        vector_constraint: Option<Vector>,
    ) -> Self {
        let jump_vectors;
        let diagonal_and_line_vectors;
        if let Some(vector_constraint) = vector_constraint {
            match vector_constraint {
                Vector::Jump(_) => {
                    jump_vectors = vec![vector_constraint];
                    diagonal_and_line_vectors = Vector::diagonal_and_line_vectors();
                }
                Vector::Line(_) | Vector::Diagonal(_) => {
                    jump_vectors = Vector::jump_vectors();
                    diagonal_and_line_vectors = vec![vector_constraint]
                }
            }
        } else {
            jump_vectors = Vector::jump_vectors();
            diagonal_and_line_vectors = Vector::diagonal_and_line_vectors();
        }
        Self {
            strategy_points,
            point,
            diagonal_and_line_vectors,
            jump_vectors,
            diagonal_and_line_vectors_pos: 0,
            jump_vectors_pos: 0,
        }
    }
}

impl<'a> Iterator for SegmentsIter<'a> {
    type Item = &'a Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let find_in_non_jump = |collection: &'a FxHashMap<Point, BTreeSet<SegmentRepr>>,
                                vec: &Vector|
         -> Option<&Segment> {
            let lower_bound =
                StrategySegments::calc_lower_bound(&self.point, vec, &self.strategy_points.dimension);
            if let Some(entries) = collection.get(&lower_bound) {
                let mut segment_repr = SegmentRepr::from(Segment::Partial(self.point, *vec));
                // In case the given point already contains a segment - we need to jump from it,
                // because we are actually interested in finding segments affecting on the given
                // point - not segments starting on the point. Thus, advance the point to prev/next
                // point depending on the given vector
                if entries.contains(&segment_repr) {
                    segment_repr =
                        SegmentRepr::from(Segment::Partial(self.point, *vec).at_prev_point());
                }
                let cursor = entries.lower_bound(Bound::Included(&segment_repr));
                let seg_repr = if vec.is_ascending() {
                    cursor.peek_prev()
                } else {
                    cursor.peek_next()
                };

                if let Some(seg_repr) = seg_repr {
                    match seg_repr.seg() {
                        Segment::DeadEnd(_, _) => (),
                        _ => {
                            if seg_repr.seg().contains_point(&self.point) {
                                return Some(seg_repr.seg());
                            }
                        }
                    }
                }
            }
            None
        };
        loop {
            if self.diagonal_and_line_vectors.len() == self.diagonal_and_line_vectors_pos {
                break;
            }
            let vec = self.diagonal_and_line_vectors[self.diagonal_and_line_vectors_pos];
            self.diagonal_and_line_vectors_pos += 1;
            let collection = self.strategy_points.non_jump_collection(&vec);
            if let Some(seg) = find_in_non_jump(collection, &vec) {
                return Some(seg);
            };
        }

        loop {
            if self.jump_vectors.len() == self.jump_vectors_pos {
                break;
            }
            let vec = self.jump_vectors[self.jump_vectors_pos];

            self.jump_vectors_pos += 1;
            let mut vector_points = VectorPoints::without_initial(
                self.point,
                self.strategy_points.dimension,
                vec.inverse(),
            );

            if let Some(lower_bound) = vector_points.next() {
                if let Some(seg_set) = self.strategy_points.jump_segments.get(&lower_bound) {
                    let segment_including_the_piece =
                        SegmentRepr::Jump(Segment::Partial(self.point, vec));
                    if let Some(seg_repr) = seg_set.get(&segment_including_the_piece) {
                        return Some(seg_repr.seg());
                    }
                }
            }
        }
        None
    }
}

impl StrategySegments {
    fn calc_lower_bound(point: &Point, vector: &Vector, dimension: &Dimension) -> Point {
        match vector {
            Vector::Line(_) | Vector::Diagonal(_) => {
                let vector = if vector.is_ascending() {
                    vector.inverse()
                } else {
                    *vector
                };
                let vector_points = VectorPoints::with_initial(*point, *dimension, vector);
                vector_points.last_point()
            }
            Vector::Jump(_) => *point,
        }
    }

    pub fn empty(dimension: &Dimension) -> Self {
        Self {
            horizontal_segments: FxHashMap::default(),
            vertical_segments: FxHashMap::default(),
            diagonal_pos_segments: FxHashMap::default(),
            diagonal_neg_segments: FxHashMap::default(),
            jump_segments: FxHashMap::default(),

            track_changes: false,
            current_changes: Some(FxHashMap::default()),
            historical_changes: vec![],
            dimension: *dimension,
        }
    }

    pub fn start_tracking_changes(&mut self) {
        self.track_changes = true;
    }

    pub fn flush_changes(&mut self) {
        let current_changes = self.current_changes.take().unwrap();
        self.current_changes = Some(FxHashMap::default());
        self.historical_changes.push(current_changes);
    }

    pub fn get_pieces(&self, strategy_point: &StrategyPoint) -> Vec<PieceId> {
        // self.pieces_affecting_on(strategy_point, false)
        vec![]
    }

    // Searches for segments, affecting on the given point
    pub fn segments_affecting_on(&'_ self, point: &Point) -> SegmentsIter<'_> {
        SegmentsIter::new(&self, *point, None)
    }

    // pub fn find_segments<'a>(&'a self, point: &Point) -> Vec<&Segment> {
    //     let mut segments: Vec<&Segment> = vec![];
    //     let find_in_non_jump = |collection: &'a FxHashMap<Point, BTreeSet<SegmentRepr>>,
    //                                 vec: &Vector|
    //      -> Option<&Segment> {
    //         let segment_repr = SegmentRepr::from(Segment::Partial(*point, *vec));
    //         let lower_bound = Self::calc_lower_bound(point, vec, &self.dimension);
    //         if let Some(entries) = collection.get(&lower_bound) {
    //             let cursor = entries.lower_bound(Bound::Included(&segment_repr));
    //             for seg_repr in [cursor.peek_prev(), cursor.peek_next()] {
    //                 if let Some(seg_repr) = seg_repr
    //                 {
    //                     match seg_repr.seg() {
    //                         Segment::DeadEnd(_, _) => (),
    //                         _ => {
    //                             if seg_repr.seg().contains_point(point) {
    //                                 return Some(seg_repr.seg())
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         None
    //     };
    //     for vec in Vector::diagonal_and_line_vectors().iter() {
    //         match vec {
    //             Vector::Line(line_vec) => match line_vec {
    //                 LineVector::Top | LineVector::Bottom => {
    //                     if let Some(seg) = find_in_non_jump(&self.vertical_segments, vec) {
    //                         segments.push(seg);
    //                     };
    //                 }
    //                 LineVector::Left | LineVector::Right => {
    //                     if let Some(seg) = find_in_non_jump(&self.horizontal_segments, vec) {
    //                         segments.push(seg);
    //                     };
    //                 }
    //             },
    //             Vector::Diagonal(diag_vector) => match diag_vector {
    //                 DiagonalVector::BottomLeft | DiagonalVector::TopRight => {
    //                     if let Some(seg) = find_in_non_jump(&self.diagonal_pos_segments, vec) {
    //                         segments.push(seg);
    //                     };
    //                 }
    //                 DiagonalVector::BottomRight | DiagonalVector::TopLeft => {
    //                     if let Some(seg) = find_in_non_jump(&self.diagonal_neg_segments, vec) {
    //                         segments.push(seg);
    //                     };
    //                 }
    //             },
    //             _ => panic!("Don't know how to handle {:?}", vec),
    //         }
    //     }
    //     for vec in Vector::jump_vectors().iter() {
    //         let mut vector_points =
    //             VectorPoints::without_initial(*point, self.dimension, *vec);
    //         if let Some(lower_bound) = vector_points.next() {
    //             if let Some(seg_set) = self.jump_segments.get(&lower_bound) {
    //                 let segment_including_the_piece = SegmentRepr::Jump(Segment::Partial(
    //                     *point,
    //                     *vec,
    //                 ));
    //                 if let Some(seg_repr) = seg_set.get(&segment_including_the_piece) {
    //                     segments.push(seg_repr.seg());
    //                 }
    //             }
    //         }
    //     }
    //     segments
    // }

    pub fn add_points(&mut self, piece_id: &PieceId, points: FxHashSet<StrategyPoint>) {
        // self.get_pieces_mut(&point).insert(*piece_id);
        // let old_points = self.piece_to_points.insert(*piece_id, points);
        // if self.track_changes
        //     && let Some(old_points) = old_points
        // {
        //     let events = self
        //         .current_changes
        //         .as_mut()
        //         .unwrap()
        //         .entry(*piece_id)
        //         .or_insert(vec![]);
        //     events.push(Event::PieceChanged(old_points));
        // };
    }

    // Looks up for first nearest segment that is located on the given direction, next to the given
    // piece's position.
    pub fn nearest_segment(&self, piece_position: &Point, direction: &Vector) -> Option<&Segment> {
        match direction {
            Vector::Line(_) | Vector::Diagonal(_) => {
                let lower_bound = Self::calc_lower_bound(piece_position, direction, &self.dimension);
                let segment_repr = SegmentRepr::from(Segment::Partial(*piece_position, *direction));
                let collection = self.non_jump_collection(direction);
                if let Some(entries) = collection.get(&lower_bound) {
                    if direction.is_ascending() {
                        let cursor = entries.lower_bound(Bound::Excluded(&segment_repr));
                        if let Some(segment_repr) = cursor.peek_next() {
                            return Some(segment_repr.seg());
                        }
                    } else {
                        let cursor = entries.lower_bound(Bound::Included(&segment_repr));
                        if let Some(segment_repr) = cursor.peek_prev() {
                            return Some(segment_repr.seg());
                        }
                    }
                }
            }
            Vector::Jump(_) => {
                let mut vector_points = VectorPoints::without_initial(
                    *piece_position,
                    self.dimension,
                    *direction,
                );
                if let Some(point) = vector_points.next() {
                    if let Some(segments) = self.jump_segments.get(&point) {
                        let seg = segments
                            .iter()
                            .next()
                            .expect(
                                "Logical error: jump lower bound exists with no jump segments"
                            )
                            .seg();
                        return Some(seg)
                    }
                }
            }
        }
        None
    }

    fn non_jump_collection(&self, vec: &Vector) -> &FxHashMap<Point, BTreeSet<SegmentRepr>> {
        match vec {
            Vector::Line(line_vec) => match line_vec {
                LineVector::Top | LineVector::Bottom => &self.vertical_segments,
                LineVector::Left | LineVector::Right => &self.horizontal_segments,
            },
            Vector::Diagonal(diag_vector) => match diag_vector {
                DiagonalVector::BottomLeft | DiagonalVector::TopRight => {
                    &self.diagonal_pos_segments
                }
                DiagonalVector::BottomRight | DiagonalVector::TopLeft => {
                    &self.diagonal_neg_segments
                }
            },
            Vector::Jump(_) => panic!("Jump vectors are not supported"),
        }
    }

    pub fn add_segments(&mut self, segments: Vec<Segment>) {
        let add_segment = |collection: &mut FxHashMap<Point, BTreeSet<SegmentRepr>>,
                           segment: Segment| {
            let lower_bound =
                Self::calc_lower_bound(segment.point1(), segment.vector(), &self.dimension);
            let entries = collection.entry(lower_bound).or_insert(BTreeSet::default());
            entries.insert(SegmentRepr::from(segment));
        };
        for segment in segments {
            match segment.vector() {
                Vector::Line(line_vec) => match line_vec {
                    LineVector::Top | LineVector::Bottom => {
                        add_segment(&mut self.vertical_segments, segment);
                    }
                    LineVector::Left | LineVector::Right => {
                        add_segment(&mut self.horizontal_segments, segment);
                    }
                },
                Vector::Diagonal(diag_vector) => match diag_vector {
                    DiagonalVector::BottomLeft | DiagonalVector::TopRight => {
                        add_segment(&mut self.diagonal_pos_segments, segment);
                    }
                    DiagonalVector::BottomRight | DiagonalVector::TopLeft => {
                        add_segment(&mut self.diagonal_neg_segments, segment);
                    }
                },
                Vector::Jump(_) => {
                    let lower_bound =
                        Self::calc_lower_bound(segment.point1(), segment.vector(), &self.dimension);
                    let entries = self
                        .jump_segments
                        .entry(lower_bound)
                        .or_insert(FxHashSet::default());
                    entries.insert(SegmentRepr::Jump(segment));
                }
            }
        }
    }

    pub fn remove_segments(&mut self, piece: &Piece) {
        let clear_non_jump = |collection: &mut FxHashMap<Point, BTreeSet<SegmentRepr>>,
                              vec: &Vector| {
            let lower_bound =
                Self::calc_lower_bound(piece.current_position(), &vec, &self.dimension);
            if let Some(entries) = collection.get_mut(&lower_bound) {
                let segment_including_the_piece =
                    SegmentRepr::from(Segment::Partial(*piece.current_position(), *vec));
                if piece.id().id() == &1 {
                    println!("{:?}", entries.get(&segment_including_the_piece));
                }

                entries.remove(&segment_including_the_piece);

                if entries.is_empty() {
                    collection.remove(&lower_bound);
                }
            }
        };
        let mut clear_segments = |vec: &Vector| match vec {
            Vector::Line(line_vec) => match line_vec {
                LineVector::Top | LineVector::Bottom => {
                    clear_non_jump(&mut self.vertical_segments, vec);
                }
                LineVector::Left | LineVector::Right => {
                    clear_non_jump(&mut self.horizontal_segments, vec);
                }
            },
            Vector::Diagonal(diag_vector) => match diag_vector {
                DiagonalVector::BottomLeft | DiagonalVector::TopRight => {
                    clear_non_jump(&mut self.diagonal_pos_segments, vec);
                }
                DiagonalVector::BottomRight | DiagonalVector::TopLeft => {
                    clear_non_jump(&mut self.diagonal_neg_segments, vec);
                }
            },
            Vector::Jump(_) => {
                let lower_bound =
                    Self::calc_lower_bound(piece.current_position(), &vec, &self.dimension);
                self.jump_segments.remove(&lower_bound);
            }
        };

        for vec in piece.attack_vectors().iter() {
            clear_segments(vec)
        }
        for vec in piece.move_vectors().iter() {
            clear_segments(vec)
        }
    }

    pub fn piece_segments<'a>(&'a self, piece: &Piece) -> Vec<&'a Segment> {
        let mut segments = vec![];
        let get_segment = |collection: &'a FxHashMap<Point, BTreeSet<SegmentRepr>>,
                              vec: &Vector| -> Option<&'a Segment> {
            let lower_bound =
                Self::calc_lower_bound(piece.current_position(), &vec, &self.dimension);
            if let Some(entries) = collection.get(&lower_bound) {
                let segment_including_the_piece =
                    SegmentRepr::from(Segment::Partial(*piece.current_position(), *vec));
                if let Some(seg_repr) = entries.get(&segment_including_the_piece) {
                    return Some(seg_repr.seg())
                }
            }
            None
        };
        let mut vectors = piece.attack_vectors();
        vectors.append(&mut piece.move_vectors());
        for vec in vectors.iter() {
            match vec {
                Vector::Line(line_vec) => match line_vec {
                    LineVector::Top | LineVector::Bottom => {
                        if let Some(seg) = get_segment(&self.vertical_segments, vec) {
                            segments.push(seg);
                        }
                    }
                    LineVector::Left | LineVector::Right => {
                        if let Some(seg) = get_segment(&self.horizontal_segments, vec) {
                            segments.push(seg);
                        }
                    }
                },
                Vector::Diagonal(diag_vector) => match diag_vector {
                    DiagonalVector::BottomLeft | DiagonalVector::TopRight => {
                        if let Some(seg) = get_segment(&self.diagonal_pos_segments, vec) {
                            segments.push(seg);
                        }
                    }
                    DiagonalVector::BottomRight | DiagonalVector::TopLeft => {
                        if let Some(seg) = get_segment(&self.diagonal_neg_segments, vec) {
                            segments.push(seg);
                        }
                    }
                },
                Vector::Jump(_) => {
                    let lower_bound =
                        Self::calc_lower_bound(piece.current_position(), &vec, &self.dimension);
                    if let Some(seg_reprs) = self.jump_segments.get(&lower_bound) {
                        for seg_repr in seg_reprs {
                            segments.push(seg_repr.seg());
                        }
                    }
                }
            }
        }
        segments
    }

    // Looks up for the segment belonging to the given piece and affecting on the given point
    pub fn find_segment<'a>(&'a self, piece: &Piece, point: &Point) -> Option<&'a Segment> {
        if let Some(vec) = Vector::calc_direction(piece.current_position(), point) {
            if let Some(seg) = SegmentsIter::new(&self, *point, Some(vec)).next()
                && seg.piece_id() == piece.id()
            {
                return Some(seg);
            }
        }
        None
    }

    // pub fn remove_piece(&mut self, piece_id: &PieceId) {
    //     let points = self.piece_to_points.remove(piece_id);
    //     if let Some(points) = points {
    //         for point in points.iter() {
    //             if let Some(pieces) = self.point_to_pieces.get_mut(point) {
    //                 pieces.remove(piece_id);
    //                 if pieces.is_empty() {
    //                     self.point_to_pieces.remove(point);
    //                 }
    //             }
    //         }
    //         if self.track_changes {
    //             let events = self
    //                 .current_changes
    //                 .as_mut()
    //                 .unwrap()
    //                 .entry(*piece_id)
    //                 .or_insert(vec![]);
    //             events.push(Event::PieceRemoved(points));
    //         }
    //     }
    // }

    pub fn is_under_attack(&self, point: &Point) -> bool {
        self.segments_affecting_on(point).any(|seg| match seg {
            Segment::Attack(_, _, _, _) => true,
            Segment::Defense(_, point2, _, _) | Segment::DeadEndAttack(_, point2, _, _) => {
                point != point2
            }
            _ => false,
        })
    }

    pub fn is_under_defense(&self, point: &Point) -> bool {
        self.segments_affecting_on(point).any(|seg| match seg {
            Segment::Defense(_, point2, _, _) => point == point2,
            _ => false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    mod add_segments {
        use super::*;
        use crate::vector::jump_vector::JumpVector;

        fn strategy_points() -> StrategySegments {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
            StrategySegments::empty(&dimension)
        }

        #[test]
        fn adding_vertical_segments() {
            let mut strategy_points = strategy_points();
            let segment1 = Segment::Attack(
                Point::new(4, 5),
                Point::new(4, 8),
                Vector::Line(LineVector::Top),
                PieceId::new(1, &Color::Black),
            );
            let segment2 = Segment::Defense(
                Point::new(4, 5),
                Point::new(4, 2),
                Vector::Line(LineVector::Bottom),
                PieceId::new(1, &Color::Black),
            );
            let segment3 = Segment::Defense(
                Point::new(4, 2),
                Point::new(4, 5),
                Vector::Line(LineVector::Top),
                PieceId::new(2, &Color::Black),
            );
            let segment4 = Segment::Attack(
                Point::new(4, 2),
                Point::new(4, 1),
                Vector::Line(LineVector::Bottom),
                PieceId::new(2, &Color::Black),
            );

            strategy_points.add_segments(vec![segment1, segment2, segment3, segment4]);

            let segments = strategy_points
                .vertical_segments
                .get(&Point::new(4, 1))
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    &SegmentRepr::Vertical(segment4),
                    &SegmentRepr::Vertical(segment3),
                    &SegmentRepr::Vertical(segment2),
                    &SegmentRepr::Vertical(segment1),
                ]
            )
        }

        #[test]
        fn adding_horizontal_segments() {
            let mut strategy_points = strategy_points();
            let segment1 = Segment::Attack(
                Point::new(3, 5),
                Point::new(1, 5),
                Vector::Line(LineVector::Left),
                PieceId::new(1, &Color::Black),
            );
            let segment2 = Segment::Defense(
                Point::new(3, 5),
                Point::new(5, 5),
                Vector::Line(LineVector::Right),
                PieceId::new(1, &Color::Black),
            );
            let segment3 = Segment::Defense(
                Point::new(6, 5),
                Point::new(3, 5),
                Vector::Line(LineVector::Left),
                PieceId::new(2, &Color::Black),
            );
            let segment4 = Segment::Attack(
                Point::new(6, 5),
                Point::new(8, 5),
                Vector::Line(LineVector::Right),
                PieceId::new(2, &Color::Black),
            );

            strategy_points.add_segments(vec![segment1, segment2, segment3, segment4]);

            let segments = strategy_points
                .horizontal_segments
                .get(&Point::new(1, 5))
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    &SegmentRepr::Horizontal(segment1),
                    &SegmentRepr::Horizontal(segment2),
                    &SegmentRepr::Horizontal(segment3),
                    &SegmentRepr::Horizontal(segment4),
                ]
            )
        }

        #[test]
        fn adding_diagonal_pos_segments() {
            let mut strategy_points = strategy_points();
            let segment1 = Segment::Attack(
                Point::new(3, 3),
                Point::new(1, 1),
                Vector::Diagonal(DiagonalVector::BottomLeft),
                PieceId::new(1, &Color::Black),
            );
            let segment2 = Segment::Defense(
                Point::new(3, 3),
                Point::new(6, 6),
                Vector::Diagonal(DiagonalVector::TopRight),
                PieceId::new(1, &Color::Black),
            );
            let segment3 = Segment::Defense(
                Point::new(6, 6),
                Point::new(3, 3),
                Vector::Diagonal(DiagonalVector::BottomLeft),
                PieceId::new(2, &Color::Black),
            );
            let segment4 = Segment::Attack(
                Point::new(6, 6),
                Point::new(8, 8),
                Vector::Diagonal(DiagonalVector::TopRight),
                PieceId::new(2, &Color::Black),
            );

            strategy_points.add_segments(vec![segment1, segment2, segment3, segment4]);

            let segments = strategy_points
                .diagonal_pos_segments
                .get(&Point::new(1, 1))
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    &SegmentRepr::DiagonalPos(segment1),
                    &SegmentRepr::DiagonalPos(segment2),
                    &SegmentRepr::DiagonalPos(segment3),
                    &SegmentRepr::DiagonalPos(segment4),
                ]
            )
        }

        #[test]
        fn adding_diagonal_neg_segments() {
            let mut strategy_points = strategy_points();
            let segment1 = Segment::Attack(
                Point::new(6, 3),
                Point::new(8, 1),
                Vector::Diagonal(DiagonalVector::BottomRight),
                PieceId::new(1, &Color::Black),
            );
            let segment2 = Segment::Defense(
                Point::new(6, 3),
                Point::new(3, 6),
                Vector::Diagonal(DiagonalVector::TopLeft),
                PieceId::new(1, &Color::Black),
            );
            let segment3 = Segment::Defense(
                Point::new(3, 6),
                Point::new(6, 3),
                Vector::Diagonal(DiagonalVector::BottomRight),
                PieceId::new(2, &Color::Black),
            );
            let segment4 = Segment::Attack(
                Point::new(3, 6),
                Point::new(1, 8),
                Vector::Diagonal(DiagonalVector::TopLeft),
                PieceId::new(2, &Color::Black),
            );

            strategy_points.add_segments(vec![segment1, segment2, segment3, segment4]);

            let segments = strategy_points
                .diagonal_neg_segments
                .get(&Point::new(8, 1))
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            assert_eq!(
                segments,
                vec![
                    &SegmentRepr::DiagonalNeg(segment1),
                    &SegmentRepr::DiagonalNeg(segment2),
                    &SegmentRepr::DiagonalNeg(segment3),
                    &SegmentRepr::DiagonalNeg(segment4),
                ]
            )
        }

        #[test]
        fn adding_jump_segments() {
            let mut strategy_points = strategy_points();
            let segment1 = Segment::Attack(
                Point::new(4, 4),
                Point::new(3, 2),
                Vector::Jump(JumpVector::BottomLeftRight),
                PieceId::new(1, &Color::Black),
            );
            let segment2 = Segment::Attack(
                Point::new(4, 4),
                Point::new(2, 3),
                Vector::Jump(JumpVector::BottomLeftLeft),
                PieceId::new(2, &Color::Black),
            );
            let segment3 = Segment::Attack(
                Point::new(4, 4),
                Point::new(2, 5),
                Vector::Jump(JumpVector::TopLeftLeft),
                PieceId::new(3, &Color::Black),
            );
            let segment4 = Segment::Attack(
                Point::new(4, 4),
                Point::new(3, 6),
                Vector::Jump(JumpVector::TopLeftRight),
                PieceId::new(4, &Color::Black),
            );
            let segment5 = Segment::Attack(
                Point::new(4, 4),
                Point::new(5, 6),
                Vector::Jump(JumpVector::TopRightLeft),
                PieceId::new(5, &Color::Black),
            );
            let segment6 = Segment::Attack(
                Point::new(4, 4),
                Point::new(6, 5),
                Vector::Jump(JumpVector::TopRightRight),
                PieceId::new(6, &Color::Black),
            );
            let segment7 = Segment::Attack(
                Point::new(4, 4),
                Point::new(6, 3),
                Vector::Jump(JumpVector::BottomRightRight),
                PieceId::new(7, &Color::Black),
            );
            let segment8 = Segment::Attack(
                Point::new(4, 4),
                Point::new(5, 1),
                Vector::Jump(JumpVector::BottomRightLeft),
                PieceId::new(8, &Color::Black),
            );

            strategy_points.add_segments(vec![
                segment1, segment2, segment3, segment4, segment5, segment6, segment7, segment8,
            ]);

            let mut segments = strategy_points
                .jump_segments
                .get(&Point::new(4, 4))
                .unwrap()
                .iter()
                .collect::<Vec<_>>();
            segments.sort();

            assert_eq!(
                segments,
                vec![
                    &SegmentRepr::Jump(segment1),
                    &SegmentRepr::Jump(segment2),
                    &SegmentRepr::Jump(segment3),
                    &SegmentRepr::Jump(segment4),
                    &SegmentRepr::Jump(segment5),
                    &SegmentRepr::Jump(segment6),
                    &SegmentRepr::Jump(segment7),
                    &SegmentRepr::Jump(segment8),
                ]
            )
        }
    }

    mod segments_affecting_on {
        use super::*;

        mod vertical_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(4, 6),
                    Point::new(4, 8),
                    Vector::Line(LineVector::Top),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(4, 6),
                    Point::new(4, 3),
                    Vector::Line(LineVector::Bottom),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(4, 3),
                    Point::new(4, 6),
                    Vector::Line(LineVector::Top),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(4, 3),
                    Point::new(4, 1),
                    Vector::Line(LineVector::Bottom),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_upper_piece() {
                let point = Point::new(4, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(), &segment3(),])
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_lower_piece() {
                let point = Point::new(4, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(), &segment3(),])
            }

            #[test]
            fn when_point_lies_on_the_top_segment() {
                let point = Point::new(4, 6);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_segment() {
                let point = Point::new(4, 3);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(),])
            }

            #[test]
            fn when_point_lies_near_top_edge() {
                let point = Point::new(4, 8);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_near_bottom_edge() {
                let point = Point::new(4, 1);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_top_edge_of_upper_piece() {
                let point = Point::new(4, 7);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_edge_of_lower_piece() {
                let point = Point::new(4, 2);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_different_vertical_axis() {
                let point = Point::new(3, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, Vec::<&Segment>::new())
            }
        }

        mod horizontal_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 5),
                    Point::new(1, 5),
                    Vector::Line(LineVector::Left),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 5),
                    Point::new(6, 5),
                    Vector::Line(LineVector::Right),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 5),
                    Point::new(3, 5),
                    Vector::Line(LineVector::Left),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Vector::Line(LineVector::Right),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_left_piece() {
                let point = Point::new(4, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_right_piece() {
                let point = Point::new(5, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_on_the_left_segment() {
                let point = Point::new(3, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(),])
            }

            #[test]
            fn when_point_lies_on_the_right_segment() {
                let point = Point::new(6, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(),])
            }

            #[test]
            fn when_point_lies_near_left_edge() {
                let point = Point::new(1, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_near_right_edge() {
                let point = Point::new(8, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_left_edge_of_left_piece() {
                let point = Point::new(2, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_right_edge_of_right_piece() {
                let point = Point::new(7, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_different_horizontal_axis() {
                let point = Point::new(3, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, Vec::<&Segment>::new())
            }
        }

        mod diagonal_pos_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 3),
                    Point::new(1, 1),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 3),
                    Point::new(6, 6),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 6),
                    Point::new(3, 3),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 6),
                    Point::new(8, 8),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_upper_piece() {
                let point = Point::new(5, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_lower_piece() {
                let point = Point::new(4, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_on_the_top_segment() {
                let point = Point::new(6, 6);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_segment() {
                let point = Point::new(3, 3);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(),])
            }

            #[test]
            fn when_point_lies_near_top_edge() {
                let point = Point::new(8, 8);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_near_bottom_edge() {
                let point = Point::new(1, 1);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_top_edge_of_upper_piece() {
                let point = Point::new(7, 7);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_edge_of_lower_piece() {
                let point = Point::new(2, 2);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_different_diagonal_pos_axis() {
                let point = Point::new(2, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, Vec::<&Segment>::new())
            }
        }

        mod diagonal_neg_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(6, 3),
                    Point::new(8, 1),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(6, 3),
                    Point::new(3, 6),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(3, 6),
                    Point::new(6, 3),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(3, 6),
                    Point::new(1, 8),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_upper_piece() {
                let point = Point::new(4, 5);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_between_two_segments_closer_to_lower_piece() {
                let point = Point::new(5, 4);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(), &segment2(),])
            }

            #[test]
            fn when_point_lies_on_the_top_segment() {
                let point = Point::new(3, 6);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment2(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_segment() {
                let point = Point::new(6, 3);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment3(),])
            }

            #[test]
            fn when_point_lies_near_top_edge() {
                let point = Point::new(1, 8);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_near_bottom_edge() {
                let point = Point::new(8, 1);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_top_edge_of_upper_piece() {
                let point = Point::new(2, 7);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment4(),])
            }

            #[test]
            fn when_point_lies_on_the_bottom_edge_of_lower_piece() {
                let point = Point::new(7, 2);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, vec![&segment1(),])
            }

            #[test]
            fn when_point_lies_on_the_different_diagonal_neg_axis() {
                let point = Point::new(7, 1);
                let strategy_points = strategy_points();
                let segments = strategy_points
                    .segments_affecting_on(&point)
                    .collect::<Vec<_>>();
                assert_eq!(segments, Vec::<&Segment>::new())
            }
        }

        mod jump_segments {
            use super::*;
            use crate::vector::jump_vector::JumpVector;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 2),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::TopRightLeft),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(2, 3),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::TopRightRight),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Attack(
                    Point::new(2, 5),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomRightRight),
                    PieceId::new(3, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(3, 6),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomRightLeft),
                    PieceId::new(4, &Color::Black),
                )
            }

            fn segment5() -> Segment {
                Segment::Attack(
                    Point::new(5, 6),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomLeftRight),
                    PieceId::new(5, &Color::Black),
                )
            }

            fn segment6() -> Segment {
                Segment::Attack(
                    Point::new(6, 5),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomLeftLeft),
                    PieceId::new(6, &Color::Black),
                )
            }

            fn segment7() -> Segment {
                Segment::Attack(
                    Point::new(6, 3),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::TopLeftLeft),
                    PieceId::new(7, &Color::Black),
                )
            }

            fn segment8() -> Segment {
                Segment::Attack(
                    Point::new(5, 2),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::TopLeftRight),
                    PieceId::new(8, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![
                    segment1(),
                    segment2(),
                    segment3(),
                    segment4(),
                    segment5(),
                    segment6(),
                    segment7(),
                    segment8(),
                ]);
                strategy_points
            }

            mod when_the_given_point_has_segments_affecting_on_it {
                use super::*;

                fn point() -> Point {
                    Point::new(4, 4)
                }

                #[test]
                fn it_finds_all_segments_affecting_on_the_given_point() {
                    let strategy_points = strategy_points();
                    let segments = strategy_points
                        .segments_affecting_on(&point())
                        .collect::<Vec<_>>();
                    assert_eq!(
                        segments,
                        vec![
                            &segment5(),
                            &segment4(),
                            &segment6(),
                            &segment3(),
                            &segment7(),
                            &segment2(),
                            &segment8(),
                            &segment1(),
                        ]
                    )
                }
            }

            mod when_the_given_point_does_not_have_segments_affecting_on_it {
                use super::*;

                fn point() -> Point {
                    Point::new(6, 6)
                }

                #[test]
                fn it_does_not_find_anything() {
                    let strategy_points = strategy_points();
                    let segments = strategy_points
                        .segments_affecting_on(&point())
                        .collect::<Vec<_>>();
                    assert_eq!(segments, Vec::<&Segment>::new(),)
                }
            }

            mod when_the_given_point_lies_on_a_segment_start {
                use super::*;

                fn point() -> Point {
                    Point::new(5, 2)
                }

                #[test]
                fn it_does_not_find_anything() {
                    let strategy_points = strategy_points();
                    let segments = strategy_points
                        .segments_affecting_on(&point())
                        .collect::<Vec<_>>();
                    assert_eq!(segments, Vec::<&Segment>::new(),)
                }
            }
        }
    }

    mod nearest_segment {
        use super::*;

        mod vertical_segments {
            use crate::color::Color;
            use crate::dimension::Dimension;
            use crate::piece_id::PieceId;
            use crate::point::Point;
            use crate::segment::Segment;
            use crate::strategy_segments::StrategySegments;
            use crate::vector::Vector;
            use crate::vector::line_vector::LineVector;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(4, 6),
                    Point::new(4, 8),
                    Vector::Line(LineVector::Top),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(4, 6),
                    Point::new(4, 3),
                    Vector::Line(LineVector::Bottom),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(4, 3),
                    Point::new(4, 6),
                    Vector::Line(LineVector::Top),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(4, 3),
                    Point::new(4, 1),
                    Vector::Line(LineVector::Bottom),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_nearest_segment_exists {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points
                        .nearest_segment(&Point::new(4, 3), &Vector::Line(LineVector::Top));
                    assert_eq!(
                        segment,
                        Some(&Segment::Defense(
                            Point::new(4, 6),
                            Point::new(4, 3),
                            Vector::Line(LineVector::Bottom),
                            PieceId::new(1, &Color::Black)
                        ))
                    )
                }
            }

            mod when_nearest_segment_does_not_exist {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points
                        .nearest_segment(&Point::new(4, 3), &Vector::Line(LineVector::Bottom));
                    assert_eq!(segment, None)
                }
            }
        }

        mod horizontal_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 5),
                    Point::new(1, 5),
                    Vector::Line(LineVector::Left),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 5),
                    Point::new(6, 5),
                    Vector::Line(LineVector::Right),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 5),
                    Point::new(3, 5),
                    Vector::Line(LineVector::Left),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Vector::Line(LineVector::Right),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_nearest_segment_exists {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points
                        .nearest_segment(&Point::new(3, 5), &Vector::Line(LineVector::Right));
                    assert_eq!(
                        segment,
                        Some(&Segment::Defense(
                            Point::new(6, 5),
                            Point::new(3, 5),
                            Vector::Line(LineVector::Left),
                            PieceId::new(2, &Color::Black)
                        ))
                    )
                }
            }

            mod when_nearest_segment_does_not_exist {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points
                        .nearest_segment(&Point::new(3, 5), &Vector::Line(LineVector::Left));
                    assert_eq!(segment, None)
                }
            }
        }

        mod diagonal_pos_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 3),
                    Point::new(1, 1),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 3),
                    Point::new(6, 6),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 6),
                    Point::new(3, 3),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 6),
                    Point::new(8, 8),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_nearest_segment_exists {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points.nearest_segment(
                        &Point::new(3, 3),
                        &Vector::Diagonal(DiagonalVector::TopRight),
                    );
                    assert_eq!(
                        segment,
                        Some(&Segment::Defense(
                            Point::new(6, 6),
                            Point::new(3, 3),
                            Vector::Diagonal(DiagonalVector::BottomLeft),
                            PieceId::new(2, &Color::Black)
                        ))
                    )
                }
            }

            mod when_nearest_segment_does_not_exist {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points.nearest_segment(
                        &Point::new(3, 3),
                        &Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(segment, None)
                }
            }
        }

        mod diagonal_neg_segments {
            use super::*;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(6, 3),
                    Point::new(8, 1),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(6, 3),
                    Point::new(3, 6),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    PieceId::new(1, &Color::Black),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(3, 6),
                    Point::new(6, 3),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(3, 6),
                    Point::new(1, 8),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    PieceId::new(2, &Color::Black),
                )
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_nearest_segment_exists {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points.nearest_segment(
                        &Point::new(6, 3),
                        &Vector::Diagonal(DiagonalVector::TopLeft),
                    );
                    assert_eq!(
                        segment,
                        Some(&Segment::Defense(
                            Point::new(3, 6),
                            Point::new(6, 3),
                            Vector::Diagonal(DiagonalVector::BottomRight),
                            PieceId::new(2, &Color::Black)
                        ))
                    )
                }
            }

            mod when_nearest_segment_does_not_exist {
                use super::*;

                #[test]
                fn it_finds_nearest_segment_from_the_given_position_by_the_given_direction() {
                    let strategy_points = strategy_points();
                    let segment = strategy_points.nearest_segment(
                        &Point::new(6, 3),
                        &Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(segment, None)
                }
            }
        }
    }

    mod find_segment {
        use super::*;

        mod vertical_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::rook::Rook;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(4, 6),
                    Point::new(4, 8),
                    Vector::Line(LineVector::Top),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(4, 6),
                    Point::new(4, 3),
                    Vector::Line(LineVector::Bottom),
                    *piece1().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(4, 3),
                    Point::new(4, 6),
                    Vector::Line(LineVector::Top),
                    *piece2().id(),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(4, 3),
                    Point::new(4, 1),
                    Vector::Line(LineVector::Bottom),
                    *piece2().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Rook(Rook::new(
                    Color::Black,
                    Point::new(4, 6),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn piece2() -> Piece {
                Piece::Rook(Rook::new(
                    Color::Black,
                    Point::new(4, 3),
                    PieceId::new(2, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_the_given_piece_has_a_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_found_segment() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(4, 3)),
                        Some(&segment2()),
                    )
                }
            }

            mod when_the_given_piece_does_not_have_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_nothing() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(4, 2)),
                        None,
                    )
                }
            }
        }

        mod horizontal_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::rook::Rook;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 5),
                    Point::new(1, 5),
                    Vector::Line(LineVector::Left),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 5),
                    Point::new(6, 5),
                    Vector::Line(LineVector::Right),
                    *piece1().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 5),
                    Point::new(3, 5),
                    Vector::Line(LineVector::Left),
                    *piece2().id(),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Vector::Line(LineVector::Right),
                    *piece2().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Rook(Rook::new(
                    Color::Black,
                    Point::new(3, 5),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn piece2() -> Piece {
                Piece::Rook(Rook::new(
                    Color::Black,
                    Point::new(6, 5),
                    PieceId::new(2, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_the_given_piece_has_a_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_found_segment() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(5, 5)),
                        Some(&segment2()),
                    )
                }
            }

            mod when_the_given_piece_does_not_have_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_nothing() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(7, 5)),
                        None,
                    )
                }
            }
        }

        mod diagonal_pos_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::bishop::Bishop;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 3),
                    Point::new(1, 1),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(3, 3),
                    Point::new(6, 6),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    *piece1().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(6, 6),
                    Point::new(3, 3),
                    Vector::Diagonal(DiagonalVector::BottomLeft),
                    *piece2().id(),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(6, 6),
                    Point::new(8, 8),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    *piece2().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::Black,
                    Point::new(3, 3),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn piece2() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::Black,
                    Point::new(6, 6),
                    PieceId::new(2, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_the_given_piece_has_a_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_found_segment() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(5, 5)),
                        Some(&segment2()),
                    )
                }
            }

            mod when_the_given_piece_does_not_have_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_nothing() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(7, 7)),
                        None,
                    )
                }
            }
        }

        mod diagonal_neg_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::bishop::Bishop;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(6, 3),
                    Point::new(8, 1),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(6, 3),
                    Point::new(3, 6),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    *piece1().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(3, 6),
                    Point::new(6, 3),
                    Vector::Diagonal(DiagonalVector::BottomRight),
                    *piece2().id(),
                )
            }

            fn segment4() -> Segment {
                Segment::Attack(
                    Point::new(3, 6),
                    Point::new(1, 8),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    *piece2().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::Black,
                    Point::new(6, 3),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn piece2() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::Black,
                    Point::new(3, 6),
                    PieceId::new(2, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3(), segment4()]);
                strategy_points
            }

            mod when_the_given_piece_has_a_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_found_segment() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(4, 5)),
                        Some(&segment2()),
                    )
                }
            }

            mod when_the_given_piece_does_not_have_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_nothing() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(2, 7)),
                        None,
                    )
                }
            }
        }

        mod jump_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::knight::Knight;
            use crate::vector::jump_vector::JumpVector;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(3, 2),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::TopRightLeft),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(5, 6),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomLeftRight),
                    *piece2().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Knight(Knight::new(
                    Color::Black,
                    Point::new(3, 2),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn piece2() -> Piece {
                Piece::Knight(Knight::new(
                    Color::Black,
                    Point::new(5, 6),
                    PieceId::new(2, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2()]);
                strategy_points
            }

            mod when_the_given_piece_has_a_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_found_segment() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(4, 4)),
                        Some(&segment1()),
                    )
                }
            }

            mod when_the_given_piece_does_not_have_segment_affecting_on_the_given_point {
                use super::*;

                #[test]
                fn it_returns_nothing() {
                    let strategy_points = strategy_points();
                    assert_eq!(
                        strategy_points.find_segment(&piece1(), &Point::new(2, 4)),
                        None,
                    )
                }
            }
        }
    }

    mod remove_segments {
        use super::*;

        mod removing_vertical_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::rook::Rook;

            fn segment1() -> Segment {
                Segment::Attack(
                    Point::new(4, 4),
                    Point::new(4, 1),
                    Vector::Line(LineVector::Bottom),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(5, 5),
                    Point::new(5, 1),
                    Vector::Line(LineVector::Bottom),
                    *piece2().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Defense(
                    Point::new(4, 6),
                    Point::new(4, 4),
                    Vector::Line(LineVector::Bottom),
                    *piece3().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(4, 4),
                    PieceId::new(1, &Color::White),
                ))
            }

            fn piece2() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(5, 5),
                    PieceId::new(2, &Color::White),
                ))
            }

            fn piece3() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(4, 6),
                    PieceId::new(3, &Color::White),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3()]);
                strategy_points
            }

            #[test]
            fn it_removes_segments_of_the_given_piece() {
                let mut strategy_points = strategy_points();
                strategy_points.remove_segments(&piece1());

                assert_eq!(
                    strategy_points
                        .vertical_segments
                        .keys()
                        .map(|p| *p)
                        .collect::<Vec<_>>(),
                    vec![Point::new(4, 1), Point::new(5, 1)],
                );

                let vertical_segments1 = strategy_points
                    .vertical_segments
                    .get(&Point::new(4, 1))
                    .unwrap();
                let vertical_segments2 = strategy_points
                    .vertical_segments
                    .get(&Point::new(5, 1))
                    .unwrap();
                assert_eq!(
                    vertical_segments1
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment3()],
                );
                assert_eq!(
                    vertical_segments2
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment2()],
                );
            }

            mod when_removing_all_segments_on_the_given_vertical {
                use super::*;

                #[test]
                fn it_removes_related_lower_bound_point() {
                    let mut strategy_points = strategy_points();
                    strategy_points.remove_segments(&piece1());
                    strategy_points.remove_segments(&piece3());

                    assert_eq!(
                        strategy_points
                            .vertical_segments
                            .keys()
                            .map(|p| *p)
                            .collect::<Vec<_>>(),
                        vec![Point::new(5, 1)],
                    );

                    let vertical_segments = strategy_points
                        .vertical_segments
                        .get(&Point::new(5, 1))
                        .unwrap();

                    assert_eq!(
                        vertical_segments
                            .iter()
                            .map(|s| *s.seg())
                            .collect::<Vec<_>>(),
                        vec![segment2()],
                    );
                }
            }
        }

        mod removing_horizontal_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::rook::Rook;

            fn segment1() -> Segment {
                Segment::Defense(
                    Point::new(3, 5),
                    Point::new(5, 5),
                    Vector::Line(LineVector::Right),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(5, 5),
                    Point::new(8, 5),
                    Vector::Line(LineVector::Right),
                    *piece2().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Attack(
                    Point::new(4, 6),
                    Point::new(8, 6),
                    Vector::Line(LineVector::Right),
                    *piece3().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(3, 5),
                    PieceId::new(1, &Color::White),
                ))
            }

            fn piece2() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(5, 5),
                    PieceId::new(2, &Color::White),
                ))
            }

            fn piece3() -> Piece {
                Piece::Rook(Rook::new(
                    Color::White,
                    Point::new(4, 6),
                    PieceId::new(3, &Color::White),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3()]);
                strategy_points
            }

            #[test]
            fn it_removes_segments_of_the_given_piece() {
                let mut strategy_points = strategy_points();
                strategy_points.remove_segments(&piece2());

                assert_eq!(
                    strategy_points
                        .horizontal_segments
                        .keys()
                        .map(|p| *p)
                        .collect::<Vec<_>>(),
                    vec![Point::new(1, 6), Point::new(1, 5)],
                );

                let horizontal_segments1 = strategy_points
                    .horizontal_segments
                    .get(&Point::new(1, 5))
                    .unwrap();
                let horizontal_segments2 = strategy_points
                    .horizontal_segments
                    .get(&Point::new(1, 6))
                    .unwrap();
                assert_eq!(
                    horizontal_segments1
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment1()],
                );
                assert_eq!(
                    horizontal_segments2
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment3()],
                );
            }

            mod when_removing_all_segments_on_the_given_horizontal {
                use super::*;

                #[test]
                fn it_removes_related_lower_bound_point() {
                    let mut strategy_points = strategy_points();
                    strategy_points.remove_segments(&piece1());
                    strategy_points.remove_segments(&piece2());

                    assert_eq!(
                        strategy_points
                            .horizontal_segments
                            .keys()
                            .map(|p| *p)
                            .collect::<Vec<_>>(),
                        vec![Point::new(1, 6)],
                    );

                    let horizontal_segments = strategy_points
                        .horizontal_segments
                        .get(&Point::new(1, 6))
                        .unwrap();

                    assert_eq!(
                        horizontal_segments
                            .iter()
                            .map(|s| *s.seg())
                            .collect::<Vec<_>>(),
                        vec![segment3()],
                    );
                }
            }
        }

        mod removing_diagonal_pos_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::bishop::Bishop;

            fn segment1() -> Segment {
                Segment::Defense(
                    Point::new(3, 3),
                    Point::new(5, 5),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(5, 5),
                    Point::new(8, 8),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    *piece2().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Attack(
                    Point::new(5, 3),
                    Point::new(8, 6),
                    Vector::Diagonal(DiagonalVector::TopRight),
                    *piece3().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(3, 3),
                    PieceId::new(1, &Color::White),
                ))
            }

            fn piece2() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(5, 5),
                    PieceId::new(2, &Color::White),
                ))
            }

            fn piece3() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(5, 3),
                    PieceId::new(3, &Color::White),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3()]);
                strategy_points
            }

            #[test]
            fn it_removes_segments_of_the_given_piece() {
                let mut strategy_points = strategy_points();
                strategy_points.remove_segments(&piece2());

                assert_eq!(
                    strategy_points
                        .diagonal_pos_segments
                        .keys()
                        .map(|p| *p)
                        .collect::<Vec<_>>(),
                    vec![Point::new(1, 1), Point::new(3, 1)],
                );

                let diagonal_segments1 = strategy_points
                    .diagonal_pos_segments
                    .get(&Point::new(1, 1))
                    .unwrap();
                let diagonal_segments2 = strategy_points
                    .diagonal_pos_segments
                    .get(&Point::new(3, 1))
                    .unwrap();
                assert_eq!(
                    diagonal_segments1
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment1()],
                );
                assert_eq!(
                    diagonal_segments2
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment3()],
                );
            }

            mod when_removing_all_segments_on_the_given_diagonal {
                use super::*;

                #[test]
                fn it_removes_related_lower_bound_point() {
                    let mut strategy_points = strategy_points();
                    strategy_points.remove_segments(&piece1());
                    strategy_points.remove_segments(&piece2());

                    assert_eq!(
                        strategy_points
                            .diagonal_pos_segments
                            .keys()
                            .map(|p| *p)
                            .collect::<Vec<_>>(),
                        vec![Point::new(3, 1)],
                    );

                    let diagonal_segments = strategy_points
                        .diagonal_pos_segments
                        .get(&Point::new(3, 1))
                        .unwrap();

                    assert_eq!(
                        diagonal_segments
                            .iter()
                            .map(|s| *s.seg())
                            .collect::<Vec<_>>(),
                        vec![segment3()],
                    );
                }
            }
        }

        mod removing_diagonal_neg_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::bishop::Bishop;

            fn segment1() -> Segment {
                Segment::Defense(
                    Point::new(7, 3),
                    Point::new(5, 5),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Attack(
                    Point::new(5, 5),
                    Point::new(1, 8),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    *piece2().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Attack(
                    Point::new(5, 3),
                    Point::new(1, 7),
                    Vector::Diagonal(DiagonalVector::TopLeft),
                    *piece3().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(7, 3),
                    PieceId::new(1, &Color::White),
                ))
            }

            fn piece2() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(5, 5),
                    PieceId::new(2, &Color::White),
                ))
            }

            fn piece3() -> Piece {
                Piece::Bishop(Bishop::new(
                    Color::White,
                    Point::new(5, 3),
                    PieceId::new(3, &Color::White),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3()]);
                strategy_points
            }

            #[test]
            fn it_removes_segments_of_the_given_piece() {
                let mut strategy_points = strategy_points();
                strategy_points.remove_segments(&piece2());

                assert_eq!(
                    strategy_points
                        .diagonal_neg_segments
                        .keys()
                        .map(|p| *p)
                        .collect::<Vec<_>>(),
                    vec![Point::new(8, 2), Point::new(7, 1)],
                );

                let diagonal_segments1 = strategy_points
                    .diagonal_neg_segments
                    .get(&Point::new(8, 2))
                    .unwrap();
                let diagonal_segments2 = strategy_points
                    .diagonal_neg_segments
                    .get(&Point::new(7, 1))
                    .unwrap();
                assert_eq!(
                    diagonal_segments1
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment1()],
                );
                assert_eq!(
                    diagonal_segments2
                        .iter()
                        .map(|s| *s.seg())
                        .collect::<Vec<_>>(),
                    vec![segment3()],
                );
            }

            mod when_removing_all_segments_on_the_given_diagonal {
                use super::*;

                #[test]
                fn it_removes_related_lower_bound_point() {
                    let mut strategy_points = strategy_points();
                    strategy_points.remove_segments(&piece1());
                    strategy_points.remove_segments(&piece2());

                    assert_eq!(
                        strategy_points
                            .diagonal_neg_segments
                            .keys()
                            .map(|p| *p)
                            .collect::<Vec<_>>(),
                        vec![Point::new(7, 1)],
                    );

                    let diagonal_segments = strategy_points
                        .diagonal_neg_segments
                        .get(&Point::new(7, 1))
                        .unwrap();

                    assert_eq!(
                        diagonal_segments
                            .iter()
                            .map(|s| *s.seg())
                            .collect::<Vec<_>>(),
                        vec![segment3()],
                    );
                }
            }
        }

        mod removing_jump_segments {
            use super::*;
            use crate::piece::PieceInit;
            use crate::piece::knight::Knight;
            use crate::vector::jump_vector::JumpVector;

            fn segment1() -> Segment {
                Segment::Defense(
                    Point::new(4, 4),
                    Point::new(5, 6),
                    Vector::Jump(JumpVector::TopRightLeft),
                    *piece1().id(),
                )
            }

            fn segment2() -> Segment {
                Segment::Defense(
                    Point::new(5, 6),
                    Point::new(4, 4),
                    Vector::Jump(JumpVector::BottomLeftRight),
                    *piece2().id(),
                )
            }

            fn segment3() -> Segment {
                Segment::Attack(
                    Point::new(5, 3),
                    Point::new(3, 2),
                    Vector::Jump(JumpVector::BottomLeftLeft),
                    *piece3().id(),
                )
            }

            fn piece1() -> Piece {
                Piece::Knight(Knight::new(
                    Color::White,
                    Point::new(4, 4),
                    PieceId::new(1, &Color::White),
                ))
            }

            fn piece2() -> Piece {
                Piece::Knight(Knight::new(
                    Color::White,
                    Point::new(5, 6),
                    PieceId::new(2, &Color::White),
                ))
            }

            fn piece3() -> Piece {
                Piece::Knight(Knight::new(
                    Color::Black,
                    Point::new(5, 3),
                    PieceId::new(1, &Color::Black),
                ))
            }

            fn strategy_points() -> StrategySegments {
                let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
                let mut strategy_points = StrategySegments::empty(&dimension);

                strategy_points.add_segments(vec![segment1(), segment2(), segment3()]);
                strategy_points
            }

            #[test]
            fn it_removes_segments_of_the_given_piece() {
                let mut strategy_points = strategy_points();
                strategy_points.remove_segments(&piece1());

                assert_eq!(
                    strategy_points
                        .jump_segments
                        .keys()
                        .map(|p| *p)
                        .collect::<Vec<_>>(),
                    vec![Point::new(5, 6), Point::new(5, 3)],
                );

                let jump_segments1 = strategy_points
                    .jump_segments
                    .get(&Point::new(5, 6))
                    .unwrap();
                let jump_segments2 = strategy_points
                    .jump_segments
                    .get(&Point::new(5, 3))
                    .unwrap();
                assert_eq!(
                    jump_segments1.iter().map(|s| *s.seg()).collect::<Vec<_>>(),
                    vec![segment2()],
                );
                assert_eq!(
                    jump_segments2.iter().map(|s| *s.seg()).collect::<Vec<_>>(),
                    vec![segment3()],
                );
            }
        }
    }
}
