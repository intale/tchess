use crate::board::INVERT_COLORS;
use crate::color::Color;
use crate::piece_id::PieceId;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, Clone)]
pub struct Square {
    piece_id: Option<PieceId>,
    color: Color,
}

impl Square {
    pub fn new(color: Color, piece_id: Option<PieceId>) -> Self {
        Self { color, piece_id }
    }

    pub fn get_piece_id(&self) -> Option<&PieceId> {
        self.piece_id.as_ref()
    }

    pub fn set_piece_id(&mut self, piece_id: &PieceId) {
        self.piece_id = Some(*piece_id);
    }

    pub fn remove_piece_id(&mut self) -> Option<PieceId> {
        if let Some(piece_id) = self.piece_id.take() {
            Some(piece_id)
        } else {
            None
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl PrettyPrint for Square {
    fn pp(&self) -> String {
        let mut output = String::new();
        let base_sym = match self.color {
            Color::White => if INVERT_COLORS { '▓' } else { '░' },
            Color::Black => if INVERT_COLORS { '░' } else { '▓' },
        };
        match &self.piece_id {
            Some(piece_id) => {
                output.push(base_sym);
                output.push_str(format!("{}", piece_id.id()).as_str());
                output.push(base_sym);
            },
            None => output.push_str(format!("{}{}{}", base_sym, base_sym, base_sym).as_str())
        }
        output
    }
}
