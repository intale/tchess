#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use libtchess::board::Board;
use libtchess::buff::Buff;
use libtchess::castle_points::CastlePoints;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_a_protected_enemy_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(4, 4));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(5, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_move_points_are_under_attack() {
    let mut board = board_default_4x4();
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("King", Color::Black, vec![], vec![], Point::new(2, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_king_is_on_the_diagonal_under_attack() {
    let mut board = board_default_4x4();
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(3, 1)),
            &PieceMove::Point(Point::new(2, 1)),
        ],
    );
}

#[test]
fn when_king_is_on_the_attack_range_to_the_enemy_piece_caused_diagonal_attack() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(3, 3)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(3, 1)),
            &PieceMove::Point(Point::new(2, 1)),
        ],
    );
}

#[test]
fn when_king_is_on_the_line_under_attack() {
    let mut board = board_default_4x4();
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(3, 3)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_king_is_on_the_attack_range_to_the_enemy_piece_caused_line_attack() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

mod when_there_are_void_squares_on_the_way {
    use super::*;

    #[test]
    fn it_does_not_include_them() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '¤', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '░', '▓', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::Point(Point::new(2, 4)),
                &PieceMove::Point(Point::new(3, 4)),
                &PieceMove::Point(Point::new(4, 3)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(3, 2)),
            ],
        );
    }
}

mod castle_tests {
    use super::*;

    #[test]
    fn when_castle_is_available_for_king_only() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let white_king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );
        board.add_piece("Rook", Color::White, vec![], vec![], Point::new(1, 1));

        let black_king = board.add_piece(
            "King",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 8),
        );
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(1, 8));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&white_king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(4, 1)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 2)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(6, 1)),
            ],
        );
        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&black_king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(4, 8)),
                &PieceMove::Point(Point::new(4, 7)),
                &PieceMove::Point(Point::new(5, 7)),
                &PieceMove::Point(Point::new(6, 7)),
                &PieceMove::Point(Point::new(6, 8)),
            ],
        );
    }

    #[test]
    fn when_castle_is_available_for_king_and_one_rook() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let white_king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );
        let white_rook = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        let black_king = board.add_piece(
            "King",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 8),
        );
        let black_rook = board.add_piece(
            "Rook",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 8),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&white_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 1),
                    Point::new(4, 1),
                    white_king.current_position(),
                    white_rook.current_position(),
                )),
                &PieceMove::Point(Point::new(4, 1)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 2)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(6, 1)),
            ],
        );
        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&black_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 8),
                    Point::new(4, 8),
                    black_king.current_position(),
                    black_rook.current_position(),
                )),
                &PieceMove::Point(Point::new(4, 8)),
                &PieceMove::Point(Point::new(4, 7)),
                &PieceMove::Point(Point::new(5, 7)),
                &PieceMove::Point(Point::new(6, 7)),
                &PieceMove::Point(Point::new(6, 8)),
            ],
        );
    }

    #[test]
    fn when_castle_is_available_for_king_and_two_rooks() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let white_king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );
        let white_rook1 = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );
        let white_rook2 = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(8, 1),
        );

        let black_king = board.add_piece(
            "King",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 8),
        );
        let black_rook1 = board.add_piece(
            "Rook",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 8),
        );
        let black_rook2 = board.add_piece(
            "Rook",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(8, 8),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&white_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 1),
                    Point::new(4, 1),
                    white_king.current_position(),
                    white_rook1.current_position(),
                )),
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(7, 1),
                    Point::new(6, 1),
                    white_king.current_position(),
                    white_rook2.current_position(),
                )),
                &PieceMove::Point(Point::new(4, 1)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 2)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(6, 1)),
            ],
        );
        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&black_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 8),
                    Point::new(4, 8),
                    black_king.current_position(),
                    black_rook1.current_position(),
                )),
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(7, 8),
                    Point::new(6, 8),
                    black_king.current_position(),
                    black_rook2.current_position(),
                )),
                &PieceMove::Point(Point::new(4, 8)),
                &PieceMove::Point(Point::new(4, 7)),
                &PieceMove::Point(Point::new(5, 7)),
                &PieceMove::Point(Point::new(6, 7)),
                &PieceMove::Point(Point::new(6, 8)),
            ],
        );
    }

    #[test]
    fn when_castle_is_available_for_king_and_two_rooks_for_non_classic_position() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let white_king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(4, 1),
        );
        let white_rook1 = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );
        let white_rook2 = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );
        // Add two white pawns so they cover castle way of both sides
        board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 2),
        );
        board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 2),
        );

        let black_king = board.add_piece(
            "King",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(4, 8),
        );
        let black_rook1 = board.add_piece(
            "Rook",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 8),
        );
        let black_rook2 = board.add_piece(
            "Rook",
            Color::Black,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 8),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&white_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 1),
                    Point::new(4, 1),
                    white_king.current_position(),
                    white_rook1.current_position(),
                )),
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(7, 1),
                    Point::new(6, 1),
                    white_king.current_position(),
                    white_rook2.current_position(),
                )),
                &PieceMove::Point(Point::new(4, 2)),
            ],
        );
        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&black_king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 8),
                    Point::new(4, 8),
                    black_king.current_position(),
                    black_rook1.current_position(),
                )),
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(7, 8),
                    Point::new(6, 8),
                    black_king.current_position(),
                    black_rook2.current_position(),
                )),
                &PieceMove::Point(Point::new(3, 7)),
                &PieceMove::Point(Point::new(4, 7)),
                &PieceMove::Point(Point::new(5, 7)),
            ],
        );
    }

    #[test]
    fn when_king_castle_point_is_under_attack() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(4, 1)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 2)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(6, 1)),
            ],
        );
    }

    #[test]
    fn when_king_castle_way_is_under_attack() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(7, 1),
        );
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(5, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(5, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(6, 1)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(7, 2)),
                &PieceMove::Point(Point::new(8, 2)),
                &PieceMove::Point(Point::new(8, 1)),
            ],
        );
    }

    #[test]
    fn when_rook_castle_way_is_under_attack() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(7, 1),
        );
        let white_rook = board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Castle(CastlePoints::new(
                    Point::new(3, 1),
                    Point::new(4, 1),
                    king.current_position(),
                    white_rook.current_position(),
                )),
                &PieceMove::Point(Point::new(6, 1)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(7, 2)),
                &PieceMove::Point(Point::new(8, 2)),
                &PieceMove::Point(Point::new(8, 1)),
            ],
        );
    }

    #[test]
    fn when_king_is_under_check() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(7, 1),
        );
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(7, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(6, 1)),
                &PieceMove::Point(Point::new(6, 2)),
                &PieceMove::Point(Point::new(8, 2)),
                &PieceMove::Point(Point::new(8, 1)),
            ],
        );
    }

    #[test]
    fn when_rook_is_pinned() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let king = board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(2, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(1, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&king).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::Point(Point::new(3, 2)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(4, 1)),
            ],
        );
    }

    mod when_there_are_void_squares_on_the_king_way {
        use super::*;

        #[test]
        fn it_does_allow_castling() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '¤', '▓'],
                ],
                &Color::White,
            );
            let config = board_config(
                Dimension::new(Point::new(1, 1), Point::new(5, 5)),
                squares_map,
            );
            let mut board = Board::empty(config);
            let king = board.add_piece(
                "King",
                Color::White,
                vec![Buff::Castle],
                vec![],
                Point::new(5, 1),
            );
            let rook = board.add_piece(
                "Rook",
                Color::White,
                vec![Buff::Castle],
                vec![],
                Point::new(1, 1),
            );

            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(&king).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(4, 2)),
                    &PieceMove::Point(Point::new(5, 2)),
                ],
            );
            compare_and_assert(
                &board.moves_of(&rook).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(1, 4)),
                    &PieceMove::Point(Point::new(1, 5)),
                    &PieceMove::Point(Point::new(2, 1)),
                    &PieceMove::Point(Point::new(3, 1)),
                ],
            );
        }
    }

    mod when_there_are_void_squares_on_the_rook_way {
        use super::*;

        #[test]
        fn it_does_allow_castling() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '¤', '▓', '░', '▓'],
                ],
                &Color::White,
            );
            let config = board_config(
                Dimension::new(Point::new(1, 1), Point::new(5, 5)),
                squares_map,
            );
            let mut board = Board::empty(config);
            let king = board.add_piece(
                "King",
                Color::White,
                vec![Buff::Castle],
                vec![],
                Point::new(5, 1),
            );
            let rook = board.add_piece(
                "Rook",
                Color::White,
                vec![Buff::Castle],
                vec![],
                Point::new(1, 1),
            );

            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(&king).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(4, 1)),
                    &PieceMove::Point(Point::new(4, 2)),
                    &PieceMove::Point(Point::new(5, 2)),
                ],
            );
            compare_and_assert(
                &board.moves_of(&rook).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(1, 4)),
                    &PieceMove::Point(Point::new(1, 5)),
                ],
            );
        }
    }
}
