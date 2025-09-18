use crate::board::{Board, INVERT_COLORS};
use crate::color::Color;
use crate::pieces::{AttackPoints, PieceColor, PieceInit};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Queen {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    initial_position: Point,
}

#[derive(Debug)]
pub enum Buff {}

#[derive(Debug)]
pub enum Debuff {
    Captured,
}

impl PieceInit for Queen {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, initial_position: Point) -> Self {
        Self { color, buffs, debuffs, initial_position }
    }
}

impl PieceColor for Queen {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl PrettyPrint for Queen {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♛' } else { '♕' }.to_string(),
            Color::Black => if INVERT_COLORS { '♕' } else { '♛' }.to_string(),
        }
    }
}

impl AttackPoints for Queen {
    fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        let current_x = current_point.get_x().get_value();
        let current_y = current_point.get_y().get_value();

        let mut points: Vec<Point> = vec![];
        let add_point_and_or_stop = |x, y, points: &mut Vec<Point>| {
            let point = Point::new(x, y);
            if Self::is_attackable(&point, board, &self.color) {
                points.push(Point::new(x, y));
                if board.is_enemy_cell(&point, &self.color) {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        };

        // from current point to top right point
        {
            let mut x = current_x;
            let mut y = current_y;

            loop {
                x += 1;
                y += 1;
                if add_point_and_or_stop(x, y, &mut points) { break }
            }
        }

        // from current point to bottom left point
        {
            let mut x = current_x;
            let mut y = current_y;

            loop {
                x -= 1;
                y -= 1;
                if add_point_and_or_stop(x, y, &mut points) { break }
            }
        }

        // from current point to top left point
        {
            let mut x = current_x;
            let mut y = current_y;

            loop {
                x -= 1;
                y += 1;
                if add_point_and_or_stop(x, y, &mut points) { break }
            }
        }

        // from current point to bottom right point
        {
            let mut x = current_x;
            let mut y = current_y;

            loop {
                x += 1;
                y -= 1;
                if add_point_and_or_stop(x, y, &mut points) { break }
            }
        }

        // from current point to right
        {
            let mut x = current_x;

            loop {
                x += 1;
                if add_point_and_or_stop(x, current_y, &mut points) { break }
            }
        }

        // from current point to left
        {
            let mut x = current_x;

            loop {
                x -= 1;
                if add_point_and_or_stop(x, current_y, &mut points) { break }
            }
        }

        // from current point to top
        {
            let mut y = current_y;

            loop {
                y += 1;
                if add_point_and_or_stop(current_x, y, &mut points) { break }
            }
        }

        // from current point to bottom
        {
            let mut y = current_y;

            loop {
                y -= 1;
                if add_point_and_or_stop(current_x, y, &mut points) { break }
            }
        }

        points
    }
}

impl Queen {
    pub fn get_initial_position(&self) -> &Point {
        &self.initial_position
    }
}
