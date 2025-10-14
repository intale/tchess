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
use crate::buff::{Buff, BuffsCollection};
use crate::color::Color;
use crate::debuff::{Debuff, DebuffsCollection};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

trait MovePiece {
    fn move_piece(&self, x: u8);
}

pub trait PieceInit: Sized {
    fn from_parts(color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                  current_position: Point, id: usize) -> Self;

    fn new(
        color: Color,
        buffs: Vec<Buff>,
        debuffs: Vec<Debuff>,
        current_position: Point,
        id: usize,
    ) -> Self {
        Self::from_parts(
            color,
            buffs,
            debuffs,
            current_position,
            id,
        )
    }

    fn empty(color: Color, current_position: Point, id: usize) -> Self {
        Self::from_parts(color, Vec::new(), Vec::new(), current_position, id)
    }
}

pub trait PieceColor {
    fn get_color(&self) -> Color;
}

trait AttackPoints {
    fn attack_points(&self, board: &Board) -> Vec<Point>;
}

trait DefensivePoints {
    fn defensive_points(&self, board: &Board) -> Vec<Point>;
}

trait Positioning {
    fn get_current_position(&self) -> Point;
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
    pub fn init_piece_by_name(name: &str, color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                              current_position: Point, id: usize) -> Self {
        match name {
            "Pawn" => Self::Pawn(Pawn::new(color, buffs, debuffs, current_position, id)),
            "Rook" => Self::Rook(Rook::new(color, buffs, debuffs, current_position, id)),
            "Knight" => Self::Knight(Knight::new(color, buffs, debuffs, current_position, id)),
            "Bishop" => Self::Bishop(Bishop::new(color, buffs, debuffs, current_position, id)),
            "Queen" => Self::Queen(Queen::new(color, buffs, debuffs, current_position, id)),
            "King" => Self::King(King::new(color, buffs, debuffs, current_position, id)),
            _ => panic!("Unknown piece: {name}")
        }
    }

    pub fn attack_points(&self, board: &Board) -> Vec<Point> {
        match self {
            Piece::Pawn(p) => p.attack_points(board),
            Piece::Rook(p) => p.attack_points(board),
            Piece::Knight(p) => p.attack_points(board),
            Piece::Bishop(p) => p.attack_points(board),
            Piece::Queen(p) => p.attack_points(board),
            Piece::King(p) => p.attack_points(board),
        }
    }

    pub fn defensive_points(&self, board: &Board) -> Vec<Point> {
        match self {
            Piece::Pawn(p) => p.defensive_points(board),
            Piece::Rook(p) => p.defensive_points(board),
            Piece::Knight(p) => p.defensive_points(board),
            Piece::Bishop(p) => p.defensive_points(board),
            Piece::Queen(p) => p.defensive_points(board),
            Piece::King(p) => p.defensive_points(board),
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

    pub fn get_current_position(&self) -> Point {
        match self {
            Piece::Pawn(p) => p.get_current_position(),
            Piece::Rook(p) => p.get_current_position(),
            Piece::Knight(p) => p.get_current_position(),
            Piece::Bishop(p) => p.get_current_position(),
            Piece::Queen(p) => p.get_current_position(),
            Piece::King(p) => p.get_current_position(),
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Piece::Pawn(p) => p.id(),
            Piece::Rook(p) => p.id(),
            Piece::Knight(p) => p.id(),
            Piece::Bishop(p) => p.id(),
            Piece::Queen(p) => p.id(),
            Piece::King(p) => p.id(),
        }
    }

    pub fn buffs(&self) -> &BuffsCollection {
        match self {
            Piece::Pawn(p) => p.buffs(),
            Piece::Rook(p) => p.buffs(),
            Piece::Knight(p) => p.buffs(),
            Piece::Bishop(p) => p.buffs(),
            Piece::Queen(p) => p.buffs(),
            Piece::King(p) => p.buffs(),
        }
    }

    pub fn debuffs(&self) -> &DebuffsCollection {
        match self {
            Piece::Pawn(p) => p.debuffs(),
            Piece::Rook(p) => p.debuffs(),
            Piece::Knight(p) => p.debuffs(),
            Piece::Bishop(p) => p.debuffs(),
            Piece::Queen(p) => p.debuffs(),
            Piece::King(p) => p.debuffs(),
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

impl Hash for Piece {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Piece::Pawn(p) => p.id().hash(hasher),
            Piece::Rook(p) => p.id().hash(hasher),
            Piece::Knight(p) => p.id().hash(hasher),
            Piece::Bishop(p) => p.id().hash(hasher),
            Piece::Queen(p) => p.id().hash(hasher),
            Piece::King(p) => p.id().hash(hasher),
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Piece::Pawn(p) => {
                match other {
                    Piece::Pawn(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
            Piece::Rook(p) => {
                match other {
                    Piece::Rook(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
            Piece::Knight(p) => {
                match other {
                    Piece::Knight(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
            Piece::Bishop(p) => {
                match other {
                    Piece::Bishop(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
            Piece::Queen(p) => {
                match other {
                    Piece::Queen(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
            Piece::King(p) => {
                match other {
                    Piece::King(other_p) => other_p.id() == p.id(),
                    _ => false,
                }
            }
        }
    }
}

impl Eq for Piece {}

impl nohash_hasher::IsEnabled for Piece {}
