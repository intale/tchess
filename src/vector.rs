use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::jump_vector::JumpVector;
use crate::vector::line_vector::LineVector;
use crate::point::Point;

pub mod diagonal_vector;
pub mod line_vector;
pub mod jump_vector;

#[derive(Debug, Copy, Clone)]
pub enum Vector {
    Diagonal(DiagonalVector),
    Jump(JumpVector),
    Line(LineVector),
}

impl Vector {
    pub fn calc_next_point(&self, current_point: &Point) -> Point {
        match self {
            Self::Diagonal(d) => d.calc_next_point(current_point),
            Self::Jump(d) => d.calc_next_point(current_point),
            Self::Line(d) => d.calc_next_point(current_point),
        }
    }

    pub fn diagonal_vectors() -> Vec<Self> {
        DiagonalVector::all_variants().into_iter().map(|d| Self::Diagonal(d)).collect::<Vec<_>>()
    }

    pub fn diagonal_and_line_vectors() -> Vec<Self> {
        let mut vectors = Self::diagonal_vectors();
        vectors.append(&mut Self::line_vectors());
        vectors
    }

    pub fn jump_vectors() -> Vec<Self> {
        JumpVector::all_variants().into_iter().map(|d| Self::Jump(d)).collect::<Vec<_>>()
    }

    pub fn line_vectors() -> Vec<Self> {
        LineVector::all_variants().into_iter().map(|d| Self::Line(d)).collect::<Vec<_>>()
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::Diagonal(v) => Self::Diagonal(v.reverse()),
            Self::Jump(v) => Self::Jump(v.reverse()),
            Self::Line(v) => Self::Line(v.reverse()),
        }
    }
}
