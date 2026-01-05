#[path = "../support/mod.rs"]
mod support;

use support::compare_and_assert;
use support::traits::ToVecRef;
use tchess::board::Board;
use tchess::board_square_builder::{
    BoardSquareBuilder, default_square_builder::DefaultSquareBuilder,
};
use tchess::color::Color;
use tchess::point::Point;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(3, 3),
            DefaultSquareBuilder::init(),
        );
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
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(3, 3),
            DefaultSquareBuilder::init(),
        );
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
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(4, 4),
            DefaultSquareBuilder::init(),
        );
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
        use support::init_square_builder_from;

        #[test]
        fn it_does_not_include_them() {
            let builder = init_square_builder_from(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '¤', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                ],
                &Color::White
            );

            let mut board = Board::empty(
                Point::new(1, 1),
                Point::new(5, 5),
                builder,
            );
            let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .attack_points(&Color::White)
                    .get_points(&pawn)
                    .to_vec(),
                &vec![
                    &Point::new(2, 4),
                ],
            );
        }
    }
}

#[cfg(test)]
mod black_pawn {
    use super::*;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(3, 3),
            DefaultSquareBuilder::init(),
        );
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
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(3, 3),
            DefaultSquareBuilder::init(),
        );
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
        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(4, 4),
            DefaultSquareBuilder::init(),
        );
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
        use support::init_square_builder_from;

        #[test]
        fn it_does_not_include_them() {
            let builder = init_square_builder_from(
                vec![
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '¤', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                    vec!['░', '▓', '░', '▓', '░'],
                    vec!['▓', '░', '▓', '░', '▓'],
                ],
                &Color::Black
            );

            let mut board = Board::empty(
                Point::new(1, 1),
                Point::new(5, 5),
                builder,
            );
            board.set_pov(Color::Black);
            let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .attack_points(&Color::Black)
                    .get_points(&pawn)
                    .to_vec(),
                &vec![
                    &Point::new(2, 2),
                ],
            );
        }
    }
}
