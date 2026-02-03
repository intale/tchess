#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::buff::Buff;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;
use std::fmt::Debug;
use support::test_heat_map::TestHeatMap;
use support::test_squares_map::TestSquaresMap;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};

mod single_piece_check {
    use super::*;

    mod defending_with_bishop {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(4, 2),
            );

            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 3),
            );
            add_piece(
                &mut board,
                "King",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 1),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Point(Point::new(3, 3)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_bishop() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::Black))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(2, 2))]);
        }
    }

    mod defending_with_knight {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(6, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(4, 2),
            );
            add_piece(
                &mut board,
                "Knight",
                Color::Black,
                vec![],
                vec![],
                Point::new(4, 1),
            );
            add_piece(
                &mut board,
                "King",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 1),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Point(Point::new(3, 3)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_knight() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::Black))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| {
                    vec![
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(3, 3)),
                    ]
                });
        }
    }

    mod defending_with_pawn {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(4, 1),
            );
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(3, 2),
            );

            add_piece(
                &mut board,
                "Pawn",
                Color::Black,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(2, 4),
            );
            add_piece(
                &mut board,
                "King",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 4),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(2, &Color::White),
                    PieceMove::LongMove(Point::new(3, 4)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_pawn() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::Black))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(2, 3))]);
        }
    }

    mod inability_to_defend_from_discovered_check_using_en_passant {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "King",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 2),
            );
            add_piece(
                &mut board,
                "Pawn",
                Color::White,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(2, 2),
            );

            add_piece(
                &mut board,
                "Pawn",
                Color::Black,
                vec![Buff::AdditionalPoint],
                vec![],
                Point::new(3, 4),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(4, 5),
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                board.pass_turn(&Color::Black);
                move_piece(
                    board,
                    PieceId::new(1, &Color::Black),
                    PieceMove::LongMove(Point::new(3, 2)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_allow_to_cover_with_en_passant() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(2, &Color::White))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(2, 3))]);
        }
    }

    mod defending_with_queen {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(6, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(4, 2),
            );

            add_piece(
                &mut board,
                "Queen",
                Color::Black,
                vec![],
                vec![],
                Point::new(3, 1),
            );
            add_piece(
                &mut board,
                "King",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 1),
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Point(Point::new(3, 3)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_queen() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::Black))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| {
                    vec![
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(3, 3)),
                    ]
                });
        }
    }

    mod defending_with_rook {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(6, 3));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(4, 2),
            );

            add_piece(
                &mut board,
                "Rook",
                Color::Black,
                vec![],
                vec![],
                Point::new(3, 2),
            );
            add_piece(
                &mut board,
                "King",
                Color::Black,
                vec![],
                vec![],
                Point::new(1, 1),
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::White),
                    PieceMove::Point(Point::new(3, 3)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_rook() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::Black))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| {
                    vec![
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(3, 3)),
                    ]
                });
        }
    }

    mod discovered_check {
        use super::*;
        use libtchess::piece_id::PieceId;
        use std::fmt::Debug;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            board.pass_turn(&Color::Black);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 4),
            );
            add_piece(
                &mut board,
                "King",
                Color::White,
                vec![],
                vec![],
                Point::new(1, 1),
            );

            add_piece(
                &mut board,
                "Knight",
                Color::Black,
                vec![],
                vec![],
                Point::new(3, 3),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(5, 5),
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::Black),
                    PieceMove::Point(Point::new(2, 1)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_block_with_bishop() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::White))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(3, 3))]);
        }
    }

    mod multiple_consecutive_checks {
        use super::*;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            board.pass_turn(&Color::Black);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 4),
            );
            add_piece(
                &mut board,
                "King",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );

            add_piece(
                &mut board,
                "Knight",
                Color::Black,
                vec![],
                vec![],
                Point::new(3, 3),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(4, 4),
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::Black),
                    PieceMove::Point(Point::new(2, 1)),
                );
                println!("{}", board.pp());

                move_piece(
                    board,
                    PieceId::new(2, &Color::White),
                    PieceMove::Point(Point::new(3, 1)),
                );
                println!("{}", board.pp());

                move_piece(
                    board,
                    PieceId::new(2, &Color::Black),
                    PieceMove::Point(Point::new(5, 3)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_cover_with_bishop() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::White))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(4, 2))]);
        }
    }

    mod multiple_consecutive_checks_using_multiple_pieces {
        use super::*;
        use libtchess::piece_id::PieceId;
        use std::fmt::Debug;

        fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
            let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
            let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
            let mut board = Board::empty(config);
            board.pass_turn(&Color::Black);
            add_piece(
                &mut board,
                "Bishop",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 4),
            );
            add_piece(
                &mut board,
                "King",
                Color::White,
                vec![],
                vec![],
                Point::new(2, 2),
            );

            add_piece(
                &mut board,
                "Knight",
                Color::Black,
                vec![],
                vec![],
                Point::new(3, 3),
            );
            add_piece(
                &mut board,
                "Bishop",
                Color::Black,
                vec![],
                vec![],
                Point::new(4, 4),
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
            let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
                Expect::setup(setup_board);
            expectation.expect(|board| {
                move_piece(
                    board,
                    PieceId::new(1, &Color::Black),
                    PieceMove::Point(Point::new(2, 1)),
                );
                println!("{}", board.pp());

                move_piece(
                    board,
                    PieceId::new(2, &Color::White),
                    PieceMove::Point(Point::new(2, 3)),
                );
                println!("{}", board.pp());

                move_piece(
                    board,
                    PieceId::new(1, &Color::Black),
                    PieceMove::Point(Point::new(4, 2)),
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_capture_the_attacking_piece() {
            expectation()
                .to_change(|board| {
                    board
                        .moves_of(&PieceId::new(1, &Color::White))
                        .to_vec()
                        .clone_moves()
                })
                .to(|_board| vec![PieceMove::Point(Point::new(4, 2))]);
        }
    }
}

mod multiple_pieces_check {
    use super::*;
    use libtchess::piece_id::PieceId;
    use std::fmt::Debug;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        add_piece(
            &mut board,
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(4, 4),
        );
        add_piece(
            &mut board,
            "Knight",
            Color::White,
            vec![],
            vec![],
            Point::new(3, 3),
        );

        add_piece(
            &mut board,
            "Bishop",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 4),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> =
            Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(2, &Color::White),
                PieceMove::Point(Point::new(4, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_to_cover_double_check() {
        expectation()
            .to_change(|board| {
                board
                    .moves_of(&PieceId::new(1, &Color::Black))
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| vec![]);
    }
}
