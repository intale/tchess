use crate::vector::line_vector::LineVector;
use crate::vector::Vector;
use crate::point::Point;

const QUEEN_SIDE_KING_CASTLE_X_POS: i16 = 3;
const QUEEN_SIDE_ROOK_CASTLE_X_POS: i16 = 4;
const KING_SIDE_KING_CASTLE_X_POS: i16 = 7;
const KING_SIDE_ROOK_CASTLE_X_POS: i16 = 6;

#[derive(Debug, Eq, PartialEq, Hash)]
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

    pub fn castle_points(&self, y: i16) -> (Point, Point, Self) {
        match self {
            Self::Queen => {
                let king_point = Point::new(QUEEN_SIDE_KING_CASTLE_X_POS, y);
                let rook_point = Point::new(QUEEN_SIDE_ROOK_CASTLE_X_POS, y);
                (king_point, rook_point, Self::Queen)
            },
            Self::King => {
                let king_point = Point::new(KING_SIDE_KING_CASTLE_X_POS, y);
                let rook_point = Point::new(KING_SIDE_ROOK_CASTLE_X_POS, y);
                (king_point, rook_point, Self::King)
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;
}
