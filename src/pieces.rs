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

    fn from_parts(color: Color, buffs: Vec<Self::Buff>, debuffs: Vec<Self::Debuff>, 
                  current_position: Point, initial_position: Point) -> Self;

    fn new(
        color: Color,
        buffs: Option<Vec<Self::Buff>>,
        debuffs: Option<Vec<Self::Debuff>>,
        current_position: Point,
        initial_position: Point,
    ) -> Self {
        Self::from_parts(
            color,
            buffs.unwrap_or_default(),
            debuffs.unwrap_or_default(),
            current_position,
            initial_position,
        )
    }

    fn empty(color: Color, current_position: Point, initial_position: Point) -> Self {
        Self::from_parts(color, Vec::new(), Vec::new(), current_position, initial_position)
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
    fn get_current_position(&self) -> &Point;
    fn get_initial_position(&self) -> &Point;
}

// trait XRayPoints: Positioning + PieceColor where Piece: PartialEq<Self> {
//     fn x_ray_points(&self, board: &Board) -> Vec<Point> {
//         let color = self.get_color();
// 
//         let mut points: Vec<Point> = vec![];
// 
//         let covering_ally: &Rc<Piece>;
//         let attacking_ = false;
// 
//         let validator = |_point: &Point| {
//             true
//         };
//         let terminator = |_point: &Point| {
//             false
//         };
// 
//         let diagonal_vector = DiagonalVector { x, y, max_x, max_y };
//         let line_vector = LineVector { x, y, max_x, max_y };
// 
//         let bounds: HashMap<Rc<Piece>, Direction>;
//         let opposite_pieces = board.get_attacked_points(self.get_color()).get_x_ray_pieces();
//         for piece in opposite_pieces {
//             let current_piece_position = piece.get_current_position();
//             match &**piece {
//                 Piece::Bishop(p) => {
//                     let direction = DiagonalDirection::calc_direction(
//                         p.get_current_position(), current_piece_position
//                     );
//                     let (bishop_x, bishop_y) = p.get_current_position().to_tuple();
//                     if let Some(direction) = direction {
//                         let diagonal_vector = DiagonalVector { x: bishop_x, y: bishop_x, max_x, max_y };
//                         let mut current_piece_on_the_way: Option<&Rc<Piece>> = None;
//                         let validator = |_point: &Point| {
//                             false
//                         };
//                         let terminator = |point: &Point| {
//                             if let Some(piece) = board.get_cell(point).get_piece() {
//                                 if piece.get_color() != &color {
//                                     return false;
//                                 }
//                                 match current_piece_on_the_way {
//                                     Some(prev_piece_on_the_way) => {
//                                         if(&**piece == self) {
// 
//                                         }
//                                     },
//                                     None => { current_piece_on_the_way = Some(piece) }
//                                 }
//                             }
// 
//                             false
//                         };
//                         diagonal_vector.calc_points(direction, validator, terminator);
//                     }
// 
//                 },
//                 _ => ()
//             }
//         }
// 
//         for direction in DiagonalDirection::all_variants() {
//             points.append(&mut diagonal_vector.calc_points(direction, validator, terminator));
//         }
//         for direction in LineDirection::all_variants() {
//             points.append(&mut line_vector.calc_points(direction, validator, terminator));
//         }
// 
//         points;
// 
//         todo!()
//     }
// }

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

    pub fn get_current_position(&self) -> &Point {
        match self {
            Piece::Pawn(p) => p.get_current_position(),
            Piece::Rook(p) => p.get_current_position(),
            Piece::Knight(p) => p.get_current_position(),
            Piece::Bishop(p) => p.get_current_position(),
            Piece::Queen(p) => p.get_current_position(),
            Piece::King(p) => p.get_current_position(),
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
