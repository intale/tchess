use crate::point::Point;

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

    pub fn calc_direction(point1: &Point, point2: &Point) -> Option<Self> {
        todo!()
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
