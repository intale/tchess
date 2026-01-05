#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use support::test_squares_map::TestSquaresMap;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{
    compare_and_assert, expect::Expect, expect_not_to_change_to::ExpectNotToChange,
    expect_to_change_to::ExpectToChangeTo,
};
use tchess::board::Board;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin {
    use super::*;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Queen", Color::White, vec![], vec![], Point::new(4, 2));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(5, 1));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(7, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(8, 1));

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_queen = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
            assert!(
                board.move_piece(&white_queen, &PieceMove::Point(Point::new(5, 1))),
                "Unable to move {:?} on e1",
                white_queen
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
                board
                    .moves(&Color::White)
                    .moves_of(white_bishop)
                    .to_vec()
                    .clone_moves()
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
                board
                    .moves(&Color::Black)
                    .moves_of(black_bishop)
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| vec![]);
    }
}

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin_by_pinned_piece {
    use super::*;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 1));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(4, 1));
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_rook = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());
            assert!(
                board.move_piece(&white_rook, &PieceMove::Point(Point::new(3, 1))),
                "Unable to move {:?} on c1",
                white_rook
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_rook() {
        expectation()
            .to_change(|board| {
                let white_rook = board.piece_at(&Point::new(3, 1)).unwrap();
                board
                    .moves(&Color::White)
                    .moves_of(white_rook)
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
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Queen", Color::White, vec![], vec![], Point::new(4, 2));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(7, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(8, 1));
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_queen = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
            assert!(
                board.move_piece(&white_queen, &PieceMove::Point(Point::new(5, 1))),
                "Unable to move {:?} on e1",
                white_queen
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
                board
                    .moves(&Color::White)
                    .moves_of(white_bishop)
                    .to_vec()
                    .clone_moves()
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
                board
                    .moves(&Color::Black)
                    .moves_of(black_rook)
                    .to_vec()
                    .clone_moves()
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
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.pass_turn(&Color::Black);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 4));
        board.add_piece("Rook", Color::White, vec![], vec![], Point::new(3, 3));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 1));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 1));
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let black_bishop = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());
            assert!(
                board.move_piece(&black_bishop, &PieceMove::Point(Point::new(1, 2))),
                "Unable to move {:?} on a1",
                black_bishop
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
                board
                    .moves(&Color::White)
                    .moves_of(white_rook)
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| vec![]);
    }
}

mod an_inability_to_en_passant_with_pinned_pawn {
    use super::*;
    use std::fmt::Debug;
    use tchess::buff::Buff;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(4, 1));
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(4, 2));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(6, 8));
        board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(5, 5),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            let black_rook = Rc::clone(board.piece_at(&Point::new(6, 8)).unwrap());
            let white_pawn = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
            let black_pawn = Rc::clone(board.piece_at(&Point::new(5, 5)).unwrap());

            assert!(
                board.move_piece(&black_rook, &PieceMove::Point(Point::new(4, 8))),
                "Unable to move {:?} on d8",
                black_rook
            );
            println!("{}", board.pp());

            assert!(
                board.move_piece(&white_pawn, &PieceMove::Point(Point::new(4, 3))),
                "Unable to move {:?} on d3",
                white_pawn
            );
            println!("{}", board.pp());

            assert!(
                board.move_piece(&black_pawn, &PieceMove::LongMove(Point::new(5, 3))),
                "Unable to move {:?} on e3",
                black_pawn
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
            &board
                .moves(&Color::White)
                .moves_of(white_pawn)
                .to_vec()
                .clone_moves(),
            &vec![PieceMove::Point(Point::new(4, 4))],
        );
    }
}

mod reapplying_the_pin_to_the_same_piece_by_pinning_with_another_piece {
    use super::*;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));

        board.add_piece("Queen", Color::Black, vec![], vec![], Point::new(7, 2));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(7, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(8, 1));

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            let black_queen = Rc::clone(board.piece_at(&Point::new(7, 2)).unwrap());

            assert!(
                board.move_piece(&black_queen, &PieceMove::Point(Point::new(6, 1))),
                "Unable to move {:?} on f1",
                black_queen
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_unpin_white_bishop() {
        expectation().not_to_change(|board| {
            let white_bishop = board.piece_at(&Point::new(2, 1)).unwrap();
            board
                .moves(&Color::White)
                .moves_of(white_bishop)
                .to_vec()
                .clone_moves()
        });
    }
}

mod blocking_pin_path_after_reapplying_the_pin {
    use super::*;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 2));

        board.add_piece("Queen", Color::Black, vec![], vec![], Point::new(7, 2));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(7, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(8, 1));

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            let black_queen = Rc::clone(board.piece_at(&Point::new(7, 2)).unwrap());
            let white_queen = Rc::clone(board.piece_at(&Point::new(3, 2)).unwrap());

            assert!(
                board.move_piece(&black_queen, &PieceMove::Point(Point::new(6, 1))),
                "Unable to move {:?} on f1",
                black_queen
            );
            println!("{}", board.pp());

            assert!(
                board.move_piece(&white_queen, &PieceMove::Point(Point::new(3, 1))),
                "Unable to move {:?} on c1",
                white_queen
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
                board
                    .moves(&Color::White)
                    .moves_of(white_bishop)
                    .to_vec()
                    .clone_moves()
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
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 1));

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(7, 1));
        board.add_piece("King", Color::Black, vec![], vec![], Point::new(8, 1));

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_bishop = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());

            assert!(
                board.move_piece(&white_bishop, &PieceMove::Point(Point::new(1, 2))),
                "Unable to move {:?} on a2",
                white_bishop
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
                board
                    .moves(&Color::White)
                    .moves_of(white_queen)
                    .to_vec()
                    .clone_moves()
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
