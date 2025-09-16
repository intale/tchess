use crate::board::INVERT_COLORS;
use crate::color::Color;
use crate::pieces::{MovePiece, PieceColor, PieceInit};
use crate::pieces::knight::Knight;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Queen {
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

impl PieceInit for Queen {
    type Buff = Buff;
    type Debuff = Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>) -> Self {
        Self { color, buffs, debuffs }
    }
}

impl PieceColor for Queen {
    fn get_color(&self) -> Color {
        self.color
    }
}

impl MovePiece for Queen {
    fn move_piece(&self, x: u8) {
        todo!()
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
