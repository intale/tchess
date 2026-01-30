use libtchess::color::Color;
use libtchess::piece::{Piece, PieceId};
use libtchess::point::Point;
use rand_xoshiro::SplitMix64;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

const SEED1: u64 = 0xa7d2c50b1827dd5c;
const SEED2: u64 = 0x63d6f65c2c508220;
const SIDE_TO_MOVE_TAG: u128 = 0xb599dc227a3a1a24100dfc8f1c9cccd6;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PieceRepr {
    Bishop(PieceData),
    King(PieceData),
    Knight(PieceData),
    Pawn(PieceData),
    Queen(PieceData),
    Rook(PieceData),
}

impl PieceRepr {
    pub fn data(&self) -> &PieceData {
        match self {
            Self::Bishop(data)
            | Self::King(data)
            | Self::Knight(data)
            | Self::Pawn(data)
            | Self::Queen(data)
            | Self::Rook(data) => data,
        }
    }

    pub fn data_mut(&mut self) -> &mut PieceData {
        match self {
            Self::Bishop(data)
            | Self::King(data)
            | Self::Knight(data)
            | Self::Pawn(data)
            | Self::Queen(data)
            | Self::Rook(data) => data,
        }
    }

    fn pack(&self) -> u64 {
        let data = self.data();
        let kind_repr: u64 = match self {
            Self::Bishop(_) => 0,
            Self::King(_) => 1,
            Self::Knight(_) => 2,
            Self::Pawn(_) => 3,
            Self::Queen(_) => 4,
            Self::Rook(_) => 5,
        };
        let color_repr: u64 = match data.color {
            Color::White => 0,
            Color::Black => 1,
        };
        let x_pos: u64 = *data.position.x().value() as u64;
        let y_pos: u64 = *data.position.y().value() as u64;
        let castle_available = data.castle as u64;
        let en_passant_available = data.castle as u64;
        // Layout(high to low bits): [kind:8; color:1; EnPassant:1; Castle:1; Ypos:16; Xpos: 16]
        x_pos
            | (y_pos << 16)
            | (castle_available << 33)
            | (en_passant_available << 34)
            | (color_repr << 35)
            | (kind_repr << 36)
    }

    pub fn zobrist_repr(&self) -> u128 {
        let payload = self.pack();
        let high_bits = SplitMix64::seed_from_u64(SEED1 ^ payload).next_u64() as u128;
        let low_bits = SplitMix64::seed_from_u64(SEED2 ^ payload).next_u64() as u128;
        (high_bits << 64) | low_bits
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct PieceData {
    pub position: Point,
    pub en_passant: bool,
    pub castle: bool,
    pub color: Color,
}

#[derive(Eq, PartialEq, Debug)]
pub struct BoardPosition {
    current_turn: Color,
    pieces: Vec<Option<PieceRepr>>,
    zkey: u128,
}

impl BoardPosition {
    pub fn new() -> Self {
        Self {
            current_turn: Color::White,
            pieces: vec![],
            zkey: 0,
        }
    }

    pub fn set_current_turn(&mut self, color: &Color) {
        if &self.current_turn == color {
            return;
        }
        self.current_turn = *color;
        self.zkey ^= SIDE_TO_MOVE_TAG;
    }

    pub fn add_piece(&mut self, piece: &Rc<Piece>) {
        let piece_data = PieceData {
            position: piece.current_position(),
            en_passant: piece.buffs().has_en_passant(),
            castle: piece.buffs().has_castle(),
            color: *piece.color(),
        };
        let piece_repr = match &**piece {
            Piece::Bishop(_) => PieceRepr::Bishop(piece_data),
            Piece::King(_) => PieceRepr::King(piece_data),
            Piece::Knight(_) => PieceRepr::Knight(piece_data),
            Piece::Pawn(_) => PieceRepr::Pawn(piece_data),
            Piece::Queen(_) => PieceRepr::Queen(piece_data),
            Piece::Rook(_) => PieceRepr::Rook(piece_data),
            Piece::UnknownPiece(_) => {
                panic!("Unknown piece can't have its BoardPosition representation")
            }
        };

        self.zkey ^= piece_repr.zobrist_repr();
        self.pieces.push(Some(piece_repr));
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) {
        let err_msg = format!(
            "Logical error: trying to remove never present piece representation of {}",
            piece,
        );
        let piece_repr = self
            .pieces
            .get_mut(Self::id_to_pos(piece.id()))
            .take()
            .expect(err_msg.as_str())
            .expect(err_msg.as_str());
        self.zkey ^= piece_repr.zobrist_repr();
    }

    pub fn update_piece_position(&mut self, piece: &Rc<Piece>) {
        self.update_piece_repr(piece, |piece_repr| {
            piece_repr.data_mut().position = piece.current_position()
        });
    }

    pub fn update_piece_en_passant(&mut self, piece: &Rc<Piece>) {
        self.update_piece_repr(piece, |piece_repr| {
            piece_repr.data_mut().en_passant = piece.buffs().has_en_passant()
        });
    }

    pub fn update_piece_castle(&mut self, piece: &Rc<Piece>) {
        self.update_piece_repr(piece, |piece_repr| {
            piece_repr.data_mut().castle = piece.buffs().has_castle()
        });
    }

    pub fn copy(&self) -> BoardPosition {
        Self {
            current_turn: self.current_turn,
            pieces: self.copy_pieces(),
            zkey: self.zkey,
        }
    }

    fn copy_pieces(&self) -> Vec<Option<PieceRepr>> {
        let pieces = &self.pieces;
        let mut new = Vec::with_capacity(pieces.len());
        unsafe { new.set_len(pieces.len()) };
        new.copy_from_slice(&pieces[..]);
        new
    }

    fn get_piece_mut(&mut self, piece: &Rc<Piece>) -> &mut PieceRepr {
        let piece_repr = self
            .pieces
            .get_mut(Self::id_to_pos(piece.id()))
            .unwrap_or_else(|| {
                panic!(
                    "Logical error: piece {} does not have its BoardPosition representation",
                    piece
                )
            })
            .as_mut();
        piece_repr.expect(format!("Can't update removed piece entity of {}", piece).as_str())
    }

    fn update_piece_repr<F: FnOnce(&mut PieceRepr)>(&mut self, piece: &Rc<Piece>, update_func: F) {
        let piece_repr = self.get_piece_mut(piece);
        let old_zkey = piece_repr.zobrist_repr();
        update_func(piece_repr);
        let new_zkey = piece_repr.zobrist_repr();
        self.zkey ^= old_zkey;
        self.zkey ^= new_zkey;
    }

    fn id_to_pos(piece_id: &PieceId) -> usize {
        piece_id.0 - 1
    }
}

impl Hash for BoardPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u128(self.zkey)
    }
}
