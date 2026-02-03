use crate::board_position::{BoardPosition, PieceRepr};
use crate::classic_heat_map::ClassicHeatMap;
use crate::classic_square_map::ClassicSquaresMap;
use crate::game_result::GameResult;
use crate::game_stats::GameStats;
use crate::move_result::MoveResult;
use libtchess::board::Board;
use libtchess::board_config::BoardConfig;
use libtchess::buff::Buff;
use libtchess::castle_x_points::{CastleXPoints, KingCastleXPoint, RookCastleXPoint};
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::heat_map::HeatMap;
use libtchess::last_board_changes::LastBoardChanges;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::player::Player;
use libtchess::point::Point;
use libtchess::squares_map::SquaresMap;

const FIFTY_MOVE_RULE_TURNS_COUNT: u8 = 100;
const MAX_NUMBER_OF_EQUAL_POSITIONS: u8 = 3;

pub struct ClassicGame<HT: HeatMap, SQ: SquaresMap> {
    current_position: BoardPosition,
    board: Board<HT, SQ>,
    game_stats: GameStats,
    game_result: Option<GameResult>,
}

impl ClassicGame<ClassicHeatMap, ClassicSquaresMap> {
    pub fn classic_board() -> Self {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = BoardConfig::new(
            CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
            CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
            ClassicHeatMap::empty(),
            ClassicSquaresMap::init(),
            dimension,
            Player::Human,
            Player::Human,
        );
        let mut board = Board::empty(config);

        for y in board.dimension().get_rows_range() {
            for x in board.dimension().get_columns_range() {
                let point = Point::new(x, y);
                match (y, x) {
                    // White pieces
                    (1, 1) | (1, 8) => {
                        board.add_piece("Rook", Color::White, vec![Buff::Castle], vec![], point);
                        ()
                    }
                    (1, 2) | (1, 7) => {
                        board.add_piece("Knight", Color::White, vec![], vec![], point);
                        ()
                    }
                    (1, 3) | (1, 6) => {
                        board.add_piece("Bishop", Color::White, vec![], vec![], point);
                        ()
                    }
                    (1, 4) => {
                        board.add_piece("Queen", Color::White, vec![], vec![], point);
                        ()
                    }
                    (1, 5) => {
                        board.add_piece("King", Color::White, vec![Buff::Castle], vec![], point);
                        ()
                    }
                    (2, _) => {
                        board.add_piece(
                            "Pawn",
                            Color::White,
                            vec![Buff::AdditionalPoint],
                            vec![],
                            point,
                        );
                        ()
                    }
                    // Black pieces
                    (8, 1) | (8, 8) => {
                        board.add_piece("Rook", Color::Black, vec![Buff::Castle], vec![], point);
                        ()
                    }
                    (8, 2) | (8, 7) => {
                        board.add_piece("Knight", Color::Black, vec![], vec![], point);
                        ()
                    }
                    (8, 3) | (8, 6) => {
                        board.add_piece("Bishop", Color::Black, vec![], vec![], point);
                        ()
                    }
                    (8, 5) => {
                        board.add_piece("King", Color::Black, vec![Buff::Castle], vec![], point);
                        ()
                    }
                    (8, 4) => {
                        board.add_piece("Queen", Color::Black, vec![], vec![], point);
                        ()
                    }
                    (7, _) => {
                        board.add_piece(
                            "Pawn",
                            Color::Black,
                            vec![Buff::AdditionalPoint],
                            vec![],
                            point,
                        );
                        ()
                    }
                    _ => (),
                };
            }
        }
        let mut classic_board = Self {
            board,
            game_stats: GameStats::new(),
            current_position: BoardPosition::new(),
            game_result: None,
        };
        classic_board.process_last_board_changes();
        classic_board
    }
}

impl<HT: HeatMap, SQ: SquaresMap> ClassicGame<HT, SQ> {
    pub fn move_piece_at(&mut self, position: &Point, piece_move: &PieceMove) -> MoveResult {
        let &piece_id = self
            .board
            .piece_id_at(position)
            .expect(format!("Could not find piece at {} position", position).as_str());
        self.move_piece(&piece_id, piece_move)
    }

    pub fn move_piece(&mut self, piece_id: &PieceId, piece_move: &PieceMove) -> MoveResult {
        if let Some(game_result) = self.game_result {
            return MoveResult::GameEnded(game_result);
        }
        let opposite_active_pieces_number_was =
            self.board.active_pieces(&piece_id.color().inverse()).len();
        let result = self.board.move_piece(piece_id, piece_move);
        if !result {
            return MoveResult::IllegalMove;
        }

        // Non pawn move and not a capture move should be tracked as a move within
        // "50 move-rule". A pawn move and a capture move should reset "50 move-rule" counter.
        let piece_repr = self.current_position.get_piece(piece_id);
        match piece_repr {
            PieceRepr::Pawn(_) => self.game_stats.reset_meaningless_moves_number(),
            _ => {
                let opposite_active_pieces_number = self
                    .board
                    .active_pieces(&piece_repr.data().color.inverse())
                    .len();
                if opposite_active_pieces_number == opposite_active_pieces_number_was {
                    self.game_stats.incr_meaningless_moves_number();
                } else {
                    self.game_stats.reset_meaningless_moves_number()
                }
            }
        }
        self.process_last_board_changes();
        self.game_stats.incr_move_number();
        self.calculate_game_result();

        if self.game_result.is_some() {
            MoveResult::GameEnded(self.game_result.unwrap())
        } else {
            MoveResult::PieceMoved
        }
    }

    pub fn board(&self) -> &Board<HT, SQ> {
        &self.board
    }

    pub fn game_result(&self) -> Option<&GameResult> {
        self.game_result.as_ref()
    }

    fn process_last_board_changes(&mut self) {
        for change in self.board.last_changes().iter() {
            match change {
                LastBoardChanges::PieceAdded(piece_id) => {
                    let piece = self.board.find_piece_by_id(piece_id).unwrap();
                    let piece_repr = self.current_position.add_piece(piece);
                    self.game_stats.add_active_piece(piece_repr);
                }
                LastBoardChanges::PieceRemoved(piece_id) => {
                    let piece_repr = self.current_position.remove_piece(piece_id);
                    self.game_stats.remove_active_piece(&piece_repr);
                }
                LastBoardChanges::PiecePositionChanged(piece_id) => {
                    let piece = self.board.find_piece_by_id(piece_id).unwrap();
                    self.current_position.update_piece_position(piece)
                }
                LastBoardChanges::EnPassantChanged(piece_id) => {
                    let piece = self.board.find_piece_by_id(piece_id).unwrap();
                    self.current_position.update_piece_en_passant(piece)
                }
                LastBoardChanges::CastleChanged(piece_id) => {
                    let piece = self.board.find_piece_by_id(piece_id).unwrap();
                    self.current_position.update_piece_en_passant(piece)
                }
            }
        }
        self.current_position
            .set_current_turn(self.board.current_turn());
        self.game_stats.persist_position(&self.current_position);
    }

    fn calculate_game_result(&mut self) {
        if self.board.has_no_moves(self.board.current_turn()) {
            if let Some(king) = self.board.king(self.board.current_turn()) {
                if king.debuffs().has_check() {
                    self.game_result = Some(GameResult::Checkmate(*self.board.current_turn()))
                } else {
                    self.game_result = Some(GameResult::Stalemate(*self.board.current_turn()))
                }
            } else {
                self.game_result = Some(GameResult::InsufficientMaterialDraw)
            }
            return;
        }
        if self.game_stats.meaningless_moves_number() == &FIFTY_MOVE_RULE_TURNS_COUNT {
            self.game_result = Some(GameResult::FiftyMoveRuleDraw);
            return;
        }
        if let Some((_, occurrences_num)) = self.game_stats.most_frequent_position()
            && occurrences_num == &MAX_NUMBER_OF_EQUAL_POSITIONS
        {
            self.game_result = Some(GameResult::DrawByRepetition);
            return;
        }
        if self.game_stats.is_insufficient_material(&Color::White)
            && self.game_stats.is_insufficient_material(&Color::Black)
        {
            self.game_result = Some(GameResult::InsufficientMaterialDraw);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libtchess::castle_points::CastlePoints;
    use libtchess::promote_piece::PromotePiece;
    use libtchess::utils::pretty_print::PrettyPrint;

    fn move_piece(
        classic_game: &mut ClassicGame<ClassicHeatMap, ClassicSquaresMap>,
        point: Point,
        piece_move: PieceMove,
    ) {
        assert_eq!(
            classic_game.move_piece_at(&point, &piece_move),
            MoveResult::PieceMoved
        );
        println!("{}", classic_game.board().pp());
    }

    fn checkmate(
        classic_game: &mut ClassicGame<ClassicHeatMap, ClassicSquaresMap>,
        point: Point,
        piece_move: PieceMove,
        color: Color,
    ) {
        assert_eq!(
            classic_game.move_piece_at(&point, &piece_move),
            MoveResult::GameEnded(GameResult::Checkmate(color))
        );
        println!("{}", classic_game.board().pp());
    }

    fn draw_by_repetition(
        classic_game: &mut ClassicGame<ClassicHeatMap, ClassicSquaresMap>,
        point: Point,
        piece_move: PieceMove,
    ) {
        assert_eq!(
            classic_game.move_piece_at(&point, &piece_move),
            MoveResult::GameEnded(GameResult::DrawByRepetition)
        );
        println!("{}", classic_game.board().pp());
    }

    mod games {
        use super::*;

        mod me_vs_random {
            use super::*;

            #[test]
            fn game() {
                let mut classic_game = ClassicGame::classic_board();

                // 1. e4 c5 2. d4 d6 3. dxc5 dxc5 4. Qxd8+ Kxd8 5. Be3 e6 6. Bb5 Bd7 7. Nc3 Bxb5 8.
                // Nxb5 Nc6 9. O-O-O+ Kc8 10. Nf3 Nf6 11. e5 Nd5 12. Nd6+ Bxd6 13. exd6 Kd7 14.
                // Bxc5 b6 15. Ba3 a5 16. c4 Nf6 17. Rhe1 Nb4 18. b3 Nxa2+ 19. Kb2 Nb4 20. Ne5+ Ke8
                // 21. d7+ Ke7 22. Nc6+ Kf8 23. Nxb4 axb4 24. Bxb4+ Kg8 25. d8=Q+ Rxd8
                // 26. Rxd8+ Ne8 27. Rxe8# 1-0

                // 1
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::LongMove(Point::new(5, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::LongMove(Point::new(3, 5)),
                );

                // 2
                move_piece(
                    &mut classic_game,
                    Point::new(4, 2),
                    PieceMove::LongMove(Point::new(4, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(4, 6)),
                );

                // 3
                move_piece(
                    &mut classic_game,
                    Point::new(4, 4),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 6),
                    PieceMove::Point(Point::new(3, 5)),
                );

                // 4
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(4, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Point(Point::new(4, 8)),
                );

                // 5
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(5, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::Point(Point::new(5, 6)),
                );

                // 6
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 7
                move_piece(
                    &mut classic_game,
                    Point::new(2, 1),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(2, 5)),
                );

                // 8
                move_piece(
                    &mut classic_game,
                    Point::new(3, 3),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(3, 6)),
                );

                // 9
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Castle(CastlePoints::new(
                        Point::new(3, 1),
                        Point::new(4, 1),
                        Point::new(5, 1),
                        Point::new(1, 1),
                    )),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(3, 8)),
                );

                // 10
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 8),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 11
                move_piece(
                    &mut classic_game,
                    Point::new(5, 4),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(4, 5)),
                );

                // 12
                move_piece(
                    &mut classic_game,
                    Point::new(2, 5),
                    PieceMove::Point(Point::new(4, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(4, 6)),
                );

                // 13
                move_piece(
                    &mut classic_game,
                    Point::new(5, 5),
                    PieceMove::Point(Point::new(4, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 14
                move_piece(
                    &mut classic_game,
                    Point::new(5, 3),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 7),
                    PieceMove::Point(Point::new(2, 6)),
                );

                // 15
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(1, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 7),
                    PieceMove::LongMove(Point::new(1, 5)),
                );

                // 16
                move_piece(
                    &mut classic_game,
                    Point::new(3, 2),
                    PieceMove::LongMove(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 5),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 17
                move_piece(
                    &mut classic_game,
                    Point::new(8, 1),
                    PieceMove::Point(Point::new(5, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 6),
                    PieceMove::Point(Point::new(2, 4)),
                );

                // 18
                move_piece(
                    &mut classic_game,
                    Point::new(2, 2),
                    PieceMove::Point(Point::new(2, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 4),
                    PieceMove::Point(Point::new(1, 2)),
                );

                // 19
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(2, 2)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 2),
                    PieceMove::Point(Point::new(2, 4)),
                );

                // 20
                move_piece(
                    &mut classic_game,
                    Point::new(6, 3),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(5, 8)),
                );

                // 21
                move_piece(
                    &mut classic_game,
                    Point::new(4, 6),
                    PieceMove::Point(Point::new(4, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Point(Point::new(5, 7)),
                );

                // 22
                move_piece(
                    &mut classic_game,
                    Point::new(5, 5),
                    PieceMove::Point(Point::new(3, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::Point(Point::new(6, 8)),
                );

                // 23
                move_piece(
                    &mut classic_game,
                    Point::new(3, 6),
                    PieceMove::Point(Point::new(2, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 5),
                    PieceMove::Point(Point::new(2, 4)),
                );

                // 24
                move_piece(
                    &mut classic_game,
                    Point::new(1, 3),
                    PieceMove::Point(Point::new(2, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(7, 8)),
                );

                // 25
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Promote(Point::new(4, 8), PromotePiece::Queen),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 8),
                    PieceMove::Point(Point::new(4, 8)),
                );

                // 26
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(4, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(5, 8)),
                );

                // 27
                checkmate(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(5, 8)),
                    Color::Black,
                );
            }
        }

        mod paul_morphy_vs_duke_karl_paris_1858 {
            use super::*;

            #[test]
            fn game() {
                let mut classic_game = ClassicGame::classic_board();

                // https://www.chess.com/games/view/765
                // 1.e4 e5 2.Nf3 d6 3.d4 Bg4 4.dxe5 Bxf3 5.Qxf3 dxe5 6.Bc4 Nf6 7.Qb3 Qe7 8.Nc3 c6
                // 9.Bg5 b5 10.Nxb5 cxb5 11.Bxb5+ Nbd7 12.O-O-O Rd8 13.Rxd7 Rxd7 14.Rd1 Qe6
                // 15.Bxd7+ Nxd7 16.Qb8+ Nxb8 17.Rd8# 1-0

                // 1
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::LongMove(Point::new(5, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::LongMove(Point::new(5, 5)),
                );

                // 2
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(4, 6)),
                );

                // 3
                move_piece(
                    &mut classic_game,
                    Point::new(4, 2),
                    PieceMove::LongMove(Point::new(4, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(7, 4)),
                );

                // 4
                move_piece(
                    &mut classic_game,
                    Point::new(4, 4),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 4),
                    PieceMove::Point(Point::new(6, 3)),
                );

                // 5
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 6),
                    PieceMove::Point(Point::new(5, 5)),
                );

                // 6
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 8),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 7
                move_piece(
                    &mut classic_game,
                    Point::new(6, 3),
                    PieceMove::Point(Point::new(2, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(5, 7)),
                );

                // 8
                move_piece(
                    &mut classic_game,
                    Point::new(2, 1),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::Point(Point::new(3, 6)),
                );

                // 9
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(7, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 7),
                    PieceMove::LongMove(Point::new(2, 5)),
                );

                // 10
                move_piece(
                    &mut classic_game,
                    Point::new(3, 3),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 6),
                    PieceMove::Point(Point::new(2, 5)),
                );

                // 11
                move_piece(
                    &mut classic_game,
                    Point::new(3, 4),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 12
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Castle(CastlePoints::new(
                        Point::new(3, 1),
                        Point::new(4, 1),
                        Point::new(5, 1),
                        Point::new(1, 1),
                    )),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 8),
                    PieceMove::Point(Point::new(4, 8)),
                );

                // 13
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(4, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 14
                move_piece(
                    &mut classic_game,
                    Point::new(8, 1),
                    PieceMove::Point(Point::new(4, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::Point(Point::new(5, 6)),
                );

                // 15
                move_piece(
                    &mut classic_game,
                    Point::new(2, 5),
                    PieceMove::Point(Point::new(4, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 16
                move_piece(
                    &mut classic_game,
                    Point::new(2, 3),
                    PieceMove::Point(Point::new(2, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(2, 8)),
                );

                // 17
                checkmate(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(4, 8)),
                    Color::Black,
                );
            }
        }

        mod donald_byrne_vs_bobby_fischer_1956 {
            use super::*;

            #[test]
            fn game() {
                let mut classic_game = ClassicGame::classic_board();
                // https://www.chess.com/games/view/75289
                // 1. Nf3 Nf6 2. c4 g6 3. Nc3 Bg7 4. d4 O-O 5. Bf4 d5 6. Qb3 $6 dxc4 7. Qxc4 c6 8.
                // e4 Nbd7 $2 9. Rd1 Nb6 $6 10. Qc5 $2 Bg4 11. Bg5 $4 Na4 $3 12. Qa3 Nxc3 13. bxc3 Nxe4
                // 14. Bxe7 Qb6 15. Bc4 Nxc3 $3 16. Bc5 Rfe8+ 17. Kf1 Be6 $3 18. Bxb6 $6 Bxc4+ 19. Kg1
                // Ne2+ $1 20. Kf1 Nxd4+ 21. Kg1 Ne2+ 22. Kf1 Nc3+ 23. Kg1 axb6 24. Qb4 Ra4 25. Qxb6
                // Nxd1 26. h3 Rxa2 27. Kh2 Nxf2 28. Re1 Rxe1 29. Qd8+ Bf8 30. Nxe1 Bd5 31. Nf3 Ne4
                // 32. Qb8 b5 33. h4 h5 34. Ne5 Kg7 35. Kg1 Bc5+ 36. Kf1 $6 Ng3+ 37. Ke1 Bb4+ 38.
                // Kd1 Bb3+ 39. Kc1 Ne2+ 40. Kb1 Nc3+ 41. Kc1 Rc2# 0-1

                // 1
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 8),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 2
                move_piece(
                    &mut classic_game,
                    Point::new(3, 2),
                    PieceMove::LongMove(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 7),
                    PieceMove::Point(Point::new(7, 6)),
                );

                // 3
                move_piece(
                    &mut classic_game,
                    Point::new(2, 1),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(7, 7)),
                );

                // 4
                move_piece(
                    &mut classic_game,
                    Point::new(4, 2),
                    PieceMove::LongMove(Point::new(4, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Castle(CastlePoints::new(
                        Point::new(7, 8),
                        Point::new(6, 8),
                        Point::new(5, 8),
                        Point::new(8, 8),
                    )),
                );

                // 5
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(6, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::LongMove(Point::new(4, 5)),
                );

                // 6
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(2, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 5),
                    PieceMove::Point(Point::new(3, 4)),
                );

                // 7
                move_piece(
                    &mut classic_game,
                    Point::new(2, 3),
                    PieceMove::Point(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::Point(Point::new(3, 6)),
                );

                // 8
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::LongMove(Point::new(5, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 9
                move_piece(
                    &mut classic_game,
                    Point::new(1, 1),
                    PieceMove::Point(Point::new(4, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(2, 6)),
                );

                // 10
                move_piece(
                    &mut classic_game,
                    Point::new(3, 4),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(7, 4)),
                );

                // 11
                move_piece(
                    &mut classic_game,
                    Point::new(6, 4),
                    PieceMove::Point(Point::new(7, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 6),
                    PieceMove::Point(Point::new(1, 4)),
                );

                // 12
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(1, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 4),
                    PieceMove::Point(Point::new(3, 3)),
                );

                // 13
                move_piece(
                    &mut classic_game,
                    Point::new(2, 2),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(5, 4)),
                );

                // 14
                move_piece(
                    &mut classic_game,
                    Point::new(7, 5),
                    PieceMove::Point(Point::new(5, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(2, 6)),
                );

                // 15
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 4),
                    PieceMove::Point(Point::new(3, 3)),
                );

                // 16
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(5, 8)),
                );

                // 17
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Point(Point::new(6, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 4),
                    PieceMove::Point(Point::new(5, 6)),
                );

                // 18
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(2, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 6),
                    PieceMove::Point(Point::new(3, 4)),
                );

                // 19
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(7, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 3),
                    PieceMove::Point(Point::new(5, 2)),
                );

                // 20
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::Point(Point::new(4, 4)),
                );

                // 21
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(7, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 4),
                    PieceMove::Point(Point::new(5, 2)),
                );

                // 22
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::Point(Point::new(3, 3)),
                );

                // 23
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(7, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 7),
                    PieceMove::Point(Point::new(2, 6)),
                );

                // 24
                move_piece(
                    &mut classic_game,
                    Point::new(1, 3),
                    PieceMove::Point(Point::new(2, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 8),
                    PieceMove::Point(Point::new(1, 4)),
                );

                // 25
                move_piece(
                    &mut classic_game,
                    Point::new(2, 4),
                    PieceMove::Point(Point::new(2, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 3),
                    PieceMove::Point(Point::new(4, 1)),
                );

                // 26
                move_piece(
                    &mut classic_game,
                    Point::new(8, 2),
                    PieceMove::Point(Point::new(8, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 4),
                    PieceMove::Point(Point::new(1, 2)),
                );

                // 27
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(8, 2)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(6, 2)),
                );

                // 28
                move_piece(
                    &mut classic_game,
                    Point::new(8, 1),
                    PieceMove::Point(Point::new(5, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Point(Point::new(5, 1)),
                );

                // 29
                move_piece(
                    &mut classic_game,
                    Point::new(2, 6),
                    PieceMove::Point(Point::new(4, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 7),
                    PieceMove::Point(Point::new(6, 8)),
                );

                // 30
                move_piece(
                    &mut classic_game,
                    Point::new(6, 3),
                    PieceMove::Point(Point::new(5, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 4),
                    PieceMove::Point(Point::new(4, 5)),
                );

                // 31
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 2),
                    PieceMove::Point(Point::new(5, 4)),
                );

                // 32
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(2, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 7),
                    PieceMove::LongMove(Point::new(2, 5)),
                );

                // 33
                move_piece(
                    &mut classic_game,
                    Point::new(8, 3),
                    PieceMove::Point(Point::new(8, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(8, 7),
                    PieceMove::LongMove(Point::new(8, 5)),
                );

                // 34
                move_piece(
                    &mut classic_game,
                    Point::new(6, 3),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 8),
                    PieceMove::Point(Point::new(7, 7)),
                );

                // 35
                move_piece(
                    &mut classic_game,
                    Point::new(8, 2),
                    PieceMove::Point(Point::new(7, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(3, 5)),
                );

                // 36
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 4),
                    PieceMove::Point(Point::new(7, 3)),
                );

                // 37
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(5, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(2, 4)),
                );

                // 38
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Point(Point::new(4, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 5),
                    PieceMove::Point(Point::new(2, 3)),
                );

                // 39
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(3, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 3),
                    PieceMove::Point(Point::new(5, 2)),
                );

                // 40
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(2, 1)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::Point(Point::new(3, 3)),
                );

                // 41
                move_piece(
                    &mut classic_game,
                    Point::new(2, 1),
                    PieceMove::Point(Point::new(3, 1)),
                );
                checkmate(
                    &mut classic_game,
                    Point::new(1, 2),
                    PieceMove::Point(Point::new(3, 2)),
                    Color::White,
                );
            }
        }

        mod robert_steel_vs_robert_macdonald_ross_1884 {
            // https://www.chess.com/analysis/collection/immortal-games-uZmNVXMY/594vz7DjGa

            use super::*;

            #[test]
            fn game() {
                let mut classic_game = ClassicGame::classic_board();
                // https://www.chess.com/analysis/collection/immortal-games-uZmNVXMY/2zXEwjhEni
                // 1. e4 e5 2. Nc3 Nc6 3. f4 exf4 4. d4 Qh4+ 5. Ke2 d5 6. exd5 Bg4+ 7. Nf3 O-O-O 8.
                // dxc6 Bc5 9. cxb7+ Kb8 10. Nb5 Nf6 11. c3 Rhe8+ 12. Kd3 Bf5+ 13. Kc4 Be6+ 14.
                // Kxc5 a5 15. Nxc7 Qh5+ 16. Ne5 Nd7+ 17. Kb5 Qxd1 18. Bxf4 Qxa1 19. Ka6 Nxe5 20.
                // Nxe8 f6 21. dxe5 f5 22. Be3 Rxe8 23. Bb5 Qxh1 24. Bc5 Rd8 25. Ba7+ {And draws by
                // perpetual check} 25... Kc7 26. Bb6+ Kb8 27. Ba7+ Kc7 28. Bb6+ Kb8 29. Ba7+
                // 1/2-1/2

                // 1
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::LongMove(Point::new(5, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 7),
                    PieceMove::LongMove(Point::new(5, 5)),
                );

                // 2
                move_piece(
                    &mut classic_game,
                    Point::new(2, 1),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(3, 6)),
                );

                // 3
                move_piece(
                    &mut classic_game,
                    Point::new(6, 2),
                    PieceMove::LongMove(Point::new(6, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 5),
                    PieceMove::Point(Point::new(6, 4)),
                );

                // 4
                move_piece(
                    &mut classic_game,
                    Point::new(4, 2),
                    PieceMove::LongMove(Point::new(4, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(8, 4)),
                );

                // 5
                move_piece(
                    &mut classic_game,
                    Point::new(5, 1),
                    PieceMove::Point(Point::new(5, 2)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::LongMove(Point::new(4, 5)),
                );

                // 6
                move_piece(
                    &mut classic_game,
                    Point::new(5, 4),
                    PieceMove::Point(Point::new(4, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(7, 4)),
                );

                // 7
                move_piece(
                    &mut classic_game,
                    Point::new(7, 1),
                    PieceMove::Point(Point::new(6, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Castle(CastlePoints::new(
                        Point::new(3, 8),
                        Point::new(4, 8),
                        Point::new(5, 8),
                        Point::new(1, 8),
                    )),
                );

                // 8
                move_piece(
                    &mut classic_game,
                    Point::new(4, 5),
                    PieceMove::Point(Point::new(3, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 8),
                    PieceMove::Point(Point::new(3, 5)),
                );

                // 9
                move_piece(
                    &mut classic_game,
                    Point::new(3, 6),
                    PieceMove::Point(Point::new(2, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 8),
                    PieceMove::Point(Point::new(2, 8)),
                );

                // 10
                move_piece(
                    &mut classic_game,
                    Point::new(3, 3),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 8),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 11
                move_piece(
                    &mut classic_game,
                    Point::new(3, 2),
                    PieceMove::Point(Point::new(3, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(8, 8),
                    PieceMove::Point(Point::new(5, 8)),
                );

                // 12
                move_piece(
                    &mut classic_game,
                    Point::new(5, 2),
                    PieceMove::Point(Point::new(4, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(7, 4),
                    PieceMove::Point(Point::new(6, 5)),
                );

                // 13
                move_piece(
                    &mut classic_game,
                    Point::new(4, 3),
                    PieceMove::Point(Point::new(3, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 5),
                    PieceMove::Point(Point::new(5, 6)),
                );

                // 14
                move_piece(
                    &mut classic_game,
                    Point::new(3, 4),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 7),
                    PieceMove::LongMove(Point::new(1, 5)),
                );

                // 15
                move_piece(
                    &mut classic_game,
                    Point::new(2, 5),
                    PieceMove::Point(Point::new(3, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(8, 4),
                    PieceMove::Point(Point::new(8, 5)),
                );

                // 16
                move_piece(
                    &mut classic_game,
                    Point::new(6, 3),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(4, 7)),
                );

                // 17
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(8, 5),
                    PieceMove::Point(Point::new(4, 1)),
                );

                // 18
                move_piece(
                    &mut classic_game,
                    Point::new(3, 1),
                    PieceMove::Point(Point::new(6, 4)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 1),
                    PieceMove::Point(Point::new(1, 1)),
                );

                // 19
                move_piece(
                    &mut classic_game,
                    Point::new(2, 5),
                    PieceMove::Point(Point::new(1, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 7),
                    PieceMove::Point(Point::new(5, 5)),
                );

                // 20
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::Point(Point::new(5, 8)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 7),
                    PieceMove::Point(Point::new(6, 6)),
                );

                // 21
                move_piece(
                    &mut classic_game,
                    Point::new(4, 4),
                    PieceMove::Point(Point::new(5, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(6, 6),
                    PieceMove::Point(Point::new(6, 5)),
                );

                // 22
                move_piece(
                    &mut classic_game,
                    Point::new(6, 4),
                    PieceMove::Point(Point::new(5, 3)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(4, 8),
                    PieceMove::Point(Point::new(5, 8)),
                );

                // 23
                move_piece(
                    &mut classic_game,
                    Point::new(6, 1),
                    PieceMove::Point(Point::new(2, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(1, 1),
                    PieceMove::Point(Point::new(8, 1)),
                );

                // 24
                move_piece(
                    &mut classic_game,
                    Point::new(5, 3),
                    PieceMove::Point(Point::new(3, 5)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(5, 8),
                    PieceMove::Point(Point::new(4, 8)),
                );

                // 25
                move_piece(
                    &mut classic_game,
                    Point::new(3, 5),
                    PieceMove::Point(Point::new(1, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(3, 7)),
                );

                // 26
                move_piece(
                    &mut classic_game,
                    Point::new(1, 7),
                    PieceMove::Point(Point::new(2, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::Point(Point::new(2, 8)),
                );

                // 27
                move_piece(
                    &mut classic_game,
                    Point::new(2, 6),
                    PieceMove::Point(Point::new(1, 7)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(2, 8),
                    PieceMove::Point(Point::new(3, 7)),
                );

                // 28
                move_piece(
                    &mut classic_game,
                    Point::new(1, 7),
                    PieceMove::Point(Point::new(2, 6)),
                );
                move_piece(
                    &mut classic_game,
                    Point::new(3, 7),
                    PieceMove::Point(Point::new(2, 8)),
                );

                // 29
                draw_by_repetition(
                    &mut classic_game,
                    Point::new(2, 6),
                    PieceMove::Point(Point::new(1, 7)),
                );
            }
        }
    }
}
