use crate::point::Point;

pub enum LineDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl LineDirection {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Top,
            Self::Bottom,
            Self::Left,
            Self::Right,
        ]
    }
}

pub struct LineVector {
    pub x: i16,
    pub y: i16,
    pub max_x: i16,
    pub max_y: i16,
}

impl LineVector {
    pub fn calc_points<F, FF>(
        &self,
        direction: LineDirection,
        validator: F,
        terminator: FF,
    ) -> Vec<Point>
    where
        F: Fn(&Point) -> bool,
        FF: Fn(&Point) -> bool,
    {
        let mut points: Vec<Point> = vec![];
        loop {
            let mut x = self.x;
            let mut y = self.y;

            match direction {
                LineDirection::Top => {
                    y += 1;
                    if y >= self.max_y {
                        break;
                    }
                }
                LineDirection::Bottom => {
                    y -= 1;
                    if y < 0 {
                        break;
                    }
                }
                LineDirection::Left => {
                    x -= 1;
                    if x < 0 {
                        break;
                    }
                }
                LineDirection::Right => {
                    x += 1;
                    if x >= self.max_x {
                        break;
                    }
                }
            }

            let point = Point::new(x, y);

            if (validator(&point)) {
                points.push(point);
            }

            if terminator(&point) {
                break;
            }
        }
        points
    }
}
