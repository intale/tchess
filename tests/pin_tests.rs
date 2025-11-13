use std::rc::Rc;
use crate::support::compare;

mod support;
use tchess::board::*;
use tchess::color::Color;
use tchess::debuff::Debuff;
use tchess::pieces::Piece;
use tchess::point::Point;
use tchess::vector::diagonal_vector::DiagonalVector;
use tchess::vector::line_vector::LineVector;
use tchess::vector::Vector;

#[test]
fn when_king_and_an_ally_piece_is_on_the_attack_line() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 6)
    );
    let knight = board.add_piece(
        "Knight", Color::Black, vec![], vec![], Point::new(4, 5)
    );
    let enemy_queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(4, 2)
    );

    compare(
        &board,
        &knight.debuffs().to_vec(),
        &vec![
            Debuff::Pin(Vector::Line(LineVector::Top)),
        ]
    );

    assert_eq!(board.pins(&Color::White).all_pinned(), vec![&knight]);
    assert_eq!(board.pins(&Color::White).all_pinned_by(), vec![&enemy_queen]);

    assert_eq!(board.pins(&Color::Black).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), Vec::<&Rc<Piece>>::new());
}

#[test]
fn when_king_is_not_on_the_attack_line() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 6)
    );
    let knight = board.add_piece(
        "Knight", Color::Black, vec![], vec![], Point::new(4, 5)
    );
    board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(7, 2)
    );

    compare(&board, &knight.debuffs().to_vec(), &vec![]);

    assert_eq!(board.pins(&Color::White).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::White).all_pinned_by(), Vec::<&Rc<Piece>>::new());

    assert_eq!(board.pins(&Color::Black).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), Vec::<&Rc<Piece>>::new());
}

#[test]
fn when_there_is_another_enemy_piece_in_front_of_a_piece_to_pin() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 6)
    );
    let knight = board.add_piece(
        "Knight", Color::Black, vec![], vec![], Point::new(4, 5)
    );
    board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(4, 4)
    );
    board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(4, 2)
    );

    compare(&board, &knight.debuffs().to_vec(), &vec![]);

    assert_eq!(board.pins(&Color::White).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::White).all_pinned_by(), Vec::<&Rc<Piece>>::new());

    assert_eq!(board.pins(&Color::Black).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), Vec::<&Rc<Piece>>::new());
}

#[test]
fn when_there_is_another_enemy_piece_in_front_of_the_king() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 6)
    );
    let knight = board.add_piece(
        "Knight", Color::Black, vec![], vec![], Point::new(5, 5)
    );
    board.add_piece(
        "Knight", Color::White, vec![], vec![], Point::new(4, 4)
    );
    board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(4, 2)
    );

    compare(&board, &knight.debuffs().to_vec(), &vec![]);
    compare(&board, &king.debuffs().to_vec(), &vec![]);

    assert_eq!(board.pins(&Color::White).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::White).all_pinned_by(), Vec::<&Rc<Piece>>::new());

    assert_eq!(board.pins(&Color::Black).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), Vec::<&Rc<Piece>>::new());
}

#[test]
fn pinning_white_pawn() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let pawn = board.add_piece(
        "Pawn", Color::White, vec![], vec![], Point::new(4, 4)
    );
    let enemy_bishop = board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(6, 6)
    );

    compare(
        &board,
        &pawn.debuffs().to_vec(),
        &vec![
            Debuff::Pin(Vector::Diagonal(DiagonalVector::BottomLeft)),
        ]
    );

    assert_eq!(board.pins(&Color::White).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::White).all_pinned_by(), Vec::<&Rc<Piece>>::new());

    assert_eq!(board.pins(&Color::Black).all_pinned(), [&pawn]);
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), [&enemy_bishop]);
}

#[test]
fn when_enemy_piece_directly_attacks_the_king() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 6)
    );
    let knight = board.add_piece(
        "Knight", Color::Black, vec![], vec![], Point::new(5, 5)
    );
    board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(4, 2)
    );

    compare(&board, &knight.debuffs().to_vec(), &vec![]);
    compare(&board, &king.debuffs().to_vec(), &vec![Debuff::Check]);

    assert_eq!(board.pins(&Color::White).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::White).all_pinned_by(), Vec::<&Rc<Piece>>::new());

    assert_eq!(board.pins(&Color::Black).all_pinned(), Vec::<&Rc<Piece>>::new());
    assert_eq!(board.pins(&Color::Black).all_pinned_by(), Vec::<&Rc<Piece>>::new());
}
