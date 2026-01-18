#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use tchess::dimension::Dimension;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3), &Point::new(3, 3)],
        );
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3), &Point::new(3, 3)],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = board_default_4x4();
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3)],
        );
    }

    mod when_there_are_void_squares_on_attack_points {
        use super::*;

        #[test]
        fn it_does_not_include_them() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '¤', '░'],
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
            let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .attack_points(&Color::White)
                    .get_points(&pawn)
                    .to_vec(),
                &vec![&Point::new(2, 4)],
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
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 1), &Point::new(3, 1)],
        );
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 1), &Point::new(3, 1)],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 3));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(4, 2)],
        );
    }

    mod when_there_are_void_squares_on_attack_points {
        use super::*;

        #[test]
        fn it_does_not_include_them() {
            let squares_map = TestSquaresMap::from_chars(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '¤', '░', '▓', '░'],
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
            let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .attack_points(&Color::Black)
                    .get_points(&pawn)
                    .to_vec(),
                &vec![&Point::new(2, 2)],
            );
        }
    }
}
