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
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&bishop)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(2, 2),
            &Point::new(4, 4),
            &Point::new(5, 5),
            &Point::new(1, 5),
            &Point::new(2, 4),
            &Point::new(4, 2),
            &Point::new(5, 1),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_a_an_enemy_pieces_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&bishop)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 3),
            &Point::new(3, 3),
            &Point::new(3, 1),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&bishop)
            .to_vec(),
        &vec![&Point::new(1, 1), &Point::new(1, 3), &Point::new(3, 1)],
    )
    .unwrap();
}

#[test]
fn when_there_is_a_an_enemy_king_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("King", Color::Black, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&bishop)
            .to_vec(),
        &vec![&Point::new(2, 2), &Point::new(3, 3)],
    )
    .unwrap();
}

#[test]
fn when_there_are_enemy_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));

    // A box of pawns around the bishop
    create_box_of(
        &mut board,
        "Pawn",
        Color::Black,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(3, 3)),
    );

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&bishop)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(3, 3),
            &Point::new(3, 1),
            &Point::new(1, 3),
        ],
    )
    .unwrap();
}
