#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use support::traits::{CloneMoves, ToVecRef};
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};
use tchess::board::Board;
use tchess::color::Color;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

mod rook_gets_pinned {
    use super::*;
    use std::fmt::Debug;
    use tchess::buff::Buff;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(8, 3));
        board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(4, 1),
        );
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );

        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(1, 2));

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            board.pass_turn(&Color::Black);
            let enemy_rook = Rc::clone(board.piece_at(&Point::new(1, 2)).unwrap());
            assert!(
                board.move_piece(&enemy_rook, &PieceMove::Point(Point::new(1, 1))),
                "Unable to move {:?} on a1",
                enemy_rook
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_disables_castle_move() {
        expectation()
            .to_change(|board| {
                let white_king = board.piece_at(&Point::new(4, 1)).unwrap();
                board
                    .moves(&Color::White)
                    .moves_of(white_king)
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(4, 2)),
                    PieceMove::Point(Point::new(5, 1)),
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(5, 2)),
                ]
            });
    }
}

mod castle_is_possible {
    use super::*;
    use crate::game_tests::castle_tests::support::traits::FindPiece;
    use std::fmt::Debug;
    use tchess::buff::Buff;
    use tchess::castle_points::CastlePoints;

    fn setup_board() -> Board {
        let mut board = Board::empty(Point::new(1, 1), Point::new(8, 3));
        // ID#1
        board.add_piece(
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );
        // ID#2
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(2, 1),
        );
        // ID#3
        board.add_piece(
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let king = Rc::clone(board.piece_at(&Point::new(3, 1)).unwrap());
            let castle_points = CastlePoints::new(
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(3, 1),
                Point::new(2, 1),
            );
            assert!(
                board.move_piece(&king, &PieceMove::Castle(castle_points)),
                "Could no perform queen-side castling using {:?}",
                king
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_places_pieces_on_correct_points() {
        expectation()
            .to_change(|board| {
                let king = board.find_piece_by_id(1).unwrap();
                let rook = board.find_piece_by_id(2).unwrap();
                vec![king.current_position(), rook.current_position()]
            })
            .to(|_board| vec![Point::new(3, 1), Point::new(4, 1)]);
    }

    #[test]
    fn it_does_not_allow_another_castle() {
        expectation()
            .to_change(|board| {
                println!("{:?}", board.to_vec());
                let king = board.find_piece_by_id(1).unwrap();
                board
                    .moves(&Color::White)
                    .moves_of(&king)
                    .to_vec()
                    .clone_moves()
            })
            .to(|_board| {
                vec![
                    PieceMove::Point(Point::new(2, 2)),
                    PieceMove::Point(Point::new(4, 2)),
                    PieceMove::Point(Point::new(3, 2)),
                    PieceMove::Point(Point::new(2, 1)),
                ]
            });
    }
}
