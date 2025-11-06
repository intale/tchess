use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum LineVector {
    Top,
    Bottom,
    Left,
    Right,
}

impl LineVector {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Top,
            Self::Bottom,
            Self::Left,
            Self::Right,
        ]
    }

    #[allow(non_contiguous_range_endpoints)]
    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        let (x1, y1) = point1.to_tuple();
        let (x2, y2) = point2.to_tuple();

        match (x1 - x2, y1 - y2) {
            (0, i16::MIN..0) => Some(Self::Top),
            (0, 1..=i16::MAX) => Some(Self::Bottom),
            (1..=i16::MAX, 0) => Some(Self::Left),
            (i16::MIN..0, 0) => Some(Self::Right),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn calc_next_point(&self, current_point: &Point) -> Point {
        let (&x, &y) = current_point.to_tuple();
        let (mut x, mut y) = (x, y);
        match self {
            Self::Top => {
                y += 1;
                
            }
            Self::Bottom => {
                y -= 1;
                
            }
            Self::Left => {
                x -= 1;
                
            }
            Self::Right => {
                x += 1;
            }
        }
        Point::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_top_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(0, 1);
        assert_eq!(LineVector::calc_direction(&point1, &point2), Some(LineVector::Top));
    }

    #[test]
    fn test_calc_bottom_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(0, -1);
        assert_eq!(LineVector::calc_direction(&point1, &point2), Some(LineVector::Bottom));
    }

    #[test]
    fn test_calc_left_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(-1, 0);
        assert_eq!(LineVector::calc_direction(&point1, &point2), Some(LineVector::Left));
    }

    #[test]
    fn test_calc_right_direction() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(1, 0);
        assert_eq!(LineVector::calc_direction(&point1, &point2), Some(LineVector::Right));
    }

    #[test]
    fn test_calc_invalid_line_direction() {
        assert_eq!(LineVector::calc_direction(&Point::new(0, 0), &Point::new(0, 0)), None);
        assert_eq!(LineVector::calc_direction(&Point::new(0, 0), &Point::new(1, 1)), None);
        assert_eq!(LineVector::calc_direction(&Point::new(0, 0), &Point::new(2, 3)), None);
    }

    #[test]
    fn test_reverse_top_direction() {
        let direction = LineVector::Top;
        assert_eq!(direction.inverse(), LineVector::Bottom);
    }

    #[test]
    fn test_reverse_bottom_direction() {
        let direction = LineVector::Bottom;
        assert_eq!(direction.inverse(), LineVector::Top);
    }

    #[test]
    fn test_reverse_left_direction() {
        let direction = LineVector::Left;
        assert_eq!(direction.inverse(), LineVector::Right);
    }

    #[test]
    fn test_reverse_right_direction() {
        let direction = LineVector::Right;
        assert_eq!(direction.inverse(), LineVector::Left);
    }

    #[test]
    fn test_calc_next_top_point() {
        let direction = LineVector::Top;
        let point = Point::new(0, 0);
        assert_eq!(direction.calc_next_point(&point), Point::new(0, 1));
    }

    #[test]
    fn test_calc_next_bottom_point() {
        let direction = LineVector::Bottom;
        let point = Point::new(0, 0);
        assert_eq!(direction.calc_next_point(&point), Point::new(0, -1));
    }

    #[test]
    fn test_calc_next_left_point() {
        let direction = LineVector::Left;
        let point = Point::new(0, 0);
        assert_eq!(direction.calc_next_point(&point), Point::new(-1, 0));
    }

    #[test]
    fn test_calc_next_right_point() {
        let direction = LineVector::Right;
        let point = Point::new(0, 0);
        assert_eq!(direction.calc_next_point(&point), Point::new(1, 0));
    }
}
