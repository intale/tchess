use crate::directions::diagonal_direction::DiagonalDirection;
use crate::directions::jump_direction::JumpDirection;
use crate::directions::line_direction::LineDirection;
use crate::point::Point;

pub mod diagonal_direction;
pub mod line_direction;
pub mod jump_direction;

pub enum Direction {
    Diagonal(DiagonalDirection),
    Jump(JumpDirection),
    Line(LineDirection),
}

impl Direction {
    pub fn calc_next_point(&self, current_point: &Point) -> Point {
        match self {
            Self::Diagonal(d) => d.calc_next_point(current_point),
            Self::Jump(d) => d.calc_next_point(current_point),
            Self::Line(d) => d.calc_next_point(current_point),
        }
    }

    pub fn diagonal_directions() -> Vec<Self> {
        DiagonalDirection::all_variants().into_iter().map(|d| Self::Diagonal(d)).collect::<Vec<_>>()
    }

    pub fn jump_directions() -> Vec<Self> {
        JumpDirection::all_variants().into_iter().map(|d| Self::Jump(d)).collect::<Vec<_>>()
    }

    pub fn line_directions() -> Vec<Self> {
        LineDirection::all_variants().into_iter().map(|d| Self::Line(d)).collect::<Vec<_>>()
    }
}
