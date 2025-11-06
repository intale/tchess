use std::cmp::PartialEq;
use crate::castle_points::CastlePoints;
use crate::point::Point;

#[derive(Hash, Debug, Eq, PartialEq)]
pub enum PieceMove {
    Point(Point),
    Castle(CastlePoints),
}

#[cfg(test)]
mod tests {
    use super::*;
}
