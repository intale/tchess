use im_rc::HashMap;
use crate::board_square::{BoardSquare};
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::piece::Piece;
use crate::piece_id::PieceId;
use crate::point::Point;
use rustc_hash::{FxBuildHasher};
use crate::square::Square;

#[derive(Clone)]
pub struct BoardMap {
    point_to_board_square: HashMap<Point, BoardSquare, FxBuildHasher>,
    active_pieces: ColoredProperty<HashMap<PieceId, Piece, FxBuildHasher>>,
    king: ColoredProperty<Option<PieceId>>,
}

impl BoardMap {
    pub fn empty() -> Self {
        Self {
            point_to_board_square: HashMap::default(),
            active_pieces: ColoredProperty([HashMap::default(), HashMap::default()]),
            king: ColoredProperty([None, None]),
        }
    }

    pub fn piece_at(&self, point: &Point) -> Option<&Piece> {
        if let Some(piece_id) = self.piece_id_at(point) {
            return self.maybe_find_piece_by_id(piece_id);
        }
        None
    }

    pub fn piece_id_at(&self, point: &Point) -> Option<&PieceId> {
        match self.point_to_board_square.get(point) {
            Some(square) => square.get_piece_id(),
            None => None,
        }
    }

    pub fn board_square(&self, point: &Point) -> &BoardSquare {
        self.point_to_board_square
            .get(point)
            .unwrap_or(&BoardSquare::VoidSquare)
    }

    pub fn add_square(&mut self, point: Point, square: BoardSquare) {
        self.point_to_board_square.insert(point, square);
    }

    pub fn add_piece(&mut self, mut piece: Piece, point: Point) {
        let square = Self::get_square_mut(&point, &mut self.point_to_board_square);

        square.set_piece_id(piece.id());
        piece.set_current_position(point);

        match piece {
            Piece::King(_) => {
                self.king[piece.color()] = Some(*piece.id());
            }
            _ => (),
        }
        self.active_pieces[piece.color()].insert(*piece.id(), piece);
    }

    pub fn remove_piece(&mut self, piece_id: &PieceId) -> Piece {
        let piece = self.active_pieces[&piece_id.color()]
            .remove(piece_id)
            .expect(format!("Logical error: could not remove piece by {} id", piece_id).as_str());
        Self::get_square_mut(&piece.current_position(), &mut self.point_to_board_square)
            .remove_piece_id();

        match piece {
            Piece::King(_) => self.king[&piece_id.color()] = None,
            _ => (),
        }
        piece
    }

    pub fn change_piece_position(&mut self, to_point: &Point, piece_id: &PieceId) -> Point {
        let piece = self.active_pieces[&piece_id.color()]
            .get_mut(piece_id)
            .expect(format!("Logical error: could not find piece by {} id", piece_id).as_str());
        Self::get_square_mut(&piece.current_position(), &mut self.point_to_board_square)
            .remove_piece_id();
        Self::get_square_mut(to_point, &mut self.point_to_board_square).set_piece_id(piece_id);
        let old_position = *piece.current_position();
        piece.set_current_position(*to_point);
        old_position
    }

    fn get_square_mut<'a>(
        point: &Point,
        point_to_board_square: &'a mut HashMap<Point, BoardSquare, FxBuildHasher>,
    ) -> &'a mut Square {
        match point_to_board_square.get_mut(&point) {
            Some(board_square) => match board_square {
                BoardSquare::Square(square) => square,
                BoardSquare::VoidSquare => {
                    panic!("Can't mutate void square at {:?} position", point)
                }
            },
            None => {
                panic!("Point {:?} is out of bounds", point)
            }
        }
    }

    pub fn king(&self, color: &Color) -> Option<&Piece> {
        if let Some(king_id) = self.king[color] {
            return self.maybe_find_piece_by_id(&king_id);
        }
        None
    }

    pub fn king_id(&self, color: &Color) -> Option<&PieceId> {
        self.king[color].as_ref()
    }

    pub fn active_pieces(&self, color: &Color) -> &HashMap<PieceId, Piece, FxBuildHasher> {
        &self.active_pieces[color]
    }

    pub fn find_piece_by_id(&self, piece_id: &PieceId) -> &Piece {
        self.maybe_find_piece_by_id(piece_id)
            .expect(format!("Couldn't find piece by id {}", piece_id).as_str())
    }

    pub fn find_piece_by_id_mut(&mut self, piece_id: &PieceId) -> &mut Piece {
        self.maybe_find_piece_by_id_mut(piece_id)
            .expect(format!("Couldn't find piece by id {}", piece_id).as_str())
    }

    pub fn maybe_find_piece_by_id(&self, piece_id: &PieceId) -> Option<&Piece> {
        self.active_pieces[&piece_id.color()].get(piece_id)
    }

    pub fn maybe_find_piece_by_id_mut(&mut self, piece_id: &PieceId) -> Option<&mut Piece> {
        self.active_pieces[&piece_id.color()].get_mut(piece_id)
    }
}
