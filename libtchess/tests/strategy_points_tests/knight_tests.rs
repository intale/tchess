#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::point::Point;
use libtchess::strategy_point::StrategyPoint;
use libtchess::utils::pretty_print::PrettyPrint;
use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(knight.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 4)),
            &StrategyPoint::Attack(Point::new(2, 5)),
            &StrategyPoint::Attack(Point::new(4, 5)),
            &StrategyPoint::Attack(Point::new(5, 4)),
            &StrategyPoint::Attack(Point::new(5, 2)),
            &StrategyPoint::Attack(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(knight.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 4)),
            &StrategyPoint::Attack(Point::new(2, 5)),
            &StrategyPoint::Attack(Point::new(4, 5)),
            &StrategyPoint::Attack(Point::new(5, 4)),
            &StrategyPoint::Attack(Point::new(5, 2)),
            &StrategyPoint::Attack(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_an_attack_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(knight.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 4)),
            &StrategyPoint::Attack(Point::new(2, 5)),
            &StrategyPoint::Attack(Point::new(5, 4)),
            &StrategyPoint::Attack(Point::new(5, 2)),
            &StrategyPoint::Attack(Point::new(4, 1)),
            &StrategyPoint::Defense(Point::new(4, 5)),
        ],
    );
}

#[test]
fn when_there_are_ally_pieces_between_the_knight_and_an_enemy_piece() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of bishops around the knight
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(2, 2), Point::new(4, 4)),
    );

    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(5, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .strategy_points(&Color::White)
            .get_points(knight.id())
            .to_vec(),
        &vec![
            &StrategyPoint::Attack(Point::new(2, 1)),
            &StrategyPoint::Attack(Point::new(1, 2)),
            &StrategyPoint::Attack(Point::new(1, 4)),
            &StrategyPoint::Attack(Point::new(2, 5)),
            &StrategyPoint::Attack(Point::new(4, 5)),
            &StrategyPoint::Attack(Point::new(5, 4)),
            &StrategyPoint::Attack(Point::new(5, 2)),
            &StrategyPoint::Attack(Point::new(4, 1)),
        ],
    );
}

mod when_knight_is_surrounded_by_void_squares {
    use super::*;

    #[test]
    fn it_ignores_them() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '¤', '▓', '¤', '▓'],
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
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(knight.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(2, 1)),
                &StrategyPoint::Attack(Point::new(1, 2)),
                &StrategyPoint::Attack(Point::new(1, 4)),
                &StrategyPoint::Attack(Point::new(2, 5)),
                &StrategyPoint::Attack(Point::new(4, 5)),
                &StrategyPoint::Attack(Point::new(5, 4)),
                &StrategyPoint::Attack(Point::new(5, 2)),
                &StrategyPoint::Attack(Point::new(4, 1)),
            ],
        );
    }
}

mod when_there_are_void_squares_on_strategy_points {
    use super::*;

    #[test]
    fn it_describes_them_as_a_dead_end() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '¤', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '¤', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .strategy_points(&Color::White)
                .get_points(knight.id())
                .to_vec(),
            &vec![
                &StrategyPoint::Attack(Point::new(2, 1)),
                &StrategyPoint::Attack(Point::new(1, 2)),
                &StrategyPoint::Attack(Point::new(1, 4)),
                &StrategyPoint::Attack(Point::new(2, 5)),
                &StrategyPoint::DeadEnd(Point::new(4, 5)),
                &StrategyPoint::DeadEnd(Point::new(5, 4)),
                &StrategyPoint::DeadEnd(Point::new(5, 2)),
                &StrategyPoint::DeadEnd(Point::new(4, 1)),
            ],
        );
    }
}
