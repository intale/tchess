#[path = "../support/mod.rs"]
mod support;

use support::traits::{FindPiece, ToVecCopy};
use support::*;
use support::{
    expect::Expect, expect_not_to_change_to::ExpectNotToChange,
    expect_to_change_to::ExpectToChangeTo,
};
use tchess::board::Board;
use tchess::color::Color;
use tchess::evaluated_move::EvaluatedMove;
use tchess::move_score::MoveScore;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::scoped_evaluated_move::ScopedEvaluatedMove;
use tchess::utils::pretty_print::PrettyPrint;

mod when_adding_new_pieces {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> =
                Expect::setup(board_default_3x3);
            expectation.expect(|board| {
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_new_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> =
                Expect::setup(board_4x4_white_computer);
            expectation.expect(|board| {
                board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_new_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![ScopedEvaluatedMove(
                        EvaluatedMove::new(
                            MoveScore::WeightDelta(-5),
                            PieceMove::Point(Point::new(2, 3)),
                        ),
                        1,
                    )]
                });
        }
    }
}

mod when_moving_a_piece {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_3x3();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let pawn = board.find_piece_by_id(1).unwrap();
                assert!(
                    board.move_piece(&pawn, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &pawn,
                    &PieceMove::Point(Point::new(2, 2))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_4x4_white_computer();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let pawn = board.find_piece_by_id(1).unwrap();
                assert!(
                    board.move_piece(&pawn, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &pawn,
                    &PieceMove::Point(Point::new(2, 2))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_moved_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![ScopedEvaluatedMove(
                        EvaluatedMove::new(
                            MoveScore::WeightDelta(-5),
                            PieceMove::Point(Point::new(2, 3)),
                        ),
                        1,
                    )]
                });
        }
    }
}

mod when_capturing_a_piece {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_3x3();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let bishop = board.find_piece_by_id(2).unwrap();
                assert!(
                    board.move_piece(&bishop, &PieceMove::Point(Point::new(2, 1))),
                    "Unable to move {} to {}",
                    &bishop,
                    &PieceMove::Point(Point::new(2, 1))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_3x3_white_computer();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let bishop = board.find_piece_by_id(2).unwrap();
                assert!(
                    board.move_piece(&bishop, &PieceMove::Point(Point::new(2, 1))),
                    "Unable to move {} to {}",
                    &bishop,
                    &PieceMove::Point(Point::new(2, 1))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_captured_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| vec![]);
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(15),
                                PieceMove::Point(Point::new(3, 2)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 2)),
                            ),
                            2,
                        ),
                    ]
                })
        }
    }
}

mod when_pinning_a_piece {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_3x3();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(3, 1))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(3, 1))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }

        #[test]
        fn it_does_not_evaluate_piece_moves_of_the_opposite_color() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_4x4_white_computer();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(3, 1))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(3, 1))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 2)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(2, 2)),
                            ),
                            2,
                        ),
                    ]
                })
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(10 + 5),
                                PieceMove::Point(Point::new(4, 1)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(100 + 5),
                                PieceMove::Point(Point::new(2, 1)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 2)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 3)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 4)),
                            ),
                            3,
                        ),
                    ]
                })
        }
    }
}

mod when_unpinning_a_piece {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_3x3();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 1));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(3, 2))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(3, 2))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }

        #[test]
        fn it_does_not_evaluate_piece_moves_of_the_opposite_color() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_4x4_white_computer();
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 1));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(3, 2))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(3, 2))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(5),
                                PieceMove::Point(Point::new(2, 2)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(505),
                                PieceMove::Point(Point::new(3, 2)),
                            ),
                            1,
                        ),
                    ]
                })
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0 + 5),
                                PieceMove::Point(Point::new(1, 2)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(2, 2)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(4, 2)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0 + 5),
                                PieceMove::Point(Point::new(3, 1)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 3)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 4)),
                            ),
                            3,
                        ),
                    ]
                })
        }
    }
}

mod when_checking_a_king {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_4x4();
            board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 3));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(2, 3))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(2, 3))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }

        #[test]
        fn it_does_not_evaluate_piece_moves_of_the_opposite_color() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_4x4_white_computer();
            board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 3));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let rook = board.find_piece_by_id(3).unwrap();
                assert!(
                    board.move_piece(&rook, &PieceMove::Point(Point::new(2, 3))),
                    "Unable to move {} to {}",
                    &rook,
                    &PieceMove::Point(Point::new(2, 3))
                );

                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(15),
                                PieceMove::Point(Point::new(2, 2)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 1)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 2)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 2)),
                            ),
                            2,
                        ),
                    ]
                })
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 3)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 3)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(4, 3)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0 + 5),
                                PieceMove::Point(Point::new(2, 4)),
                            ),
                            3,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0 + 5),
                                PieceMove::Point(Point::new(2, 2)),
                            ),
                            3,
                        ),
                    ]
                })
        }
    }
}

mod when_capturing_the_piece_caused_the_check {
    use super::*;

    mod when_evaluation_is_not_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_default_4x4();
            board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let black_rook = board.find_piece_by_id(3).unwrap();
                let white_bishop = board.find_piece_by_id(1).unwrap();

                assert!(
                    board.move_piece(&black_rook, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &black_rook,
                    &PieceMove::Point(Point::new(2, 2))
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&white_bishop, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &white_bishop,
                    &PieceMove::Point(Point::new(2, 2))
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_does_not_evaluate_piece_moves() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
        }

        #[test]
        fn it_does_not_evaluate_piece_moves_of_the_opposite_color() {
            expectation()
                .not_to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
        }
    }

    mod when_evaluation_is_required {
        use super::*;

        fn setup() -> Board {
            let mut board = board_4x4_white_computer();
            board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
            board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
            board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 2));
            board.pass_turn(&Color::Black);
            board
        }

        fn expectation() -> Expect<Vec<ScopedEvaluatedMove>, Board> {
            let mut expectation: Expect<Vec<ScopedEvaluatedMove>, Board> = Expect::setup(setup);
            expectation.expect(|board| {
                let black_rook = board.find_piece_by_id(3).unwrap();
                let white_bishop = board.find_piece_by_id(1).unwrap();

                assert!(
                    board.move_piece(&black_rook, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &black_rook,
                    &PieceMove::Point(Point::new(2, 2))
                );
                println!("{}", board.pp());

                assert!(
                    board.move_piece(&white_bishop, &PieceMove::Point(Point::new(2, 2))),
                    "Unable to move {} to {}",
                    &white_bishop,
                    &PieceMove::Point(Point::new(2, 2))
                );
                println!("{}", board.pp());
            });
            expectation
        }

        #[test]
        fn it_evaluates_piece_moves() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::White).to_vec())
                .to(|_board| {
                    vec![
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(-25),
                                PieceMove::Point(Point::new(1, 1)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 3)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(-15),
                                PieceMove::Point(Point::new(3, 1)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 3)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(20),
                                PieceMove::Point(Point::new(4, 4)),
                            ),
                            1,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 1)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(1, 2)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(0),
                                PieceMove::Point(Point::new(3, 2)),
                            ),
                            2,
                        ),
                        ScopedEvaluatedMove(
                            EvaluatedMove::new(
                                MoveScore::WeightDelta(20),
                                PieceMove::Point(Point::new(3, 1)),
                            ),
                            2,
                        ),
                    ]
                })
        }

        #[test]
        fn it_evaluates_piece_moves_of_the_opposite_color() {
            expectation()
                .to_change(|board| board.evaluated_moves_collection(&Color::Black).to_vec())
                .to(|_board| vec![])
        }
    }
}