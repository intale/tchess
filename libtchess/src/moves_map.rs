use crate::move_score::MoveScore;
use crate::piece_id::PieceId;
use crate::piece_move::PieceMove;
use crate::point::Point;
use im_rc::{HashMap, HashSet, OrdMap, Vector};
use rustc_hash::FxBuildHasher;

#[derive(Clone)]
pub struct MovesMap {
    score_to_moves: OrdMap<MoveScore, OrdMap<PieceId, Vector<PieceMove>>>,
    piece_to_moves: HashMap<PieceId, HashMap<PieceMove, MoveScore, FxBuildHasher>, FxBuildHasher>,
    // This is Point-to-Pieces-to-Moves structure that allows to fetch all moves for the certain
    // piece in the given point. It is useful because there can be several moves that ends on the
    // same point, but have different meaning. E.g. a promotion of a pawn ends up on the same point,
    // but with different promotion piece
    point_to_pieces: HashMap<
        Point,
        HashMap<PieceId, HashSet<PieceMove, FxBuildHasher>, FxBuildHasher>,
        FxBuildHasher,
    >,
}

impl MovesMap {
    pub fn empty() -> Self {
        let score_to_moves = OrdMap::default();
        let piece_to_moves = HashMap::default();
        let point_to_pieces = HashMap::default();
        Self {
            score_to_moves,
            piece_to_moves,
            point_to_pieces,
        }
    }

    fn s2m_moves_mut(&mut self, move_score: &MoveScore, piece_id: &PieceId) -> &mut Vector<PieceMove> {
        if !self.score_to_moves.contains_key(move_score) {
            self.score_to_moves.insert(*move_score, OrdMap::default());
        }
        let moves = self.score_to_moves.get_mut(move_score).unwrap();
        if !moves.contains_key(piece_id) {
            moves.insert(*piece_id, Vector::new());
        }
        moves.get_mut(piece_id).unwrap()
    }

    fn p2m_moves_mut(&mut self, piece_id: &PieceId) -> &mut HashMap<PieceMove, MoveScore, FxBuildHasher> {
        if !self.piece_to_moves.contains_key(piece_id) {
            self.piece_to_moves.insert(*piece_id, HashMap::default());
        }
        self.piece_to_moves.get_mut(piece_id).unwrap()
    }

    fn p2p_moves_mut(
        &mut self,
        point: Point,
        piece_id: &PieceId,
    ) -> &mut HashSet<PieceMove, FxBuildHasher> {
        if !self.point_to_pieces.contains_key(&point) {
            self.point_to_pieces.insert(point, HashMap::default());
        }
        let pieces_hashmap = self.point_to_pieces.get_mut(&point).unwrap();
        if !pieces_hashmap.contains_key(piece_id) {
            pieces_hashmap.insert(*piece_id, HashSet::default());
        }
        pieces_hashmap.get_mut(piece_id).unwrap()
    }

    pub fn moves_of<'a>(&self, piece_id: &PieceId) -> Option<&HashMap<PieceMove, MoveScore, FxBuildHasher>> {
        self.piece_to_moves.get(piece_id)
    }

    pub fn pieces_to_move_onto(
        &self,
        point: &Point,
    ) -> Option<&HashMap<PieceId, HashSet<PieceMove, FxBuildHasher>, FxBuildHasher>> {
        self.point_to_pieces.get(point)
    }

    pub fn add(&mut self, piece_id: &PieceId, piece_move: PieceMove, score: MoveScore) {
        self.p2m_moves_mut(piece_id)
            .insert(piece_move, score);
        self.p2p_moves_mut(*piece_move.destination(), piece_id)
            .insert(piece_move);
        self.s2m_moves_mut(&score, piece_id).push_front(piece_move);
    }

    pub fn move_score(&self, piece_id: &PieceId, piece_move: &PieceMove) -> Option<&MoveScore> {
        if let Some(move_to_score) = self.piece_to_moves.get(piece_id) {
            move_to_score.get(piece_move)
        } else {
            None
        }
    }

    pub fn remove_piece(
        &mut self,
        piece_id: &PieceId,
    ) -> Option<HashMap<PieceMove, MoveScore, FxBuildHasher>> {
        let moves = self.piece_to_moves.remove(piece_id);

        if let Some(moves) = moves {
            for (piece_move, move_score) in moves.iter() {
                if let Some(piece_to_moves) =
                    self.score_to_moves.get_mut(move_score)
                {
                    piece_to_moves.remove(piece_id);
                    if piece_to_moves.is_empty() {
                        self.score_to_moves.remove(move_score);
                    }
                }
                if let Some(pieces) = self
                    .point_to_pieces
                    .get_mut(piece_move.destination())
                {
                    pieces.remove(piece_id);
                    if pieces.is_empty() {
                        self.point_to_pieces
                            .remove(piece_move.destination());
                    }
                }
            }
            return Some(moves);
        }
        None
    }

    pub fn score_to_moves(&self) -> &OrdMap<MoveScore, OrdMap<PieceId, Vector<PieceMove>>> {
        &self.score_to_moves
    }

    pub fn is_empty(&self) -> bool {
        self.piece_to_moves.is_empty()
    }
}
