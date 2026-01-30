use crate::vector::line_vector::LineVector;
use crate::vector::Vector;
use crate::point::Point;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum CastleSide {
    Queen,
    King,
}

impl CastleSide {
    pub fn direction(&self) -> Vector {
        match self {
            Self::Queen => Vector::Line(LineVector::Left),
            Self::King => Vector::Line(LineVector::Right),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct CastlePoints {
    king_point: Point,
    rook_point: Point,
    initial_king_point: Point,
    initial_rook_point: Point,
    side: CastleSide,
}

impl CastlePoints {
    pub fn new(king_point: Point, rook_point: Point,
               initial_king_point: Point, initial_rook_point: Point) -> Self {
        let side = if king_point.x() < rook_point.x() {
            CastleSide::Queen
        } else {
            CastleSide::King
        };
        Self { king_point, rook_point, initial_king_point, initial_rook_point, side }
    }

    pub fn king_point(&self) -> &Point {
        &self.king_point
    }

    pub fn rook_point(&self) -> &Point {
        &self.rook_point
    }

    pub fn initial_king_point(&self) -> &Point {
        &self.initial_king_point
    }

    pub fn initial_rook_point(&self) -> &Point {
        &self.initial_rook_point
    }

    pub fn side(&self) -> &CastleSide {
        &self.side
    }
}
