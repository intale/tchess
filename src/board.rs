use std::cmp::PartialEq;
use std::hash::{BuildHasherDefault};
use std::rc::Rc;
use nohash_hasher::NoHashHasher;

use crate::color::Color;
use crate::pieces::{bishop, king, knight, pawn, queen, rook, Piece, PieceInit};
use crate::pieces::{
    bishop::Bishop,
    king::King,
    knight::Knight,
    pawn::Pawn,
    queen::Queen,
    rook::Rook,
};
use crate::utils::pretty_print::PrettyPrint;
use crate::point::{Point};
use crate::cell::{Cell};
use indexmap::{IndexMap};
use crate::moves_map::MovesMap;

// Invert colors of chess symbols so they look more meaningful in the terminal window with black
// background. Debugging purpose only.
pub const INVERT_COLORS: bool = true;

// Determines whether to render the board turned to white side. Setting it to false will render the
// board turned to black side. Debugging purpose only.
pub const WHITE_SIDE: bool = true;

// https://docs.rs/indexmap/latest/indexmap/
type BoardMap = IndexMap<Point, Cell, BuildHasherDefault<NoHashHasher<Point>>>;

pub struct BoardDimension {
    columns: u8,
    rows: u8,
}

impl BoardDimension {
    fn new(columns: u8, rows: u8) -> Self {
        // TODO: implement constraints
        Self { columns, rows }
    }

    fn get_rows_num(&self) -> u8 {
        self.rows
    }

    fn get_columns_num(&self) -> u8 {
        self.columns
    }
}

pub struct Board {
    board: BoardMap,
    dimension: BoardDimension,
    white_attack_moves: MovesMap,
    black_attack_moves: MovesMap,
    white_moves: MovesMap,
    black_moves: MovesMap,
    white_defensive_moves: MovesMap,
    black_defensive_moves: MovesMap,
    pub white_king: Option<Rc<Piece>>,
    pub black_king: Option<Rc<Piece>>,
}

impl Board {
    pub fn classic_chess_board() -> Self {
        let mut board = Self::empty(8, 8);

        for y in 0..board.get_dimension().get_rows_num() {
            for x in 0..board.get_dimension().get_columns_num() {
                let color = {
                    if (x + y) % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                };
                let piece: Option<Rc<Piece>>;
                let point = Point::new(x as i16, y as i16);
                piece = match (y, x) {
                    // White pieces
                    (0, 0) | (0, 7) => Some(
                        Rc::new(Piece::Rook(Rook::new(Color::White, Some(vec![rook::Buff::Castle]), None, point)))
                    ),
                    (0, 1) | (0, 6) => Some(Rc::new(Piece::Knight(Knight::new(Color::White, None, None, point)))),
                    (0, 2) | (0, 5) => Some(Rc::new(Piece::Bishop(Bishop::new(Color::White, None, None, point)))),
                    (0, 3) => Some(Rc::new(Piece::Queen(Queen::new(Color::White, None, None, point)))),
                    (0, 4) => Some(Rc::new(Piece::King(King::new(Color::White, Some(vec![king::Buff::Castle]), None, point)))),
                    (1, _) => Some(Rc::new(Piece::Pawn(Pawn::new(Color::White, None, None, point)))),
                    // Black pieces
                    (7, 0) | (7, 7) => Some(
                        Rc::new(Piece::Rook(Rook::new(Color::Black, Some(vec![rook::Buff::Castle]), None, point)))
                    ),
                    (7, 1) | (7, 6) => Some(Rc::new(Piece::Knight(Knight::new(Color::Black, None, None, point)))),
                    (7, 2) | (7, 5) => Some(Rc::new(Piece::Bishop(Bishop::new(Color::Black, None, None, point)))),
                    (7, 4) => Some(Rc::new(Piece::King(King::new(Color::Black, Some(vec![king::Buff::Castle]), None, point)))),
                    (7, 3) => Some(Rc::new(Piece::Queen(Queen::new(Color::Black, None, None, point)))),
                    (6, _) => Some(Rc::new(Piece::Pawn(Pawn::new(Color::Black, None, None, point)))),
                    _ => None
                };
                board.get_board_mut().insert(point, Cell::new(color, piece));
            }
        }

        board.white_king = board.get_board().get(&Point::new(0, 4)).unwrap().get_piece_rc();
        board.black_king = board.get_board().get(&Point::new(7, 4)).unwrap().get_piece_rc();
        board.calculate_attacks();
        board
    }

    pub fn get_board(&self) -> &BoardMap {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut BoardMap {
        &mut self.board
    }

    pub fn get_dimension_mut(&mut self) -> &mut BoardDimension {
        &mut self.dimension
    }

    pub fn get_dimension(&self) -> &BoardDimension {
        &self.dimension
    }
    
    pub fn get_white_attack_moves(&self) -> &MovesMap {
        &self.white_attack_moves
    }

    pub fn get_black_attack_moves(&self) -> &MovesMap {
        &self.black_attack_moves
    }

    pub fn empty(columns: u8, rows: u8) -> Self {
        Self {
            board: IndexMap::with_hasher(BuildHasherDefault::default()),
            white_king: None,
            black_king: None,
            dimension: BoardDimension::new(columns, rows),
            white_attack_moves: MovesMap::empty(Color::White),
            black_attack_moves: MovesMap::empty(Color::Black),
            white_moves: MovesMap::empty(Color::White),
            black_moves: MovesMap::empty(Color::Black),
            white_defensive_moves: MovesMap::empty(Color::Black),
            black_defensive_moves: MovesMap::empty(Color::Black),
        }
    }

    fn calculate_attacks(&mut self) {
        for (point, cell) in &self.board {
            if let Some(piece) = cell.get_piece() {
                let attacks = piece.attack_points(self, point);
                for attack_point in attacks.into_iter() {
                    match piece.get_color() {
                        Color::White => self.white_attack_moves.add_move(attack_point, piece),
                        Color::Black => self.black_attack_moves.add_move(attack_point, piece),
                    };
                }
            }
        }
        ()
    }

    pub fn is_in_boundaries(&self, point: &Point) -> bool {
        let point_x = point.get_x().get_value();
        let point_y = point.get_y().get_value();
        let columns = self.dimension.get_columns_num() as i16;
        let rows = self.dimension.get_rows_num() as i16;
        point_x >= 0 && point_x < columns && point_y >= 0 && point_y < rows
    }

    pub fn is_empty_cell(&self, point: &Point) -> bool {
        self.board.get(point).unwrap().get_piece().is_none()
    }

    pub fn is_enemy_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.board.get(point).unwrap().get_piece() {
            return piece.get_color() != color;
        }
        false
    }
}

impl PrettyPrint for Board {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut buf: Vec<String> = vec![];
        for (point, cell) in &self.board {
            if point.get_x() == 0i16 {
                output.push_str(point.get_y().pp().as_str());
                output.push_str(" ");
            }
            output.push_str(cell.pp().as_str());
            output.push(' ');
            if point.get_x() + 1i16 == self.dimension.get_columns_num() as i16 {
                output.push_str("\n");
                buf.push(output.clone());
                output = String::new();
            }
        }
        output.push_str("  ");
        let slice = self.board.get_range(
            ((self.dimension.get_rows_num() - 1u8) * self.dimension.get_columns_num()) as usize..
                (self.dimension.get_rows_num() * self.dimension.get_columns_num()) as usize
        );
        if let Some(slice) = slice {
            for (point, cell) in slice {
                output.push_str(" ");
                output.push_str(point.get_x().pp().as_str());
                output.push_str("  ");
            }
        }
        if(WHITE_SIDE){
            buf = buf.into_iter().rev().collect();
        }
        buf.push(output);
        buf.join("")
    }
}
