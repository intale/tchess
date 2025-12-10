use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_move::PieceMove;
use crate::pieces::Piece;
use crate::point::Point;

type MovesSetT = FxHashSet<PieceMove>;
type PiecesSetT = FxHashMap<Rc<Piece>, MovesSetT>;
type PieceToMovesMapT = FxHashMap<Rc<Piece>, MovesSetT>;
type PointToPiecesMapT = FxHashMap<Point, PiecesSetT>;

struct MoveConstraints {
    constraints: PieceToMovesMapT,
    has_constraints: bool,
}

impl MoveConstraints {
    pub fn empty() -> Self {
        let constraints = FxHashMap::default();
        Self { constraints, has_constraints: false }
    }

    pub fn is_enabled(&self) -> bool {
        self.has_constraints
    }

    pub fn enable(&mut self) {
        self.has_constraints = true;
    }

    pub fn clear(&mut self) {
        self.has_constraints = false;
        self.constraints.clear();
    }

    pub fn get_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.constraints.contains_key(piece) {
            self.constraints.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.constraints.get_mut(piece).unwrap()
    }

    pub fn get(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.constraints.get(piece)
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

pub struct MovesMap {
    piece_to_moves: PieceToMovesMapT,
    // This is Point-to-Pieces-to-Moves structure that allows to fetch all moves for the certain
    // piece in the given point. It is useful because there can be several moves that ends on the
    // same point, but have different meaning. E.g. a promotion of a pawn ends up on the same point,
    // but with different promotion piece
    point_to_pieces: PointToPiecesMapT,
    // Moves map of pieces, except the king when the king is in check
    general_constraints: MoveConstraints,
}

impl MovesMap {
    pub fn empty() -> Self {
        let piece_to_moves = FxHashMap::default();
        let point_to_pieces = FxHashMap::default();
        let general_constraints = MoveConstraints::empty();
        Self { piece_to_moves, point_to_pieces, general_constraints }
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
        match &**piece {
            Piece::King(_) => { self.piece_to_moves.get(piece) },
            _ => {
                if self.general_constraints.is_enabled() {
                    self.general_constraints.get(piece)
                } else {
                    self.piece_to_moves.get(piece)
                }
            },
        }
    }

    pub fn pieces_at(&self, point: &Point) -> Option<&PiecesSetT> {
        self.point_to_pieces.get(point)
    }

    pub fn all_pieces(&self) -> Vec<&Rc<Piece>> {
        self.piece_to_moves.keys().collect()
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

    pub fn clear_general_constraints(&mut self) {
        self.general_constraints.clear();
    }

    pub fn add_general_constraints(&mut self, piece_move: PieceMove) {
        self.general_constraints.enable();
        if let Some(point) = piece_move.destination() {
            if let Some(pieces) = self.point_to_pieces.get(&point) {
                for (piece, piece_moves) in pieces {
                    for piece_move in piece_moves {
                        self.general_constraints.get_mut(piece).insert(*piece_move);
                    }
                }
            }
        }
    }

    pub fn is_empty(&self, king: Option<&Rc<Piece>>) -> bool {
        if let Some(king) = king && self.general_constraints.is_enabled() {
            self.general_constraints.is_empty() && 
                self.piece_to_moves.get(king).unwrap_or(&MovesSetT::default()).is_empty()
        } else {
            self.piece_to_moves.is_empty()
        }
    }
}
