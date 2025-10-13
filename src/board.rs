use std::cmp::PartialEq;
use std::hash::{BuildHasherDefault};
use std::rc::Rc;
use nohash_hasher::NoHashHasher;

use crate::color::Color;
use crate::pieces::{Piece, PieceInit};
use crate::utils::pretty_print::PrettyPrint;
use crate::point::{Point};
use crate::board_cell::{BoardCell};
use indexmap::{IndexMap};
use crate::buff::Buff;
use crate::debuff::Debuff;
use crate::dimension::Dimension;
use crate::point_to_piece_association::{PointToPieceAssociation};
use crate::vector::diagonal_vector::DiagonalVector;
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
type BoardMap = IndexMap<Point, BoardCell, BuildHasherDefault<NoHashHasher<Point>>>;

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
    white_king: Option<Rc<Piece>>,
    black_king: Option<Rc<Piece>>,
    next_piece_id: usize,
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
                let point = Point::new(x, y);
                match (y, x) {
                    // White pieces
                    (1, 1) | (1, 8) => board.add_piece(
                        "Rook", Color::White, vec![Buff::Castle], vec![], point
                    ),
                    (1, 2) | (1, 7) => board.add_piece(
                        "Knight", Color::White, vec![], vec![], point
                    ),
                    (1, 3) | (1, 6) => board.add_piece(
                        "Bishop", Color::White, vec![], vec![], point
                    ),
                    (1, 4) => board.add_piece(
                        "Queen", Color::White, vec![], vec![], point
                    ),
                    (1, 5) => board.add_piece(
                        "King", Color::White, vec![Buff::Castle], vec![], point
                    ),
                    (2, _) => board.add_piece(
                        "Pawn", Color::White, vec![], vec![], point
                    ),
                    // Black pieces
                    (8, 1) | (8, 8) => board.add_piece(
                        "Rook", Color::Black, vec![Buff::Castle], vec![], point
                    ),
                    (8, 2) | (8, 7) => board.add_piece(
                        "Knight", Color::Black, vec![], vec![], point
                    ),
                    (8, 3) | (8, 6) => board.add_piece(
                        "Bishop", Color::Black, vec![], vec![], point
                    ),
                    (8, 5) => board.add_piece(
                        "King", Color::Black, vec![Buff::Castle], vec![], point
                    ),
                    (8, 4) => board.add_piece(
                        "Queen", Color::Black, vec![], vec![], point
                    ),
                    (7, _) => board.add_piece(
                        "Pawn", Color::Black, vec![], vec![], point
                    ),
                    _ => ()
                };
            }
        }
        board.white_king = board.get_board().get(&Point::new(5, 1)).unwrap().get_piece_rc();
        board.black_king = board.get_board().get(&Point::new(5, 8)).unwrap().get_piece_rc();
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

    pub fn get_white_attack_points_mut(&mut self) -> &mut PointToPieceAssociation {
        &mut self.white_attack_points
    }

    pub fn get_black_attack_points_mut(&mut self) -> &mut PointToPieceAssociation {
        &mut self.black_attack_points
    }


    pub fn get_attacked_points(&self, color: &Color) -> &PointToPieceAssociation {
        match color {
            Color::White => self.get_black_attack_points(),
            Color::Black => self.get_white_attack_points(),
        }
    }

    pub fn get_attacked_points_mut(&mut self, color: &Color) -> &mut PointToPieceAssociation {
        match color {
            Color::White => self.get_black_attack_points_mut(),
            Color::Black => self.get_white_attack_points_mut(),
        }
    }

    pub fn get_attack_points(&self, color: &Color) -> &PointToPieceAssociation {
        match color {
            Color::White => self.get_white_attack_points(),
            Color::Black => self.get_black_attack_points(),
        }
    }

    pub fn get_attack_points_mut(&mut self, color: &Color) -> &mut PointToPieceAssociation {
        match color {
            Color::White => self.get_white_attack_points_mut(),
            Color::Black => self.get_black_attack_points_mut(),
        }
    }

    pub fn get_white_defensive_points(&self) -> &PointToPieceAssociation {
        &self.white_defensive_points
    }

    pub fn get_black_defensive_points(&self) -> &PointToPieceAssociation {
        &self.black_defensive_points
    }

    pub fn get_white_defensive_points_mut(&mut self) -> &mut PointToPieceAssociation {
        &mut self.white_defensive_points
    }

    pub fn get_black_defensive_points_mut(&mut self) -> &mut PointToPieceAssociation {
        &mut self.black_defensive_points
    }

    pub fn get_defensive_points(&self, color: &Color) -> &PointToPieceAssociation {
        match color {
            Color::White => self.get_white_defensive_points(),
            Color::Black => self.get_black_defensive_points(),
        }
    }

    pub fn get_defensive_points_mut(&mut self, color: &Color) -> &mut PointToPieceAssociation {
        match color {
            Color::White => self.get_white_defensive_points_mut(),
            Color::Black => self.get_black_defensive_points_mut(),
        }
    }

    pub fn empty(min_point: Point, max_point: Point) -> Self {
        let mut board = Self {
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
            next_piece_id: 0,
        };
        for y in board.get_dimension().get_rows_range() {
            for x in board.get_dimension().get_columns_range() {
                let color = {
                    if (x + y) % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                };
                let point = Point::new(x, y);
                board.get_board_mut().insert(point, BoardCell::new(color, None));
            }
        }
        board
    }

    fn pieces_to_recalculate(&mut self, point: &Point) -> Vec<Rc<Piece>> {
        let mut pieces =
            self.white_attack_points.get_pieces_mut(&point).iter().collect::<Vec<_>>();
        pieces.append(
            &mut self.black_attack_points.get_pieces_mut(&point).iter().collect::<Vec<_>>()
        );
        pieces.into_iter().map(|piece| Rc::clone(piece)).collect::<Vec<_>>()
    }

    fn calculate_attacks_for(&mut self, piece: &Rc<Piece>) {
        self.get_attack_points_mut(&piece.get_color()).clear_moves(piece);

        let attacks = piece.attack_points(self);
        for attack_point in attacks.into_iter() {
            self.get_attack_points_mut(&piece.get_color()).add_move(attack_point, piece);
        }
    }

    fn calculate_defends_for(&mut self, piece: &Rc<Piece>) {
        self.get_defensive_points_mut(&piece.get_color()).clear_moves(piece);

        let defends = piece.defensive_points(self);
        for defend_point in defends.into_iter() {
            self.get_defensive_points_mut(&piece.get_color()).add_move(defend_point, piece);
        }
    }

    fn calculate_pins_for(&self, piece: &Rc<Piece>) {
        for pinned_by in self.get_attacked_points(&piece.get_color()).get_x_ray_pieces() {
            self.add_pins(piece, pinned_by)
        }
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

    pub fn get_cell(&self, point: &Point) -> &BoardCell {
        self.board.get(point).unwrap()
    }

    pub fn add_pins(&self, pin_to: &Rc<Piece>, pinned_by: &Rc<Piece>) {
        let points = self.get_attacked_points(&pin_to.get_color()).get_points(pinned_by);
        if let Some(points) = points {
            if points.contains(&pin_to.get_current_position()) {
                // No need to calculate pinned pieces, because pin_to piece is directly attacked by the
                // given pinned_by piece
                return;
            }
        }

        let enemy_color = pinned_by.get_color();
        let x_ray_direction =
            match &**pinned_by {
                Piece::Bishop(_) => {
                    if let Some(vector) = DiagonalVector::calc_direction(
                        &pinned_by.get_current_position(), &pin_to.get_current_position()
                    ) {
                        Some(Vector::Diagonal(vector))
                    } else {
                        None
                    }
                },
                Piece::Rook(_) => {
                    if let Some(vector) = LineVector::calc_direction(
                        &pinned_by.get_current_position(), &pin_to.get_current_position()
                    ) {
                        Some(Vector::Line(vector))
                    } else {
                        None
                    }
                },
                Piece::Queen(_) => {
                    if let Some(vector) = DiagonalVector::calc_direction(
                        &pinned_by.get_current_position(), &pin_to.get_current_position()
                    ) {
                        Some(Vector::Diagonal(vector))
                    } else if let Some(vector) = LineVector::calc_direction(
                        &pinned_by.get_current_position(), &pin_to.get_current_position()
                    ) {
                        Some(Vector::Line(vector))
                    } else {
                        None
                    }
                },
                _ => None,
            };

        match x_ray_direction {
            Some(direction) => {
                let mut current_piece_on_the_way: Option<&Rc<Piece>> = None;
                let vector_points = VectorPoints::without_initial(
                    pinned_by.get_current_position(),
                    *self.get_dimension(),
                    direction
                );

                for point in vector_points {
                    if let Some(piece) = self.get_cell(&point).get_piece() {
                        // Enemy piece meets his ally
                        if piece.get_color() == &enemy_color {
                            break
                        }
                        match current_piece_on_the_way {
                            Some(p) => {
                                if piece == pin_to {
                                    // Current piece meets itself. We have a bound!
                                    Rc::get_mut(&mut Rc::clone(p)).unwrap().add_debuff(
                                        Debuff::Pin(direction.reverse())
                                    );
                                }
                                break
                            },
                            None => { current_piece_on_the_way = Some(piece) }
                        }
                    }
                }
                ()
            }
            None => ()
        }
    }

    pub fn get_next_piece_id(&mut self) -> usize {
        self.next_piece_id += 1;
        self.next_piece_id
    }

    pub fn add_piece(&mut self, name: &str, color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                     position: Point) {
        let piece = Rc::new(
            Piece::init_piece_by_name(
                name, color, buffs, debuffs, position, self.get_next_piece_id()
            )
        );
        let cell = &mut self.get_board_mut().get_mut(&position).unwrap();
        cell.set_piece_rc(&piece);
        match &*piece {
            Piece::King(_) => self.set_king(&position),
            _ => (),
        }
        let mut pieces_to_recalculate = self.pieces_to_recalculate(&position);
        pieces_to_recalculate.push(piece);
        for piece in pieces_to_recalculate {
            self.calculate_attacks_for(&piece);
            self.calculate_defends_for(&piece);
        }
        if let Some(white_king) = &self.white_king {
            self.calculate_pins_for(&white_king);
        }
        if let Some(black_king) = &self.black_king {
            self.calculate_pins_for(&black_king);
        }
    }

    pub fn set_king(&mut self, position: &Point) {
        let cell = self.get_board().get(position).unwrap();
        match cell.get_piece() {
            Some(p) => {
                match &**p {
                    Piece::King(_) => {
                        match p.get_color() {
                            Color::White => {
                                self.white_king = cell.get_piece_rc();
                                self.calculate_pins_for(&self.white_king.as_ref().unwrap());
                            },
                            Color::Black => {
                                self.black_king = cell.get_piece_rc();
                                self.calculate_pins_for(&self.black_king.as_ref().unwrap());
                            },
                        }
                    },
                    _ => panic!("Can't assign {} as {:?} king!", p.pp(), p.get_color())
                }
                ()
            }
            _ => ()
        }
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

        let vector_points = VectorPoints::with_initial(
            Point::new(
                *self.dimension.get_min_point().get_x().get_value(),
                *self.dimension.get_max_point().get_y().get_value()
            ),
            self.dimension,
            Vector::Line(LineVector::Right)
        );
        for point in vector_points {
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

#[cfg(test)]
pub mod test_util {
    use crate::buff::Buff;
    use crate::debuff::Debuff;

    pub fn compare_buffs(vec1: &Vec<Buff>, vec2: &Vec<Buff>) {
        let lh_rest = vec1.iter().filter(|buff| vec2.contains(buff)).collect::<Vec<_>>();
        let rh_rest = vec2.iter().filter(|buff| vec1.contains(buff)).collect::<Vec<_>>();
        if lh_rest.len() > 0 && rh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Missing elements: {lh_rest:?}. Extra elements: {rh_rest:?}.")
        }
        if lh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Missing elements: {lh_rest:?}.")
        }
        if rh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Extra elements: {rh_rest:?}.")
        }
    }

    pub fn compare_debuffs(vec1: &Vec<Debuff>, vec2: &Vec<Debuff>) {
        let lh_rest = vec1.iter().filter(|debuff| vec2.contains(debuff)).collect::<Vec<_>>();
        let rh_rest = vec2.iter().filter(|debuff| vec1.contains(debuff)).collect::<Vec<_>>();
        if lh_rest.len() > 0 && rh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Missing elements: {lh_rest:?}. Extra elements: {rh_rest:?}.")
        }
        if lh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Missing elements: {lh_rest:?}.")
        }
        if rh_rest.len() > 0 {
            panic!("Expected {vec2:?} to match {vec1:?}. Extra elements: {rh_rest:?}.")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::test_util::*;
    use super::*;

    #[test]
    fn test_white_pin_points() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
        board.add_piece(
            "King", Color::Black, vec![], vec![], Point::new(4, 6)
        );
        board.add_piece(
            "Knight", Color::Black, vec![], vec![], Point::new(4, 5)
        );
        board.add_piece(
            "Queen", Color::White, vec![], vec![], Point::new(4, 2)
        );

        let knight = board.get_cell(&Point::new(4, 5)).get_piece().as_ref().unwrap();
        println!("{}", board.pp());
        println!("{:?}", knight.debuffs());
        todo!();
        compare_debuffs(&knight.debuffs(), &vec![])
    }
}
