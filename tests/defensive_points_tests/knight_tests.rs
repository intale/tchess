#[path = "../support/mod.rs"]
mod support;

use support::create_box_of;
use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&knight)
            .to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_a_move_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&knight)
            .to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_a_move_point() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&knight)
            .to_vec(),
        &vec![&Point::new(4, 5)],
    );
}

#[test]
fn when_there_are_ally_pieces_between_the_knight_and_ally_pieces_knight_defends() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of bishops around the knight, first layer
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(2, 2), Point::new(4, 4)),
    );

    // A box of bishops around the knight, second layer
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(5, 5)),
    );

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&knight)
            .to_vec(),
        &vec![
            &Point::new(2, 1),
            &Point::new(1, 2),
            &Point::new(1, 4),
            &Point::new(2, 5),
            &Point::new(4, 5),
            &Point::new(5, 4),
            &Point::new(5, 2),
            &Point::new(4, 1),
        ],
    );
}
