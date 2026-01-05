#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::buff::Buff;
use tchess::color::Color;
use tchess::piece_move::PieceMove;
use tchess::point::Point;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use tchess::dimension::Dimension;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_are_no_pieces_around_and_additional_move_point_is_available() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece(
            "Pawn",
            Color::White,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(2, 2),
        );

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_the_way() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_the_way() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 3))],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::LongMove(Point::new(2, 4)),
                &PieceMove::Point(Point::new(3, 3)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::White).moves_of(&pawn).to_vec(),
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
                &board.moves(&Color::White).moves_of(&pawn).to_vec(),
                &vec![],
            );
        }
    }
}

#[cfg(test)]
mod black_pawn {
    use super::*;
    use tchess::dimension::Dimension;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_are_no_pieces_around_and_additional_move_point_is_available() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::LongMove(Point::new(2, 1)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_the_way() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_the_way() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
            &vec![&PieceMove::Point(Point::new(2, 2))],
        );
    }

    #[test]
    fn when_there_is_an_enemy_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 2)),
                &PieceMove::LongMove(Point::new(2, 1)),
                &PieceMove::Point(Point::new(3, 2)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
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
            &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
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
                &board.moves(&Color::Black).moves_of(&pawn).to_vec(),
                &vec![],
            );
        }
    }
}
