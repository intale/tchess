use std::fmt::{Display, Formatter};
use crate::piece_id::PieceId;
use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Segment {
    Attack(Point, Point, Vector, PieceId),
    Defense(Point, Point, Vector, PieceId),
    Move(Point, Point, Vector, PieceId),
    BlockedMove(Point, Point, Vector, PieceId),
    DeadEndMove(Point, Point, Vector, PieceId),
    DeadEndAttack(Point, Point, Vector, PieceId),
    DeadEnd(Point, Vector),
    Partial(Point, Vector)
}

impl Segment {
    pub fn point1(&self) -> &Point {
        match self {
            Self::Attack(point1, _, _, _) | Self::Defense(point1, _, _, _) |
            Self::Move(point1, _, _, _) | Self::BlockedMove(point1, _, _, _) |
            Self::DeadEndMove(point1, _, _, _) | Self::DeadEndAttack(point1, _, _, _) |
            Self::DeadEnd(point1, _) | Self::Partial(point1, _) => point1
        }
    }

    pub fn point2(&self) -> &Point {
        match self {
            Self::Attack(_, point2, _, _) | Self::Defense(_, point2, _, _) |
            Self::Move(_, point2, _, _) | Self::BlockedMove(_, point2, _, _) |
            Self::DeadEndMove(_, point2, _, _) |
            Self::DeadEndAttack(_, point2, _, _) => point2,
            Self::DeadEnd(_, _) | Self::Partial(_, _) => panic!("{:?} does not hold point2", self)
        }
    }

    pub fn vector(&self) -> &Vector {
        match self {
            Self::Attack(_, _, vec, _) | Self::Defense(_, _, vec, _) |
            Self::Move(_, _, vec, _) | Self::BlockedMove(_, _, vec, _) |
            Self::DeadEndMove(_, _, vec, _) | Self::DeadEndAttack(_, _, vec, _) |
            Self::DeadEnd(_, vec) | Self::Partial(_, vec) => vec
        }
    }

    pub fn piece_id(&self) -> &PieceId {
        match self {
            Self::Attack(_, _, _, piece_id) | Self::Defense(_, _, _, piece_id) |
            Self::Move(_, _, _, piece_id) | Self::BlockedMove(_, _, _, piece_id) |
            Self::DeadEndMove(_, _, _, piece_id) |
            Self::DeadEndAttack(_, _, _, piece_id) => piece_id,
            Self::DeadEnd(_, _) | Self::Partial(_, _) => panic!("{:?} does not hold piece_id", self)
        }
    }

    pub fn at_prev_point(&self) -> Self {
        match self {
            Self::Partial(point, vec) => {
                let next_point = vec.inverse().calc_next_point(point);
                Self::Partial(next_point, *vec)
            },
            _ => panic!("Can't calculate next partial segment of {:?}", self),
        }
    }

    pub fn at_next_point(&self) -> Self {
        match self {
            Self::Partial(point, vec) => {
                let next_point = vec.calc_next_point(point);
                Self::Partial(next_point, *vec)
            },
            _ => panic!("Can't calculate next partial segment of {:?}", self),
        }
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        if point == self.point2() {
            return true
        }
        let vec_from_point1 = Vector::calc_direction(self.point1(), point);
        let vec_from_point2 = Vector::calc_direction(self.point2(), point);
        if let Some(vec_from_point1) = vec_from_point1
            && let Some(vec_from_point2) = vec_from_point2
        {
            self.vector() == &vec_from_point1 && self.vector() == &vec_from_point2.inverse()
        } else {
            false
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attack(point1, point2, vec, piece_id) => {
                write!(f, "⚔({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::Move(point1, point2, vec, piece_id) => {
                write!(f, "⇧({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::Defense(point1, point2, vec, piece_id) => {
                write!(f, "⛨({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::BlockedMove(point1, point2, vec, piece_id) => {
                write!(f, "🛇({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::DeadEndMove(point1, point2, vec, piece_id) => {
                write!(f, "⇧🧱({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::DeadEndAttack(point1, point2, vec, piece_id) => {
                write!(f, "⚔🧱({} {} {})#{}", point1, vec, point2, piece_id)
            },
            Self::DeadEnd(point, vec) => {
                write!(f, "🧱({} {})", point, vec)
            },
            Self::Partial(point, vec) => {
                write!(f, "❓({} {})", point, vec)
            },
        }
    }
}
