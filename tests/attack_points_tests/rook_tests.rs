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
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![
            &Point::new(1, 3),
            &Point::new(2, 3),
            &Point::new(3, 4),
            &Point::new(3, 5),
            &Point::new(4, 3),
            &Point::new(5, 3),
            &Point::new(3, 1),
            &Point::new(3, 2),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![
            &Point::new(1, 2),
            &Point::new(2, 3),
            &Point::new(3, 2),
            &Point::new(2, 1),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_a_an_enemy_king_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("King", Color::Black, vec![], vec![], Point::new(2, 1));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![
            &Point::new(1, 2),
            &Point::new(1, 3),
            &Point::new(2, 1),
            &Point::new(3, 1),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_on_an_attack_point() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 2));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![&Point::new(1, 2), &Point::new(2, 3), &Point::new(2, 1)],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_between_the_rook_and_an_enemy_piece() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(1, 1));

    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 1));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![&Point::new(1, 2), &Point::new(1, 3)],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_enemy_piece_between_the_rook_and_another_enemy_piece() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(1, 1));

    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 1));

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![&Point::new(1, 2), &Point::new(1, 3), &Point::new(2, 1)],
    )
    .unwrap();
}

#[test]
fn when_there_are_enemy_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of pawns around the rook
    create_box_of(
        &mut board,
        "Pawn",
        Color::Black,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(5, 5)),
    );

    println!("{}", board.pp());
    compare(
        &board
            .attack_points(&Color::White)
            .get_points(&rook)
            .to_vec(),
        &vec![
            &Point::new(1, 3),
            &Point::new(2, 3),
            &Point::new(3, 4),
            &Point::new(3, 5),
            &Point::new(4, 3),
            &Point::new(5, 3),
            &Point::new(3, 1),
            &Point::new(3, 2),
        ],
    )
    .unwrap();
}
