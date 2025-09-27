use crate::point::Point;

pub enum JumpDirection {
    TopLeftLeft,
    TopLeftRight,
    TopRightLeft,
    TopRightRight,
    BottomLeftLeft,
    BottomLeftRight,
    BottomRightLeft,
    BottomRightRight,
}

impl JumpDirection {
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
