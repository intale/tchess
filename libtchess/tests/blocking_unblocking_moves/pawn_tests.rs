#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;
use std::rc::Rc;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};

mod white_pawn {
    use super::*;

    mod blocking_the_move_square {
        use super::*;

        mod when_moving_a_piece_of_the_same_color {
            use super::*;
            use libtchess::piece::PieceId;
            use std::fmt::Debug;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 2));
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    let bishop =
                        Rc::clone(board.find_piece_by_id(&Color::White, &PieceId(2)).unwrap());

                    board.move_piece(&bishop, &PieceMove::Point(Point::new(2, 3)));
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        let pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                        board.moves_of(pawn).to_vec().clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }

        mod when_moving_a_piece_of_the_opposite_color {
            use super::*;
            use libtchess::piece::PieceId;
            use std::fmt::Debug;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));
                board.pass_turn(&Color::Black);
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    let bishop =
                        Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(2)).unwrap());

                    board.move_piece(&bishop, &PieceMove::Point(Point::new(2, 3)));
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        let pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                        board.moves_of(pawn).to_vec().clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }
    }

    mod unblocking_the_move_square {
        use super::*;

        mod when_moving_a_piece_of_the_same_color {
            use super::*;
            use libtchess::piece::PieceId;
            use std::fmt::Debug;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 2));
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    let bishop =
                        Rc::clone(board.find_piece_by_id(&Color::White, &PieceId(2)).unwrap());

                    board.move_piece(&bishop, &PieceMove::Point(Point::new(2, 3)));
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        let pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                        board.moves_of(pawn).to_vec().clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }

        mod when_moving_a_piece_of_the_opposite_color {
            use super::*;
            use crate::blocking_unblocking_moves::pawn_tests::support::traits::CloneMoves;
            use libtchess::piece::PieceId;
            use std::fmt::Debug;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 3));
                board.pass_turn(&Color::Black);
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    let bishop =
                        Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(2)).unwrap());

                    board.move_piece(&bishop, &PieceMove::Point(Point::new(3, 2)));
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        let pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                        board.moves_of(pawn).to_vec().clone_moves()
                    })
                    .to(|board| vec![PieceMove::Point(Point::new(2, 3))]);
            }
        }
    }
}
