pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

use pawn::*;
use rook::*;
use knight::*;
use bishop::*;
use queen::*;
use king::*;
use crate::board::{Board, BoardDimension};
use crate::cell::Cell;
use crate::color::Color;
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

trait MovePiece {
    fn move_piece(&self, x: u8);
}

pub trait PieceInit: Sized {
    type Buff;
    type Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>) -> Self;

    fn new(
        color: Color,
        buffs: Option<Vec<Self::Buff>>,
        debuffs: Option<Vec<Self::Debuff>>,
    ) -> Self {
        Self::from_parts(
            color,
            buffs.unwrap_or_default(),
            debuffs.unwrap_or_default(),
        )
    }

    fn with(
        color: Color,
        buffs: impl IntoIterator<Item = Self::Buff>,
        debuffs: impl IntoIterator<Item = Self::Debuff>,
    ) -> Self {
        Self::from_parts(color, buffs.into_iter().collect(), debuffs.into_iter().collect())
    }

    fn empty(color: Color) -> Self {
        Self::from_parts(color, Vec::new(), Vec::new())
    }
}

pub trait PieceColor {
    fn get_color(&self) -> Color;
}

trait PieceConstraints {
    fn constraints(&self, current_position: Point, board_dimension: BoardDimension) -> Vec<Point>;
}

trait AttackedCell {
    fn attack_cell(&self, current_cell: &Cell, cell: &Cell);
}


#[derive(Debug)]
pub enum Piece {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl Piece {
    pub fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point> {
        match self {
            Piece::Pawn(p) => p.attack_points(board, current_point),
            Piece::Rook(p) => vec![],
            Piece::Knight(p) => vec![],
            Piece::Bishop(p) => vec![],
            Piece::Queen(p) => vec![],
            Piece::King(p) => vec![],
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(p) => p.get_color(),
            Piece::Rook(p) => p.get_color(),
            Piece::Knight(p) => p.get_color(),
            Piece::Bishop(p) => p.get_color(),
            Piece::Queen(p) => p.get_color(),
            Piece::King(p) => p.get_color(),
        }
    }
}

impl PrettyPrint for Piece {
    fn pp(&self) -> String {
        match self { 
            Piece::Pawn(p) => p.pp(),
            Piece::Rook(p) => p.pp(),
            Piece::Knight(p) => p.pp(),
            Piece::Bishop(p) => p.pp(),
            Piece::Queen(p) => p.pp(),
            Piece::King(p) => p.pp(),
        }
    }
}
