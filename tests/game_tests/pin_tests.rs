#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::traits::{ToVecRef, CloneMoves};
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;
use support::Expect;

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin {
    use std::fmt::Debug;
    use super::*;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(8, 3));
        board.add_piece(
            "King", Color::White, vec![], vec![], Point::new(1, 1)
        );
        board.add_piece(
            "Rook", Color::White, vec![], vec![], Point::new(2, 1)
        );
        board.add_piece(
            "Queen", Color::White, vec![], vec![], Point::new(4, 2)
        );

        board.add_piece(
            "Rook", Color::Black, vec![], vec![], Point::new(5, 1)
        );
        board.add_piece(
            "Bishop", Color::Black, vec![], vec![], Point::new(7, 1)
        );
        board.add_piece(
            "King", Color::Black, vec![], vec![], Point::new(8, 1)
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_queen = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
            assert!(
                board.move_piece(&white_queen, &PieceMove::Point(Point::new(5, 1))),
                "Unable to move {:?} on e1", white_queen
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_rook() {
        expectation().to_change(|board| {
            let white_rook = board.piece_at(&Point::new(2, 1)).unwrap();
            board.moves(&Color::White).moves_of(white_rook).to_vec().clone_moves()
        }).to(|_board| {
            vec![
                PieceMove::Point(Point::new(2, 2)),
                PieceMove::Point(Point::new(3, 1)),
                PieceMove::Point(Point::new(2, 3)),
                PieceMove::Point(Point::new(4, 1)),
           ]
        });
    }

    #[test]
    fn it_adds_pins_to_black_bishop() {
        expectation().to_change(|board| {
            let black_bishop = board.piece_at(&Point::new(7, 1)).unwrap();
            board.moves(&Color::Black).moves_of(black_bishop).to_vec().clone_moves()
        }).to(|_board| {
            vec![]
        });
    }
}

mod breaking_the_pin_by_capturing_the_piece_caused_the_pin_by_pinned_piece {
    use std::fmt::Debug;
    use super::*;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(4, 3));
        board.add_piece(
            "King", Color::White, vec![], vec![], Point::new(1, 1)
        );
        board.add_piece(
            "Rook", Color::White, vec![], vec![], Point::new(2, 1)
        );

        board.add_piece(
            "Rook", Color::Black, vec![], vec![], Point::new(3, 1)
        );
        board.add_piece(
            "King", Color::Black, vec![], vec![], Point::new(4, 1)
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_rook = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());
            assert!(
                board.move_piece(&white_rook, &PieceMove::Point(Point::new(3, 1))),
                "Unable to move {:?} on c1", white_rook
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_rook() {
        expectation().to_change(|board| {
            let white_rook = board.piece_at(&Point::new(3, 1)).unwrap();
            board.moves(&Color::White).moves_of(white_rook).to_vec().clone_moves()
        }).to(|_board| {
            vec![
                PieceMove::Point(Point::new(3, 2)),
                PieceMove::Point(Point::new(3, 3)),
                PieceMove::Point(Point::new(2, 1)),
            ]
        });
    }
}

mod breaking_the_pin_by_covering_attack_points_of_the_piece_caused_the_pin {
    use std::fmt::Debug;
    use super::*;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(8, 3));
        board.add_piece(
            "King", Color::White, vec![], vec![], Point::new(1, 1)
        );
        board.add_piece(
            "Rook", Color::White, vec![], vec![], Point::new(2, 1)
        );
        board.add_piece(
            "Queen", Color::White, vec![], vec![], Point::new(4, 2)
        );

        board.add_piece(
            "Rook", Color::Black, vec![], vec![], Point::new(7, 1)
        );
        board.add_piece(
            "King", Color::Black, vec![], vec![], Point::new(8, 1)
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let white_queen = Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap());
            assert!(
                board.move_piece(&white_queen, &PieceMove::Point(Point::new(5, 1))),
                "Unable to move {:?} on e1", white_queen
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_unpins_white_rook() {
        expectation().to_change(|board| {
            let white_rook = board.piece_at(&Point::new(2, 1)).unwrap();
            board.moves(&Color::White).moves_of(white_rook).to_vec().clone_moves()
        }).to(|_board| {
            vec![
                PieceMove::Point(Point::new(2, 2)),
                PieceMove::Point(Point::new(3, 1)),
                PieceMove::Point(Point::new(2, 3)),
                PieceMove::Point(Point::new(4, 1)),
            ]
        });
    }

    #[test]
    fn it_adds_pins_to_black_rook() {
        expectation().to_change(|board| {
            let black_rook = board.piece_at(&Point::new(7, 1)).unwrap();
            board.moves(&Color::Black).moves_of(black_rook).to_vec().clone_moves()
        }).to(|_board| {
            vec![
                PieceMove::Point(Point::new(5, 1)),
                PieceMove::Point(Point::new(6, 1)),
            ]
        });
    }
}

mod an_inability_to_cover_with_pinned_piece {
    use std::fmt::Debug;
    use super::*;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
        board.pass_turn(&Color::Black);
        board.add_piece(
            "King", Color::White, vec![], vec![], Point::new(3, 4)
        );
        board.add_piece(
            "Rook", Color::White, vec![], vec![], Point::new(3, 3)
        );

        board.add_piece(
            "Rook", Color::Black, vec![], vec![], Point::new(3, 1)
        );
        board.add_piece(
            "Bishop", Color::Black, vec![], vec![], Point::new(2, 1)
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let black_bishop = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());
            assert!(
                board.move_piece(&black_bishop, &PieceMove::Point(Point::new(1, 2))),
                "Unable to move {:?} on a1", black_bishop
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_to_cover_from_check_with_a_pinned_piece() {
        expectation().to_change(|board| {
            let white_rook = board.piece_at(&Point::new(3, 3)).unwrap();
            board.moves(&Color::White).moves_of(white_rook).to_vec().clone_moves()
        }).to(|_board| {
            vec![]
        });
    }
}
