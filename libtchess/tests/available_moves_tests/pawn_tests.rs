#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use libtchess::board::Board;
use libtchess::buff::Buff;
use libtchess::color::Color;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use libtchess::dimension::Dimension;
    use libtchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_are_no_pieces_around_and_additional_move_point_is_available() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_the_way() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 4));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_the_way() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 4));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_attack_point() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
                &PieceMove::Point(Point::new(3, 3)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_attack_point() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
            ],
        );
    }

    #[test]
    fn when_pawn_is_pinned_vertically_and_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 5));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
            ],
        );
    }

    #[test]
    fn when_pawn_is_pinned_horizontally_and_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 2));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(5, 2));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![],
        );
    }

    #[test]
    fn when_pawn_is_pinned_by_diagonal_and_there_is_an_enemy_piece_on_attack_point_but_on_another_axis()
     {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(1, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![],
        );
    }

    #[test]
    fn when_pawn_is_pinned_by_diagonal_by_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(3, 3))],
        );
    }

    #[test]
    fn unblocking_the_move_square_by_moving_the_same_color_piece() {
        let mut board = board_default_5x5();
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        let bishop = board.add_piece(
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 3),
        );

        board.move_piece(&bishop, &PieceMove::Point(Point::new(3, 2)));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn unblocking_the_move_square_by_moving_the_opposite_color_piece() {
        let mut board = board_default_5x5();
        let white_pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        let black_bishop = board.add_piece(
            "Bishop",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 3),
        );

        board.pass_turn(&Color::Black);
        board.move_piece(&black_bishop, &PieceMove::Point(Point::new(3, 2)));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&white_pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    mod when_there_are_void_squares_on_move_points {
        use super::*;

        #[test]
        fn it_restricts_pawn_moves() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '¤', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                ],
                &Color::White,
            );
            let config = board_config(
                Dimension::new(Point::new(1, 1), Point::new(5, 5)),
                squares_map,
            );
            let mut board = Board::empty(config);
            let pawn = board.add_piece(
                "Pawn",
                Color::White,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(3, 3),
            );

            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(&pawn).to_vec(),
                &vec![],
            );
        }
    }
}

#[cfg(test)]
mod black_pawn {
    use super::*;
    use libtchess::dimension::Dimension;
    use libtchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_are_no_pieces_around_and_additional_move_point_is_available() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 3),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::LongMove(Point::new(2, 1)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_the_way() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 3),
        );
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_the_way() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 3),
        );
        board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_attack_point() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 3),
        );
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::LongMove(Point::new(2, 1)),
                &PieceMove::Point(Point::new(3, 2)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_attack_point() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 3),
        );
        board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::LongMove(Point::new(2, 1)),
            ],
        );
    }

    #[test]
    fn when_pawn_is_pinned_vertically_and_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 4),
        );
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(2, 5));
        board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 2)),
            ],
        );
    }

    #[test]
    fn when_pawn_is_pinned_horizontally_and_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 4),
        );
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(1, 4));
        board.add_piece("Rook", Color::White, vec![], vec![], Point::new(5, 4));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![],
        );
    }

    #[test]
    fn when_pawn_is_pinned_by_diagonal_and_there_is_an_enemy_piece_on_attack_point_but_on_another_axis()
     {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 4),
        );
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(1, 5));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(1, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![],
        );
    }

    #[test]
    fn when_pawn_is_pinned_by_diagonal_by_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 4),
        );
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(1, 5));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(3, 3))],
        );
    }

    mod when_there_are_void_squares_on_move_points {
        use super::*;

        #[test]
        fn it_restricts_pawn_moves() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '¤', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                ],
                &Color::Black,
            );
            let config = board_config(
                Dimension::new(Point::new(1, 1), Point::new(5, 5)),
                squares_map,
            );
            let mut board = Board::empty(config);
            board.set_pov(Color::Black);
            let pawn = board.add_piece(
                "Pawn",
                Color::Black,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(3, 3),
            );

            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(&pawn).to_vec(),
                &vec![],
            );
        }
    }
}
