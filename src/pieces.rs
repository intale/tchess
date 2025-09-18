pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

use std::hash::{Hash, Hasher};
use pawn::*;
use rook::*;
use knight::*;
use bishop::*;
use queen::*;
use king::*;
use crate::board::{Board};
use crate::color::Color;
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

trait MovePiece {
    fn move_piece(&self, x: u8);
}

pub trait PieceInit: Sized {
    type Buff;
    type Debuff;

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, initial_position: Point) -> Self;

    fn new(
        color: Color,
        buffs: Option<Vec<Self::Buff>>,
        debuffs: Option<Vec<Self::Debuff>>,
        initial_position: Point,
    ) -> Self {
        Self::from_parts(
            color,
            buffs.unwrap_or_default(),
            debuffs.unwrap_or_default(),
            initial_position
        )
    }

    fn empty(color: Color, initial_position: Point) -> Self {
        Self::from_parts(color, Vec::new(), Vec::new(), initial_position)
    }
}

pub trait PieceColor {
    fn get_color(&self) -> Color;
}

trait AttackPoints {
    fn attack_points(&self, board: &Board, current_point: &Point) -> Vec<Point>;
    fn is_attackable(point: &Point, board: &Board, color: &Color) -> bool {
        board.is_in_boundaries(&point)
          && (board.is_empty_cell(&point) || board.is_enemy_cell(&point, &color))
    }
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
            Piece::Rook(p) => p.attack_points(board, current_point),
            Piece::Knight(p) => p.attack_points(board, current_point),
            Piece::Bishop(p) => p.attack_points(board, current_point),
            Piece::Queen(p) => p.attack_points(board, current_point),
            Piece::King(p) => p.attack_points(board, current_point),
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

    pub fn get_initial_position(&self) -> &Point {
        match self {
            Piece::Pawn(p) => p.get_initial_position(),
            Piece::Rook(p) => p.get_initial_position(),
            Piece::Knight(p) => p.get_initial_position(),
            Piece::Bishop(p) => p.get_initial_position(),
            Piece::Queen(p) => p.get_initial_position(),
            Piece::King(p) => p.get_initial_position(),
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

impl std::hash::Hash for Piece {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Piece::Pawn(p) => p.get_initial_position().hash(hasher),
            Piece::Rook(p) => p.get_initial_position().hash(hasher),
            Piece::Knight(p) => p.get_initial_position().hash(hasher),
            Piece::Bishop(p) => p.get_initial_position().hash(hasher),
            Piece::Queen(p) => p.get_initial_position().hash(hasher),
            Piece::King(p) => p.get_initial_position().hash(hasher),
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Piece::Pawn(p) => {
                match other {
                    Piece::Pawn(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
            Piece::Rook(p) => {
                match other {
                    Piece::Rook(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
            Piece::Knight(p) => {
                match other {
                    Piece::Knight(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
            Piece::Bishop(p) => {
                match other {
                    Piece::Bishop(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
            Piece::Queen(p) => {
                match other {
                    Piece::Queen(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
            Piece::King(p) => {
                match other {
                    Piece::King(other_p) => other_p.get_initial_position() == p.get_initial_position(),
                    _ => false,
                }
            }
        }
    }
}

impl Eq for Piece {}

impl nohash_hasher::IsEnabled for Piece {}
