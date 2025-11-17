use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_move::PieceMove;
use crate::pieces::Piece;
use crate::point::Point;

type MovesSetT = FxHashSet<PieceMove>;
type PiecesSetT = FxHashSet<(Rc<Piece>, PieceMove)>;
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
}

pub struct MovesMap {
    piece_to_moves: PieceToMovesMapT,
    point_to_pieces: PointToPiecesMapT,
    // Moves map of pieces, except the king when the king is in check
    constraints: MoveConstraints,
}

impl MovesMap {
    pub fn empty() -> Self {
        let piece_to_moves = FxHashMap::default();
        let point_to_pieces = FxHashMap::default();
        let constraints = MoveConstraints::empty();
        Self { piece_to_moves, point_to_pieces, constraints }
    }

    fn moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.piece_to_moves.contains_key(piece) {
            self.piece_to_moves.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_moves.get_mut(piece).unwrap()
    }

    fn pieces_mut(&mut self, point: Point) -> &mut PiecesSetT {
        if !self.point_to_pieces.contains_key(&point) {
            self.point_to_pieces.insert(point, FxHashSet::default());
        }
        self.point_to_pieces.get_mut(&point).unwrap()
    }

    pub fn moves_of(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        match &**piece {
            Piece::King(_) => { self.piece_to_moves.get(piece) },
            _ => {
                if self.constraints.is_enabled() {
                    self.constraints.get(piece)
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
                    && self.pieces_mut(point).insert((Rc::clone(piece), piece_move))
            },
            None => self.moves_mut(piece).insert(piece_move),
        }
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) {
        let moves = self.piece_to_moves.remove(piece);
        if let Some(moves) = moves {
            for piece_move in moves.iter() {
                if let Some(point) = piece_move.destination() {
                    if let Some(pieces) = self.point_to_pieces.get_mut(&point) {
                        pieces.remove(&(Rc::clone(piece), *piece_move));
                        if pieces.is_empty() {
                            self.point_to_pieces.remove(&point);
                        }
                    }
                }
            }
        }
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    pub fn add_constraints(&mut self, piece_move: PieceMove) {
        self.constraints.enable();
        if let Some(point) = piece_move.destination() {
            if let Some(pieces) = self.point_to_pieces.get(&point) {
                for (piece, piece_move) in pieces {
                    self.constraints.get_mut(piece).insert(*piece_move);
                }
            }
        }
    }
}
