#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::to_vec::{ToVecRef, CloneMoves};
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;
use support::Expect;

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

    mod defending_with_knight {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(6, 3));
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(4, 2)
            );
            board.add_piece(
                "Knight", Color::Black, vec![], vec![], Point::new(4, 1)
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
        fn it_limits_moves_of_enemy_knight() {
            expectation().to_change(|board| {
                let enemy_knight = board.piece_at(&Point::new(4, 1)).unwrap();
                board.moves(&Color::Black).moves_of(enemy_knight).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(2, 2)), PieceMove::Point(Point::new(3, 3))]
            });
        }
    }

    mod defending_with_pawn {
        use std::fmt::Debug;
        use tchess::buff::Buff;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(4, 1)
            );
            board.add_piece(
                "Pawn", Color::White, vec![Buff::AdditionalPoint], vec![], Point::new(3, 2)
            );

            board.add_piece(
                "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], Point::new(2, 4)
            );
            board.add_piece(
                "King", Color::Black, vec![], vec![], Point::new(1, 4)
            );
            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let ally_pawn = Rc::clone(board.piece_at(&Point::new(3, 2)).unwrap());
                assert!(
                    board.move_piece(&ally_pawn, &PieceMove::LongMove(Point::new(3, 4))),
                    "Unable to move {:?} on c4", ally_pawn
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_limits_moves_of_enemy_pawn() {
            expectation().to_change(|board| {
                let enemy_pawn = board.piece_at(&Point::new(2, 4)).unwrap();
                board.moves(&Color::Black).moves_of(enemy_pawn).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(2, 3))]
            });
        }
    }

    mod defending_with_queen {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(6, 3));
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(4, 2)
            );

            board.add_piece(
                "Queen", Color::Black, vec![], vec![], Point::new(3, 1)
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
        fn it_limits_moves_of_enemy_queen() {
            expectation().to_change(|board| {
                let enemy_queen = board.piece_at(&Point::new(3, 1)).unwrap();
                board.moves(&Color::Black).moves_of(enemy_queen).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(2, 2)), PieceMove::Point(Point::new(3, 3))]
            });
        }
    }

    mod defending_with_rook {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(6, 3));
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(4, 2)
            );

            board.add_piece(
                "Rook", Color::Black, vec![], vec![], Point::new(3, 2)
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
        fn it_limits_moves_of_enemy_rook() {
            expectation().to_change(|board| {
                let enemy_rook = board.piece_at(&Point::new(3, 2)).unwrap();
                board.moves(&Color::Black).moves_of(enemy_rook).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(2, 2)), PieceMove::Point(Point::new(3, 3))]
            });
        }
    }

    mod discovered_check {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
            board.pass_turn(&Color::Black);
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(2, 4)
            );
            board.add_piece(
                "King", Color::White, vec![], vec![], Point::new(1, 1)
            );

            board.add_piece(
                "Knight", Color::Black, vec![], vec![], Point::new(3, 3)
            );
            board.add_piece(
                "Bishop", Color::Black, vec![], vec![], Point::new(5, 5)
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let enemy_knight = Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap());
                assert!(
                    board.move_piece(&enemy_knight, &PieceMove::Point(Point::new(2, 1))),
                    "Unable to move {:?} on b1", enemy_knight
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_block_with_bishop() {
            expectation().to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 4)).unwrap();
                board.moves(&Color::White).moves_of(white_bishop).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(3, 3))]
            });
        }
    }

    mod multiple_consecutive_checks {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
            board.pass_turn(&Color::Black);
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(2, 4)
            );
            board.add_piece(
                "King", Color::White, vec![], vec![], Point::new(2, 2)
            );

            board.add_piece(
                "Knight", Color::Black, vec![], vec![], Point::new(3, 3)
            );
            board.add_piece(
                "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let enemy_knight = Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap());
                let enemy_bishop = Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap());
                let ally_king = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());

                assert!(
                    board.move_piece(&enemy_knight, &PieceMove::Point(Point::new(2, 1))),
                    "Unable to move {:?} on b1", enemy_knight
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&ally_king, &PieceMove::Point(Point::new(3, 1))),
                    "Unable to move {:?} on c1", ally_king
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&enemy_bishop, &PieceMove::Point(Point::new(5, 3))),
                    "Unable to move {:?} on e3", enemy_bishop
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_cover_with_bishop() {
            expectation().to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 4)).unwrap();
                board.moves(&Color::White).moves_of(white_bishop).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(4, 2))]
            });
        }
    }

    mod multiple_consecutive_checks_using_multiple_pieces {
        use std::fmt::Debug;
        use super::*;

        fn setup_board() -> Board {
            let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
            board.pass_turn(&Color::Black);
            board.add_piece(
                "Bishop", Color::White, vec![], vec![], Point::new(2, 4)
            );
            board.add_piece(
                "King", Color::White, vec![], vec![], Point::new(2, 2)
            );

            board.add_piece(
                "Knight", Color::Black, vec![], vec![], Point::new(3, 3)
            );
            board.add_piece(
                "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
            );

            println!("{}", board.pp());
            board
        }

        fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
            let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
            expectation.expect(|board| {
                let enemy_knight = Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap());
                let ally_king = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());

                assert!(
                    board.move_piece(&enemy_knight, &PieceMove::Point(Point::new(2, 1))),
                    "Unable to move {:?} on b1", enemy_knight
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&ally_king, &PieceMove::Point(Point::new(2, 3))),
                    "Unable to move {:?} on b3", ally_king
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&enemy_knight, &PieceMove::Point(Point::new(4, 2))),
                    "Unable to move {:?} on d2", enemy_knight
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_allows_to_capture_the_attacking_piece() {
            expectation().to_change(|board| {
                let white_bishop = board.piece_at(&Point::new(2, 4)).unwrap();
                board.moves(&Color::White).moves_of(white_bishop).to_vec().clone_moves()
            }).to(|_board| {
                vec![PieceMove::Point(Point::new(4, 2))]
            });
        }
    }
}

mod multiple_pieces_check {
    use std::fmt::Debug;
    use super::*;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
        board.add_piece(
            "Bishop", Color::White, vec![], vec![], Point::new(4, 4)
        );
        board.add_piece(
            "Knight", Color::White, vec![], vec![], Point::new(3, 3)
        );

        board.add_piece(
            "Bishop", Color::Black, vec![], vec![], Point::new(2, 4)
        );
        board.add_piece(
            "King", Color::Black, vec![], vec![], Point::new(2, 2)
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let ally_knight = Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap());
            assert!(
                board.move_piece(&ally_knight, &PieceMove::Point(Point::new(4, 1))),
                "Unable to move {:?} on d1", ally_knight
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_to_cover_double_check() {
        expectation().to_change(|board| {
            let enemy_bishop = board.piece_at(&Point::new(2, 4)).unwrap();
            board.moves(&Color::Black).moves_of(enemy_bishop).to_vec().clone_moves()
        }).to(|_board| {
            vec![]
        });
    }
}
