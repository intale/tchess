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
use support::test_squares_map::TestSquaresMap;
use support::test_heat_map::TestHeatMap;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{
    compare_and_assert, expect::Expect, expect_not_to_change_to::ExpectNotToChange,
    expect_to_change_to::ExpectToChangeTo,
};

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );
        add_piece(
            &mut board,
            "Queen",
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
            Point::new(5, 1),
        );
        add_piece(
            &mut board,
            "Bishop",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 1),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(8, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(3, &Color::White),
                PieceMove::Point(Point::new(5, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_bishop() {
        expectation()
            .to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 1)).unwrap();
                board.moves_of(white_bishop.id()).to_vec().clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(1, 2)),
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(4, 3)),
                ]
            });
    }

    #[test]
    fn it_adds_pins_to_black_bishop() {
        expectation()
            .to_change(|board| {
                let black_bishop = board.piece_at(&Point::new(7, 1)).unwrap();
                board.moves_of(black_bishop.id()).to_vec().clone_moves()
            })
            .to(|_board| vec![]);
    }
}

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin_by_pinned_piece {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Rook",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );

        add_piece(
            &mut board,
            "Rook",
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
            Point::new(4, 1),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(2, &Color::White),
                PieceMove::Point(Point::new(3, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_rook() {
        expectation()
            .to_change(|board| {
                board
                    .moves_of(&PieceId::new(2, &Color::White))
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(3, 3)),
                    PieceMove::Point(Point::new(2, 1)),
                ]
            });
    }
}

mod breaking_the_pin_by_covering_attack_points_of_the_piece_caused_the_pin {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );
        add_piece(
            &mut board,
            "Queen",
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
            Point::new(7, 1),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(8, 1),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(3, &Color::White),
                PieceMove::Point(Point::new(5, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_bishop() {
        expectation()
            .to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 1)).unwrap();
                board.moves_of(white_bishop.id()).to_vec().clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(1, 2)),
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(4, 3)),
                ]
            });
    }

    #[test]
    fn it_adds_pins_to_black_rook() {
        expectation()
            .to_change(|board| {
                let black_rook = board.piece_at(&Point::new(7, 1)).unwrap();
                board.moves_of(black_rook.id()).to_vec().clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(5, 1)),
                    PieceMove::Point(Point::new(6, 1)),
                ]
            });
    }
}

mod an_inability_to_cover_with_pinned_piece {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        add_piece(
            &mut board,
            "King",
            Color::White,
            vec![],
            vec![],
            Point::new(3, 4),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![],
            vec![],
            Point::new(3, 3),
        );

        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(3, 1),
        );
        add_piece(
            &mut board,
            "Bishop",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 1),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(2, &Color::Black),
                PieceMove::Point(Point::new(1, 2)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_to_cover_from_check_with_a_pinned_piece() {
        expectation()
            .to_change(|board| {
                let white_rook = board.piece_at(&Point::new(3, 3)).unwrap();
                board.moves_of(white_rook.id()).to_vec().clone_moves()
            })
            .to(|_board| vec![]);
    }
}

mod an_inability_to_en_passant_with_pinned_pawn {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        add_piece(
            &mut board,
            "King",
            Color::White,
            vec![],
            vec![],
            Point::new(4, 1),
        );
        add_piece(
            &mut board,
            "Pawn",
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
            Point::new(6, 8),
        );
        add_piece(
            &mut board,
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(5, 5),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::Point(Point::new(4, 8)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(2, &Color::White),
                PieceMove::Point(Point::new(4, 3)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(2, &Color::Black),
                PieceMove::LongMove(Point::new(5, 3)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_to_perform_en_passant() {
        let board = expectation::<usize>().run_expectation();
        let white_pawn = board.piece_at(&Point::new(4, 3)).unwrap();

        println!("{}", board.pp());
        let _ = compare_and_assert(
            &board.moves_of(white_pawn.id()).to_vec().clone_moves(),
            &vec![PieceMove::Point(Point::new(4, 4))],
        );
    }
}

mod reapplying_the_pin_to_the_same_piece_by_pinning_with_another_piece {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );

        add_piece(
            &mut board,
            "Queen",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 2),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 1),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(8, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::Point(Point::new(6, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_unpin_white_bishop() {
        expectation().not_to_change(|board| {
            let white_bishop = board.piece_at(&Point::new(2, 1)).unwrap();
            board.moves_of(white_bishop.id()).to_vec().clone_moves()
        });
    }
}

mod blocking_pin_path_after_reapplying_the_pin {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );
        add_piece(
            &mut board,
            "Queen",
            Color::White,
            vec![],
            vec![],
            Point::new(3, 2),
        );

        add_piece(
            &mut board,
            "Queen",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 2),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 1),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(8, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::Point(Point::new(6, 1)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(3, &Color::White),
                PieceMove::Point(Point::new(3, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_bishop() {
        expectation()
            .to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 1)).unwrap();
                board.moves_of(white_bishop.id()).to_vec().clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(1, 2)),
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(4, 3)),
                ]
            });
    }
}

mod applying_pin_to_the_ally_piece_by_moving_another_ally_piece {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 1),
        );
        add_piece(
            &mut board,
            "Queen",
            Color::White,
            vec![],
            vec![],
            Point::new(3, 1),
        );

        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(7, 1),
        );
        add_piece(
            &mut board,
            "King",
            Color::Black,
            vec![],
            vec![],
            Point::new(8, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(2, &Color::White),
                PieceMove::Point(Point::new(1, 2)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_pins_white_queen() {
        expectation()
            .to_change(|board| {
                let white_queen = board.piece_at(&Point::new(3, 1)).unwrap();
                board.moves_of(white_queen.id()).to_vec().clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(5, 1)),
                    PieceMove::Point(Point::new(7, 1)),
                    PieceMove::Point(Point::new(2, 1)),
                    PieceMove::Point(Point::new(4, 1)),
                    PieceMove::Point(Point::new(6, 1)),
                ]
            });
    }
}

mod capturing_piece_that_previously_captured_pinned_piece {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 3),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 4),
        );
        add_piece(
            &mut board,
            "Queen",
            Color::White,
            vec![],
            vec![],
            Point::new(4, 2),
        );
        add_piece(
            &mut board,
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(4, 3),
        );
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
            "King",
            Color::Black,
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
            Point::new(1, 2),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 1),
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
            "Knight",
            Color::Black,
            vec![],
            vec![],
            Point::new(2, 2),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let piece_moves = [
                (Point::new(2, 3), PieceMove::Point(Point::new(2, 2))),
                (Point::new(2, 1), PieceMove::Point(Point::new(2, 2))),
                (Point::new(4, 4), PieceMove::Point(Point::new(2, 2))),
                (Point::new(4, 1), PieceMove::Point(Point::new(2, 2))),
                (Point::new(4, 2), PieceMove::Point(Point::new(4, 1))),
                (Point::new(2, 2), PieceMove::Point(Point::new(4, 1))),
                (Point::new(2, 4), PieceMove::Point(Point::new(2, 1))),
            ];
            for (piece_position, piece_move) in piece_moves {
                let piece = board.piece_at(&piece_position).unwrap();
                move_piece(board, *piece.id(), piece_move);
                println!("{}", board.pp());
            }
        });
        expectation
    }

    #[test]
    fn it_leaves_no_legal_moves_for_black() {
        expectation()
            .to_change(|board| all_moves(board, &Color::Black))
            .to(|_board| vec![]);
    }
}
