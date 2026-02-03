use crate::board_position::{BoardPosition, PieceRepr, ZKey};
use libtchess::color::Color;
use libtchess::colored_property::ColoredProperty;
use rustc_hash::FxHashMap;

#[derive(Eq, PartialEq)]
struct ActivePiecesStats {
    pub bishop_count: u16,
    pub king_count: u16,
    pub knight_count: u16,
    pub pawn_count: u16,
    pub queen_count: u16,
    pub rook_count: u16,
}

impl ActivePiecesStats {
    pub fn empty() -> Self {
        Self {
            bishop_count: 0,
            king_count: 0,
            knight_count: 0,
            pawn_count: 0,
            queen_count: 0,
            rook_count: 0,
        }
    }

    pub fn is_insufficient_material(&self) -> bool {
        let no_other_pieces = self.rook_count == 0 && self.queen_count == 0 && self.pawn_count == 0;
        no_other_pieces
            && ((self.knight_count == 2 && self.bishop_count == 0)
                || (self.knight_count == 1 && self.bishop_count == 0)
                || (self.knight_count == 0 && self.bishop_count == 1)
                || (self.knight_count == 0 && self.bishop_count == 0))
    }
}

pub struct GameStats {
    active_pieces_stats: ColoredProperty<ActivePiecesStats>,
    move_number: usize,
    // Moves number made to trigger so called "50 move-rule"
    meaningless_moves_number: u8,
    persisted_positions: FxHashMap<ZKey, u8>,
    most_frequent_position: Option<(ZKey, u8)>,
}

impl GameStats {
    pub fn new() -> Self {
        Self {
            active_pieces_stats: ColoredProperty([
                ActivePiecesStats::empty(),
                ActivePiecesStats::empty(),
            ]),
            move_number: 1,
            meaningless_moves_number: 0,
            persisted_positions: FxHashMap::default(),
            most_frequent_position: None,
        }
    }

    pub fn incr_move_number(&mut self) {
        self.move_number += 1;
    }

    pub fn incr_meaningless_moves_number(&mut self) {
        self.meaningless_moves_number += 1;
    }

    pub fn reset_meaningless_moves_number(&mut self) {
        self.meaningless_moves_number = 0;
    }

    pub fn meaningless_moves_number(&self) -> &u8 {
        &self.meaningless_moves_number
    }

    pub fn add_active_piece(&mut self, piece: &PieceRepr) {
        match piece {
            PieceRepr::Bishop(_) => self.active_pieces_stats[&piece.data().color].bishop_count += 1,
            PieceRepr::King(_) => self.active_pieces_stats[&piece.data().color].king_count += 1,
            PieceRepr::Knight(_) => self.active_pieces_stats[&piece.data().color].knight_count += 1,
            PieceRepr::Pawn(_) => self.active_pieces_stats[&piece.data().color].pawn_count += 1,
            PieceRepr::Queen(_) => self.active_pieces_stats[&piece.data().color].queen_count += 1,
            PieceRepr::Rook(_) => self.active_pieces_stats[&piece.data().color].rook_count += 1,
        }
    }

    pub fn remove_active_piece(&mut self, piece: &PieceRepr) {
        match piece {
            PieceRepr::Bishop(_) => self.active_pieces_stats[&piece.data().color].bishop_count -= 1,
            PieceRepr::King(_) => self.active_pieces_stats[&piece.data().color].king_count -= 1,
            PieceRepr::Knight(_) => self.active_pieces_stats[&piece.data().color].knight_count -= 1,
            PieceRepr::Pawn(_) => self.active_pieces_stats[&piece.data().color].pawn_count -= 1,
            PieceRepr::Queen(_) => self.active_pieces_stats[&piece.data().color].queen_count -= 1,
            PieceRepr::Rook(_) => self.active_pieces_stats[&piece.data().color].rook_count -= 1,
        }
    }

    pub fn is_insufficient_material(&self, color: &Color) -> bool {
        self.active_pieces_stats[color].is_insufficient_material()
    }

    pub fn persist_position(&mut self, board_position: &BoardPosition) {
        let zkey = board_position.zkey();
        if !self.persisted_positions.contains_key(zkey) {
            self.persisted_positions.insert(*zkey, 0);
        }
        let occurrences_num = self.persisted_positions.get_mut(zkey).unwrap();
        *occurrences_num += 1;
        if let Some((_, old_occurrences)) = self.most_frequent_position.as_mut() {
            if occurrences_num > old_occurrences {
                self.most_frequent_position = Some((*zkey, *occurrences_num))
            }
        } else {
            self.most_frequent_position = Some((*zkey, *occurrences_num))
        }
    }

    pub fn most_frequent_position(&self) -> Option<&(ZKey, u8)> {
        self.most_frequent_position.as_ref()
    }
}
