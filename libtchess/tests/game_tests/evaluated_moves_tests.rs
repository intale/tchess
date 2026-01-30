#[path = "../support/mod.rs"]
mod support;

use support::traits::{ToVecCopy};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo, scored_moves::ScoredMoves};
use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::move_score::MoveScore;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;

mod when_adding_new_pieces {
    use super::*;

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(board_4x4_white_computer);
        expectation.expect(|board| {
            board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_updates_move_scores_collection() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(-5)]);
    }

    #[test]
    fn it_evaluates_new_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                if let Some(pawn) = board.piece_at(&Point::new(2, 2)) {
                    scored_moves_of(board, vec![pawn])
                } else {
                    vec![]
                }
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "Pawn",
                    Point::new(2, 2),
                    MoveScore::WeightDelta(-5),
                    vec![PieceMove::Point(Point::new(2, 3))],
                )]
            });
    }
}

mod when_moving_a_piece {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_4x4_white_computer();
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let pawn = Rc::clone(board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap());
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
    fn it_updates_move_scores_collection() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(-5)]);
    }

    #[test]
    fn it_evaluates_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let pawn = board
                    .piece_at(&Point::new(2, 2))
                    .unwrap_or_else(|| board.piece_at(&Point::new(2, 1)).unwrap());
                scored_moves_of(board, vec![pawn])
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "Pawn",
                    Point::new(2, 2),
                    MoveScore::WeightDelta(-5),
                    vec![PieceMove::Point(Point::new(2, 3))],
                )]
            });
    }
}

mod when_capturing_a_piece {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_3x3_white_computer();
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));
        board.pass_turn(&Color::Black);
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let bishop = Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(2)).unwrap());
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
    fn it_updates_white_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![]);
    }

    #[test]
    fn it_updates_black_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::Black).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0), MoveScore::WeightDelta(15)]);
    }

    #[test]
    fn it_evaluates_white_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                if let Some(pawn) = board.piece_at(&Point::new(2, 1))
                    && pawn.name() == "Pawn"
                {
                    scored_moves_of(board, vec![pawn])
                } else {
                    vec![]
                }
            })
            .to(|_board| vec![])
    }

    #[test]
    fn it_evaluates_black_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let bishop = board
                    .piece_at(&Point::new(2, 1))
                    .unwrap_or_else(|| board.piece_at(&Point::new(3, 2)).unwrap());
                scored_moves_of(board, vec![bishop])
            })
            .to(|_board| {
                vec![
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 1),
                        MoveScore::WeightDelta(0),
                        vec![PieceMove::Point(Point::new(1, 2))],
                    ),
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 1),
                        MoveScore::WeightDelta(15),
                        vec![PieceMove::Point(Point::new(3, 2))],
                    ),
                ]
            })
    }
}

mod when_pinning_a_piece {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_4x4_white_computer();
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 2));
        board.pass_turn(&Color::Black);
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let rook = Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(3)).unwrap());
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
    fn it_updates_white_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0)]);
    }

    #[test]
    fn it_updates_black_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::Black).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0), MoveScore::WeightDelta(10)]);
    }

    #[test]
    fn it_evaluates_white_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let pawn = board.piece_at(&Point::new(2, 1)).unwrap();
                let king = board.piece_at(&Point::new(1, 1)).unwrap();
                scored_moves_of(board, vec![pawn, king])
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "King",
                    Point::new(1, 1),
                    MoveScore::WeightDelta(0),
                    vec![
                        PieceMove::Point(Point::new(1, 2)),
                        PieceMove::Point(Point::new(2, 2)),
                    ],
                )]
            })
    }

    #[test]
    fn it_evaluates_black_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let rook = board
                    .piece_at(&Point::new(3, 1))
                    .unwrap_or_else(|| board.piece_at(&Point::new(3, 2)).unwrap());
                scored_moves_of(board, vec![rook])
            })
            .to(|_board| {
                vec![
                    ScoredMoves::new(
                        "Rook",
                        Point::new(3, 1),
                        MoveScore::WeightDelta(0),
                        vec![
                            PieceMove::Point(Point::new(2, 1)),
                            PieceMove::Point(Point::new(3, 2)),
                            PieceMove::Point(Point::new(3, 3)),
                            PieceMove::Point(Point::new(3, 4)),
                        ],
                    ),
                    ScoredMoves::new(
                        "Rook",
                        Point::new(3, 1),
                        MoveScore::WeightDelta(10),
                        vec![PieceMove::Point(Point::new(4, 1))],
                    ),
                ]
            })
    }
}

mod when_unpinning_a_piece {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_4x4_white_computer();
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 1));
        board.pass_turn(&Color::Black);
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let rook = Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(3)).unwrap());
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
    fn it_updates_white_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(5)]);
    }

    #[test]
    fn it_updates_black_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::Black).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0)]);
    }

    #[test]
    fn it_evaluates_white_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let pawn = board.piece_at(&Point::new(2, 1)).unwrap();
                let king = board.piece_at(&Point::new(1, 1)).unwrap();
                scored_moves_of(board, vec![pawn, king])
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "Pawn",
                    Point::new(2, 1),
                    MoveScore::WeightDelta(5),
                    vec![
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(3, 2)),
                    ],
                )]
            })
    }

    #[test]
    fn it_evaluates_black_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let rook = board
                    .piece_at(&Point::new(3, 2))
                    .unwrap_or_else(|| board.piece_at(&Point::new(3, 1)).unwrap());
                scored_moves_of(board, vec![rook])
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "Rook",
                    Point::new(3, 2),
                    MoveScore::WeightDelta(0),
                    vec![
                        PieceMove::Point(Point::new(3, 1)),
                        PieceMove::Point(Point::new(1, 2)),
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(4, 2)),
                        PieceMove::Point(Point::new(3, 3)),
                        PieceMove::Point(Point::new(3, 4)),
                    ],
                )]
            })
    }
}

mod when_checking_the_king {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_4x4_white_computer();
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
        board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 3));
        board.pass_turn(&Color::Black);
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let rook = Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(3)).unwrap());
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
    fn it_updates_white_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0), MoveScore::WeightDelta(15)]);
    }

    #[test]
    fn it_updates_black_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::Black).to_vec())
            .to(|_board| vec![MoveScore::WeightDelta(0)]);
    }

    #[test]
    fn it_evaluates_white_piece_moves() {
        expectation()
            .to_change(|board| {
                let bishop = board.piece_at(&Point::new(3, 1)).unwrap();
                let king = board.piece_at(&Point::new(2, 1)).unwrap();
                scored_moves_of(board, vec![bishop, king])
            })
            .to(|_board| {
                vec![
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(3, 1),
                        MoveScore::WeightDelta(15),
                        vec![PieceMove::Point(Point::new(2, 2))],
                    ),
                    ScoredMoves::new(
                        "King",
                        Point::new(2, 1),
                        MoveScore::WeightDelta(0),
                        vec![
                            PieceMove::Point(Point::new(1, 1)),
                            PieceMove::Point(Point::new(1, 2)),
                            PieceMove::Point(Point::new(3, 2)),
                        ],
                    ),
                ]
            })
    }

    #[test]
    fn it_evaluates_black_piece_moves() {
        expectation()
            .to_change(|board| {
                let rook = board
                    .piece_at(&Point::new(2, 3))
                    .unwrap_or_else(|| board.piece_at(&Point::new(4, 3)).unwrap());
                scored_moves_of(board, vec![rook])
            })
            .to(|_board| {
                vec![ScoredMoves::new(
                    "Rook",
                    Point::new(2, 3),
                    MoveScore::WeightDelta(0),
                    vec![
                        PieceMove::Point(Point::new(1, 3)),
                        PieceMove::Point(Point::new(3, 3)),
                        PieceMove::Point(Point::new(4, 3)),
                        PieceMove::Point(Point::new(2, 2)),
                        PieceMove::Point(Point::new(2, 4)),
                    ],
                )]
            })
    }
}

mod when_capturing_the_piece_caused_the_check {
    use std::rc::Rc;
    use libtchess::piece::PieceId;
    use super::*;

    fn setup() -> Board {
        let mut board = board_4x4_white_computer();
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));
        board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(4, 2));
        board.pass_turn(&Color::Black);
        board
    }

    fn expectation<T: std::fmt::Debug + PartialEq>() -> Expect<Vec<T>, Board> {
        let mut expectation: Expect<Vec<T>, Board> = Expect::setup(setup);
        expectation.expect(|board| {
            let black_rook = Rc::clone(board.find_piece_by_id(&Color::Black, &PieceId(3)).unwrap());
            let white_bishop = Rc::clone(board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap());

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
    fn it_updates_white_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::White).to_vec())
            .to(|_board| {
                vec![
                    MoveScore::WeightDelta(-25),
                    MoveScore::WeightDelta(-15),
                    MoveScore::WeightDelta(0),
                    MoveScore::WeightDelta(20),
                ]
            });
    }

    #[test]
    fn it_updates_black_move_scores() {
        expectation::<MoveScore>()
            .to_change(|board| board.move_scores(&Color::Black).to_vec())
            .to(|_board| vec![]);
    }

    #[test]
    fn it_evaluates_white_piece_moves() {
        expectation::<ScoredMoves>()
            .to_change(|board| {
                let bishop = board
                    .piece_at(&Point::new(2, 2))
                    .unwrap_or_else(|| board.piece_at(&Point::new(3, 1)).unwrap());
                let king = board.piece_at(&Point::new(2, 1)).unwrap();
                scored_moves_of(board, vec![bishop, king])
            })
            .to(|_board| {
                vec![
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 2),
                        MoveScore::WeightDelta(-25),
                        vec![PieceMove::Point(Point::new(1, 1))],
                    ),
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 2),
                        MoveScore::WeightDelta(-15),
                        vec![PieceMove::Point(Point::new(3, 1))],
                    ),
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 2),
                        MoveScore::WeightDelta(0),
                        vec![
                            PieceMove::Point(Point::new(1, 3)),
                            PieceMove::Point(Point::new(3, 3)),
                        ],
                    ),
                    ScoredMoves::new(
                        "Bishop",
                        Point::new(2, 2),
                        MoveScore::WeightDelta(20),
                        vec![PieceMove::Point(Point::new(4, 4))],
                    ),
                    ScoredMoves::new(
                        "King",
                        Point::new(2, 1),
                        MoveScore::WeightDelta(0),
                        vec![
                            PieceMove::Point(Point::new(1, 1)),
                            PieceMove::Point(Point::new(1, 2)),
                            PieceMove::Point(Point::new(3, 2)),
                        ],
                    ),
                    ScoredMoves::new(
                        "King",
                        Point::new(2, 1),
                        MoveScore::WeightDelta(20),
                        vec![PieceMove::Point(Point::new(3, 1))],
                    ),
                ]
            })
    }

    #[test]
    fn it_evaluates_black_piece_moves() {
        expectation()
            .to_change(|board| {
                if let Some(rook) = board.piece_at(&Point::new(4, 2)) {
                    scored_moves_of(board, vec![rook])
                } else {
                    vec![]
                }
            })
            .to(|_board| vec![])
    }
}
