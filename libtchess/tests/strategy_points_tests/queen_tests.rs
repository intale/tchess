#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::point::Point;
use libtchess::strategy_point::StrategyPoint;
use libtchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 1)),
            &StrategyPoint::Attack(Point::new(2, 2)),
            &StrategyPoint::Attack(Point::new(4, 4)),
            &StrategyPoint::Attack(Point::new(5, 5)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 3)),
            &StrategyPoint::Attack(Point::new(4, 3)),
            &StrategyPoint::Attack(Point::new(5, 3)),
            &StrategyPoint::Attack(Point::new(2, 4)),
            &StrategyPoint::Attack(Point::new(1, 5)),
            &StrategyPoint::Attack(Point::new(4, 2)),
            &StrategyPoint::Attack(Point::new(5, 1)),
            &StrategyPoint::Attack(Point::new(3, 5)),
            &StrategyPoint::Attack(Point::new(3, 4)),
            &StrategyPoint::Attack(Point::new(3, 2)),
            &StrategyPoint::Attack(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(2, 2));
    add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 3)),
            &StrategyPoint::Attack(Point::new(3, 3)),
            &StrategyPoint::Attack(Point::new(3, 2)),
            &StrategyPoint::Attack(Point::new(3, 1)),
            &StrategyPoint::Attack(Point::new(2, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_king_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(1, 1));
    add_piece(&mut board,"King", Color::Black, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 2)),
            &StrategyPoint::Attack(Point::new(3, 3)),
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Attack(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_an_attack_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(2, 2));
    add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 3)),
            &StrategyPoint::Attack(Point::new(3, 2)),
            &StrategyPoint::Attack(Point::new(3, 1)),
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Defense(Point::new(3, 3)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_between_the_queen_and_an_enemy_piece() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(1, 1));

    add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(3, 1)),
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Defense(Point::new(2, 2)),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_between_the_queen_and_another_enemy_piece() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(1, 1));

    add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(2, 2));
    add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 2)),
            &StrategyPoint::Attack(Point::new(3, 1)),
            &StrategyPoint::Attack(Point::new(2, 1)),
        ],
    );
}

#[test]
fn when_there_are_enemy_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of pawns around the queen
    create_box_of(
        &mut board,
        "Pawn",
        Color::Black,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(5, 5)),
    );

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(queen.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(1, 1)),
            &StrategyPoint::Attack(Point::new(2, 2)),
            &StrategyPoint::Attack(Point::new(4, 4)),
            &StrategyPoint::Attack(Point::new(5, 5)),
            &StrategyPoint::Attack(Point::new(1, 3)),
            &StrategyPoint::Attack(Point::new(2, 3)),
            &StrategyPoint::Attack(Point::new(4, 3)),
            &StrategyPoint::Attack(Point::new(5, 3)),
            &StrategyPoint::Attack(Point::new(2, 4)),
            &StrategyPoint::Attack(Point::new(1, 5)),
            &StrategyPoint::Attack(Point::new(4, 2)),
            &StrategyPoint::Attack(Point::new(5, 1)),
            &StrategyPoint::Attack(Point::new(3, 5)),
            &StrategyPoint::Attack(Point::new(3, 4)),
            &StrategyPoint::Attack(Point::new(3, 2)),
            &StrategyPoint::Attack(Point::new(3, 1)),
        ],
    );
}

mod when_there_are_void_squares_on_the_way {
    use super::*;

    #[test]
    fn it_describes_them_as_a_dead_end() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '¤', '░'],
                vec!['▓', '░', '▓', '¤', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let queen = add_piece(&mut board,"Queen", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(queen.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(1, 3)),
                &StrategyPoint::Attack(Point::new(1, 5)),
                &StrategyPoint::Attack(Point::new(2, 3)),
                &StrategyPoint::Attack(Point::new(2, 4)),
                &StrategyPoint::Attack(Point::new(3, 4)),
                &StrategyPoint::Attack(Point::new(3, 5)),
                &StrategyPoint::DeadEnd(Point::new(2, 2)),
                &StrategyPoint::DeadEnd(Point::new(3, 2)),
                &StrategyPoint::DeadEnd(Point::new(4, 2)),
                &StrategyPoint::DeadEnd(Point::new(4, 3)),
                &StrategyPoint::DeadEnd(Point::new(4, 4)),
            ],
        );
    }
}
