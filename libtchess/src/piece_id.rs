use crate::color::Color;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy, Clone)]
pub struct PieceId(isize);

impl PieceId {
    pub fn new(id: isize, color: &Color) -> Self {
        match color {
            Color::White => Self(id),
            Color::Black => Self(-id),
        }
    }

    pub fn color(&self) -> Color {
        if self.0 > 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn id(&self) -> &isize {
        &self.0
    }
}

impl Display for PieceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
