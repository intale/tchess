use crate::dimension::Dimension;
use crate::directions::diagonal_direction::DiagonalDirection;
use crate::point::Point;

pub struct DiagonalVector {
    start_point: Point,
    dimension: Dimension,
}

impl DiagonalVector {
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
