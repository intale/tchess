#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::compare;
use support::to_vec::ToVecRef;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&knight).to_vec(),
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
        &board.attack_points(&Color::White).get_points(&knight).to_vec(),
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

#[test]
fn when_there_is_a_protected_enemy_piece_on_an_attack_point() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let knight = board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 5)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(3, 4)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&knight).to_vec(),
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
        &board.attack_points(&Color::White).get_points(&knight).to_vec(),
        &vec![
            &Point::new(2, 1),
            &Point::new(1, 2),
            &Point::new(1, 4),
            &Point::new(2, 5),
            &Point::new(5, 4),
            &Point::new(5, 2),
            &Point::new(4, 1),
        ],
    );
}
