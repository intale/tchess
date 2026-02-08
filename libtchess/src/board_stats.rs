use crate::board_summary::{ActivePiecesStats, ZKey};
use crate::colored_property::ColoredProperty;

pub struct BoardStats<'a> {
    pub active_pieces_stats: &'a ColoredProperty<ActivePiecesStats>,
    pub turn_number: &'a usize,
    pub last_capture_turn_number: &'a usize,
    pub last_promote_turn_number: &'a usize,
    pub last_pawn_move_turn_number: &'a usize,
    pub zposition: &'a ZKey,
}
