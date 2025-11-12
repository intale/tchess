#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::compare;
use support::to_vec::ToVecRef;
use tchess::dimension::Dimension;
use tchess::piece_move::PieceMove;
use crate::available_moves_tests::knight_tests::support::create_box_of;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 5)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_an_attack_point() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(4, 5)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_are_ally_pieces_between_the_knight_and_an_enemy_piece() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );

    // A box of bishops around the knight
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(2,2), Point::new(4, 4)),
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_knight_is_pinned() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(2, 2)
    );
    board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(2, 1)
    );
    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(2, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&knight).to_vec(),
        &vec![],
    );
}
