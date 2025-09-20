use crate::point::Point;

pub enum DiagonalDirection {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DiagonalDirection {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }
}

pub struct DiagonalVector {
    pub x: i16,
    pub y: i16,
    pub max_x: i16,
    pub max_y: i16,
}

impl DiagonalVector {
    pub fn calc_points<F, FF>(
        &self,
        direction: DiagonalDirection,
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
                DiagonalDirection::TopLeft => {
                    x -= 1;
                    y += 1;
                    if x < 0 || y >= self.max_y {
                        break;
                    }
                }
                DiagonalDirection::TopRight => {
                    x += 1;
                    y += 1;
                    if x >= self.max_x || y >= self.max_y {
                        break;
                    }
                }
                DiagonalDirection::BottomLeft => {
                    x -= 1;
                    y -= 1;
                    if x < 0 || y < 0 {
                        break;
                    }
                }
                DiagonalDirection::BottomRight => {
                    x += 1;
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
