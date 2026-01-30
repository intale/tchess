#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;
use std::rc::Rc;
use support::test_squares_map::TestSquaresMap;
use support::traits::{CloneMoves, ToVecRef};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};

mod rook_gets_pinned {
    use super::*;
    use libtchess::buff::Buff;
    use libtchess::dimension::Dimension;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
                board.moves_of(white_king).to_vec().clone_moves()
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
    use libtchess::buff::Buff;
    use libtchess::castle_points::CastlePoints;
    use libtchess::dimension::Dimension;
    use libtchess::piece::PieceId;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
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
                let king = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                let rook = board.find_piece_by_id(&Color::White, &PieceId(2)).unwrap();
                vec![king.current_position(), rook.current_position()]
            })
            .to(|_board| vec![Point::new(3, 1), Point::new(4, 1)]);
    }

    #[test]
    fn it_does_not_allow_another_castle() {
        expectation()
            .to_change(|board| {
                println!("{:?}", board.to_vec());
                let king = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                board.moves_of(&king).to_vec().clone_moves()
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
