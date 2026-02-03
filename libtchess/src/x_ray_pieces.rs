use rustc_hash::{FxHashMap};
use crate::piece_id::PieceId;
use crate::piece::{Piece};
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector::Vector;

#[derive(Debug)]
struct XRayData {
    direction: Vector,
    pin: Option<PieceId>,
}

#[derive(Debug)]
pub struct XRayPieces {
    direction_to_piece: FxHashMap<Vector, PieceId>,
    x_ray_data: FxHashMap<PieceId, XRayData>,
}

impl XRayPieces {
    pub fn empty() -> Self {
        Self { direction_to_piece: FxHashMap::default(), x_ray_data: FxHashMap::default() }
    }

    pub fn add_x_ray_vector<'a>(&mut self, vector: &Vector, current_piece: Option<&'a Piece>, new_piece: &'a Piece) -> &'a Piece {
        if let Some(current_piece) = current_piece {
            let should_replace =
                match vector {
                    Vector::Line(line) => {
                        match line {
                            LineVector::Top => {
                                new_piece.current_position().y() > current_piece.current_position().y()
                            },
                            LineVector::Bottom => {
                                new_piece.current_position().y() < current_piece.current_position().y()
                            },
                            LineVector::Left => {
                                new_piece.current_position().x() < current_piece.current_position().x()
                            },
                            LineVector::Right => {
                                new_piece.current_position().x() > current_piece.current_position().x()
                            },
                        }
                    },
                    Vector::Diagonal(diagonal) => {
                        match diagonal {
                            DiagonalVector::TopLeft => {
                                new_piece.current_position().x() < current_piece.current_position().x() &&
                                    new_piece.current_position().y() > current_piece.current_position().y()
                            },
                            DiagonalVector::TopRight => {
                                new_piece.current_position().x() > current_piece.current_position().x() &&
                                    new_piece.current_position().y() > current_piece.current_position().y()
                            },
                            DiagonalVector::BottomLeft => {
                                new_piece.current_position().x() < current_piece.current_position().x() &&
                                    new_piece.current_position().y() < current_piece.current_position().y()
                            },
                            DiagonalVector::BottomRight => {
                                new_piece.current_position().x() > current_piece.current_position().x() &&
                                    new_piece.current_position().y() < current_piece.current_position().y()
                            },
                        }
                    },
                    _ => panic!("Unsupported vector: {:?}", vector)
                };
            if should_replace {
                self.add_or_replace_pin_vector(vector, new_piece);
                return new_piece
            }
            current_piece
        } else {
            self.add_or_replace_pin_vector(vector, new_piece);
            new_piece
        }
    }

    pub fn add_pin(&mut self, pinned: &Piece, pinned_by: &Piece) {
        let data = self.x_ray_data.get_mut(pinned_by.id()).unwrap_or_else(|| {
            panic!(
                "Logical error: {:?} must already be recorded in this collection already!.",
                pinned_by
            )
        });
        data.pin = Some(*pinned.id());
    }

    pub fn pinned_piece(&self, pinned_by_id: &PieceId) -> Option<&PieceId> {
        if let Some(data) = self.x_ray_data.get(pinned_by_id) {
            data.pin.as_ref()
        } else {
            None
        }
    }

    pub fn pinned_pieces(&self) -> Vec<&PieceId> {
        self.x_ray_data.values().filter_map(|data| data.pin.as_ref()).collect::<Vec<_>>()
    }

    pub fn remove_piece(&mut self, piece_id: &PieceId) -> Option<PieceId> {
        if let Some(data) = self.x_ray_data.remove(piece_id) {
            self.direction_to_piece.remove(&data.direction)
        } else {
            None
        }
    }

    pub fn remove_pinned_piece(&mut self, vector: &Vector) {
        if let Some(pinned_by) = self.direction_to_piece.get(vector) {
            if let Some(data) = self.x_ray_data.get_mut(pinned_by) {
                data.pin = None;
            }
        }
    }

    pub fn pieces(&self) -> Vec<&PieceId> {
        self.x_ray_data.keys().collect()
    }

    pub fn pieces_owned(&self) -> Vec<PieceId> {
        self.x_ray_data.keys().map(|piece_id| *piece_id).collect::<Vec<_>>()
    }

    pub fn direction(&self, piece_id: &PieceId) -> Option<&Vector> {
        match self.x_ray_data.get(piece_id) {
            Some(data) => Some(&data.direction),
            None => None,
        }
    }

    pub fn piece_by_direction(&self, direction: &Vector) -> Option<&PieceId> {
        self.direction_to_piece.get(direction)
    }

    fn add_or_replace_pin_vector(&mut self, vector: &Vector, piece: &Piece) {
        if let Some(current_piece) = self.direction_to_piece.get(vector) {
            self.x_ray_data.remove(current_piece);
        };
        self.direction_to_piece.insert(*vector, *piece.id());
        if let Some(data) = self.x_ray_data.get_mut(piece.id()) {
            data.direction = *vector;
            data.pin = None;
        }
        self.x_ray_data.insert(*piece.id(), XRayData { direction: *vector, pin: None });
    }
}
