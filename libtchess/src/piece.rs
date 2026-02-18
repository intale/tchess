pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use pawn::*;
use rook::*;
use knight::*;
use bishop::*;
use queen::*;
use king::*;
use crate::color::Color;
use crate::piece_id::PieceId;
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;

pub trait PieceInit: Sized {
    fn from_parts(color: Color, current_position: Point, id: PieceId) -> Self;

    fn new(
        color: Color,
        current_position: Point,
        id: PieceId,
    ) -> Self {
        Self::from_parts(
            color,
            current_position,
            id,
        )
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    UnknownPiece(PieceId),
}

impl Piece {
    pub fn init_piece_by_name(name: &str, color: Color, current_position: Point, id: PieceId) -> Self {
        match name {
            "Pawn" => Self::Pawn(Pawn::new(color, current_position, id)),
            "Rook" => Self::Rook(Rook::new(color, current_position, id)),
            "Knight" => Self::Knight(Knight::new(color, current_position, id)),
            "Bishop" => Self::Bishop(Bishop::new(color, current_position, id)),
            "Queen" => Self::Queen(Queen::new(color, current_position, id)),
            "King" => Self::King(King::new(color, current_position, id)),
            _ => panic!("Unknown piece: {name}")
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            Self::Pawn(_) => "Pawn",
            Self::Rook(_) => "Rook",
            Self::Knight(_) => "Knight",
            Self::Bishop(_) => "Bishop",
            Self::Queen(_) => "Queen",
            Self::King(_) => "King",
            Self::UnknownPiece(_) => panic!("Unknown piece can't be named properly!"),
        }
    }

    pub fn color(&self) -> &Color {
        match self {
            Self::Pawn(p) => p.color(),
            Self::Rook(p) => p.color(),
            Self::Knight(p) => p.color(),
            Self::Bishop(p) => p.color(),
            Self::Queen(p) => p.color(),
            Self::King(p) => p.color(),
            Self::UnknownPiece(_) => panic!("Unknown piece does not have a color!"),
        }
    }

    pub fn current_position(&self) -> &Point {
        match self {
            Self::Pawn(p) => p.current_position(),
            Self::Rook(p) => p.current_position(),
            Self::Knight(p) => p.current_position(),
            Self::Bishop(p) => p.current_position(),
            Self::Queen(p) => p.current_position(),
            Self::King(p) => p.current_position(),
            Self::UnknownPiece(_) => panic!("Unknown piece does not have a position!"),
        }
    }

    pub fn set_current_position(&mut self, point: Point) {
        match self {
            Self::Pawn(p) => p.set_current_position(point),
            Self::Rook(p) => p.set_current_position(point),
            Self::Knight(p) => p.set_current_position(point),
            Self::Bishop(p) => p.set_current_position(point),
            Self::Queen(p) => p.set_current_position(point),
            Self::King(p) => p.set_current_position(point),
            Self::UnknownPiece(_) => panic!("Can't set a position of an unknown piece!"),
        }
    }

    pub fn id(&self) -> &PieceId {
        match self {
            Self::Pawn(p) => p.id(),
            Self::Rook(p) => p.id(),
            Self::Knight(p) => p.id(),
            Self::Bishop(p) => p.id(),
            Self::Queen(p) => p.id(),
            Self::King(p) => p.id(),
            Self::UnknownPiece(id) => id,
        }
    }


    pub fn is_ally(&self, color: &Color) -> bool {
        self.color() == color
    }

    pub fn is_enemy(&self, color: &Color) -> bool {
        !self.is_ally(color)
    }

    pub fn attack_vector(&self, point1: &Point, point2: &Point) -> Option<Vector> {
        match self {
            Self::Pawn(p) => p.attack_vector(point1, point2),
            Self::Rook(p) => p.attack_vector(point1, point2),
            Self::Knight(p) => p.attack_vector(point1, point2),
            Self::Bishop(p) => p.attack_vector(point1, point2),
            Self::Queen(p) => p.attack_vector(point1, point2),
            Self::King(p) => p.attack_vector(point1, point2),
            Self::UnknownPiece(_) => panic!("Can't calculate attack vector for an unknown piece!"),
        }
    }
}

impl PrettyPrint for Piece {
    fn pp(&self) -> String {
        match self {
            Self::Pawn(p) => p.pp(),
            Self::Rook(p) => p.pp(),
            Self::Knight(p) => p.pp(),
            Self::Bishop(p) => p.pp(),
            Self::Queen(p) => p.pp(),
            Self::King(p) => p.pp(),
            Self::UnknownPiece(_) => panic!("Don't know how to display an unknown piece!"),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::Rook(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::Knight(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::Bishop(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::Queen(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::King(p) => write!(f, "{}{}", p.pp(), p.current_position()),
            Self::UnknownPiece(piece_id) => write!(f, "U#{}", piece_id.id()),
        }
    }
}

impl Hash for Piece {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Self::Pawn(p) => { p.id().hash(hasher) },
            Self::Rook(p) => p.id().hash(hasher),
            Self::Knight(p) => p.id().hash(hasher),
            Self::Bishop(p) => p.id().hash(hasher),
            Self::Queen(p) => p.id().hash(hasher),
            Self::King(p) => p.id().hash(hasher),
            Self::UnknownPiece(id) => id.hash(hasher),
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Piece {}
