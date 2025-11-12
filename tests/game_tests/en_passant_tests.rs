#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::to_vec::ToVecRef;
use tchess::buff::Buff;
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;

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
    println!("{}", board.pp());
    assert_eq!(board.piece_at(&Point::new(3, 3)), Some(&ally_pawn));
    assert_eq!(board.to_vec(), vec![&ally_pawn], "Black pawn is expected to be captured.");
}
