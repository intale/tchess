use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::piece_move::PieceMove;
use crate::pieces::Piece;
use crate::point::Point;

type MovesSetT = FxHashSet<PieceMove>;
type PiecesSetT = FxHashSet<Rc<Piece>>;
type PieceToMovesMapT = FxHashMap<Rc<Piece>, MovesSetT>;
type MoveToPiecesMapT = FxHashMap<PieceMove, PiecesSetT>;

pub struct MovesMap {
    piece_to_moves: PieceToMovesMapT,
    move_to_pieces: MoveToPiecesMapT,
    constraints: MovesSetT,
}

impl MovesMap {
    pub fn empty() -> Self {
        let piece_to_moves = FxHashMap::default();
        let move_to_pieces = FxHashMap::default();
        let constraints = FxHashSet::default();
        Self { piece_to_moves, move_to_pieces, constraints }
    }

    fn moves_mut(&mut self, piece: &Rc<Piece>) -> &mut MovesSetT {
        if !self.piece_to_moves.contains_key(piece) {
            self.piece_to_moves.insert(Rc::clone(piece), FxHashSet::default());
        }
        self.piece_to_moves.get_mut(piece).unwrap()
    }

    fn pieces_mut(&mut self, piece_move: PieceMove) -> &mut PiecesSetT {
        if !self.move_to_pieces.contains_key(&piece_move) {
            self.move_to_pieces.insert(piece_move, FxHashSet::default());
        }
        self.move_to_pieces.get_mut(&piece_move).unwrap()
    }

    pub fn moves(&self, piece: &Rc<Piece>) -> Option<&MovesSetT> {
        self.piece_to_moves.get(piece)
    }

    pub fn pawns(&self, point: &Point) -> Vec<&Rc<Piece>> {
        let mut pawns = vec![];
        let moves = [
            PieceMove::Point(*point),
            PieceMove::LongMove(*point),
        ];
        for piece_move in moves {
            if let Some(pieces) = self.move_to_pieces.get(&piece_move) {
                pawns.append(
                    &mut pieces.iter().filter(|piece|
                        match &***piece {
                            Piece::Pawn(_) => true,
                            _ => false,
                        }
                    ).collect()
                )
            };
        }
        pawns
    }

    pub fn add(&mut self, piece: &Rc<Piece>, piece_move: PieceMove) -> bool {
        self.moves_mut(piece).insert(piece_move)
            && self.pieces_mut(piece_move).insert(Rc::clone(piece))
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) {
        let moves = self.piece_to_moves.remove(piece);
        if let Some(moves) = moves {
            for piece_move in moves.iter() {
                if let Some(pieces) = self.move_to_pieces.get_mut(piece_move) {
                    pieces.remove(piece);
                    if pieces.is_empty() {
                        self.move_to_pieces.remove(piece_move);
                    }
                }
            }
        }
    }

    pub fn clear_constraints(&mut self) {
        self.constraints.clear()
    }

    pub fn add_constraint(&mut self, piece_move: PieceMove) -> bool {
        self.constraints.insert(piece_move)
    }

    pub fn matches_constraints(&self, piece_move: &PieceMove) -> bool {
        if self.constraints.is_empty() {
            return true;
        }
        match piece_move { 
            PieceMove::Point(_) => self.constraints.contains(piece_move), 
            PieceMove::EnPassant(en_passant, _) => 
                self.constraints.contains(&PieceMove::Point(*en_passant)),
            PieceMove::LongMove(point) => self.constraints.contains(&PieceMove::Point(*point)),
            _ => false,
        }
    }
}
