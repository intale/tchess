use std::ops::Add;
use crate::dimension::Dimension;
use crate::point::Point;
use crate::vector::Vector;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;

#[derive(Debug, Copy, Clone)]
pub struct VectorPoints {
    current_point: Point,
    dimension: Dimension,
    vector: Vector,
}

impl VectorPoints {
    pub fn with_initial(starting_point: Point, dimension: Dimension, vector: Vector) -> Self {
        Self {
            current_point: starting_point,
            dimension,
            vector,
        }
    }

    pub fn without_initial(starting_point: Point, dimension: Dimension, vector: Vector) -> Self {
        let mut vector_points = Self {
            current_point: starting_point,
            dimension,
            vector,
        };
        vector_points.next();
        vector_points
    }

    pub fn last_point(&self) -> Point {
        let point = match self.vector {
            Vector::Line(line_vec) => match line_vec {
                LineVector::Top => {
                    Point::new(**self.current_point.x(), **self.dimension.max_point().y())
                }
                LineVector::Bottom => {
                    Point::new(**self.current_point.x(), **self.dimension.min_point().y())
                }
                LineVector::Left => {
                    Point::new(**self.dimension.min_point().x(), **self.current_point.y())
                }
                LineVector::Right => {
                    Point::new(**self.dimension.max_point().x(), **self.current_point.y())
                }
            },
            Vector::Diagonal(diag_vec) => {
                let current_x = **self.current_point.x() as i32;
                let current_y = **self.current_point.y() as i32;
                let min_x = **self.dimension.min_point().x() as i32;
                let min_y = **self.dimension.min_point().y() as i32;
                let max_x = **self.dimension.max_point().x() as i32;
                let max_y = **self.dimension.max_point().y() as i32;
                let find_y = |x: i32| match diag_vec {
                    DiagonalVector::TopRight | DiagonalVector::BottomLeft => {
                        x + current_y - current_x
                    }
                    DiagonalVector::TopLeft | DiagonalVector::BottomRight => {
                        current_x + current_y - x
                    }
                };
                let find_x = |y: i32| match diag_vec {
                    DiagonalVector::TopRight | DiagonalVector::BottomLeft => {
                        y - (current_y - current_x)
                    }
                    DiagonalVector::TopLeft | DiagonalVector::BottomRight => {
                        current_x + current_y - y
                    }
                };

                let variants = match diag_vec {
                    DiagonalVector::TopRight => [
                        Point::new(
                            find_x(max_y) as i16,
                            max_y as i16,
                        ),
                        Point::new(
                            max_x as i16,
                            find_y(max_x) as i16,
                        ),
                    ],
                    DiagonalVector::BottomLeft => [
                        Point::new(
                            find_x(min_y) as i16,
                            min_y as i16,
                        ),
                        Point::new(
                            min_x as i16,
                            find_y(min_x) as i16,
                        ),
                    ],
                    DiagonalVector::TopLeft => [
                        Point::new(
                            min_x as i16,
                            find_y(min_x) as i16,
                        ),
                        Point::new(
                            find_x(max_y) as i16,
                            max_y as i16,
                        ),
                    ],
                    DiagonalVector::BottomRight => [
                        Point::new(
                            find_x(min_y) as i16,
                            min_y as i16,
                        ),
                        Point::new(
                            max_x as i16,
                            find_y(max_x) as i16,
                        ),
                    ],
                };
                let res = variants
                    .into_iter()
                    .find(|point| self.dimension.is_in_boundaries(point));
                res.expect(
                    format!(
                        "Logical error: could not find last point of {}{}",
                        diag_vec, self.current_point
                    )
                        .as_str(),
                )
            }
            Vector::Jump(_) => todo!(),
        };
        point
    }

    pub fn point_at_distance(&self, distance: u16) -> Option<Point> {
        let (x, y) = self.current_point.to_tuple();
        let point = match self.vector {
            Vector::Line(line_vec) => {
                match line_vec {
                    LineVector::Top => {
                        let y = y.wrapping_add_unsigned(distance);
                        Point::new(*x, y)
                    }
                    LineVector::Bottom => {
                        let y = y.wrapping_sub_unsigned(distance);
                        Point::new(*x, y)
                    }
                    LineVector::Left => {
                        let x = x.wrapping_sub_unsigned(distance);
                        Point::new(x, *y)
                    }
                    LineVector::Right => {
                        let x = x.wrapping_add_unsigned(distance);
                        Point::new(x, *y)
                    }
                }
            }
            Vector::Diagonal(diag_vec) => {
                match diag_vec {
                    DiagonalVector::TopRight => {
                        let x = x.wrapping_add_unsigned(distance);
                        let y = y.wrapping_add_unsigned(distance);
                        Point::new(x, y)
                    }
                    DiagonalVector::BottomLeft => {
                        let x = x.wrapping_sub_unsigned(distance);
                        let y = y.wrapping_sub_unsigned(distance);
                        Point::new(x, y)
                    }
                    DiagonalVector::TopLeft => {
                        let x = x.wrapping_sub_unsigned(distance);
                        let y = y.wrapping_add_unsigned(distance);
                        Point::new(x, y)
                    }
                    DiagonalVector::BottomRight => {
                        let x = x.wrapping_add_unsigned(distance);
                        let y = y.wrapping_sub_unsigned(distance);
                        Point::new(x, y)
                    }
                }
            }
            Vector::Jump(_) => panic!("Don't know how to add distance to jump vector")
        };
        if self.dimension.is_in_boundaries(&point) {
            return Some(point)
        }
        None
    }
}

impl Iterator for VectorPoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.dimension.is_in_boundaries(&self.current_point) {
            return None;
        }

        let current_point = self.current_point;
        self.current_point = self.vector.calc_next_point(&self.current_point);
        Some(current_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::line_vector::LineVector;

    #[test]
    fn test_iteration_including_initial_point() {
        let point = Point::new(1, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::with_initial(point, dimension, vector);
        assert_eq!(vector_points.next(), Some(point));
        assert_eq!(vector_points.next(), Some(Point::new(1, 2)));
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_out_ouf_bounce_iteration_including_initial_point() {
        let point = Point::new(3, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::with_initial(point, dimension, vector);
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_iteration_excluding_initial_point() {
        let point = Point::new(1, 1);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::without_initial(point, dimension, vector);
        assert_eq!(vector_points.next(), Some(Point::new(1, 2)));
        assert_eq!(vector_points.next(), None);
    }

    #[test]
    fn test_out_of_bounce_iteration_excluding_initial_point() {
        let point = Point::new(1, 2);
        let dimension = Dimension::new(Point::new(0, 0), Point::new(2, 2));
        let vector = Vector::Line(LineVector::Top);
        let mut vector_points = VectorPoints::without_initial(point, dimension, vector);
        assert_eq!(vector_points.next(), None);
    }

    mod last_point {
        use super::*;

        fn vector_points(starting_point: Point, vector: Vector) -> VectorPoints {
            let dimension = Dimension::new(Point::new(-2, -2), Point::new(8, 8));
            VectorPoints::with_initial(starting_point, dimension, vector)
        }

        mod line_top {
            use super::*;

            #[test]
            fn it_calculates_last_point() {
                let vector_points = vector_points(Point::new(6, 4), Vector::Line(LineVector::Top));
                assert_eq!(vector_points.last_point(), Point::new(6, 8))
            }
        }

        mod line_bottom {
            use super::*;

            #[test]
            fn it_calculates_last_point() {
                let vector_points =
                    vector_points(Point::new(6, 4), Vector::Line(LineVector::Bottom));
                assert_eq!(vector_points.last_point(), Point::new(6, -2))
            }
        }

        mod line_left {
            use super::*;

            #[test]
            fn it_calculates_last_point() {
                let vector_points = vector_points(Point::new(6, 4), Vector::Line(LineVector::Left));
                assert_eq!(vector_points.last_point(), Point::new(-2, 4))
            }
        }

        mod line_right {
            use super::*;

            #[test]
            fn it_calculates_last_point() {
                let vector_points =
                    vector_points(Point::new(6, 4), Vector::Line(LineVector::Right));
                assert_eq!(vector_points.last_point(), Point::new(8, 4))
            }
        }

        mod diagonal_top_right {
            use super::*;

            mod when_position_is_above_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(1, 5), Vector::Diagonal(DiagonalVector::TopRight));
                    assert_eq!(vector_points.last_point(), Point::new(4, 8))
                }
            }

            mod when_position_is_under_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(4, 2), Vector::Diagonal(DiagonalVector::TopRight));
                    assert_eq!(vector_points.last_point(), Point::new(8, 6))
                }
            }

            mod when_position_is_on_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(0, 0), Vector::Diagonal(DiagonalVector::TopRight));
                    assert_eq!(vector_points.last_point(), Point::new(8, 8))
                }
            }

            mod when_position_in_on_the_right_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(8, 7), Vector::Diagonal(DiagonalVector::TopRight));
                    assert_eq!(vector_points.last_point(), Point::new(8, 7))
                }
            }

            mod when_position_in_on_the_top_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(7, 8), Vector::Diagonal(DiagonalVector::TopRight));
                    assert_eq!(vector_points.last_point(), Point::new(7, 8))
                }
            }
        }

        mod diagonal_bottom_left {
            use super::*;

            mod when_position_is_above_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(1, 5),
                        Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(-2, 2))
                }
            }

            mod when_position_is_under_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(4, 2),
                        Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(0, -2))
                }
            }

            mod when_position_is_on_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(0, 0),
                        Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(-2, -2))
                }
            }

            mod when_position_is_on_the_left_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(-2, 0),
                        Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(-2, 0))
                }
            }

            mod when_position_is_on_the_bottom_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(0, -2),
                        Vector::Diagonal(DiagonalVector::BottomLeft),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(0, -2))
                }
            }
        }

        mod diagonal_top_left {
            use super::*;

            mod when_position_is_above_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(7, 5), Vector::Diagonal(DiagonalVector::TopLeft));
                    assert_eq!(vector_points.last_point(), Point::new(4, 8))
                }
            }

            mod when_position_is_under_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(2, 1), Vector::Diagonal(DiagonalVector::TopLeft));
                    assert_eq!(vector_points.last_point(), Point::new(-2, 5))
                }
            }

            mod when_position_is_on_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(5, 1), Vector::Diagonal(DiagonalVector::TopLeft));
                    assert_eq!(vector_points.last_point(), Point::new(-2, 8))
                }
            }

            mod when_position_in_on_the_top_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(7, 8), Vector::Diagonal(DiagonalVector::TopLeft));
                    assert_eq!(vector_points.last_point(), Point::new(7, 8))
                }
            }

            mod when_position_is_on_the_left_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points =
                        vector_points(Point::new(-2, 0), Vector::Diagonal(DiagonalVector::TopLeft));
                    assert_eq!(vector_points.last_point(), Point::new(-2, 0))
                }
            }
        }

        mod diagonal_bottom_right {
            use super::*;

            mod when_position_is_above_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(7, 5),
                        Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(8, 4))
                }
            }

            mod when_position_is_under_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(2, 1),
                        Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(5, -2))
                }
            }

            mod when_position_is_on_the_central_diagonal {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(5, 1),
                        Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(8, -2))
                }
            }

            mod when_position_is_on_the_right_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(8, 3),
                        Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(8, 3))
                }
            }

            mod when_position_is_on_the_bottom_edge {
                use super::*;

                #[test]
                fn it_calculates_last_point() {
                    let vector_points = vector_points(
                        Point::new(0, -2),
                        Vector::Diagonal(DiagonalVector::BottomRight),
                    );
                    assert_eq!(vector_points.last_point(), Point::new(0, -2))
                }
            }
        }
    }
}
