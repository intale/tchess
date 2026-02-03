#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::piece::Piece;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::promote_piece::PromotePiece;
use libtchess::utils::pretty_print::PrettyPrint;
use std::fmt::Debug;
use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};

mod promote_via_move {
    use super::*;

    mod promote_to_bishop {
        use super::*;

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Bishop),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_bishop() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let bishop = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match bishop {
                        Piece::Bishop(_) => (),
                        _ => panic!("Promoted piece is not a bishop!"),
                    }
                    vec![*bishop.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let bishop = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(bishop.id()).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_knight {
        use super::*;

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Knight),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_knight() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let knight = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match knight {
                        Piece::Knight(_) => (),
                        _ => panic!("Promoted piece is not a knight!"),
                    }
                    vec![*knight.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let knight = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(knight.id()).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 1)),
                    &PieceMove::Point(Point::new(3, 1)),
                ],
            );
        }
    }

    mod promote_to_queen {
        use super::*;

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Queen),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_queen() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let queen = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match queen {
                        Piece::Queen(_) => (),
                        _ => panic!("Promoted piece is not a queen!"),
                    }
                    vec![*queen.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let queen = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(queen.id()).to_vec(),
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

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Rook),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_promotes_the_pawn_to_the_rook() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let rook = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match rook {
                        Piece::Rook(_) => (),
                        _ => panic!("Promoted piece is not a rook!"),
                    }
                    vec![*rook.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let rook = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(rook.id()).to_vec(),
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

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 2),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(2, 3),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Bishop),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect())
                .to(|_board| vec![]);
        }

        #[test]
        fn it_promotes_the_pawn_to_the_bishop() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let bishop = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match bishop {
                        Piece::Bishop(_) => (),
                        _ => panic!("Promoted piece is not a bishop!"),
                    }
                    vec![*bishop.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let bishop = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(bishop.id()).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 2)),
                    &PieceMove::Point(Point::new(3, 2)),
                ],
            );
        }
    }

    mod promote_to_knight {
        use super::*;

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 2),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(2, 3),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Knight),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect())
                .to(|_board| vec![]);
        }

        #[test]
        fn it_promotes_the_pawn_to_the_knight() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let knight = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match knight {
                        Piece::Knight(_) => (),
                        _ => panic!("Promoted piece is not a knight!"),
                    }
                    vec![*knight.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let knight = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(knight.id()).to_vec(),
                &vec![
                    &PieceMove::Point(Point::new(1, 1)),
                    &PieceMove::Point(Point::new(3, 1)),
                ],
            );
        }
    }

    mod promote_to_queen {
        use super::*;

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 2),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(2, 3),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Queen),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect())
                .to(|_board| vec![]);
        }

        #[test]
        fn it_promotes_the_pawn_to_the_queen() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let queen = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match queen {
                        Piece::Queen(_) => (),
                        _ => panic!("Promoted piece is not a queen!"),
                    }
                    vec![*queen.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let queen = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(queen.id()).to_vec(),
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

        fn setup_board() -> Board {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 2),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(2, 3),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Promote(Point::new(2, 3), PromotePiece::Rook),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_captures_enemy_piece() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect())
                .to(|_board| vec![]);
        }

        #[test]
        fn it_promotes_the_pawn_to_the_rook() {
            expectation::<Vec<PieceId>>()
                .to_change(|board| board.active_pieces(&Color::White).keys().copied().collect())
                .to(|board| {
                    let rook = board
                        .find_piece_by_id(&PieceId::new(2, &Color::White))
                        .unwrap();
                    match rook {
                        Piece::Rook(_) => (),
                        _ => panic!("Promoted piece is not a rook!"),
                    }
                    vec![*rook.id()]
                });
        }

        #[test]
        fn it_calculates_moves_of_promoted_piece_properly() {
            let board = expectation::<usize>().run_expectation();
            let rook = board
                .find_piece_by_id(&PieceId::new(2, &Color::White))
                .unwrap();
            println!("{}", board.pp());
            compare_and_assert(
                &board.moves_of(rook.id()).to_vec(),
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
