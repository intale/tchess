use std::rc::Rc;
use crate::board::INVERT_COLORS;
use crate::color::Color;
use crate::pieces::Piece;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug)]
pub struct Square {
    piece: Option<Rc<Piece>>,
    color: Color,
}

impl Square {
    pub fn new(color: Color, piece: Option<Rc<Piece>>) -> Self {
        Self { color, piece }
    }

    pub fn get_piece_rc(&self) -> Option<Rc<Piece>> {
        match &self.piece {
            Some(piece) => Some(Rc::clone(piece)),
            _ => None
        }
    }

    pub fn set_piece_rc(&mut self, piece: &Rc<Piece>) {
        self.piece = Some(Rc::clone(piece));
    }

    pub fn remove_piece(&mut self) -> Option<Rc<Piece>> {
        if let Some(piece) = self.piece.take() {
            self.piece = None;
            Some(piece)
        } else {
            None
        }
    }

    pub fn get_piece(&self) -> Option<&Rc<Piece>> {
        self.piece.as_ref()
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
        match &self.piece {
            Some(piece) => {
                output.push(base_sym);
                output.push_str(piece.pp().as_str());
                output.push(base_sym);
            },
            None => output.push_str(format!("{}{}{}", base_sym, base_sym, base_sym).as_str())
        }
        output
    }
}
