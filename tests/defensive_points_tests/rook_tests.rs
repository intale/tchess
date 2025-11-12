#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::compare;
use support::to_vec::ToVecRef;
use tchess::dimension::Dimension;
use support::create_box_of;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(2, 2)
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_enemy_piece_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(2, 2)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(2, 2)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(3, 2)
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![
            &Point::new(3, 2)
        ],
    );
}

#[test]
fn when_there_are_ally_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(3, 3)
    );

    // A box of pawns around the rook
    create_box_of(
        &mut board,
        "Pawn",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(1,1), Point::new(5, 5)),
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![
            &Point::new(1, 3),
            &Point::new(3, 5),
            &Point::new(5, 3),
            &Point::new(3, 1),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_between_ally_pieces() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(1, 1)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(4, 1)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(3, 1)
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_between_ally_pieces() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let rook = board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(1, 1)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(4, 1)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(3, 1)
    );

    compare(
        &board,
        &board.defensive_points(&Color::White).get_points(&rook).to_vec(),
        &vec![
            &Point::new(3, 1)
        ],
    );
}
