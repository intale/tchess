#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::point::Point;
use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use libtchess::dimension::Dimension;
    use libtchess::strategy_point::StrategyPoint;
    use libtchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 3)),
                &StrategyPoint::Attack(Point::new(3, 3)),
                &StrategyPoint::Move(Point::new(2, 3)),
            ],
        );
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 3)),
                &StrategyPoint::Attack(Point::new(3, 3)),
                &StrategyPoint::Move(Point::new(2, 3)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = board_default_4x4();
        let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 3)),
                &StrategyPoint::Move(Point::new(2, 3)),
                &StrategyPoint::Defense(Point::new(3, 3)),
            ],
        );
    }

    #[test]
    fn when_an_ally_piece_is_too_far_from_pawn() {
        let mut board = board_default_4x4();
        let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(1, 1));
        add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(4, 4));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(2, 2)),
                &StrategyPoint::Move(Point::new(1, 2)),
            ],
        );
    }

    mod when_there_are_void_squares_on_attack_strategy_points {
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
            let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .strategy_points(&Color::White)
                    .get_points(pawn.id())
                    .to_vec(),
                &vec![
                    &StrategyPoint::Attack(Point::new(2, 4)),
                    &StrategyPoint::Move(Point::new(3, 4)),
                ],
            );
        }
    }

    mod when_there_are_void_squares_on_move_strategy_points {
        use super::*;

        #[test]
        fn it_describes_them_as_a_dead_end() {
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
            let pawn = add_piece(&mut board,"Pawn", Color::White, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .strategy_points(&Color::White)
                    .get_points(pawn.id())
                    .to_vec(),
                &vec![
                    &StrategyPoint::Attack(Point::new(2, 4)),
                    &StrategyPoint::Attack(Point::new(4, 4)),
                    &StrategyPoint::DeadEnd(Point::new(3, 4)),
                ],
            );
        }
    }
}

#[cfg(test)]
mod black_pawn {
    use super::*;
    use libtchess::dimension::Dimension;
    use libtchess::strategy_point::StrategyPoint;
    use libtchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::Black)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 1)),
                &StrategyPoint::Attack(Point::new(3, 1)),
                &StrategyPoint::Move(Point::new(2, 1)),
            ],
        );
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.set_pov(Color::Black);
        let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(2, 2));
        add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(3, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::Black)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 1)),
                &StrategyPoint::Attack(Point::new(3, 1)),
                &StrategyPoint::Move(Point::new(2, 1)),
            ],
        );
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(3, 3));
        add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::Black)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(4, 2)),
                &StrategyPoint::Defense(Point::new(2, 2)),
                &StrategyPoint::Move(Point::new(3, 2)),
            ],
        );
    }

    #[test]
    fn when_an_ally_piece_is_too_far_from_pawn() {
        let mut board = board_default_4x4();
        board.set_pov(Color::Black);
        let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(1, 4));
        add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(4, 1));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::Black)
                .get_points(pawn.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(2, 3)),
                &StrategyPoint::Move(Point::new(1, 3)),
            ],
        );
    }

    mod when_there_are_void_squares_on_attack_strategy_points {
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
            let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .strategy_points(&Color::Black)
                    .get_points(pawn.id())
                    .to_vec(),
                &vec![
                    &StrategyPoint::Attack(Point::new(2, 2)),
                    &StrategyPoint::Move(Point::new(3, 2)),
                ],
            );
        }
    }

    mod when_there_are_void_squares_on_move_strategy_points {
        use super::*;

        #[test]
        fn it_describes_them_as_a_dead_end() {
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
            let pawn = add_piece(&mut board,"Pawn", Color::Black, vec![], vec![], Point::new(3, 3));

            println!("{}", board.pp());
            compare_and_assert(
                &board
                    .strategy_points(&Color::Black)
                    .get_points(pawn.id())
                    .to_vec(),
                &vec![
                    &StrategyPoint::Attack(Point::new(2, 2)),
                    &StrategyPoint::Attack(Point::new(4, 2)),
                    &StrategyPoint::DeadEnd(Point::new(3, 2)),
                ],
            );
        }
    }
}
