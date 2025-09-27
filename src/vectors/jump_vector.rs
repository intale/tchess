use crate::dimension::Dimension;
use crate::directions::jump_direction::JumpDirection;
use crate::point::Point;

pub struct JumpVector {
    start_point: Point,
    dimension: Dimension,
}

impl JumpVector {
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
