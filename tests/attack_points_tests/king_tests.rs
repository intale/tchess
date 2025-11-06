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
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&king).to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&king).to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_a_protected_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(5, 5)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&king).to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.attack_points(&Color::White).get_points(&king).to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}
