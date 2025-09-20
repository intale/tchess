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
}

pub struct JumpVector {
    pub x: i16,
    pub y: i16,
    pub max_x: i16,
    pub max_y: i16,
}

impl JumpVector {
    pub fn calc_points<F, FF>(
        &self,
        direction: JumpDirection,
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
                JumpDirection::TopLeftLeft => {
                    x -= 2;
                    y += 1;
                    if x < 0 || y >= self.max_y {
                        break;
                    }
                }
                JumpDirection::TopLeftRight => {
                    x -= 1;
                    y += 2;
                    if x < 0 || y >= self.max_y {
                        break;
                    }
                }
                JumpDirection::TopRightLeft => {
                    x += 1;
                    y += 2;
                    if x >= self.max_x || y >= self.max_y {
                        break;
                    }
                }
                JumpDirection::TopRightRight => {
                    x += 2;
                    y += 1;
                    if x >= self.max_x || y >= self.max_y {
                        break;
                    }
                }
                JumpDirection::BottomLeftLeft => {
                    x -= 2;
                    y -= 1;
                    if x < 0 || y < 0 {
                        break;
                    }
                }
                JumpDirection::BottomLeftRight => {
                    x -= 1;
                    y -= 2;
                    if x < 0 || y < 0 {
                        break;
                    }
                }
                JumpDirection::BottomRightLeft => {
                    x += 1;
                    y -= 2;
                    if x >= self.max_x || y < 0 {
                        break;
                    }
                }
                JumpDirection::BottomRightRight => {
                    x += 2;
                    y -= 1;
                    if x >= self.max_x || y < 0 {
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
