use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece::{Piece};
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

type PieceHashSetT = FxHashSet<Rc<Piece>>;
type PointHashSetT = FxHashSet<Point>;
type PointToPieceMapT = FxHashMap<
    Point,
    PieceHashSetT
>;

type PieceToPointMapT = FxHashMap<
    Rc<Piece>,
    PointHashSetT
>;

#[derive(Debug)]
pub struct PointToPieceAssociation {
    point_to_pieces: PointToPieceMapT,
    piece_to_points: PieceToPointMapT,
}

impl PointToPieceAssociation {
    pub fn empty() -> Self {
        let point_to_pieces: PointToPieceMapT = FxHashMap::default();
        let piece_to_points: PieceToPointMapT = FxHashMap::default();
        Self { point_to_pieces, piece_to_points }
    }

    fn get_pieces_mut(&mut self, point: &Point) -> &mut PieceHashSetT {
        if !self.point_to_pieces.contains_key(point) {
            self.point_to_pieces.insert(*point, FxHashSet::default());
        }
        self.point_to_pieces.get_mut(point).unwrap()
    }

    fn get_points_mut(&mut self, piece: &Rc<Piece>) -> &mut PointHashSetT {
        if !self.piece_to_points.contains_key(piece) {
            self.piece_to_points.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_points.get_mut(piece).unwrap()
    }

    pub fn get_all_pieces(&self) -> Vec<&Rc<Piece>> {
        self.piece_to_points.keys().collect()
    }

    pub fn has_pieces(&self, point: &Point)  -> bool {
        if let Some(pieces) = self.get_pieces(point) {
            !pieces.is_empty()
        } else {
            false
        }
    }

    pub fn get_points(&self, piece: &Rc<Piece>) -> Option<&PointHashSetT> {
        self.piece_to_points.get(piece)
    }

    pub fn get_pieces(&self, point: &Point) -> Option<&PieceHashSetT> {
        self.point_to_pieces.get(point)
    }

    pub fn add_association(&mut self, point: Point, piece: &Rc<Piece>) -> bool {
        self.get_pieces_mut(&point).insert(Rc::clone(piece)) && self.get_points_mut(piece).insert(point)
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) {
        let points = self.piece_to_points.remove(piece);
        if let Some(points) = points {
            for point in points.iter() {
                if let Some(pieces) = self.point_to_pieces.get_mut(point) {
                    pieces.remove(piece);
                    if pieces.is_empty() {
                        self.point_to_pieces.remove(point);
                    }
                }
            }
        }
    }

    pub fn pp_pieces(&self) -> String {
        self.point_to_pieces.pp()
    }

    pub fn pp_points(&self) -> String {
        self.piece_to_points.pp()
    }
}

impl PrettyPrint for PointToPieceMapT {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut keys: Vec<&Point> = vec![];

        for key in self.keys() {
            if self.get(key).unwrap().len() > 0 {
                keys.push(key);
            }
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
                x.id().cmp(&x1.id())
            );
            for piece in pieces {
                output.push_str(piece.pp().as_str());
                output.push_str(piece.current_position().pp().as_str());
            }
            output.push_str("\n");
        }
        output
    }
}

impl PrettyPrint for PieceToPointMapT {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut keys: Vec<&Rc<Piece>> = vec![];

        for key in self.keys() {
            keys.push(key);
        }
        // Sort points to later output them in sorted order
        keys.sort_by(|x, x1| x.id().cmp(&x1.id()));

        for piece in keys {
            output.push_str(piece.pp().as_str());
            output.push_str(piece.current_position().pp().as_str());
            output.push_str(": ");
            let mut points: Vec<&Point> = vec![];
            for point in self.get(piece).unwrap() {
                points.push(point);
            }
            // Sort pieces to later output them in sorted order
            points.sort_by(|x, x1|
                x.pp().cmp(&x1.pp())
            );
            for point in points {
                output.push_str(point.pp().as_str());
            }
            output.push_str("\n");
        }
        output
    }
}
