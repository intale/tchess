#[path = "../support/mod.rs"]
mod support;

use support::compare;
use support::create_box_of;
use support::traits::ToVecRef;
use tchess::board::Board;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_enemy_piece_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![&Point::new(3, 3)],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_too_far_from_king() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![],
    )
    .unwrap();
}

#[test]
fn when_there_are_ally_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));

    // A box of bishops around the king
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(3, 3)),
    );

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 2),
            &Point::new(1, 3),
            &Point::new(2, 3),
            &Point::new(3, 3),
            &Point::new(3, 2),
            &Point::new(3, 1),
            &Point::new(2, 1),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_enemy_piece_between_ally_pieces() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_between_ally_pieces() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));
    board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .defensive_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![&Point::new(3, 3)],
    )
    .unwrap();
}
