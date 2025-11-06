use std::hash::{BuildHasherDefault};
use std::rc::Rc;

use crate::color::Color;
use crate::pieces::{Piece};
use crate::utils::pretty_print::PrettyPrint;
use crate::point::{Point};
use crate::board_cell::{BoardCell};
use rustc_hash::{FxHashMap, FxHashSet};
use crate::buff::Buff;
use crate::debuff::Debuff;
use crate::dimension::Dimension;
use crate::moves_map::MovesMap;
use crate::piece_move::PieceMove;
use crate::pins_map::PinsMap;
use crate::point_to_piece_association::{PieceHashSetT, PointToPieceAssociation};
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
type BoardMap = FxHashMap<Point, BoardCell>;

pub struct Board {
    board: BoardMap,
    dimension: Dimension,
    white_attack_points: PointToPieceAssociation,
    black_attack_points: PointToPieceAssociation,
    white_x_ray_pieces: PieceHashSetT,
    black_x_ray_pieces: PieceHashSetT,
    white_defensive_points: PointToPieceAssociation,
    black_defensive_points: PointToPieceAssociation,
    white_moves: MovesMap,
    black_moves: MovesMap,
    white_pins: PinsMap,
    black_pins: PinsMap,
    white_king: Option<Rc<Piece>>,
    black_king: Option<Rc<Piece>>,
    next_piece_id: usize,
    current_turn: Color,
}

impl Board {
    pub fn classic_chess_board() -> Self {
        let mut board = Self::empty(
            Point::new(1, 1),
            Point::new(8, 8),
        );

        for y in board.get_dimension().get_rows_range() {
            for x in board.get_dimension().get_columns_range() {
                let point = Point::new(x, y);
                match (y, x) {
                    // White pieces
                    (1, 1) | (1, 8) => {
                        board.add_piece(
                            "Rook", Color::White, vec![Buff::Castle], vec![], point,
                        );
                            ()
                    },
                    (1, 2) | (1, 7) => {
                        board.add_piece(
                            "Knight", Color::White, vec![], vec![], point,
                        );
                        ()
                    },
                    (1, 3) | (1, 6) => {
                        board.add_piece(
                            "Bishop", Color::White, vec![], vec![], point,
                        );
                        ()
                    },
                    (1, 4) => {
                        board.add_piece(
                            "Queen", Color::White, vec![], vec![], point,
                        );
                        ()
                    },
                    (1, 5) => {
                        board.add_piece(
                            "King", Color::White, vec![Buff::Castle], vec![], point,
                        );
                        ()
                    },
                    (2, _) => {
                        board.add_piece(
                            "Pawn", Color::White, vec![Buff::AdditionalPoint], vec![], point,
                        );
                        ()
                    },
                    // Black pieces
                    (8, 1) | (8, 8) => {
                        board.add_piece(
                            "Rook", Color::Black, vec![Buff::Castle], vec![], point,
                        );
                        ()
                    },
                    (8, 2) | (8, 7) => {
                        board.add_piece(
                            "Knight", Color::Black, vec![], vec![], point,
                        );
                        ()
                    },
                    (8, 3) | (8, 6) => {
                        board.add_piece(
                            "Bishop", Color::Black, vec![], vec![], point,
                        );
                        ()
                    },
                    (8, 5) => {
                        board.add_piece(
                            "King", Color::Black, vec![Buff::Castle], vec![], point,
                        );
                        ()
                    },
                    (8, 4) => {
                        board.add_piece(
                            "Queen", Color::Black, vec![], vec![], point,
                        );
                        ()
                    },
                    (7, _) => {
                        board.add_piece(
                            "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], point,
                        );
                        ()
                    },
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

    pub fn attack_points(&self, color: &Color) -> &PointToPieceAssociation {
        match color {
            Color::White => &self.white_attack_points,
            Color::Black => &self.black_attack_points,
        }
    }

    fn attack_points_mut(&mut self, color: &Color) -> &mut PointToPieceAssociation {
        match color {
            Color::White => &mut self.white_attack_points,
            Color::Black => &mut self.black_attack_points,
        }
    }

    pub fn defensive_points(&self, color: &Color) -> &PointToPieceAssociation {
        match color {
            Color::White => &self.white_defensive_points,
            Color::Black => &self.black_defensive_points,
        }
    }

    fn defensive_points_mut(&mut self, color: &Color) -> &mut PointToPieceAssociation {
        match color {
            Color::White => &mut self.white_defensive_points,
            Color::Black => &mut self.black_defensive_points,
        }
    }

    fn x_ray_pieces(&self, color: &Color) -> &PieceHashSetT {
        match color {
            Color::White => &self.white_x_ray_pieces,
            Color::Black => &self.black_x_ray_pieces,
        }
    }

    fn x_ray_pieces_mut(&mut self, color: &Color) -> &mut PieceHashSetT {
        match color {
            Color::White => &mut self.white_x_ray_pieces,
            Color::Black => &mut self.black_x_ray_pieces,
        }
    }

    pub fn moves(&self, color: &Color) -> &MovesMap {
        match color {
            Color::White => &self.white_moves,
            Color::Black => &self.black_moves,
        }
    }

    fn moves_mut(&mut self, color: &Color) -> &mut MovesMap {
        match color {
            Color::White => &mut self.white_moves,
            Color::Black => &mut self.black_moves,
        }
    }

    pub fn pins(&self, color: &Color) -> &PinsMap {
        match color {
            Color::White => &self.white_pins,
            Color::Black => &self.black_pins,
        }
    }

    fn pins_mut(&mut self, color: &Color) -> &mut PinsMap {
        match color {
            Color::White => &mut self.white_pins,
            Color::Black => &mut self.black_pins,
        }
    }

    pub fn king(&self, color: &Color) -> Option<&Rc<Piece>> {
        match color {
            Color::White => self.white_king.as_ref(),
            Color::Black => self.black_king.as_ref(),
        }
    }

    fn cell_mut(&mut self, point: &Point) -> &mut BoardCell {
        self.get_board_mut().get_mut(point).unwrap()
    }

    pub fn empty(min_point: Point, max_point: Point) -> Self {
        let mut board = Self {
            board: FxHashMap::default(),
            white_king: None,
            black_king: None,
            dimension: Dimension::new(min_point, max_point),
            white_attack_points: PointToPieceAssociation::empty(),
            black_attack_points: PointToPieceAssociation::empty(),
            white_x_ray_pieces: FxHashSet::default(),
            black_x_ray_pieces: FxHashSet::default(),
            white_defensive_points: PointToPieceAssociation::empty(),
            black_defensive_points: PointToPieceAssociation::empty(),
            white_moves: MovesMap::empty(),
            black_moves: MovesMap::empty(),
            white_pins: PinsMap::empty(),
            black_pins: PinsMap::empty(),
            next_piece_id: 0,
            current_turn: Color::White,
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

    fn pieces_to_recalculate(&mut self, point: &Point, caused_by_color: &Color) -> Vec<Rc<Piece>> {
        let mut pieces: Vec<&Rc<Piece>> = vec![];
        if let Some(attack_pieces) =
            self.attack_points(&caused_by_color.inverse()).get_pieces(point) {
            pieces.append(&mut attack_pieces.iter().collect::<Vec<_>>());
        }
        if let Some(defense_pieces) =
            self.defensive_points(caused_by_color).get_pieces(point) {
            pieces.append(&mut defense_pieces.iter().collect::<Vec<_>>());
        }

        pieces.into_iter().map(|piece| Rc::clone(piece)).collect::<Vec<_>>()
    }

    fn calculate_attacks_for(&mut self, piece: &Rc<Piece>) {
        self.attack_points_mut(&piece.color()).remove_piece(piece);

        let attacks = piece.attack_points(self);

        if attacks.is_empty() {
            match **piece {
                Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => {
                    self.x_ray_pieces_mut(&piece.color()).remove(piece)
                },
                _ => false,
            };
        } else {
            match **piece {
                Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => {
                    self.x_ray_pieces_mut(&piece.color()).insert(Rc::clone(piece))
                },
                _ => false
            };
        }

        for attack_point in attacks.into_iter() {
            self.x_ray_pieces_mut(&piece.color()).insert(Rc::clone(piece));
            self.attack_points_mut(&piece.color()).add_association(attack_point, piece);
        }
    }

    fn calculate_defends_for(&mut self, piece: &Rc<Piece>) {
        self.defensive_points_mut(&piece.color()).remove_piece(piece);

        let defends = piece.defensive_points(self);
        for defend_point in defends.into_iter() {
            self.defensive_points_mut(&piece.color()).add_association(defend_point, piece);
        }
    }

    fn calculate_moves_for(&mut self, piece: &Rc<Piece>) {
        self.moves_mut(piece.color()).clear(piece);

        let moves = piece.moves(&self);
        for piece_move in moves.into_iter() {
            self.moves_mut(piece.color()).add(piece, piece_move);
        }
    }

    fn calculate_check(&mut self, piece: &Rc<Piece>) {
        if self.is_under_attack(&piece.current_position(), piece.color()) {
            piece.debuffs().add(Debuff::Check);

            let mut constraints: Vec<PieceMove> = vec![];
            let pieces_caused_check =
                self.attack_points(&piece.color().inverse())
                    .get_pieces(&piece.current_position())
                    .unwrap();
            if pieces_caused_check.len() == 1 {
                let piece_caused_check = pieces_caused_check.iter().next().unwrap();
                // Add the position of the piece caused check.
                constraints.push(PieceMove::Point(piece_caused_check.current_position()));

                let direction = Vector::calc_direction(
                    &piece_caused_check.current_position(), &piece.current_position()
                ).unwrap();
                match direction {
                    // Jump checks can't be blocked. The only way to remove it is to eliminate the
                    // knight.
                    Vector::Jump(_) => (),
                    _ => {
                        let vector_points = VectorPoints::without_initial(
                            piece_caused_check.current_position(), self.dimension, direction
                        );
                        for point in vector_points {
                            // Exclude king's position
                            if point == piece.current_position() {
                                break
                            }
                            constraints.push(PieceMove::Point(point));
                        }
                    }
                }
            } else {
                // When the king is in check by more than one piece, no legal moves can be made by
                // any piece except the king itself.
                constraints.push(PieceMove::Point(Point::new(-1, -1)));
            }
            for piece_move in constraints {
                self.moves_mut(piece.color()).add_constraint(
                    piece_move
                );
            }
        } else {
            piece.debuffs().remove_check();
            self.moves_mut(piece.color()).clear_constraints();
        }
    }

    fn calculate_pins_for(&mut self, piece: &Rc<Piece>) {
        let pieces =
            self.x_ray_pieces(&piece.color().inverse())
                .iter()
                .map(|piece| Rc::clone(piece))
                .collect::<Vec<_>>();
        for pinned_by in pieces {
            self.add_pins(piece, &pinned_by)
        }
    }

    fn pass_turn(&mut self) {
        self.current_turn = self.current_turn.inverse();
    }

    pub fn is_empty_cell(&self, point: &Point) -> bool {
        self.piece_at(point).is_none()
    }

    pub fn is_enemy_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.piece_at(point){
            return !piece.is_ally(color);
        }
        false
    }

    pub fn is_capturable_enemy_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.piece_at(point) {
            if piece.is_ally(color) {
                return false
            }
            return match &**piece {
                Piece::King(_) => false,
                _ => true,
            }
        }
        false
    }

    pub fn is_ally_cell(&self, point: &Point, color: &Color) -> bool {
        if let Some(piece) = self.piece_at(point) {
            return piece.is_ally(color);
        }
        false
    }

    pub fn piece_at(&self, point: &Point) -> Option<&Rc<Piece>> {
        if let Some(cell) = self.board.get(point) {
            if let Some(piece) = cell.get_piece() {
                return Some(piece);
            }
        }
        None
    }

    pub fn is_under_attack(&self, point: &Point, color: &Color) -> bool {
        self.attack_points(&color.inverse()).has_pieces(point)
    }

    pub fn is_under_enemy_defense(&self, point: &Point, color: &Color) -> bool {
        self.defensive_points(&color.inverse()).has_pieces(point)
    }

    pub fn matches_constraints(&self, piece_move: &PieceMove, color: &Color) -> bool {
        self.moves(color).matches_constraints(piece_move)
    }

    fn add_pins(&mut self, pin_to: &Rc<Piece>, pinned_by: &Rc<Piece>) {
        let points = self.attack_points(&pin_to.color().inverse()).get_points(pinned_by);
        if let Some(points) = points {
            if points.contains(&pin_to.current_position()) {
                // No need to calculate pinned pieces, because pin_to piece is directly attacked by
                // the given pinned_by piece
                return;
            }
        }

        let enemy_color = pinned_by.color();
        let x_ray_direction =
            match &**pinned_by {
                Piece::Bishop(_) => {
                    if let Some(vector) = DiagonalVector::calc_direction(
                        &pinned_by.current_position(), &pin_to.current_position()
                    ) {
                        Some(Vector::Diagonal(vector))
                    } else {
                        None
                    }
                },
                Piece::Rook(_) => {
                    if let Some(vector) = LineVector::calc_direction(
                        &pinned_by.current_position(), &pin_to.current_position()
                    ) {
                        Some(Vector::Line(vector))
                    } else {
                        None
                    }
                },
                Piece::Queen(_) => {
                    if let Some(vector) = DiagonalVector::calc_direction(
                        &pinned_by.current_position(), &pin_to.current_position()
                    ) {
                        Some(Vector::Diagonal(vector))
                    } else if let Some(vector) = LineVector::calc_direction(
                        &pinned_by.current_position(), &pin_to.current_position()
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
                let mut current_piece_on_the_way: Option<Rc<Piece>> = None;
                let vector_points = VectorPoints::without_initial(
                    pinned_by.current_position(),
                    *self.get_dimension(),
                    direction
                );

                for point in vector_points {
                    if let Some(piece) = self.piece_at(&point) {
                        let piece = Rc::clone(piece);
                        // Enemy piece meets his ally
                        if piece.color() == enemy_color {
                            break
                        }
                        match current_piece_on_the_way {
                            Some(p) => {
                                if &piece == pin_to {
                                    // Current piece is pin_to. We have a bound!
                                    p.debuffs().add(Debuff::Pin(direction));
                                    self.pins_mut(pinned_by.color()).add_association(&p, pinned_by);
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

    fn get_next_piece_id(&mut self) -> usize {
        self.next_piece_id += 1;
        self.next_piece_id
    }

    pub fn add_piece(&mut self, name: &str, color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                     position: Point) -> Rc<Piece> {
        let piece = Rc::new(
            Piece::init_piece_by_name(
                name, color, buffs, debuffs, position, self.get_next_piece_id()
            )
        );
        self.cell_mut(&position).set_piece_rc(&piece);
        match &*piece {
            Piece::King(_) => {
                self.set_king(&position)
            },
            _ => (),
        }
        self.recalculate_connected_positions(&position, &color);
        self.recalculate_connected_positions(&position, &color.inverse());
        piece
    }

    pub fn move_piece_unchecked(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove) {
        match piece_move {
            PieceMove::Point(new_position) => {
                let old_position = piece.current_position();
                let mut piece_captured = false;
                if let Some(pinned_piece) = self.pins(piece.color()).pinned(piece) {
                    if Vector::calc_direction(&old_position, &pinned_piece.current_position())
                        != Vector::calc_direction(new_position, &pinned_piece.current_position()) {
                        pinned_piece.debuffs().remove_pin()
                    }
                }
                if let Some(piece) = self.piece_at(new_position) {
                    let piece = Rc::clone(piece);
                    self.capture_piece(&piece);
                    piece_captured = true;
                }
                self.cell_mut(&old_position).remove_piece();
                self.cell_mut(&new_position).set_piece_rc(piece);
                piece.set_current_position(*new_position);
                self.recalculate_connected_positions(&old_position, piece.color());
                self.recalculate_connected_positions(&new_position, piece.color());
                if !piece_captured {
                    // If move was performed on a free square - we need to re-calculate
                    // connected positions for the opposite pieces as well
                    self.recalculate_connected_positions(&new_position, &piece.color().inverse());
                }
            },
            PieceMove::Castle(castle_points) => {
                let king = Rc::clone(self.piece_at(castle_points.initial_king_point()).unwrap());
                let rook = Rc::clone(self.piece_at(castle_points.initial_rook_point()).unwrap());
                self.move_piece_unchecked(&rook, &PieceMove::Point(*castle_points.rook_point()));
                self.move_piece_unchecked(&king, &PieceMove::Point(*castle_points.king_point()));
            },
        }
    }

    pub fn move_piece(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove) -> bool {
        if &self.current_turn != piece.color() {
            return false;
        }

        if let Some(moves) = self.moves(piece.color()).moves(piece)
            && moves.contains(piece_move) {
            self.move_piece_unchecked(piece, piece_move);
            self.pass_turn();
            true
        } else {
            false
        }
    }

    fn capture_piece(&mut self, piece: &Rc<Piece>) {
        self.pins_mut(piece.color()).remove_pinned_by(piece);
        self.attack_points_mut(piece.color()).remove_piece(piece);
        self.defensive_points_mut(piece.color()).remove_piece(piece);
        self.moves_mut(piece.color()).clear(piece);
        self.cell_mut(&piece.current_position()).remove_piece();
    }

    fn recalculate_connected_positions(&mut self, point: &Point, caused_by_color: &Color) {
        let mut pieces_to_recalculate = self.pieces_to_recalculate(point, caused_by_color);

        if let Some(piece) = self.piece_at(point) {
            pieces_to_recalculate.push(Rc::clone(piece))
        }

        for piece in pieces_to_recalculate.iter() {
            self.calculate_attacks_for(&piece);
            self.calculate_defends_for(&piece);
        }

        if let Some(king) = self.king(&caused_by_color.inverse()) {
            let king = Rc::clone(king);
            self.calculate_pins_for(&king);
            self.calculate_check(&king);
            self.calculate_moves_for(&king);
        }

        for piece in pieces_to_recalculate.iter() {
            match &**piece {
                Piece::King(_) => {
                    // We already calculated king's moves for opposite color. Thus, we only
                    // calculate moves of the king of the given color if the piece is in the list.
                    if piece.color() == caused_by_color {
                        self.calculate_moves_for(&piece)
                    }
                },
                _ => self.calculate_moves_for(&piece)
            }
        }
    }

    fn set_king(&mut self, position: &Point) {
        let cell = self.get_board().get(position).unwrap();
        match cell.get_piece() {
            Some(p) => {
                match &**p {
                    Piece::King(_) => {
                        match p.color() {
                            Color::White => {
                                self.white_king = cell.get_piece_rc();
                                let king = Rc::clone(self.white_king.as_ref().unwrap());
                                self.calculate_pins_for(&king);
                            },
                            Color::Black => {
                                self.black_king = cell.get_piece_rc();
                                let king = Rc::clone(self.black_king.as_ref().unwrap());
                                self.calculate_pins_for(&king);
                            },
                        }
                    },
                    _ => panic!("Can't assign {} as {:?} king!", p.pp(), p.color())
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

        for y in *self.dimension.min_point().y().value()..=*self.dimension.max_point().y().value() {
            for x in *self.dimension.min_point().x().value()..=*self.dimension.max_point().x().value() {
                let point = Point::new(x, y);
                if let Some(cell) = self.board.get(&point) {
                    if point.x() == self.dimension.min_point().x() {
                        output.push_str(point.y().pp().as_str());
                        output.push_str(" ");
                    }
                    output.push_str(cell.pp().as_str());
                    output.push(' ');
                    if point.x() == self.dimension.max_point().x() {
                        output.push_str("\n");
                        buf.push(output.clone());
                        output = String::new();
                    }
                }
            }
        }
        output.push_str("  ");

        let vector_points = VectorPoints::with_initial(
            Point::new(
                *self.dimension.min_point().x().value(),
                *self.dimension.max_point().y().value()
            ),
            self.dimension,
            Vector::Line(LineVector::Right)
        );
        for point in vector_points {
            output.push_str(" ");
            output.push_str(point.x().pp().as_str());
            output.push_str("  ");
        }
        if WHITE_SIDE {
            buf = buf.into_iter().rev().collect();
        }
        buf.push(output);
        buf.join("")
    }
}

#[cfg(test)]
mod tests {

}
