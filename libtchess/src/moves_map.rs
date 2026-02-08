use crate::move_score::MoveScore;
use crate::piece_id::PieceId;
use crate::piece_move::PieceMove;
use crate::point::Point;
use im_rc::{HashMap, HashSet, OrdSet};
use rustc_hash::FxBuildHasher;

pub type MovesSetT = HashSet<PieceMove>;
pub type PieceToMovesMapT = HashMap<PieceId, MovesSetT, FxBuildHasher>;
pub type PointToPiecesMapT = HashMap<Point, PieceToMovesMapT, FxBuildHasher>;

#[derive(Clone)]
pub struct MovesMap {
    scores: OrdSet<MoveScore>,
    score_to_piece_moves: HashMap<MoveScore, PieceToMovesMapT, FxBuildHasher>,
    piece_to_scores: HashMap<PieceId, HashSet<MoveScore>, FxBuildHasher>,
    piece_to_moves: PieceToMovesMapT,
    // This is Point-to-Pieces-to-Moves structure that allows to fetch all moves for the certain
    // piece in the given point. It is useful because there can be several moves that ends on the
    // same point, but have different meaning. E.g. a promotion of a pawn ends up on the same point,
    // but with different promotion piece
    point_to_pieces: PointToPiecesMapT,
}

impl MovesMap {
    pub fn empty() -> Self {
        let scores = OrdSet::default();
        let score_to_piece_moves = HashMap::default();
        let piece_to_scores = HashMap::default();
        let piece_to_moves = HashMap::default();
        let point_to_pieces = HashMap::default();
        Self {
            scores,
            score_to_piece_moves,
            piece_to_scores,
            piece_to_moves,
            point_to_pieces,
        }
    }

    fn p2m_moves_mut(&mut self, piece_id: &PieceId) -> &mut MovesSetT {
        if !self.piece_to_moves.contains_key(piece_id) {
            self.piece_to_moves.insert(*piece_id, HashSet::default());
        }
        self.piece_to_moves.get_mut(piece_id).unwrap()
    }

    fn p2p_moves_mut(&mut self, point: Point, piece_id: &PieceId) -> &mut MovesSetT {
        if !self.point_to_pieces.contains_key(&point) {
            self.point_to_pieces.insert(point, HashMap::default());
        }
        let pieces_hashmap = self.point_to_pieces.get_mut(&point).unwrap();
        if !pieces_hashmap.contains_key(piece_id) {
            pieces_hashmap.insert(*piece_id, HashSet::default());
        }
        pieces_hashmap.get_mut(piece_id).unwrap()
    }

    fn s2p_moves_mut(&mut self, score: MoveScore, piece_id: &PieceId) -> &mut MovesSetT {
        if !self.score_to_piece_moves.contains_key(&score) {
            self.score_to_piece_moves.insert(score, HashMap::default());
        }
        let pieces_hashmap = self.score_to_piece_moves.get_mut(&score).unwrap();
        if !pieces_hashmap.contains_key(piece_id) {
            pieces_hashmap.insert(*piece_id, HashSet::default());
        }
        pieces_hashmap.get_mut(piece_id).unwrap()
    }

    fn p2s_mut(&mut self, piece_id: &PieceId) -> &mut HashSet<MoveScore> {
        if !self.piece_to_scores.contains_key(piece_id) {
            self.piece_to_scores.insert(*piece_id, HashSet::default());
        }
        self.piece_to_scores.get_mut(piece_id).unwrap()
    }

    pub fn moves_of(&self, piece_id: &PieceId) -> Option<&MovesSetT> {
        self.piece_to_moves.get(piece_id)
    }

    pub fn pieces_to_move_onto(&self, point: &Point) -> Option<&PieceToMovesMapT> {
        self.point_to_pieces.get(point)
    }

    pub fn add(&mut self, piece_id: &PieceId, piece_move: PieceMove, score: MoveScore) {
        self.p2m_moves_mut(piece_id).insert(piece_move);
        self.p2p_moves_mut(*piece_move.destination(), piece_id)
            .insert(piece_move);
        self.s2p_moves_mut(score, piece_id).insert(piece_move);
        self.scores.insert(score);
        self.p2s_mut(piece_id).insert(score);
    }

    pub fn remove_piece(&mut self, piece_id: &PieceId) -> Option<MovesSetT> {
        let moves = self.piece_to_moves.remove(piece_id);
        let scores = self.piece_to_scores.remove(piece_id);

        if let Some(scores) = scores {
            for score in scores.iter() {
                if let Some(pieces) = self.score_to_piece_moves.get_mut(score) {
                    pieces.remove(piece_id);
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
                    pieces.remove(piece_id);
                    if pieces.is_empty() {
                        self.point_to_pieces.remove(piece_move.destination());
                    }
                }
            }
            return Some(moves);
        }
        None
    }

    pub fn move_scores(&self) -> &OrdSet<MoveScore> {
        &self.scores
    }

    pub fn moves_by_score(&self, score: &MoveScore) -> Option<&PieceToMovesMapT> {
        self.score_to_piece_moves.get(score)
    }

    pub fn is_empty(&self) -> bool {
        self.piece_to_moves.is_empty()
    }
}
