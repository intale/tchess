use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_move::PieceMove;
use crate::piece::Piece;
use crate::point::Point;

pub type MovesSetT = FxHashSet<PieceMove>;
pub type PieceToMovesMapT = FxHashMap<Rc<Piece>, MovesSetT>;
pub type PointToPiecesMapT = FxHashMap<Point, PieceToMovesMapT>;

pub struct MovesMap {
    piece_to_moves: PieceToMovesMapT,
    // This is Point-to-Pieces-to-Moves structure that allows to fetch all moves for the certain
    // piece in the given point. It is useful because there can be several moves that ends on the
    // same point, but have different meaning. E.g. a promotion of a pawn ends up on the same point,
    // but with different promotion piece
    point_to_pieces: PointToPiecesMapT,
}

impl MovesMap {
    pub fn empty() -> Self {
        let piece_to_moves = FxHashMap::default();
        let point_to_pieces = FxHashMap::default();
        Self { piece_to_moves, point_to_pieces }
    }

    fn moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.piece_to_moves.contains_key(piece) {
            self.piece_to_moves.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_moves.get_mut(piece).unwrap()
    }

    fn pieces_mut(&mut self, point: Point, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.point_to_pieces.contains_key(&point) {
            self.point_to_pieces.insert(point, FxHashMap::default());
        }
        let pieces_hashmap = self.point_to_pieces.get_mut(&point).unwrap();
        if !pieces_hashmap.contains_key(piece) {
            pieces_hashmap.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.point_to_pieces.get_mut(&point).unwrap().get_mut(piece).unwrap()
    }

    pub fn moves_of(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.piece_to_moves.get(piece)
    }

    pub fn pieces_to_move_onto(&self, point: &Point) -> Option<&PieceToMovesMapT> {
        self.point_to_pieces.get(point)
    }

    pub fn add(&mut self, piece: &Rc<Piece>, piece_move: PieceMove) -> bool {
        match piece_move.destination() {
            Some(point) => {
                self.moves_mut(piece).insert(piece_move)
                    && self.pieces_mut(point, piece).insert(piece_move)
            },
            None => self.moves_mut(piece).insert(piece_move),
        }
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) -> Option<MovesSetT> {
        let moves = self.piece_to_moves.remove(piece);
        if let Some(moves) = moves {
            for piece_move in moves.iter() {
                if let Some(point) = piece_move.destination() {
                    if let Some(pieces) = self.point_to_pieces.get_mut(&point) {
                        pieces.remove(piece);
                        if pieces.is_empty() {
                            self.point_to_pieces.remove(&point);
                        }
                    }
                }
            }
            return Some(moves);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.piece_to_moves.is_empty()
    }
}
