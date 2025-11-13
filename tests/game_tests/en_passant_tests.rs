#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::to_vec::ToVecRef;
use tchess::buff::Buff;
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;
use support::compare;

#[test]
fn when_enemy_pawn_crosses_attack_point_of_ally_pawn_by_performing_long_move() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    board.pass_turn(&Color::Black);
    let ally_pawn = board.add_piece(
        "Pawn", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let enemy_pawn = board.add_piece(
        "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], Point::new(3, 4)
    );

    println!("{}", board.pp());

    assert!(
        board.move_piece(&enemy_pawn, &PieceMove::LongMove(Point::new(3, 2))),
        "Unable to move {:?} on c2", enemy_pawn
    );
    println!("{}", board.pp());

    assert!(
        board.move_piece(
            &ally_pawn, &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
        ),
        "Unable to move {:?} on c3", ally_pawn
    );

    compare(&board, &board.to_vec(), &vec![&ally_pawn]);
    assert_eq!(board.piece_at(&Point::new(3, 3)), Some(&ally_pawn));
}

#[test]
fn when_enemy_piece_crosses_attack_point_of_ally_pawn() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    board.pass_turn(&Color::Black);
    let ally_pawn = board.add_piece(
        "Pawn", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let enemy_rook = board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(3, 4)
    );

    println!("{}", board.pp());

    assert!(
        board.move_piece(&enemy_rook, &PieceMove::Point(Point::new(3, 1))),
        "Unable to move {:?} on c1", enemy_rook
    );
    println!("{}", board.pp());

    assert!(
        !board.move_piece(
            &ally_pawn, &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
        ),
        "En passant must not be possible for {:?}", ally_pawn
    );

    compare(&board, &board.to_vec(), &vec![&ally_pawn, &enemy_rook]);
    assert_eq!(board.piece_at(&Point::new(2, 2)), Some(&ally_pawn));
    assert_eq!(board.piece_at(&Point::new(3, 1)), Some(&enemy_rook));
}

#[test]
fn when_enemy_pawn_steps_from_attack_point_of_ally_pawn() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    board.pass_turn(&Color::Black);
    let ally_pawn = board.add_piece(
        "Pawn", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let enemy_pawn = board.add_piece(
        "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], Point::new(3, 3)
    );

    println!("{}", board.pp());

    assert!(
        board.move_piece(&enemy_pawn, &PieceMove::Point(Point::new(3, 2))),
        "Unable to move {:?} on c2", enemy_pawn
    );
    println!("{}", board.pp());

    assert!(
        !board.move_piece(
            &ally_pawn, &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
        ),
        "En passant must not be possible for {:?}", ally_pawn
    );

    compare(&board, &board.to_vec(), &vec![&ally_pawn, &enemy_pawn]);
    assert_eq!(board.piece_at(&Point::new(2, 2)), Some(&ally_pawn));
    assert_eq!(board.piece_at(&Point::new(3, 2)), Some(&enemy_pawn));
}

#[test]
fn when_ally_pawn_does_not_utilize_en_passant_on_ally_turn() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    board.pass_turn(&Color::Black);
    let ally_pawn = board.add_piece(
        "Pawn", Color::White, vec![], vec![], Point::new(2, 2)
    );
    let ally_bishop = board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(1, 3)
    );
    let enemy_pawn = board.add_piece(
        "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], Point::new(3, 4)
    );
    let enemy_bishop = board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(2, 1)
    );

    println!("{}", board.pp());

    assert!(
        board.move_piece(&enemy_pawn, &PieceMove::LongMove(Point::new(3, 2))),
        "Unable to move {:?} on c2", enemy_pawn
    );
    println!("{}", board.pp());

    assert!(
        board.move_piece(
            &ally_bishop, &PieceMove::Point(Point::new(2, 4))
        ),
        "Unable to move {:?} on b4", ally_bishop
    );
    println!("{}", board.pp());

    assert!(
        board.move_piece(
            &enemy_bishop, &PieceMove::Point(Point::new(1, 2))
        ),
        "Unable to move {:?} on a2", enemy_bishop
    );
    println!("{}", board.pp());

    assert!(
        !board.move_piece(
            &ally_pawn, &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
        ),
        "En passant must not be possible for {:?}", ally_pawn
    );

    compare(&board, &board.to_vec(), &vec![&ally_pawn, &ally_bishop, &enemy_pawn, &enemy_bishop]);
    assert_eq!(board.piece_at(&Point::new(2, 2)), Some(&ally_pawn));
    assert_eq!(board.piece_at(&Point::new(3, 2)), Some(&enemy_pawn));
    assert_eq!(board.piece_at(&Point::new(2, 4)), Some(&ally_bishop));
    assert_eq!(board.piece_at(&Point::new(1, 2)), Some(&enemy_bishop));
}
