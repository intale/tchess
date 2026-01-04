use std::rc::Rc;

use crate::color::Color;
use crate::pieces::{Piece};
use crate::utils::pretty_print::PrettyPrint;
use crate::point::{Point};
use crate::board_square::{BoardSquare};
use rustc_hash::{FxHashSet};
use crate::board_map::{BoardMap};
use crate::buff::Buff;
use crate::debuff::Debuff;
use crate::dimension::Dimension;
use crate::moves_map::MovesMap;
use crate::piece_move::PieceMove;
use crate::point_to_piece_association::{PointToPieceAssociation};
use crate::vector::diagonal_vector::DiagonalVector;
use crate::vector::line_vector::LineVector;
use crate::vector::Vector;
use crate::vector_points::VectorPoints;
use crate::x_ray_pieces::XRayPieces;
use crate::board_square_builders::{BoardSquareBuilder, default_square_builder::DefaultSquareBuilder};

// Invert colors of chess symbols so they look more meaningful in the terminal window with black
// background. Debugging purpose only.
pub const INVERT_COLORS: bool = true;


pub struct Board {
    board_map: BoardMap,
    dimension: Dimension,
    white_attack_points: PointToPieceAssociation,
    black_attack_points: PointToPieceAssociation,
    white_x_ray_pieces: XRayPieces,
    black_x_ray_pieces: XRayPieces,
    white_pawns_with_en_passant: FxHashSet<Rc<Piece>>,
    black_pawns_with_en_passant: FxHashSet<Rc<Piece>>,
    white_defensive_points: PointToPieceAssociation,
    black_defensive_points: PointToPieceAssociation,
    white_moves: MovesMap,
    black_moves: MovesMap,
    next_piece_id: usize,
    current_turn: Color,
    // Determines board's point of view. Debugging purpose only.
    pov: Color,
}

impl Board {
    pub fn classic_chess_board() -> Self {
        let mut board = Self::empty(
            Point::new(1, 1),
            Point::new(8, 8),
            DefaultSquareBuilder::init(),
        );

        for y in board.dimension().get_rows_range() {
            for x in board.dimension().get_columns_range() {
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
        board
    }

    pub fn board_map(&self) -> &BoardMap {
        &self.board_map
    }

    fn board_map_mut(&mut self) -> &mut BoardMap {
        &mut self.board_map
    }

    pub fn dimension(&self) -> &Dimension {
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

    pub fn x_ray_pieces(&self, color: &Color) -> &XRayPieces {
        match color {
            Color::White => &self.white_x_ray_pieces,
            Color::Black => &self.black_x_ray_pieces,
        }
    }

    fn x_ray_pieces_mut(&mut self, color: &Color) -> &mut XRayPieces {
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

    pub fn king(&self, color: &Color) -> Option<&Rc<Piece>> {
        self.board_map.king(color)
    }

    pub fn pawns_with_en_passant(&self, color: &Color) -> &FxHashSet<Rc<Piece>> {
        match color {
            Color::White => &self.white_pawns_with_en_passant,
            Color::Black => &self.black_pawns_with_en_passant,
        }
    }

    fn pawns_with_en_passant_mut(&mut self, color: &Color) -> &mut FxHashSet<Rc<Piece>> {
        match color {
            Color::White => &mut self.white_pawns_with_en_passant,
            Color::Black => &mut self.black_pawns_with_en_passant,
        }
    }

    pub fn empty<T: BoardSquareBuilder>(min_point: Point, max_point: Point, square_builder: T) -> Self {
        let mut board = Self {
            board_map: BoardMap::empty(),
            dimension: Dimension::new(min_point, max_point),
            white_attack_points: PointToPieceAssociation::empty(),
            black_attack_points: PointToPieceAssociation::empty(),
            white_x_ray_pieces: XRayPieces::empty(),
            black_x_ray_pieces: XRayPieces::empty(),
            white_pawns_with_en_passant: FxHashSet::default(),
            black_pawns_with_en_passant: FxHashSet::default(),
            white_defensive_points: PointToPieceAssociation::empty(),
            black_defensive_points: PointToPieceAssociation::empty(),
            white_moves: MovesMap::empty(),
            black_moves: MovesMap::empty(),
            next_piece_id: 0,
            current_turn: Color::White,
            pov: Color::White,
        };
        for y in board.dimension().get_rows_range() {
            for x in board.dimension().get_columns_range() {
                let point = Point::new(x, y);
                if let Some(square) = square_builder.build(&point) {
                    board.board_map_mut().add_square(point, square);
                }
            }
        }
        board
    }

    // Pins points and castle points are not taken into account here. They require more complex
    // computations than a simple look up.
    fn pieces_to_recalculate(&mut self, point: &Point, caused_by_color: &Color) -> FxHashSet<Rc<Piece>> {
        let mut pieces: FxHashSet<Rc<Piece>> = FxHashSet::default();
        if let Some(attack_pieces) =
            self.attack_points(&caused_by_color.inverse()).get_pieces(point) {
            for piece in attack_pieces.iter() {
                pieces.insert(Rc::clone(piece));
            }
        }

        if let Some(defense_pieces) =
            self.defensive_points(caused_by_color).get_pieces(point) {
            for piece in defense_pieces.iter() {
                pieces.insert(Rc::clone(piece));
            }
        }
        for piece in self.pawns_with_en_passant_mut(&caused_by_color.inverse()).iter() {
            pieces.insert(Rc::clone(piece));
        }
        // Since the pawn is the only piece whose move points do not match its attack points, we
        // need to look at the moves map to see if we need to recalculate the pawn's moves as well.
        if let Some(pieces_at_point) = self.moves(&caused_by_color.inverse()).pieces_at(&point) {
            for (piece, _) in pieces_at_point {
                match &**piece {
                    Piece::Pawn(_) => { pieces.insert(Rc::clone(piece)); },
                    _ => ()
                }
            }
        }
        pieces
    }

    fn calculate_attacks_for(&mut self, piece: &Rc<Piece>) {
        self.attack_points_mut(&piece.color()).remove_piece(piece);

        let attacks = piece.attack_points(self);

        for attack_point in attacks.into_iter() {
            self.attack_points_mut(&piece.color()).add_association(attack_point, piece);
        }
    }

    fn calculate_x_ray(&mut self, piece: &Rc<Piece>) {
        let opposite_king = match self.king(&piece.color().inverse()) {
            Some(piece) => Rc::clone(piece),
            None => return,
        };

        let attacks = self.attack_points(piece.color()).get_points(piece);
        if let Some(attacks) = attacks && !attacks.is_empty() {
            if let Some(direction) = Self::x_ray_direction(piece, &opposite_king) {
                // Get current piece which occupies this direction
                let current_piece =
                    self.x_ray_pieces(piece.color())
                        .piece_by_direction(&direction);
                let current_piece =
                    match current_piece {
                        Some(piece) => Some(Rc::clone(piece)),
                        None => None,
                    };

                // Add or try to replace the direction by the given piece
                let new_piece =
                    self.x_ray_pieces_mut(piece.color())
                        .add_x_ray_vector(&direction, piece);

                // The direction was replaced by the new piece. We have to remove a pin of the old
                // piece if any.
                if let Some(current_piece) = current_piece.as_ref() && current_piece != &new_piece {
                    self.remove_x_ray_piece(&current_piece);
                }

                if &new_piece == piece {
                    // We are calculating pins for the piece which already occupies the given
                    // direction or have just occupied it. In this case we need to recalculate moves
                    // of possible pinned piece.
                    self.clear_existing_pins(&piece);
                    self.add_pins(&opposite_king, piece);
                } else {
                    // The given direction was not replaced by the given piece. This means that
                    // there is already another piece, positioned closer to the opposite king that
                    // the given one.
                    self.remove_x_ray_piece(&piece);
                }
            } else {
                // The given piece does not have any connections to the opposite king - we have to
                // remove previously existing pin if any.
                self.remove_x_ray_piece(piece);
            }
        } else {
            // The given piece does not have any attack points - we have to remove previously
            // existing pin if any.
            self.remove_x_ray_piece(piece);
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
        self.moves_mut(piece.color()).remove_piece(piece);

        let moves = piece.moves(&self);
        for piece_move in moves.into_iter() {
            self.moves_mut(piece.color()).add(piece, piece_move);
        }
    }

    fn calculate_check_for(&mut self, king: &Rc<Piece>) {
        king.debuffs().remove_check();
        self.moves_mut(king.color()).clear_general_constraints();

        if self.is_under_attack(&king.current_position(), king.color()) {
            king.debuffs().add(Debuff::Check);

            let mut constraints: Vec<PieceMove> = vec![];
            let pieces_caused_check =
                self.attack_points(&king.color().inverse())
                    .get_pieces(&king.current_position())
                    .unwrap();
            if pieces_caused_check.len() == 1 {
                let piece_caused_check = pieces_caused_check.iter().next().unwrap();
                // Add the position of the piece caused check.
                constraints.push(PieceMove::Point(piece_caused_check.current_position()));

                let direction = Vector::calc_direction(
                    &piece_caused_check.current_position(), &king.current_position()
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
                            if point == king.current_position() {
                                break
                            }
                            constraints.push(PieceMove::Point(point));
                        }
                    }
                }
            } else {
                // When the king is in check by more than one piece, no legal moves can be made by
                // any piece except the king itself.
                constraints.push(PieceMove::UnreachablePoint);
            }

            for piece_move in constraints {
                self.moves_mut(king.color()).add_general_constraints(piece_move);
            }
        }
    }

    fn calc_en_passant(&mut self, position: &Point, caused_by_color: &Color) {
        let en_passant_position =
            match caused_by_color {
                Color::White => Point::new(*position.x().value(), *position.y().value() - 1),
                Color::Black => Point::new(*position.x().value(), *position.y().value() + 1),
            };

        let mut pawns: Vec<Rc<Piece>> = vec![];
        if let Some(pieces) =
            self.attack_points(&caused_by_color.inverse()).get_pieces(&en_passant_position) {
            for piece in pieces {
                match &**piece {
                    Piece::Pawn(_) => {
                        piece.buffs().add(Buff::EnPassant(en_passant_position, *position));
                        pawns.push(Rc::clone(piece));
                    },
                    _ => ()
                }
            }
        }
        for pawn in pawns {
            self.pawns_with_en_passant_mut(&caused_by_color.inverse()).insert(pawn);
        }
    }

    fn clear_en_passant(&mut self) {
        for pawn in self.pawns_with_en_passant_mut(&Color::White).drain() {
            pawn.buffs().remove_en_passant();
        }
        for pawn in self.pawns_with_en_passant_mut(&Color::Black).drain() {
            pawn.buffs().remove_en_passant();
        }
    }

    pub fn pass_turn(&mut self, color: &Color) {
        self.current_turn = *color;
    }

    pub fn piece_at(&self, point: &Point) -> Option<&Rc<Piece>> {
        match self.board_square(point) {
            BoardSquare::Square(square) => square.get_piece(),
            BoardSquare::VoidSquare => None
        }
    }

    pub fn board_square(&self, point: &Point) -> &BoardSquare {
        self.board_map.board_square(point)
    }

    pub fn is_under_attack(&self, point: &Point, color: &Color) -> bool {
        self.attack_points(&color.inverse()).has_pieces(point)
    }

    pub fn is_under_enemy_defense(&self, point: &Point, color: &Color) -> bool {
        self.defensive_points(&color.inverse()).has_pieces(point)
    }

    fn add_pins(&mut self, pin_to: &Rc<Piece>, pinned_by: &Rc<Piece>) -> bool {
        let points = self.attack_points(pinned_by.color()).get_points(pinned_by);
        if let Some(points) = points {
            if points.contains(&pin_to.current_position()) {
                // No need to calculate pinned pieces, because pin_to piece is directly attacked by
                // the given pinned_by piece
                return false;
            }
        }

        let enemy_color = pinned_by.color();
        let x_ray_direction = Self::x_ray_direction(pinned_by, pin_to).unwrap_or_else(|| {
            panic!(
                "Logical mistake: {:?} must have a connection to {:?} at this point!",
                pinned_by, pin_to
            )
        });

        let mut current_piece_on_the_way: Option<Rc<Piece>> = None;
        let vector_points = VectorPoints::without_initial(
            pinned_by.current_position(),
            *self.dimension(),
            x_ray_direction
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
                            p.debuffs().add(Debuff::Pin(x_ray_direction));
                            self.calculate_moves_for(&p);
                            self.x_ray_pieces_mut(pinned_by.color()).add_pin(&p, pinned_by);
                            return true;
                        }
                        break
                    },
                    None => { current_piece_on_the_way = Some(piece) }
                }
            }
        }
        false
    }

    fn get_next_piece_id(&mut self) -> usize {
        self.next_piece_id += 1;
        self.next_piece_id
    }

    pub fn add_piece(&mut self, name: &str, color: Color, buffs: Vec<Buff>, debuffs: Vec<Debuff>,
                     position: Point) -> Rc<Piece> {
        if !self.board_square(&position).is_empty_square() {
            panic!("Can't add {} piece. Position {:?} is not empty!", name, position)
        }
        self.add_piece_unchecked(name, color, buffs, debuffs, position, true)
    }

    fn add_piece_unchecked(&mut self, name: &str, color: Color, buffs: Vec<Buff>,
                               debuffs: Vec<Debuff>, position: Point, calculate_mechanics: bool) -> Rc<Piece> {
        let piece = Rc::new(
            Piece::init_piece_by_name(
                name, color, buffs, debuffs, position, self.get_next_piece_id()
            )
        );
        self.board_map.place_piece(&position, &piece, true);
        if calculate_mechanics {
            self.recalculate_connected_positions(&position, &color, true);
            self.recalculate_connected_positions(&position, &color.inverse(), false);
            self.recalculate_king_mechanics(&color);
            self.recalculate_king_mechanics(&color.inverse());
        }
        piece
    }

    pub fn move_piece(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove) -> bool {
        if &self.current_turn != piece.color() {
            return false;
        }

        if let Some(moves) = self.moves(piece.color()).moves_of(piece)
            && moves.contains(piece_move) {
            self.move_piece_unchecked(piece, piece_move, true);
            self.pass_turn(&piece.color().inverse());
            true
        } else {
            false
        }
    }

    fn move_piece_unchecked(&mut self, piece: &Rc<Piece>, piece_move: &PieceMove,
                                calculate_king: bool) {
        self.clear_en_passant();
        match &**piece {
            Piece::King(_) | Piece::Rook(_) => piece.buffs().remove_castle(),
            Piece::Pawn(_) => {
                piece.buffs().remove_additional_point();
                match piece_move {
                    PieceMove::LongMove(new_position) => {
                        self.calc_en_passant(new_position, &piece.color());
                    },
                    _ => (),
                }
            },
            _ => (),
        };
        match piece_move {
            PieceMove::Point(new_position) | PieceMove::LongMove(new_position) => {
                let enemy_piece: Option<Rc<Piece>> =
                    if let Some(piece) = self.piece_at(new_position) {
                        Some(Rc::clone(piece))
                    } else {
                        None
                    };
                self.perform_move(piece, &piece.current_position(), &new_position, enemy_piece);
            },
            PieceMove::EnPassant(new_position, enemy_position) => {
                let captured_pawn = Some(Rc::clone(self.piece_at(enemy_position).unwrap_or_else(|| {
                    panic!(
                        "Logical mistake: enemy pawn should be present at {:?} during en passant",
                        enemy_position
                    );
                })));
                self.perform_move(piece, &piece.current_position(), &new_position, captured_pawn);
            },
            PieceMove::Castle(castle_points) => {
                let king = Rc::clone(self.piece_at(castle_points.initial_king_point()).unwrap());
                let rook = Rc::clone(self.piece_at(castle_points.initial_rook_point()).unwrap());
                // In chess960 king or rook may keep staying on their places during the castling
                if &rook.current_position() != castle_points.rook_point() {
                    self.move_piece_unchecked(&rook, &PieceMove::Point(*castle_points.rook_point()), false);
                }
                if &king.current_position() != castle_points.king_point() {
                    self.move_piece_unchecked(&king, &PieceMove::Point(*castle_points.king_point()), false);
                }
            },
            PieceMove::Promote(point, promote_piece) => {
                self.remove_piece(piece);
                let promoted_piece = self.add_piece_unchecked(
                    &promote_piece.name(),
                    *piece.color(),
                    vec![],
                    vec![],
                    piece.current_position(),
                    false,
                );
                self.move_piece_unchecked(&promoted_piece, &PieceMove::Point(*point), false);
            },
            PieceMove::UnreachablePoint => panic!("Unreachable point!"),
        }
        if calculate_king {
            self.recalculate_king_mechanics(piece.color());
            self.recalculate_king_mechanics(&piece.color().inverse());
        }
    }

    fn perform_move(&mut self, piece_to_move: &Rc<Piece>, old_position: &Point,
                    new_position: &Point, enemy_piece: Option<Rc<Piece>>) {
        if let Some(piece) = enemy_piece {
            self.capture_piece(&piece);
        }
        self.board_map.take_off_piece(&old_position, false);
        self.board_map.place_piece(&new_position, piece_to_move, false);
        self.recalculate_connected_positions(&old_position, piece_to_move.color(), false);
        self.recalculate_connected_positions(&new_position, piece_to_move.color(), true);
        self.recalculate_connected_positions(
            &new_position, &piece_to_move.color().inverse(), false
        );
    }

    fn capture_piece(&mut self, piece: &Rc<Piece>) {
        self.remove_piece(piece);
    }

    // Not every piece removal from the board is capturing. For example, when promoting a pawn - we
    // need to remove it from the board without any other potential actions
    fn remove_piece(&mut self, piece: &Rc<Piece>) {
        self.attack_points_mut(piece.color()).remove_piece(piece);
        self.defensive_points_mut(piece.color()).remove_piece(piece);
        self.moves_mut(piece.color()).remove_piece(piece);
        self.board_map.take_off_piece(&piece.current_position(), true);
        self.remove_x_ray_piece(piece);
    }

    fn recalculate_connected_positions(&mut self, point: &Point, caused_by_color: &Color,
                                       include_piece_at_position: bool) {
        let mut pieces_to_recalculate = self.pieces_to_recalculate(point, caused_by_color);

        if include_piece_at_position && let Some(piece) = self.piece_at(point) {
            pieces_to_recalculate.insert(Rc::clone(piece));
        }

        for piece in pieces_to_recalculate.iter() {
            self.calculate_attacks_for(piece);
            self.calculate_defends_for(piece);
        }

        for piece in pieces_to_recalculate.iter() {
            match &**piece {
                Piece::King(_) => {
                    // King moves are calculated separately
                },
                _ => {
                    self.calculate_moves_for(&piece);
                }
            }
        }

        // This covers the case when an ally piece moves from the x-ray direction, thus causing
        // another ally piece, standing in front of it, be pinned. Example, white bishop(or black -
        // if you are using light theme in your IDE/editor) causes white queen be pinned by moving
        // to a2:
        // 3 ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓
        // 2 ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░
        // 1 ░♚░ ▓♝▓ ░♛░ ▓▓▓ ░░░ ▓▓▓ ░♖░ ▓♔▓
        //    a   b   c   d   e   f   g   h
        // 3 ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓
        // 2 ▓♝▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░
        // 1 ░♚░ ▓▓▓ ░♛░ ▓▓▓ ░░░ ▓▓▓ ░♖░ ▓♔▓
        //    a   b   c   d   e   f   g   h
        for piece in self.x_ray_pieces(&caused_by_color.inverse()).pieces_owned() {
            if let Some(x_ray_direction) = self.x_ray_pieces(&caused_by_color.inverse()).direction(&piece) {
                if let Some(direction) = Vector::calc_direction(&piece.current_position(), point) {
                    if x_ray_direction == &direction {
                        self.calculate_x_ray(&piece);
                    }
                }
            }
        }

        // Re-calculate x-rays of x-ray pieces, explicitly affected by the current position
        for piece in pieces_to_recalculate {
            match &*piece {
                Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => {
                    self.calculate_x_ray(&piece);
                },
                _ => ()
            }
        }
    }

    fn recalculate_king_mechanics(&mut self, color: &Color) {
        if let Some(king) = self.king(&color) {
            let king = Rc::clone(king);
            self.calculate_check_for(&king);
            self.calculate_moves_for(&king);
        }
    }

    pub fn set_pov(&mut self, color: Color) {
        self.pov = color;
    }

    pub fn current_turn(&self) -> &Color {
        &self.current_turn
    }

    fn x_ray_direction(piece: &Rc<Piece>, opposite_king: &Rc<Piece>) -> Option<Vector> {
        match &**piece {
            Piece::Bishop(_) => {
                if let Some(vector) = DiagonalVector::calc_direction(
                    &piece.current_position(), &opposite_king.current_position()
                ) {
                    Some(Vector::Diagonal(vector))
                } else {
                    None
                }
            },
            Piece::Rook(_) => {
                if let Some(vector) = LineVector::calc_direction(
                    &piece.current_position(), &opposite_king.current_position()
                ) {
                    Some(Vector::Line(vector))
                } else {
                    None
                }
            },
            Piece::Queen(_) => {
                if let Some(vector) = DiagonalVector::calc_direction(
                    &piece.current_position(), &opposite_king.current_position()
                ) {
                    Some(Vector::Diagonal(vector))
                } else if let Some(vector) = LineVector::calc_direction(
                    &piece.current_position(), &opposite_king.current_position()
                ) {
                    Some(Vector::Line(vector))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn remove_x_ray_piece(&mut self, piece: &Rc<Piece>) {
        self.clear_existing_pins(piece);
        self.x_ray_pieces_mut(&piece.color()).remove_piece(piece);
    }

    fn clear_existing_pins(&mut self, piece: &Rc<Piece>) {
        if let Some(pinned) = self.x_ray_pieces_mut(&piece.color()).pinned_piece(piece) {
            let pinned = Rc::clone(pinned);
            pinned.debuffs().remove_pin();
            self.calculate_moves_for(&pinned);
        }
    }
}

impl PrettyPrint for Board {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut buf: Vec<String> = vec![];

        let y_range: Vec<i16> = if self.pov == Color::White {
            (*self.dimension.min_point().y().value()..=*self.dimension.max_point().y().value()).rev().collect()
        } else {
            (*self.dimension.min_point().y().value()..=*self.dimension.max_point().y().value()).collect()
        };
        let x_range: Vec<i16> = if self.pov == Color::White {
            (*self.dimension.min_point().x().value()..=*self.dimension.max_point().x().value()).collect()
        } else {
            (*self.dimension.min_point().x().value()..=*self.dimension.max_point().x().value()).rev().collect()
        };

        for y in y_range {
            for x in x_range.clone() {
                let point = Point::new(x, y);
                let square = self.board_square(&point);
                if (self.pov == Color::White && point.x() == self.dimension.min_point().x())
                    || (self.pov == Color::Black && point.x() == self.dimension.max_point().x()) {
                    output.push_str(point.y().pp().as_str());
                    output.push_str(" ");
                }
                output.push_str(square.pp().as_str());
                output.push(' ');
                if (self.pov == Color::White && point.x() == self.dimension.max_point().x())
                    || (self.pov == Color::Black && point.x() == self.dimension.min_point().x()) {
                    output.push_str("\n");
                    buf.push(output.clone());
                    output = String::new();
                }
            }
        }
        output.push_str("  ");

        let vector_points = if self.pov == Color::White {
            VectorPoints::with_initial(
                Point::new(
                    *self.dimension.min_point().x().value(),
                    *self.dimension.max_point().y().value()
                ),
                self.dimension,
                Vector::Line(LineVector::Right)
            )
        } else {
            VectorPoints::with_initial(
                Point::new(
                    *self.dimension.max_point().x().value(),
                    *self.dimension.max_point().y().value()
                ),
                self.dimension,
                Vector::Line(LineVector::Left)
            )
        };
        for point in vector_points {
            output.push_str(" ");
            output.push_str(point.x().pp().as_str());
            output.push_str("  ");
        }
        buf.push(output);
        buf.join("")
    }
}

#[cfg(test)]
mod tests {

}
