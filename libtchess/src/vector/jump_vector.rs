use std::fmt::{Display, Formatter};
use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum JumpVector {
    BottomLeftRight,
    BottomRightLeft,
    BottomLeftLeft,
    BottomRightRight,
    TopLeftLeft,
    TopRightRight,
    TopLeftRight,
    TopRightLeft,
}

impl JumpVector {
    pub const fn all_variants() -> [Self; 8] {
        [
            Self::BottomLeftRight,
            Self::BottomRightLeft,
            Self::BottomLeftLeft,
            Self::BottomRightRight,
            Self::TopLeftLeft,
            Self::TopRightRight,
            Self::TopLeftRight,
            Self::TopRightLeft,
        ]
    }

    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        let (x1, y1) = point1.to_tuple();
        let (x2, y2) = point2.to_tuple();

        let delta_x = x1.wrapping_sub(*x2);
        let delta_y = y1.wrapping_sub(*y2);

        if !(delta_x.abs() == 1 && delta_y.abs() == 2 ||
            delta_x.abs() == 2 && delta_y.abs() == 1) {
            return None;
        }

        match (delta_x, delta_y) {
            (1, 2) => Some(Self::BottomLeftRight),
            (2, 1) => Some(Self::BottomLeftLeft),
            (2, -1) => Some(Self::TopLeftLeft),
            (1, -2) => Some(Self::TopLeftRight),
            (-1, -2) => Some(Self::TopRightLeft),
            (-2, -1) => Some(Self::TopRightRight),
            (-2, 1) => Some(Self::BottomRightRight),
            (-1, 2) => Some(Self::BottomRightLeft),
            _ => None
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::TopLeftLeft => Self::BottomRightRight,
            Self::TopLeftRight => Self::BottomRightLeft,
            Self::TopRightLeft => Self::BottomLeftRight,
            Self::TopRightRight => Self::BottomLeftLeft,
            Self::BottomRightRight => Self::TopLeftLeft,
            Self::BottomRightLeft => Self::TopLeftRight,
            Self::BottomLeftRight => Self::TopRightLeft,
            Self::BottomLeftLeft => Self::TopRightRight,
        }
    }

    pub fn is_ascending(&self) -> bool {
        match self {
            Self::TopLeftLeft | Self::TopLeftRight | Self::TopRightLeft | Self::TopRightRight => true,
            Self::BottomRightRight | Self::BottomRightLeft | Self::BottomLeftRight | Self::BottomLeftLeft => false,
        }
    }

    pub fn calc_next_point(&self, current_point: &Point) -> Point {
        let (&x, &y) = current_point.to_tuple();
        let (mut x, mut y) = (x, y);
        match self {
            Self::TopLeftLeft => {
                x -= 2;
                y += 1;
            }
            Self::TopLeftRight => {
                x -= 1;
                y += 2;
            }
            Self::TopRightLeft => {
                x += 1;
                y += 2;
            }
            Self::TopRightRight => {
                x += 2;
                y += 1;
            }
            Self::BottomLeftLeft => {
                x -= 2;
                y -= 1;
            }
            Self::BottomLeftRight => {
                x -= 1;
                y -= 2;
            }
            Self::BottomRightLeft => {
                x += 1;
                y -= 2;
            }
            Self::BottomRightRight => {
                x += 2;
                y -= 1;
            }
        }
        Point::new(x, y)
    }

    pub fn seq_num(&self) -> u8 {
        match self {
            Self::TopLeftLeft => 0,
            Self::TopLeftRight => 1,
            Self::TopRightLeft => 2,
            Self::TopRightRight => 3,
            Self::BottomLeftLeft => 4,
            Self::BottomLeftRight => 5,
            Self::BottomRightLeft => 6,
            Self::BottomRightRight => 7,
        }
    }
}

impl Display for JumpVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arrow = match self {
            Self::BottomLeftRight => '↲',
            Self::BottomRightLeft => '↳',
            Self::BottomLeftLeft => '⬐',
            Self::BottomRightRight => '⬎',
            Self::TopLeftLeft => '⬑',
            Self::TopRightRight => '⬏',
            Self::TopLeftRight => '↰',
            Self::TopRightLeft => '↱',
        };
        write!(f, "{}", arrow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_next_top_left_left_point() {
        let direction = JumpVector::TopLeftLeft;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(2, 5));
    }

    #[test]
    fn test_calc_next_top_left_right_point() {
        let direction = JumpVector::TopLeftRight;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(3, 6));
    }

    #[test]
    fn test_calc_next_top_right_left_point() {
        let direction = JumpVector::TopRightLeft;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(5, 6));
    }

    #[test]
    fn test_calc_next_top_right_right_point() {
        let direction = JumpVector::TopRightRight;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(6, 5));
    }

    #[test]
    fn test_calc_next_bottom_left_left_point() {
        let direction = JumpVector::BottomLeftLeft;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(2, 3));
    }

    #[test]
    fn test_calc_next_bottom_left_right_point() {
        let direction = JumpVector::BottomLeftRight;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(3, 2));
    }

    #[test]
    fn test_calc_next_bottom_right_left_point() {
        let direction = JumpVector::BottomRightLeft;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(5, 2));
    }

    #[test]
    fn test_calc_next_bottom_right_right_point() {
        let direction = JumpVector::BottomRightRight;
        let point = Point::new(4, 4);
        assert_eq!(direction.calc_next_point(&point), Point::new(6, 3));
    }

    mod direction_between_two_points {
        use super::*;

        #[test]
        fn bottom_left_right_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(2, 1);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::BottomLeftRight)
            );
        }

        #[test]
        fn bottom_left_left_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(1, 2);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::BottomLeftLeft)
            );
        }

        #[test]
        fn top_left_left_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(1, 4);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::TopLeftLeft)
            );
        }

        #[test]
        fn top_left_right_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(2, 5);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::TopLeftRight)
            );
        }

        #[test]
        fn top_right_left_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(4, 5);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::TopRightLeft)
            );
        }

        #[test]
        fn top_right_right_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(5, 4);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::TopRightRight)
            );
        }

        #[test]
        fn bottom_right_right_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(5, 2);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::BottomRightRight)
            );
        }

        #[test]
        fn bottom_right_left_direction() {
            let point1 = Point::new(3, 3);
            let point2 = Point::new(4, 1);
            assert_eq!(
                JumpVector::calc_direction(&point1, &point2),
                Some(JumpVector::BottomRightLeft)
            );
        }
    }

    mod inverting_direction {
        use crate::dimension::Dimension;
        use crate::vector::Vector;
        use crate::vector_points::VectorPoints;
        use super::*;

        fn dimension() -> Dimension {
            Dimension::new(Point::new(1, 1), Point::new(8, 8))
        }

        fn inverted_point(forward_vec: JumpVector, initial_point: Point) -> Point {
            let backward = forward_vec.inverse();
            let mut vector_points_forward = VectorPoints::without_initial(
                initial_point, dimension(), Vector::Jump(forward_vec)
            );
            let mut vector_points_backward = VectorPoints::without_initial(
                vector_points_forward.next().unwrap(), dimension(), Vector::Jump(backward)
            );
            vector_points_backward.next().unwrap()
        }

        #[test]
        fn inverting_top_left_left() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::TopLeftLeft, initial_point), initial_point);
        }

        #[test]
        fn inverting_top_left_right() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::TopLeftRight, initial_point), initial_point);
        }

        #[test]
        fn inverting_top_right_left() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::TopRightLeft, initial_point), initial_point);
        }

        #[test]
        fn inverting_top_right_right() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::TopRightRight, initial_point), initial_point);
        }

        #[test]
        fn inverting_bottom_right_right() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::BottomRightRight, initial_point), initial_point);
        }

        #[test]
        fn inverting_bottom_right_left() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::BottomRightLeft, initial_point), initial_point);
        }

        #[test]
        fn inverting_bottom_left_right() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::BottomLeftRight, initial_point), initial_point);
        }

        #[test]
        fn inverting_bottom_left_left() {
            let initial_point = Point::new(4, 4);
            assert_eq!(inverted_point(JumpVector::BottomLeftLeft, initial_point), initial_point);
        }
    }
}
