use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::board_square::{BoardSquare};
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::piece::Piece;
use crate::point::Point;

pub struct BoardMap {
    point_to_board_square: FxHashMap<Point, BoardSquare>,
    active_pieces: ColoredProperty<FxHashSet<Rc<Piece>>>,
    king: ColoredProperty<Option<Rc<Piece>>>,
}

impl BoardMap {
    pub fn empty() -> Self {
        Self {
            point_to_board_square: FxHashMap::default(),
            active_pieces: ColoredProperty([FxHashSet::default(), FxHashSet::default()]),
            king: ColoredProperty([None, None]),
        }
    }

    pub fn board_square(&self, point: &Point) -> &BoardSquare {
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
            self.active_pieces[piece.color()].insert(Rc::clone(piece));

            match &**piece {
                Piece::King(_) => {
                    self.king[piece.color()] = Some(Rc::clone(piece));
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
            self.active_pieces[piece.color()].remove(&piece);
            match &*piece {
                Piece::King(_) => {
                    self.king[piece.color()] = None;
                },
                _ => ()
            }
        }
    }

    pub fn king(&self, color: &Color) -> Option<&Rc<Piece>> {
        self.king[color].as_ref()
    }

    pub fn active_pieces(&self, color: &Color) -> &FxHashSet<Rc<Piece>> {
        &self.active_pieces[color]
    }
}
