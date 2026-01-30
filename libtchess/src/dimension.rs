use std::ops::{RangeInclusive};
use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Dimension {
    min_point: Point,
    max_point: Point,
}

impl Dimension {
    pub fn new(min_point: Point, max_point: Point) -> Self {
        Self { min_point, max_point }
    }

    pub fn is_in_boundaries(&self, point: &Point) -> bool {
        let (min_x, min_y) = self.min_point.to_tuple();
        let (max_x, max_y) = self.max_point.to_tuple();
        let (point_x, point_y) = point.to_tuple();
        (min_x..=max_x).contains(&point_x) && (min_y..=max_y).contains(&point_y)
    }

    pub fn get_columns_num(&self) -> i16 {
        self.max_point.x() - self.min_point.x()
    }

    pub fn get_rows_num(&self) -> i16 {
        self.max_point.y() - self.min_point.y()
    }

    pub fn min_point(&self) -> &Point {
        &self.min_point
    }

    pub fn max_point(&self) -> &Point {
        &self.max_point
    }

    pub fn get_columns_range(&self) -> RangeInclusive<i16> {
        *self.min_point.x().value()..=*self.max_point.x().value()
    }

    pub fn get_rows_range(&self) -> RangeInclusive<i16> {
        *self.min_point.y().value()..=*self.max_point.y().value()
    }
}
