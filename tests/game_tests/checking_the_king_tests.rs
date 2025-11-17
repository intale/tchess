#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::to_vec::{ToVecRef, ClonePieces, CloneMoves};
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;
use support::Expect;
use tchess::pieces::Piece;

mod single_piece_check {
    use super::*;

    mod defending_with_bishop {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(4, 2)
            );
            board.add_piece(
                "Bishop", Color::Black, vec![], vec![], Point::new(1, 3)
            );
            board.add_piece(
                "King", Color::Black, vec![], vec![], Point::new(1, 1)
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let ally_bishop = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
                assert!(
                    board.move_piece(&ally_bishop, &PieceMove::Point(Point::new(3, 3))),
                    "Unable to move {:?} on c3", ally_bishop
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_bishop() {
            expectation().to_change(|board| {
                let enemy_bishop = board.piece_at(&Point::new(1, 3)).unwrap();
                board.moves(&Color::Black).moves_of(enemy_bishop).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(2, 2))]
            });
        }
    }
}
