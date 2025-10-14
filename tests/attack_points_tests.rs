mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use crate::support::compare;
use crate::support::to_vec::ToVecRef;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let bishop = board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(2, 2)
    );

    compare(
        &board,
        &board.get_attack_points(&Color::White).get_points(&bishop).to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 3),
            &Point::new(3, 3),
            &Point::new(3, 1),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let bishop = board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let enemy_bishop1 = board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(3, 3)
    );
    let enemy_bishop2 = board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.get_attack_points(&Color::White).get_points(&bishop).to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 3),
            &Point::new(3, 3),
            &Point::new(3, 1),
        ],
    );
    compare(
        &board,
        &board.get_attack_points(&Color::Black).get_points(&enemy_bishop1).to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 4),
            &Point::new(4, 2),
        ],
    );
    compare(
        &board,
        &board.get_attack_points(&Color::Black).get_points(&enemy_bishop2).to_vec(),
        &vec![],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let bishop1 = board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let bishop2 = board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(3, 3)
    );
    let enemy_bishop = board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.get_attack_points(&Color::White).get_points(&bishop1).to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 3),
            &Point::new(3, 1),
        ],
    );
    compare(
        &board,
        &board.get_attack_points(&Color::White).get_points(&bishop2).to_vec(),
        &vec![
            &Point::new(2, 4),
            &Point::new(4, 2),
            &Point::new(4, 4),
        ],
    );
    compare(
        &board,
        &board.get_attack_points(&Color::Black).get_points(&enemy_bishop).to_vec(),
        &vec![
            &Point::new(3, 3),
        ],
    );
}
