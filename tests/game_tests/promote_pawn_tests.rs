#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use support::traits::{ClonePieces, FindPiece, ToVecRef};
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo, compare_and_assert};
use tchess::board::Board;
use tchess::color::Color;
use tchess::piece_move::PieceMove;
use tchess::pieces::Piece;
use tchess::point::Point;
use tchess::promote_piece::PromotePiece;
use tchess::utils::pretty_print::PrettyPrint;

mod promote_via_move {
    use super::*;

    mod promote_to_bishop {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Bishop)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_bishop() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let bishop = board.find_piece_by_id(2).unwrap();
                    match *bishop {
                        Piece::Bishop(_) => (),
                        _ => panic!("Promoted piece is not a bishop!"),
                    }
                    vec![bishop]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let bishop = board.find_piece_by_id(2).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&bishop).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_knight {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Knight)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_knight() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let knight = board.find_piece_by_id(2).unwrap();
                    match *knight {
                        Piece::Knight(_) => (),
                        _ => panic!("Promoted piece is not a knight!"),
                    }
                    vec![knight]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let knight = board.find_piece_by_id(2).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&knight).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 1)),
                    &PieceMove::Point(Point::new(3, 1)),
                ],
            );
        }
    }

    mod promote_to_queen {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Queen)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_queen() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let queen = board.find_piece_by_id(2).unwrap();
                    match *queen {
                        Piece::Queen(_) => (),
                        _ => panic!("Promoted piece is not a queen!"),
                    }
                    vec![queen]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let queen = board.find_piece_by_id(2).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&queen).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(2, 2)),
                    &PieceMove::Point(Point::new(2, 1)),
                    &PieceMove::Point(Point::new(3, 3)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_rook {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Rook)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_rook() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let rook = board.find_piece_by_id(2).unwrap();
                    match *rook {
                        Piece::Rook(_) => (),
                        _ => panic!("Promoted piece is not a rook!"),
                    }
                    vec![rook]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let rook = board.find_piece_by_id(2).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&rook).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(3, 3)),
                    &PieceMove::Point(Point::new(2, 2)),
                    &PieceMove::Point(Point::new(2, 1)),
                ],
            );
        }
    }
}

mod promote_via_capturing {
    use super::*;

    mod promote_to_bishop {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(1, 2));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 3));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(1, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Bishop)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece_and_promotes_the_pawn_to_the_bishop() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let bishop = board.find_piece_by_id(3).unwrap();
                    match *bishop {
                        Piece::Bishop(_) => (),
                        _ => panic!("Promoted piece is not a bishop!"),
                    }
                    vec![bishop]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let bishop = board.find_piece_by_id(3).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&bishop).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_knight {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(1, 2));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 3));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(1, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Knight)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece_and_promotes_the_pawn_to_the_knight() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let knight = board.find_piece_by_id(3).unwrap();
                    match *knight {
                        Piece::Knight(_) => (),
                        _ => panic!("Promoted piece is not a knight!"),
                    }
                    vec![knight]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let knight = board.find_piece_by_id(3).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&knight).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 1)),
                    &PieceMove::Point(Point::new(3, 1)),
                ],
            );
        }
    }

    mod promote_to_queen {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(1, 2));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 3));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(1, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Queen)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece_and_promotes_the_pawn_to_the_queen() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let queen = board.find_piece_by_id(3).unwrap();
                    match *queen {
                        Piece::Queen(_) => (),
                        _ => panic!("Promoted piece is not a queen!"),
                    }
                    vec![queen]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let queen = board.find_piece_by_id(3).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&queen).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(2, 2)),
                    &PieceMove::Point(Point::new(2, 1)),
                    &PieceMove::Point(Point::new(3, 3)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_rook {
        use super::*;
        use std::fmt::Debug;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(1, 2));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 3));
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let pawn = Rc::clone(board.piece_at(&Point::new(1, 2)).unwrap());
                assert!(
                    board.move_piece(
                        &pawn,
                        &PieceMove::Promote(Point::new(2, 3), PromotePiece::Rook)
                    ),
                    "Unable to move {:?} on b3",
                    pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece_and_promotes_the_pawn_to_the_rook() {
            expectation()
                .to_change(|board| board.to_vec().clone_pieces())
                .to(|board| {
                    let rook = board.find_piece_by_id(3).unwrap();
                    match *rook {
                        Piece::Rook(_) => (),
                        _ => panic!("Promoted piece is not a rook!"),
                    }
                    vec![rook]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let rook = board.find_piece_by_id(3).unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves(&Color::White).moves_of(&rook).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 3)),
                    &PieceMove::Point(Point::new(3, 3)),
                    &PieceMove::Point(Point::new(2, 2)),
                    &PieceMove::Point(Point::new(2, 1)),
                ],
            );
        }
    }
}
