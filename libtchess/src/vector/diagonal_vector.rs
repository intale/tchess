use std::fmt::{Display, Formatter};
use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum DiagonalVector {
    BottomLeft,
    TopRight,
    BottomRight,
    TopLeft,
}

impl DiagonalVector {
    pub const fn all_variants() -> [Self; 4] {
        [
            Self::BottomLeft,
            Self::TopRight,
            Self::BottomRight,
            Self::TopLeft,
        ]
    }
}

impl DiagonalVector {
    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        let (x1, y1) = point1.to_tuple();
        let (x2, y2) = point2.to_tuple();

        let delta_x = x1.wrapping_sub(*x2);
        let delta_y = y1.wrapping_sub(*y2);

        if delta_x.abs() != delta_y.abs() {
            return None;
        }

        match (delta_x, delta_y) {
            (i16::MIN..0, 1..=i16::MAX) => Some(Self::BottomRight),
            (1..=i16::MAX, 1..=i16::MAX) => Some(Self::BottomLeft),
            (i16::MIN..0, i16::MIN..0) => Some(Self::TopRight),
            (1..=i16::MAX, i16::MIN..0) => Some(Self::TopLeft),
            (_, 0) | (0, _) => None,
        }
    }

    pub fn distance(point1: &Point, point2: &Point) -> Option<i32> {
        if Self::calc_direction(point1, point2).is_some() {
            let x1 = **point1.x() as i32;
            let x2 = **point2.x() as i32;
            return Some((x1 - x2).abs())
        }
        None
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::TopLeft => Self::BottomRight,
            Self::TopRight => Self::BottomLeft,
            Self::BottomLeft => Self::TopRight,
            Self::BottomRight => Self::TopLeft,
        }
    }
    
    pub fn is_ascending(&self) -> bool {
        match self {
            Self::TopLeft | Self::TopRight => true,
            Self::BottomLeft | Self::BottomRight => false,
        }
    }

    pub fn calc_next_point(&self, current_point: &Point) -> Point {
        let (&x, &y) = current_point.to_tuple();
        let (mut x, mut y) = (x, y);
        match self {
            Self::TopLeft => {
                x -= 1;
                y += 1;
            }
            Self::TopRight => {
                x += 1;
                y += 1;
            }
            Self::BottomLeft => {
                x -= 1;
                y -= 1;
            }
            Self::BottomRight => {
                x += 1;
                y -= 1;
            }
        }
        Point::new(x, y)
    }
}

impl Display for DiagonalVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arrow = match self {
            Self::TopRight => '↗',
            Self::BottomLeft => '↙',
            Self::TopLeft => '↖',
            Self::BottomRight => '↘',
        };
        write!(f, "{}", arrow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_top_right_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(1, 1);
        assert_eq!(DiagonalVector::calc_direction(&point1, &point2), Some(DiagonalVector::TopRight));
    }

    #[test]
    fn test_calc_top_left_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(-1, 1);
        assert_eq!(DiagonalVector::calc_direction(&point1, &point2), Some(DiagonalVector::TopLeft));
    }

    #[test]
    fn test_calc_bottom_left_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(-1, -1);
        assert_eq!(DiagonalVector::calc_direction(&point1, &point2), Some(DiagonalVector::BottomLeft));
    }

    #[test]
    fn test_calc_bottom_right_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(1, -1);
        assert_eq!(DiagonalVector::calc_direction(&point1, &point2), Some(DiagonalVector::BottomRight));
    }

    #[test]
    fn test_calc_invalid_diagonal_direction() {
        assert_eq!(DiagonalVector::calc_direction(&Point::new(0, 0), &Point::new(0, 0)), None);
        assert_eq!(DiagonalVector::calc_direction(&Point::new(0, 0), &Point::new(0, 1)), None);
        assert_eq!(DiagonalVector::calc_direction(&Point::new(0, 0), &Point::new(2, 3)), None);
    }

    #[test]
    fn test_reverse_top_left_direction() {
        let direction = DiagonalVector::TopLeft;
        assert_eq!(direction.inverse(), DiagonalVector::BottomRight);
    }

    #[test]
    fn test_reverse_top_right_direction() {
        let direction = DiagonalVector::TopRight;
        assert_eq!(direction.inverse(), DiagonalVector::BottomLeft);
    }

    #[test]
    fn test_reverse_bottom_left_direction() {
        let direction = DiagonalVector::BottomLeft;
        assert_eq!(direction.inverse(), DiagonalVector::TopRight);
    }

    #[test]
    fn test_reverse_bottom_right_direction() {
        let direction = DiagonalVector::BottomRight;
        assert_eq!(direction.inverse(), DiagonalVector::TopLeft);
    }

    #[test]
    fn test_calc_next_top_right_point() {
        let direction = DiagonalVector::TopRight;
        let point = Point::new(1, 1);
        assert_eq!(direction.calc_next_point(&point), Point::new(2, 2));
    }

    #[test]
    fn test_calc_next_top_left_point() {
        let direction = DiagonalVector::TopLeft;
        let point = Point::new(1, 1);
        assert_eq!(direction.calc_next_point(&point), Point::new(0, 2));
    }

    #[test]
    fn test_calc_next_bottom_right_point() {
        let direction = DiagonalVector::BottomRight;
        let point = Point::new(1, 1);
        assert_eq!(direction.calc_next_point(&point), Point::new(2, 0));
    }

    #[test]
    fn test_calc_next_bottom_left_point() {
        let direction = DiagonalVector::BottomLeft;
        let point = Point::new(1, 1);
        assert_eq!(direction.calc_next_point(&point), Point::new(0, 0));
    }
}
