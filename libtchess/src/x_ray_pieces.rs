use std::rc::Rc;
use rustc_hash::{FxHashMap};
use crate::piece::Piece;
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector::Vector;

#[derive(Debug)]
struct XRayData {
    direction: Vector,
    pin: Option<Rc<Piece>>,
}

#[derive(Debug)]
pub struct XRayPieces {
    direction_to_piece: FxHashMap<Vector, Rc<Piece>>,
    x_ray_data: FxHashMap<Rc<Piece>, XRayData>,
}

impl XRayPieces {
    pub fn empty() -> Self {
        Self { direction_to_piece: FxHashMap::default(), x_ray_data: FxHashMap::default() }
    }

    pub fn add_x_ray_vector(&mut self, vector: &Vector, piece: &Rc<Piece>) -> Rc<Piece> {
        if let Some(existing_piece) = self.direction_to_piece.get(vector) {
            let should_replace =
                match vector {
                    Vector::Line(line) => {
                        match line {
                            LineVector::Top => {
                                piece.current_position().y() > existing_piece.current_position().y()
                            },
                            LineVector::Bottom => {
                                piece.current_position().y() < existing_piece.current_position().y()
                            },
                            LineVector::Left => {
                                piece.current_position().x() < existing_piece.current_position().x()
                            },
                            LineVector::Right => {
                                piece.current_position().x() > existing_piece.current_position().x()
                            },
                        }
                    },
                    Vector::Diagonal(diagonal) => {
                        match diagonal {
                            DiagonalVector::TopLeft => {
                                piece.current_position().x() < existing_piece.current_position().x() &&
                                    piece.current_position().y() > existing_piece.current_position().y()
                            },
                            DiagonalVector::TopRight => {
                                piece.current_position().x() > existing_piece.current_position().x() &&
                                    piece.current_position().y() > existing_piece.current_position().y()
                            },
                            DiagonalVector::BottomLeft => {
                                piece.current_position().x() < existing_piece.current_position().x() &&
                                    piece.current_position().y() < existing_piece.current_position().y()
                            },
                            DiagonalVector::BottomRight => {
                                piece.current_position().x() > existing_piece.current_position().x() &&
                                    piece.current_position().y() < existing_piece.current_position().y()
                            },
                        }
                    },
                    _ => panic!("Unsupported vector: {:?}", vector)
                };
            if should_replace {
                self.add_or_replace_pin_vector(vector, piece);
                return Rc::clone(piece)
            }
            Rc::clone(existing_piece)
        } else {
            self.add_or_replace_pin_vector(vector, piece);
            Rc::clone(piece)
        }
    }

    pub fn add_pin(&mut self, pinned: &Rc<Piece>, pinned_by: &Rc<Piece>) {
        let data = self.x_ray_data.get_mut(pinned_by).unwrap_or_else(|| {
            panic!(
                "Logical error: {:?} must already be recorded in this collection already!.",
                pinned_by
            )
        });
        data.pin = Some(Rc::clone(pinned));
    }

    pub fn pinned_piece(&self, pinned_by: &Rc<Piece>) -> Option<&Rc<Piece>> {
        if let Some(data) = self.x_ray_data.get(pinned_by) {
            data.pin.as_ref()
        } else {
            None
        }
    }

    pub fn pinned_pieces(&self) -> Vec<&Rc<Piece>> {
        self.x_ray_data.values().filter_map(|data| data.pin.as_ref()).collect::<Vec<_>>()
    }

    pub fn remove_piece(&mut self, piece: &Rc<Piece>) -> Option<Rc<Piece>> {
        if let Some(data) = self.x_ray_data.remove(piece) {
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

    pub fn pieces(&self) -> Vec<&Rc<Piece>> {
        self.x_ray_data.keys().collect()
    }

    pub fn pieces_owned(&self) -> Vec<Rc<Piece>> {
        self.x_ray_data.keys().map(|piece| Rc::clone(piece)).collect::<Vec<_>>()
    }

    pub fn direction(&self, piece: &Rc<Piece>) -> Option<&Vector> {
        match self.x_ray_data.get(piece) {
            Some(data) => Some(&data.direction),
            None => None,
        }
    }

    pub fn piece_by_direction(&self, direction: &Vector) -> Option<&Rc<Piece>> {
        self.direction_to_piece.get(direction)
    }

    fn add_or_replace_pin_vector(&mut self, vector: &Vector, piece: &Rc<Piece>) {
        if let Some(current_piece) = self.direction_to_piece.get(vector) {
            self.x_ray_data.remove(current_piece);
        };
        self.direction_to_piece.insert(*vector, Rc::clone(piece));
        if let Some(data) = self.x_ray_data.get_mut(piece) {
            data.direction = *vector;
            data.pin = None;
        }
        self.x_ray_data.insert(Rc::clone(piece), XRayData { direction: *vector, pin: None });
    }
}
