use crate::dimension::Dimension;
use crate::point::Point;
use crate::vector::Vector;

pub struct VectorPoints {
    start_point: Point,
    dimension: Dimension,
}

impl VectorPoints {
    pub fn new(start_point: Point, dimension: Dimension) -> Self {
        Self { start_point, dimension }
    }

    pub fn calc_points<F, FF>(
        &self,
        vector: Vector,
        mut validator: F,
        mut terminator: FF,
    ) -> Vec<Point>
    where
        F: FnMut(&Point) -> bool,
        FF: FnMut(&Point) -> bool,
    {
        let mut points: Vec<Point> = vec![];
        let current_point = self.start_point;

        loop {
            let current_point = vector.calc_next_point(&current_point);
            if !self.dimension.is_in_boundaries(&current_point) {
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
}
