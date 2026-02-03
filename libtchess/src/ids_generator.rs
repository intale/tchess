use crate::color::Color;
use crate::piece_id::PieceId;

#[derive(Clone)]
pub struct IdsGenerator {
    current_val: isize,
}

impl IdsGenerator {
    pub fn init() -> Self {
        Self { current_val: 0 }
    }

    pub fn next_val(&mut self, color: &Color) -> PieceId {
        self.current_val += 1;

        PieceId::new(self.current_val, color)
    }
}

