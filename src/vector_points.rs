use crate::dimension::Dimension;
use crate::point::Point;
use crate::vector::Vector;

pub struct VectorPoints {
    current_point: Point,
    dimension: Dimension,
    vector: Vector,
}

impl VectorPoints {
    pub fn with_initial(starting_point: Point, dimension: Dimension, vector: Vector) -> Self {
        Self { current_point: starting_point, dimension, vector }
    }

    pub fn without_initial(starting_point: Point, dimension: Dimension, vector: Vector) -> Self {
        let mut vector_points = Self {
            current_point: starting_point, dimension, vector
        };
        vector_points.next();
        vector_points
    }
}

impl Iterator for VectorPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.dimension.is_in_boundaries(&self.current_point) {
            return None
        }

        let current_point = self.current_point;
        self.current_point = self.vector.calc_next_point(&self.current_point);
        Some(current_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::line_vector::LineVector;
    use super::*;

    #[test]
    fn test_iteration_including_initial_point() {
        let point = Point::new(1, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::with_initial(
            point,
            dimension,
            vector
        );
        assert_eq!(vector_points.next(), Some(point));
        assert_eq!(vector_points.next(), Some(Point::new(1, 2)));
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_out_ouf_bounce_iteration_including_initial_point() {
        let point = Point::new(3, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::with_initial(
            point,
            dimension,
            vector
        );
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_iteration_excluding_initial_point() {
        let point = Point::new(1, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::without_initial(
            point,
            dimension,
            vector
        );
        assert_eq!(vector_points.next(), Some(Point::new(1, 2)));
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_out_of_bounce_iteration_excluding_initial_point() {
        let point = Point::new(1, 2);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::without_initial(
            point,
            dimension,
            vector
        );
        assert_eq!(vector_points.next(), None);
    }
}
