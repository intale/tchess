use crate::board::INVERT_COLORS;
use crate::color::Color;
use crate::pieces::{MovePiece, PieceColor, PieceInit};
use crate::pieces::king::King;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Knight {
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
}

#[derive(Debug)]
pub enum Buff {}

#[derive(Debug)]
pub enum Debuff {
    Captured,
}

impl PieceInit for Knight {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>) -> Self {
        Self { color, buffs, debuffs }
    }
}

impl PieceColor for Knight {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl MovePiece for Knight {
    fn move_piece(&self, x: u8) {
        todo!()
    }
}

impl PrettyPrint for Knight {
    fn pp(&self) -> String {
        match self.color {
            Color::White => if INVERT_COLORS { '♞' } else { '♘' }.to_string(),
            Color::Black => if INVERT_COLORS { '♘' } else { '♞' }.to_string(),
        }
    }
}
