use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum JumpVector {
    TopLeftLeft,
    TopLeftRight,
    TopRightLeft,
    TopRightRight,
    BottomLeftLeft,
    BottomLeftRight,
    BottomRightLeft,
    BottomRightRight,
}

impl JumpVector {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::TopLeftLeft,
            Self::TopLeftRight,
            Self::TopRightLeft,
            Self::TopRightRight,
            Self::BottomLeftLeft,
            Self::BottomLeftRight,
            Self::BottomRightLeft,
            Self::BottomRightRight,
        ]
    }

    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        let (x1, y1) = point1.to_tuple();
        let (x2, y2) = point2.to_tuple();

        if !((x1 - x2).abs() == 1 && (y1 - y2).abs() == 2 ||
            (x1 - x2).abs() == 2 && (y1 - y2).abs() == 1) {
            return None;
        }

        match (x1 - x2, y1 - y2) {
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
        use super::*;

        #[test]
        fn inverting_top_left_left() {
            assert_eq!(JumpVector::TopLeftLeft.inverse(), JumpVector::BottomRightRight);
        }

        #[test]
        fn inverting_top_left_right() {
            assert_eq!(JumpVector::TopLeftRight.inverse(), JumpVector::BottomRightLeft);
        }

        #[test]
        fn inverting_top_right_left() {
            assert_eq!(JumpVector::TopRightLeft.inverse(), JumpVector::BottomLeftRight);
        }

        #[test]
        fn inverting_top_right_right() {
            assert_eq!(JumpVector::TopRightRight.inverse(), JumpVector::BottomLeftLeft);
        }

        #[test]
        fn inverting_bottom_right_right() {
            assert_eq!(JumpVector::BottomRightRight.inverse(), JumpVector::TopLeftLeft);
        }

        #[test]
        fn inverting_bottom_right_left() {
            assert_eq!(JumpVector::BottomRightLeft.inverse(), JumpVector::TopLeftRight);
        }

        #[test]
        fn inverting_bottom_left_right() {
            assert_eq!(JumpVector::BottomLeftRight.inverse(), JumpVector::TopRightLeft);
        }

        #[test]
        fn inverting_bottom_left_left() {
            assert_eq!(JumpVector::BottomLeftLeft.inverse(), JumpVector::TopRightRight);
        }
    }
}
