#[path = "../support/mod.rs"]
mod support;

use support::create_box_of;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::board_square_builders::{
    BoardSquareBuilder, default_square_builder::DefaultSquareBuilder,
};
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(3, 3),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_enemy_piece_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(3, 3),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(3, 3),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![&Point::new(3, 3)],
    );
}

#[test]
fn when_there_are_ally_pieces_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(5, 5),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of pawns around the rook
    create_box_of(
        &mut board,
        "Pawn",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(5, 5)),
    );

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 3),
            &Point::new(1, 5),
            &Point::new(3, 5),
            &Point::new(5, 5),
            &Point::new(5, 3),
            &Point::new(5, 1),
            &Point::new(3, 1),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_between_ally_pieces() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_between_ally_pieces() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));
    board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .defensive_points(&Color::White)
            .get_points(&queen)
            .to_vec(),
        &vec![&Point::new(3, 3)],
    );
}
