#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;
use std::fmt::Debug;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};

mod white_pawn {
    use super::*;

    mod blocking_the_move_square {
        use super::*;

        mod when_moving_a_piece_of_the_same_color {
            use super::*;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                add_piece(
                    &mut board,
                    "Pawn",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(2, 2),
                );
                add_piece(
                    &mut board,
                    "Bishop",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(3, 2),
                );
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    move_piece(
                        board,
                        PieceId::new(2, &Color::White),
                        PieceMove::Point(Point::new(2, 3)),
                    );
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        board
                            .moves_of(&PieceId::new(1, &Color::White))
                            .to_vec()
                            .clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }

        mod when_moving_a_piece_of_the_opposite_color {
            use super::*;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                add_piece(
                    &mut board,
                    "Pawn",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(2, 2),
                );
                add_piece(
                    &mut board,
                    "Bishop",
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(3, 2),
                );
                board.pass_turn(&Color::Black);
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    move_piece(
                        board,
                        PieceId::new(1, &Color::Black),
                        PieceMove::Point(Point::new(2, 3)),
                    );
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        board
                            .moves_of(&PieceId::new(1, &Color::White))
                            .to_vec()
                            .clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }
    }

    mod unblocking_the_move_square {
        use super::*;

        mod when_moving_a_piece_of_the_same_color {
            use super::*;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                add_piece(
                    &mut board,
                    "Pawn",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(2, 2),
                );
                add_piece(
                    &mut board,
                    "Bishop",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(3, 2),
                );
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    move_piece(
                        board,
                        PieceId::new(2, &Color::White),
                        PieceMove::Point(Point::new(2, 3)),
                    );
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        board
                            .moves_of(&PieceId::new(1, &Color::White))
                            .to_vec()
                            .clone_moves()
                    })
                    .to(|_board| vec![]);
            }
        }

        mod when_moving_a_piece_of_the_opposite_color {
            use super::*;

            fn setup() -> Board {
                let mut board = board_default_4x4();
                add_piece(
                    &mut board,
                    "Pawn",
                    Color::White,
                    vec![],
                    vec![],
                    Point::new(2, 2),
                );
                add_piece(
                    &mut board,
                    "Bishop",
                    Color::Black,
                    vec![],
                    vec![],
                    Point::new(2, 3),
                );
                board.pass_turn(&Color::Black);
                board
            }

            fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
                let mut expectation: Expect<T, Board> = Expect::setup(setup);
                expectation.expect(|board| {
                    move_piece(
                        board,
                        PieceId::new(1, &Color::Black),
                        PieceMove::Point(Point::new(3, 2)),
                    );
                    println!("{}", board.pp());
                });
                expectation
            }

            #[test]
            fn it_unblocks_the_move_square() {
                expectation()
                    .to_change(|board| {
                        board
                            .moves_of(&PieceId::new(1, &Color::White))
                            .to_vec()
                            .clone_moves()
                    })
                    .to(|_board| vec![PieceMove::Point(Point::new(2, 3))]);
            }
        }
    }
}
