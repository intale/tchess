use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DiagonalVector {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DiagonalVector {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }
}

impl DiagonalVector {
    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        let (x1, y1) = point1.to_tuple();
        let (x2, y2) = point2.to_tuple();

        if (x1 - x2).abs() != (y1 - y2).abs() {
            return None;
        }

        match (x1 - x2, y1 - y2) {
            (i16::MIN..0, 1..=i16::MAX) => Some(Self::BottomRight),
            (1..=i16::MAX, 1..=i16::MAX) => Some(Self::BottomLeft),
            (i16::MIN..0, i16::MIN..0) => Some(Self::TopRight),
            (1..=i16::MAX, i16::MIN..0) => Some(Self::TopLeft),
            (_, 0) | (0, _) => None,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::TopLeft => Self::BottomRight,
            Self::TopRight => Self::BottomLeft,
            Self::BottomLeft => Self::TopRight,
            Self::BottomRight => Self::TopLeft,
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
        assert_eq!(direction.reverse(), DiagonalVector::BottomRight);
    }

    #[test]
    fn test_reverse_top_right_direction() {
        let direction = DiagonalVector::TopRight;
        assert_eq!(direction.reverse(), DiagonalVector::BottomLeft);
    }

    #[test]
    fn test_reverse_bottom_left_direction() {
        let direction = DiagonalVector::BottomLeft;
        assert_eq!(direction.reverse(), DiagonalVector::TopRight);
    }

    #[test]
    fn test_reverse_bottom_right_direction() {
        let direction = DiagonalVector::BottomRight;
        assert_eq!(direction.reverse(), DiagonalVector::TopLeft);
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
