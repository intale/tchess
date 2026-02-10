use crate::board_stats::BoardStats;
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::piece::Piece;
use crate::piece_id::PieceId;
use crate::point::Point;
use im_rc::HashMap;
use rand_xoshiro::SplitMix64;
use rand_xoshiro::rand_core::{SeedableRng, TryRng};
use rustc_hash::FxBuildHasher;
use std::ops::BitXorAssign;

const SEED1: u64 = 0xa7d2c50b1827dd5c;
const SEED2: u64 = 0x63d6f65c2c508220;
const SIDE_TO_MOVE_TAG: u128 = 0xb599dc227a3a1a24100dfc8f1c9cccd6;

#[derive(Eq, PartialEq, Clone)]
pub struct ActivePiecesStats {
    pub bishops_count: isize,
    pub kings_count: isize,
    pub knights_count: isize,
    pub pawns_count: isize,
    pub queens_count: isize,
    pub rooks_count: isize,
}

impl ActivePiecesStats {
    pub fn empty() -> Self {
        Self {
            bishops_count: 0,
            kings_count: 0,
            knights_count: 0,
            pawns_count: 0,
            queens_count: 0,
            rooks_count: 0,
        }
    }
}

const BISHOP_REPR: u64 = 0;
const KING_REPR: u64 = 1;
const KNIGHT_REPR: u64 = 2;
const PAWN_REPR: u64 = 3;
const QUEEN_REPR: u64 = 4;
const ROOK_REPR: u64 = 5;

const PIECE_KIND_OFFSET: u64 = 35;
const PIECE_KIND_SIZE: u32 = 3;

#[derive(Copy, Clone)]
struct PieceRepr {
    // Layout(high to low bits) and size(bits):
    // [kind: 3; color: 1; EnPassant: 1; Castle: 1; Ypos: 16; Xpos: 16]
    packed: u64,
    initialized: bool,
}

impl PieceRepr {
    pub fn from_u64(packed: u64) -> Self {
        Self {
            packed,
            initialized: true,
        }
    }

    pub fn from_piece(piece: &Piece) -> Self {
        let mut packed = Self {
            packed: 0,
            initialized: false,
        };
        packed.pack_kind(piece);
        packed.pack_color(piece.color());
        packed.pack_position(piece.current_position());
        packed.pack_castle(piece.buffs().has_castle());
        packed.pack_en_passant(piece.buffs().has_en_passant());
        packed.initialized = true;
        packed
    }

    pub fn pack_kind(&mut self, piece: &Piece) {
        let kind_repr: u64 = match piece {
            Piece::Bishop(_) => BISHOP_REPR,
            Piece::King(_) => KING_REPR,
            Piece::Knight(_) => KNIGHT_REPR,
            Piece::Pawn(_) => PAWN_REPR,
            Piece::Queen(_) => QUEEN_REPR,
            Piece::Rook(_) => ROOK_REPR,
            Piece::UnknownPiece(_) => panic!("Can't pack unknown piece!"),
        };
        self.pack(
            PIECE_KIND_OFFSET,
            PIECE_KIND_SIZE,
            kind_repr,
            self.initialized,
        );
    }

    pub fn pack_color(&mut self, color: &Color) {
        let color_repr: u64 = match color {
            Color::White => 0,
            Color::Black => 1,
        };
        self.pack(34, 1, color_repr, self.initialized);
    }

    pub fn pack_position(&mut self, position: &Point) {
        let x_pos: u64 = (*position.x().value() as u16) as u64;
        let y_pos: u64 = (*position.y().value() as u16) as u64;
        self.pack(0, 16, x_pos, self.initialized);
        self.pack(16, 16, y_pos, self.initialized);
    }

    pub fn pack_castle(&mut self, has_castle: bool) {
        let castle_available = has_castle as u64;
        self.pack(32, 1, castle_available, self.initialized);
    }

    pub fn pack_en_passant(&mut self, has_en_passant: bool) {
        let en_passant_available = has_en_passant as u64;
        self.pack(33, 1, en_passant_available, self.initialized);
    }

    pub fn packed(&self) -> u64 {
        self.packed
    }

    pub fn piece_type_repr(&self) -> u64 {
        let bits_to_extract = 2u64.pow(PIECE_KIND_SIZE) - 1;
        let shifted = self.packed >> PIECE_KIND_OFFSET;
        shifted & bits_to_extract
    }

    fn pack(&mut self, offset: u64, size: u32, value: u64, clear_bits: bool) {
        self.packed = if clear_bits {
            let bits_to_clear = (2u64.pow(size) - 1) << offset;
            ((self.packed | bits_to_clear) ^ bits_to_clear) | value << offset
        } else {
            self.packed | value << offset
        };
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
pub struct ZKey(pub u128);

impl BitXorAssign<u128> for ZKey {
    fn bitxor_assign(&mut self, rhs: u128) {
        self.0 ^= rhs;
    }
}

#[derive(Clone)]
pub struct BoardSummary {
    active_pieces_stats: ColoredProperty<ActivePiecesStats>,
    turn_number: usize,
    last_capture_turn_number: usize,
    last_promote_turn_number: usize,
    last_pawn_move_turn_number: usize,
    last_captured_piece: Option<Piece>,
    // Zobrist representation of the current position
    zposition: ZKey,
    packed_pieces: HashMap<PieceId, PieceRepr, FxBuildHasher>,
}

impl BoardSummary {
    pub fn new() -> Self {
        Self {
            active_pieces_stats: ColoredProperty([
                ActivePiecesStats::empty(),
                ActivePiecesStats::empty(),
            ]),
            turn_number: 1,
            last_capture_turn_number: 0,
            last_promote_turn_number: 0,
            last_pawn_move_turn_number: 0,
            last_captured_piece: None,
            zposition: ZKey(0),
            packed_pieces: HashMap::default(),
        }
    }

    pub fn next_turn(&mut self) {
        self.zposition ^= SIDE_TO_MOVE_TAG;
        self.turn_number += 1;
    }

    pub fn piece_captured(&mut self, captured_piece: Piece) {
        self.last_capture_turn_number = self.turn_number;
        self.last_captured_piece = Some(captured_piece);
    }

    pub fn piece_promoted(&mut self) {
        self.last_promote_turn_number = self.turn_number;
    }

    pub fn pawn_moved(&mut self) {
        self.last_pawn_move_turn_number = self.turn_number;
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        match piece {
            Piece::Bishop(_) => self.active_pieces_stats[piece.color()].bishops_count += 1,
            Piece::King(_) => self.active_pieces_stats[piece.color()].kings_count += 1,
            Piece::Knight(_) => self.active_pieces_stats[piece.color()].knights_count += 1,
            Piece::Pawn(_) => self.active_pieces_stats[piece.color()].pawns_count += 1,
            Piece::Queen(_) => self.active_pieces_stats[piece.color()].queens_count += 1,
            Piece::Rook(_) => self.active_pieces_stats[piece.color()].rooks_count += 1,
            Piece::UnknownPiece(_) => panic!("Can't add unknown piece to active pieces list!"),
        }
        let packed_piece = PieceRepr::from_piece(piece);
        self.zposition ^= Self::zobrist_repr(packed_piece.packed());
        self.packed_pieces.insert(*piece.id(), packed_piece);
    }

    pub fn remove_piece(&mut self, piece_id: &PieceId) {
        let packed_repr = self.packed_pieces.remove(piece_id).expect(
            format!(
                "Logical error: failed to remove {} from packed pieces representation!",
                piece_id
            )
            .as_str(),
        );

        match packed_repr.piece_type_repr() {
            BISHOP_REPR => self.active_pieces_stats[&piece_id.color()].bishops_count -= 1,
            KING_REPR => self.active_pieces_stats[&piece_id.color()].kings_count -= 1,
            KNIGHT_REPR => self.active_pieces_stats[&piece_id.color()].knights_count -= 1,
            PAWN_REPR => self.active_pieces_stats[&piece_id.color()].pawns_count -= 1,
            QUEEN_REPR => self.active_pieces_stats[&piece_id.color()].queens_count -= 1,
            ROOK_REPR => self.active_pieces_stats[&piece_id.color()].rooks_count -= 1,
            _ => panic!("Can't add unknown piece to active pieces list!"),
        }

        self.zposition ^= Self::zobrist_repr(packed_repr.packed());
    }

    pub fn stats(&'_ self) -> BoardStats<'_> {
        BoardStats {
            active_pieces_stats: &self.active_pieces_stats,
            turn_number: &self.turn_number,
            last_capture_turn_number: &self.last_capture_turn_number,
            last_promote_turn_number: &self.last_promote_turn_number,
            last_pawn_move_turn_number: &self.last_pawn_move_turn_number,
            zposition: &self.zposition,
            last_captured_piece: self.last_captured_piece.as_ref(),
        }
    }

    pub fn update_piece_position(&mut self, piece_id: &PieceId, new_position: &Point) {
        self.update_piece_repr(piece_id, |piece_repr| {
            piece_repr.pack_position(new_position);
        });
    }

    pub fn update_piece_en_passant(&mut self, piece_id: &PieceId, has_en_passant: bool) {
        self.update_piece_repr(piece_id, |piece_repr| {
            piece_repr.pack_en_passant(has_en_passant);
        });
    }

    pub fn update_piece_castle(&mut self, piece_id: &PieceId, has_castle: bool) {
        self.update_piece_repr(piece_id, |piece_repr| {
            piece_repr.pack_castle(has_castle);
        });
    }

    fn update_piece_repr<F: FnOnce(&mut PieceRepr)>(&mut self, piece_id: &PieceId, update_func: F) {
        let piece_repr = self.get_packed_piece_mut(piece_id);
        let packed_was = piece_repr.packed();
        update_func(piece_repr);
        let packed_now = piece_repr.packed();
        self.zposition ^= Self::zobrist_repr(packed_was);
        self.zposition ^= Self::zobrist_repr(packed_now);
    }

    fn get_packed_piece_mut(&mut self, piece_id: &PieceId) -> &mut PieceRepr {
        self.packed_pieces.get_mut(piece_id).unwrap_or_else(|| {
            panic!(
                "Logical error: piece with ID {} does not have its packed representation!",
                piece_id
            )
        })
    }

    fn zobrist_repr(packed: u64) -> u128 {
        let high_bits = SplitMix64::seed_from_u64(SEED1 ^ packed)
            .try_next_u64()
            .unwrap() as u128;
        let low_bits = SplitMix64::seed_from_u64(SEED2 ^ packed)
            .try_next_u64()
            .unwrap() as u128;
        (high_bits << 64) | low_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod piece_repr_tests {
        use super::*;
        use crate::buff::Buff;
        use crate::piece::PieceInit;
        use crate::piece::bishop::Bishop;
        use crate::piece::king::King;
        use crate::piece::knight::Knight;
        use crate::piece::pawn::Pawn;
        use crate::piece::queen::Queen;
        use crate::piece::rook::Rook;

        fn format_packed(packed: u64) -> String {
            let binary_repr = format!("{:b}", packed);
            let adjusted_repr = format!("{:0>38}", binary_repr);

            println!("Layout: [kind: 3; color: 1; EnPassant: 1; Castle: 1; Ypos: 16; Xpos: 16]");
            format!(
                // Bits, from highest to lowest
                "{} {} {} {} {} {}",
                // kind, 3 bits
                adjusted_repr[0..=2].to_string(),
                // color, 1 bit
                adjusted_repr[3..=3].to_string(),
                // EnPassant, 1 bit
                adjusted_repr[4..=4].to_string(),
                // Castle, 1 bit
                adjusted_repr[5..=5].to_string(),
                // Ypos, 16 bit
                adjusted_repr[6..=21].to_string(),
                // Xpos, 16 bit
                adjusted_repr[22..=37].to_string(),
            )
        }

        mod packing_the_whole_piece {
            use super::*;

            #[test]
            fn it_packs_max_x_min_y_position_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(i16::MAX, i16::MIN),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 0 0 0 1000000000000000 0111111111111111"
                )
            }

            #[test]
            fn it_packs_min_x_max_y_position_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(i16::MIN, i16::MAX),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 0 0 0 0111111111111111 1000000000000000"
                )
            }

            #[test]
            fn it_packs_max_x_max_y_position_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(i16::MAX, i16::MAX),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 0 0 0 0111111111111111 0111111111111111"
                )
            }

            #[test]
            fn it_packs_min_x_min_y_position_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(i16::MIN, i16::MIN),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 0 0 0 1000000000000000 1000000000000000"
                )
            }

            #[test]
            fn it_packs_white_bishop_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_bishop_correctly() {
                let piece = Piece::Bishop(Bishop::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "000 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_king_with_castle_correctly() {
                let piece = Piece::King(King::new(
                    Color::White,
                    vec![Buff::Castle],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "001 0 0 1 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_king_without_castle_correctly() {
                let piece = Piece::King(King::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "001 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_king_with_castle_correctly() {
                let piece = Piece::King(King::new(
                    Color::Black,
                    vec![Buff::Castle],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "001 1 0 1 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_king_without_castle_correctly() {
                let piece = Piece::King(King::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "001 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_knight_correctly() {
                let piece = Piece::Knight(Knight::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(2, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "010 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_knight_correctly() {
                let piece = Piece::Knight(Knight::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "010 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_pawn_with_en_passant_correctly() {
                let piece = Piece::Pawn(Pawn::new(
                    Color::White,
                    vec![Buff::EnPassant(Point::new(2, 3), Point::new(2, 2))],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "011 0 1 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_pawn_without_en_passant_correctly() {
                let piece = Piece::Pawn(Pawn::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "011 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_pawn_with_en_passant_correctly() {
                let piece = Piece::Pawn(Pawn::new(
                    Color::Black,
                    vec![Buff::EnPassant(Point::new(2, 3), Point::new(2, 2))],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "011 1 1 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_pawn_without_en_passant_correctly() {
                let piece = Piece::Pawn(Pawn::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "011 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_queen_correctly() {
                let piece = Piece::Queen(Queen::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "100 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_queen_correctly() {
                let piece = Piece::Queen(Queen::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "100 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_rook_with_castle_correctly() {
                let piece = Piece::Rook(Rook::new(
                    Color::White,
                    vec![Buff::Castle],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "101 0 0 1 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_white_rook_without_castle_correctly() {
                let piece = Piece::Rook(Rook::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "101 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_rook_with_castle_correctly() {
                let piece = Piece::Rook(Rook::new(
                    Color::Black,
                    vec![Buff::Castle],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "101 1 0 1 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_packs_black_rook_without_castle_correctly() {
                let piece = Piece::Rook(Rook::new(
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::Black),
                ));
                let piece_repr = PieceRepr::from_piece(&piece);
                assert_eq!(
                    format_packed(piece_repr.packed),
                    "101 1 0 0 0000000000000010 0000000000000001"
                )
            }
        }

        mod update_packed_piece {
            use super::*;

            fn packed_piece() -> PieceRepr {
                let piece = Piece::King(King::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                PieceRepr::from_piece(&piece)
            }

            #[test]
            fn it_correctly_updates_kind() {
                let another_piece = Piece::Knight(Knight::new(
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(1, 2),
                    PieceId::new(1, &Color::White),
                ));
                let mut packed = packed_piece();
                packed.pack_kind(&another_piece);

                assert_eq!(
                    format_packed(packed.packed()),
                    "010 0 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_correctly_updates_color() {
                let mut packed = packed_piece();
                packed.pack_color(&Color::Black);

                assert_eq!(
                    format_packed(packed.packed()),
                    "001 1 0 0 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn it_correctly_updates_position() {
                let mut packed = packed_piece();
                packed.pack_position(&Point::new(2, 1));

                assert_eq!(
                    format_packed(packed.packed()),
                    "001 0 0 0 0000000000000001 0000000000000010"
                )
            }

            #[test]
            fn it_correctly_updates_castle() {
                let mut packed = packed_piece();
                packed.pack_castle(true);

                assert_eq!(
                    format_packed(packed.packed()),
                    "001 0 0 1 0000000000000010 0000000000000001"
                )
            }

            #[test]
            fn pack_en_passant() {
                let mut packed = packed_piece();
                packed.pack_en_passant(true);

                assert_eq!(
                    format_packed(packed.packed()),
                    "001 0 1 0 0000000000000010 0000000000000001"
                )
            }
        }
    }
}
