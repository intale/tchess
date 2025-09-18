use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::rc::Rc;
use nohash_hasher::NoHashHasher;
use crate::color::Color;
use crate::pieces::{Piece};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

type PieceHashSetT = HashSet<Rc<Piece>, BuildHasherDefault<NoHashHasher<Piece>>>;
type PointHashSetT = HashSet<Point, BuildHasherDefault<NoHashHasher<Point>>>;
type PointToPieceMapT = HashMap<
    Point,
    PieceHashSetT,
    BuildHasherDefault<NoHashHasher<Point>>
>;

type PieceToPointMapT = HashMap<
    Rc<Piece>,
    PointHashSetT,
    BuildHasherDefault<NoHashHasher<Piece>>
>;

#[derive(Debug)]
pub struct PointToPieceAssociation {
    point_to_pieces: PointToPieceMapT,
    piece_to_points: PieceToPointMapT,
    color: Color
}

impl PointToPieceAssociation {
    pub fn empty(color: Color) -> Self {
        let point_to_pieces: PointToPieceMapT = HashMap::with_hasher(BuildHasherDefault::default());
        let piece_to_points: PieceToPointMapT = HashMap::with_hasher(BuildHasherDefault::default());
        Self { color, point_to_pieces, piece_to_points }
    }

    pub fn get_pieces_mut(&mut self, point: &Point) -> &mut PieceHashSetT {
        if !self.point_to_pieces.contains_key(point) {
            self.point_to_pieces.insert(point.clone(), HashSet::with_hasher(BuildHasherDefault::default()));
        }
        self.point_to_pieces.get_mut(point).unwrap()
    }

    pub fn get_points_mut(&mut self, piece: &Rc<Piece>) -> &mut PointHashSetT {
        if !self.piece_to_points.contains_key(piece) {
            self.piece_to_points.insert(Rc::clone(piece), HashSet::with_hasher(BuildHasherDefault::default()));
        }
        self.piece_to_points.get_mut(piece).unwrap()
    }

    pub fn add_move(&mut self, point: Point, piece: &Rc<Piece>) -> bool {
        self.get_pieces_mut(&point).insert(Rc::clone(piece)) && self.get_points_mut(piece).insert(point)
    }

    pub fn pp_pieces(&self) -> String {
        self.point_to_pieces.pp()
    }
}

impl PrettyPrint for PointToPieceMapT {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut keys: Vec<&Point> = vec![];

        for key in self.keys() {
            keys.push(key);
        }
        // Sort points to later output them in sorted order
        keys.sort_by(|x, x1| x.pp().cmp(&x1.pp()));
        for point in keys {
            output.push_str(point.pp().as_str());
            output.push_str(": ");
            let mut pieces: Vec<&Rc<Piece>> = vec![];
            for piece in self.get(point).unwrap() {
                pieces.push(piece);
            }
            // Sort pieces to later output them in sorted order
            pieces.sort_by(|x, x1|
                x.get_initial_position().pp().cmp(&x1.get_initial_position().pp())
            );
            for piece in pieces {
                output.push_str(piece.pp().as_str());
                output.push_str(piece.get_initial_position().pp().as_str());
            }
            output.push_str("\n");
        }
        output
    }
}
