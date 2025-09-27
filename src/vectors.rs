use crate::dimension::Dimension;
use crate::directions::diagonal_direction::DiagonalDirection;
use crate::directions::Direction;
use crate::point::Point;
use crate::vectors::diagonal_vector::DiagonalVector;
use crate::vectors::jump_vector::JumpVector;
use crate::vectors::line_vector::LineVector;

pub mod diagonal_vector;
pub mod jump_vector;
pub mod line_vector;

trait VectorStartPoint {
    fn get_start_point(&self) -> &Point;
}

trait VectorDimension {
    fn get_dimension(&self) -> &Dimension;
}

pub enum Vector {
    Diagonal(DiagonalVector),
    Jump(JumpVector),
    Line(LineVector),
}

impl Vector {
    pub fn calc_points<F, FF>(
        &self,
        direction: Direction,
        mut validator: F,
        mut terminator: FF,
    ) -> Vec<Point>
    where
        F: FnMut(&Point) -> bool,
        FF: FnMut(&Point) -> bool,
    {
        let mut points: Vec<Point> = vec![];
        let current_point = self.get_start_point();

        loop {
            let current_point = direction.calc_next_point(&current_point);
            if !self.get_dimension().is_in_boundaries(&current_point) {
                break
            }

            if (validator(&current_point)) {
                points.push(current_point);
            }

            if terminator(&current_point) {
                break;
            }
        }
        points
    }
    
    fn get_start_point(&self) -> &Point {
        match self {
            Self::Diagonal(v) => v.get_start_point(),
            Self::Jump(v) => v.get_start_point(),
            Self::Line(v) => v.get_start_point(),
        }
    }

    fn get_dimension(&self) -> &Dimension {
        match self {
            Self::Diagonal(v) => v.get_dimension(),
            Self::Jump(v) => v.get_dimension(),
            Self::Line(v) => v.get_dimension(),
        }
    }
}
