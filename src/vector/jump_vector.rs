use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
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
        todo!()
    }

    pub fn reverse(&self) -> Self { todo!() }

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
}
