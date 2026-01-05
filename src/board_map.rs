use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::board_square::{BoardSquare};
use crate::color::Color;
use crate::piece::Piece;
use crate::point::Point;

pub struct BoardMap {
    point_to_board_square: FxHashMap<Point, BoardSquare>,
    active_white_pieces: FxHashSet<Rc<Piece>>,
    active_black_pieces: FxHashSet<Rc<Piece>>,
    white_king: Option<Rc<Piece>>,
    black_king: Option<Rc<Piece>>,
}

impl BoardMap {
    pub fn empty() -> Self {
        Self {
            point_to_board_square: FxHashMap::default(),
            active_white_pieces: FxHashSet::default(),
            active_black_pieces: FxHashSet::default(),
            white_king: None,
            black_king: None,
        }
    }

    pub fn board_square(&'_ self, point: &Point) -> &BoardSquare {
        self.point_to_board_square.get(point).unwrap_or(&BoardSquare::VoidSquare)
    }

    pub fn add_square(&mut self, point: Point, square: BoardSquare) {
        self.point_to_board_square.insert(point, square);
    }

    pub fn place_piece(&mut self, point: &Point, piece: &Rc<Piece>, add_to_active_pieces: bool) {
        let square =
            match self.point_to_board_square.get_mut(point) {
                Some(board_square) => {
                    match board_square {
                        BoardSquare::Square(square) => square,
                        BoardSquare::VoidSquare => {
                            panic!(
                                "Can't place {:?} piece onto void square at position {:?}",
                                piece, point
                            )
                        }
                    }
                },
                None => {
                    panic!("Point {:?} is out of bounds",point)
                }
            };

        square.set_piece_rc(&piece);
        piece.set_current_position(*point);
        if add_to_active_pieces {
            self.active_pieces_mut(piece.color()).insert(Rc::clone(piece));

            match &**piece {
                Piece::King(_) => {
                    match piece.color() {
                        Color::White => self.white_king = Some(Rc::clone(piece)),
                        Color::Black => self.black_king = Some(Rc::clone(piece)),
                    }

                },
                _ => ()
            }
        }
    }

    pub fn take_off_piece(&mut self, point: &Point, remove_from_active_pieces: bool) {
        let square =
            match self.point_to_board_square.get_mut(point) {
                Some(board_square) => {
                    match board_square {
                        BoardSquare::Square(square) => square,
                        BoardSquare::VoidSquare => {
                            panic!(
                                "Can't take off a piece from the void square at position: {:?}",
                                point
                            )
                        }
                    }
                },
                None => {
                    panic!("Point {:?} is out of bounds",point)
                }
            };

        if let Some(piece) = square.remove_piece() && remove_from_active_pieces {
            self.active_pieces_mut(piece.color()).remove(&piece);
            match &*piece {
                Piece::King(_) => {
                    match piece.color() {
                        Color::White => self.white_king = None,
                        Color::Black => self.black_king = None,
                    }
                },
                _ => ()
            }
        }
    }

    pub fn king(&self, color: &Color) -> Option<&Rc<Piece>> {
        match color {
            Color::White => self.white_king.as_ref(),
            Color::Black => self.black_king.as_ref(),
        }
    }

    pub fn active_pieces(&self, color: &Color) -> &FxHashSet<Rc<Piece>> {
        match color {
            Color::White => &self.active_white_pieces,
            Color::Black => &self.active_black_pieces,
        }
    }

    fn active_pieces_mut(&mut self, color: &Color) -> &mut FxHashSet<Rc<Piece>> {
        match color {
            Color::White => &mut self.active_white_pieces,
            Color::Black => &mut self.active_black_pieces,
        }
    }
}
