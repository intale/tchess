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
use crate::dimension::Dimension;
use crate::point_to_piece_association::{PointToPieceAssociation};
use crate::vector::line_vector::LineVector;
use crate::vector::Vector;
use crate::vector_points::VectorPoints;

// Invert colors of chess symbols so they look more meaningful in the terminal window with black
// background. Debugging purpose only.
pub const INVERT_COLORS: bool = true;

// Determines whether to render the board turned to white side. Setting it to false will render the
// board turned to black side. Debugging purpose only.
pub const WHITE_SIDE: bool = true;

// https://docs.rs/indexmap/latest/indexmap/
type BoardMap = IndexMap<Point, Cell, BuildHasherDefault<NoHashHasher<Point>>>;

pub struct Board {
    board: BoardMap,
    dimension: Dimension,
    white_attack_points: PointToPieceAssociation,
    black_attack_points: PointToPieceAssociation,
    white_moves: PointToPieceAssociation,
    black_moves: PointToPieceAssociation,
    white_x_ray_points: PointToPieceAssociation,
    black_x_ray_points: PointToPieceAssociation,
    white_defensive_points: PointToPieceAssociation,
    black_defensive_points: PointToPieceAssociation,
    pub white_king: Option<Rc<Piece>>,
    pub black_king: Option<Rc<Piece>>,
}

impl Board {
    pub fn classic_chess_board() -> Self {
        let mut board = Self::empty(
            Point::new(1, 1),
            Point::new(8, 8),
        );

        for y in board.get_dimension().get_rows_range() {
            for x in board.get_dimension().get_columns_range() {
                let color = {
                    if (x + y) % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                };
                let piece: Option<Rc<Piece>>;
                let point = Point::new(x, y);
                piece = match (y, x) {
                    // White pieces
                    (1, 1) | (1, 8) => Some(
                        Rc::new(
                            Piece::Rook(
                                Rook::new(
                                    Color::White, Some(vec![rook::Buff::Castle]), None, point, point
                                )
                            )
                        )
                    ),
                    (1, 2) | (1, 7) => Some(
                        Rc::new(
                            Piece::Knight(
                                Knight::new(Color::White, None, None, point, point)
                            )
                        )
                    ),
                    (1, 3) | (1, 6) => Some(
                        Rc::new(
                            Piece::Bishop(
                                Bishop::new(Color::White, None, None, point, point)
                            )
                        )
                    ),
                    (1, 4) => Some(
                        Rc::new(
                            Piece::Queen(
                                Queen::new(Color::White, None, None, point, point)
                            )
                        )
                    ),
                    (1, 5) => Some(
                        Rc::new(
                            Piece::King(
                                King::new(
                                    Color::White, Some(vec![king::Buff::Castle]), None, point, point
                                )
                            )
                        )
                    ),
                    (2, _) => Some(
                        Rc::new(
                            Piece::Pawn(Pawn::new(Color::White, None, None, point, point))
                        )
                    ),
                    // Black pieces
                    (8, 1) | (8, 8) => Some(
                        Rc::new(
                            Piece::Rook(
                                Rook::new(
                                    Color::Black, Some(vec![rook::Buff::Castle]), None, point, point
                                )
                            )
                        )
                    ),
                    (8, 2) | (8, 7) => Some(
                        Rc::new(
                            Piece::Knight(Knight::new(Color::Black, None, None, point, point))
                        )
                    ),
                    (8, 3) | (8, 6) => Some(
                        Rc::new(
                            Piece::Bishop(Bishop::new(Color::Black, None, None, point, point))
                        )
                    ),
                    (8, 5) => Some(
                        Rc::new(
                            Piece::King(
                                King::new(
                                    Color::Black, Some(vec![king::Buff::Castle]), None, point, point
                                )
                            )
                        )
                    ),
                    (8, 4) => Some(
                        Rc::new(
                            Piece::Queen(Queen::new(Color::Black, None, None, point, point))
                        )
                    ),
                    (7, _) => Some(
                        Rc::new(
                            Piece::Pawn(Pawn::new(Color::Black, None, None, point, point))
                        )
                    ),
                    _ => None
                };
                board.get_board_mut().insert(point, Cell::new(color, piece));
            }
        }

        board.white_king = board.get_board().get(&Point::new(1, 5)).unwrap().get_piece_rc();
        board.black_king = board.get_board().get(&Point::new(8, 5)).unwrap().get_piece_rc();
        board.calculate_attacks();
        board.calculate_defends();
        board
    }

    pub fn get_board(&self) -> &BoardMap {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut BoardMap {
        &mut self.board
    }

    pub fn get_dimension(&self) -> &Dimension {
        &self.dimension
    }
    
    pub fn get_white_attack_points(&self) -> &PointToPieceAssociation {
        &self.white_attack_points
    }

    pub fn get_black_attack_points(&self) -> &PointToPieceAssociation {
        &self.black_attack_points
    }

    pub fn get_attacked_points(&self, color: Color) -> &PointToPieceAssociation {
        match color {
            Color::White => self.get_black_attack_points(),
            Color::Black => self.get_white_attack_points(),
        }
    }

    pub fn get_white_defensive_points(&self) -> &PointToPieceAssociation {
        &self.white_defensive_points
    }

    pub fn get_black_defensive_points(&self) -> &PointToPieceAssociation {
        &self.black_defensive_points
    }

    pub fn empty(min_point: Point, max_point: Point) -> Self {
        Self {
            board: IndexMap::with_hasher(BuildHasherDefault::default()),
            white_king: None,
            black_king: None,
            dimension: Dimension::new(min_point, max_point),
            white_attack_points: PointToPieceAssociation::empty(Color::White),
            black_attack_points: PointToPieceAssociation::empty(Color::Black),
            white_moves: PointToPieceAssociation::empty(Color::White),
            black_moves: PointToPieceAssociation::empty(Color::Black),
            white_x_ray_points: PointToPieceAssociation::empty(Color::White),
            black_x_ray_points: PointToPieceAssociation::empty(Color::Black),
            white_defensive_points: PointToPieceAssociation::empty(Color::Black),
            black_defensive_points: PointToPieceAssociation::empty(Color::Black),
        }
    }

    fn calculate_attacks(&mut self) {
        for (point, cell) in &self.board {
            if let Some(piece) = cell.get_piece() {
                let attacks = piece.attack_points(self);
                for attack_point in attacks.into_iter() {
                    match piece.get_color() {
                        Color::White => self.white_attack_points.add_move(attack_point, piece),
                        Color::Black => self.black_attack_points.add_move(attack_point, piece),
                    };
                }
            }
        }
        ()
    }

    fn calculate_defends(&mut self) {
        for (point, cell) in &self.board {
            if let Some(piece) = cell.get_piece() {
                let defends = piece.defensive_points(self);
                for defend_point in defends.into_iter() {
                    match piece.get_color() {
                        Color::White => self.white_defensive_points.add_move(defend_point, piece),
                        Color::Black => self.black_defensive_points.add_move(defend_point, piece),
                    };
                }
            }
        }
        ()
    }

    fn calculate_x_ray_points(&mut self) {
        // white_xray_points
        if let Some(king) = &self.white_king {

        }
        todo!()
    }

    pub fn is_empty_cell(&self, point: &Point) -> bool {
        self.get_cell(point).get_piece().is_none()
    }

    pub fn is_enemy_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.get_cell(point).get_piece() {
            return piece.get_color() != color;
        }
        false
    }

    pub fn is_ally_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.get_cell(point).get_piece() {
            return piece.get_color() == color;
        }
        false
    }

    pub fn get_cell(&self, point: &Point) -> &Cell {
        self.board.get(point).unwrap()
    }
}


impl PrettyPrint for Board {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut buf: Vec<String> = vec![];
        for (point, cell) in &self.board {
            if point.get_x() == self.dimension.get_min_point().get_x() {
                output.push_str(point.get_y().pp().as_str());
                output.push_str(" ");
            }
            output.push_str(cell.pp().as_str());
            output.push(' ');
            if point.get_x() == self.dimension.get_max_point().get_x() {
                output.push_str("\n");
                buf.push(output.clone());
                output = String::new();
            }
        }
        output.push_str("  ");

        let vector_points = VectorPoints::new(
            Point::new(
                *self.dimension.get_min_point().get_x().get_value() - 1,
                *self.dimension.get_max_point().get_y().get_value()
            ),
            self.dimension
        );
        for point in vector_points.calc_points(Vector::Line(LineVector::Right), |_| true, |_| false) {
            output.push_str(" ");
            output.push_str(point.get_x().pp().as_str());
            output.push_str("  ");
        }
        if(WHITE_SIDE){
            buf = buf.into_iter().rev().collect();
        }
        buf.push(output);
        buf.join("")
    }
}
