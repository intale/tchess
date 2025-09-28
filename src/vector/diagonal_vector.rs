use crate::point::Point;

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
            (i16::MIN..0, 1..=i16::MAX) => Some(Self::TopLeft),
            (1..=i16::MAX, 1..=i16::MAX) => Some(Self::TopRight),
            (i16::MIN..0, i16::MIN..0) => Some(Self::BottomLeft),
            (1..=i16::MAX, i16::MIN..0) => Some(Self::BottomRight),
            (_, 0) | (0, _) => None,
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
