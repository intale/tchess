use std::collections::BTreeSet;
use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::move_score::MoveScore;
use crate::piece_move::PieceMove;
use crate::piece::Piece;
use crate::point::Point;

pub type MovesSetT = FxHashSet<PieceMove>;
pub type PieceToMovesMapT = FxHashMap<Rc<Piece>, MovesSetT>;
pub type PointToPiecesMapT = FxHashMap<Point, PieceToMovesMapT>;

pub struct MovesMap {
    scores: BTreeSet<MoveScore>,
    score_to_piece_moves: FxHashMap<MoveScore, PieceToMovesMapT>,
    piece_to_scores: FxHashMap<Rc<Piece>, FxHashSet<MoveScore>>,
    piece_to_moves: PieceToMovesMapT,
    // This is Point-to-Pieces-to-Moves structure that allows to fetch all moves for the certain
    // piece in the given point. It is useful because there can be several moves that ends on the
    // same point, but have different meaning. E.g. a promotion of a pawn ends up on the same point,
    // but with different promotion piece
    point_to_pieces: PointToPiecesMapT,
}

impl MovesMap {
    pub fn empty() -> Self {
        let scores = BTreeSet::default();
        let score_to_piece_moves = FxHashMap::default();
        let piece_to_scores = FxHashMap::default();
        let piece_to_moves = FxHashMap::default();
        let point_to_pieces = FxHashMap::default();
        Self {
            scores,
            score_to_piece_moves,
            piece_to_scores,
            piece_to_moves,
            point_to_pieces,
        }
    }

    fn p2m_moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.piece_to_moves.contains_key(piece) {
            self.piece_to_moves.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_moves.get_mut(piece).unwrap()
    }

    fn p2p_moves_mut(&mut self, point: Point, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.point_to_pieces.contains_key(&point) {
            self.point_to_pieces.insert(point, FxHashMap::default());
        }
        let pieces_hashmap = self.point_to_pieces.get_mut(&point).unwrap();
        if !pieces_hashmap.contains_key(piece) {
            pieces_hashmap.insert(Rc::clone(piece), FxHashSet::default());
        }
        pieces_hashmap.get_mut(piece).unwrap()
    }

    fn s2p_moves_mut(&mut self, score: MoveScore, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.score_to_piece_moves.contains_key(&score) {
            self.score_to_piece_moves.insert(score, FxHashMap::default());
        }
        let pieces_hashmap = self.score_to_piece_moves.get_mut(&score).unwrap();
        if !pieces_hashmap.contains_key(piece) {
            pieces_hashmap.insert(Rc::clone(piece), FxHashSet::default());
        }
        pieces_hashmap.get_mut(piece).unwrap()
    }

    fn p2s_mut(&mut self, piece: &Rc<Piece>) -> &mut FxHashSet<MoveScore> {
        if !self.piece_to_scores.contains_key(piece) {
            self.piece_to_scores.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_scores.get_mut(piece).unwrap()
    }

    pub fn moves_of(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.piece_to_moves.get(piece)
    }

    pub fn pieces_to_move_onto(&self, point: &Point) -> Option<&PieceToMovesMapT> {
        self.point_to_pieces.get(point)
    }

    pub fn add(&mut self, piece: &Rc<Piece>, piece_move: PieceMove, score: MoveScore) -> bool {
        self.p2m_moves_mut(piece).insert(piece_move)
            && self.p2p_moves_mut(*piece_move.destination(), piece).insert(piece_move)
            && self.s2p_moves_mut(score, piece).insert(piece_move)
            && self.scores.insert(score)
            && self.p2s_mut(piece).insert(score)
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) -> Option<MovesSetT> {
        let moves = self.piece_to_moves.remove(piece);
        let scores = self.piece_to_scores.remove(piece);

        if let Some(scores) = scores {
            for score in scores.iter() {
                if let Some(pieces) = self.score_to_piece_moves.get_mut(score) {
                    pieces.remove(piece);
                    if pieces.is_empty() {
                        self.score_to_piece_moves.remove(score);
                        self.scores.remove(score);
                    }
                }
            }
        }
        if let Some(moves) = moves {
            for piece_move in moves.iter() {
                if let Some(pieces) = self.point_to_pieces.get_mut(piece_move.destination()) {
                    pieces.remove(piece);
                    if pieces.is_empty() {
                        self.point_to_pieces.remove(piece_move.destination());
                    }
                }
            }
            return Some(moves);
        }
        None
    }
    
    pub fn move_scores(&self) -> &BTreeSet<MoveScore> {
        &self.scores
    }
    
    pub fn moves_by_score(&self, piece: &Rc<Piece>, score: &MoveScore) -> Option<&MovesSetT> {
        if let Some(pieces) = &self.score_to_piece_moves.get(score) {
            return pieces.get(piece)
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.piece_to_moves.is_empty()
    }
}
