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
}

impl MovesMap {
    pub fn empty() -> Self {
        let score_to_moves = OrdMap::default();
        let piece_to_moves = HashMap::default();
        Self {
            score_to_moves,
            piece_to_moves,
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

    pub fn moves_of<'a>(&self, piece_id: &PieceId) -> Option<&HashMap<PieceMove, MoveScore, FxBuildHasher>> {
        self.piece_to_moves.get(piece_id)
    }

    pub fn add(&mut self, piece_id: &PieceId, piece_move: PieceMove, score: MoveScore) {
        self.p2m_moves_mut(piece_id)
            .insert(piece_move, score);
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
