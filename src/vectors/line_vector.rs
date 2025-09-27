use crate::dimension::Dimension;
use crate::directions::line_direction::LineDirection;
use crate::point::Point;

pub struct LineVector {
    start_point: Point,
    dimension: Dimension,
}

impl LineVector {
    pub fn new(start_point: Point, dimension: Dimension) -> Self {
        Self { start_point, dimension }
    }

    pub fn get_start_point(&self) -> &Point {
        &self.start_point
    }

    pub fn get_dimension(&self) -> &Dimension {
        &self.dimension
    }
}
