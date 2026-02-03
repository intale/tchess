#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::buff::Buff;
use libtchess::castle_points::CastlePoints;
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

mod rook_gets_pinned {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        add_piece(
            &mut board,
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(4, 1),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );

        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(1, 2),
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
                PieceMove::Point(Point::new(1, 1)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_disables_castle_move() {
        expectation()
            .to_change(|board| {
                board
                    .moves_of(&PieceId::new(1, &Color::White))
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

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 3));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        // ID#1
        add_piece(
            &mut board,
            "King",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(3, 1),
        );
        // ID#2
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(2, 1),
        );
        // ID#3
        add_piece(
            &mut board,
            "Rook",
            Color::White,
            vec![Buff::Castle],
            vec![],
            Point::new(1, 1),
        );

        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let castle_points = CastlePoints::new(
                Point::new(3, 1),
                Point::new(4, 1),
                Point::new(3, 1),
                Point::new(2, 1),
            );
            move_piece(
                board,
                PieceId::new(1, &Color::White),
                PieceMove::Castle(castle_points),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_places_pieces_on_correct_points() {
        expectation()
            .to_change(|board| {
                let king = board
                    .find_piece_by_id(&PieceId::new(1, &Color::White))
                    .unwrap();
                let rook = board
                    .find_piece_by_id(&PieceId::new(2, &Color::White))
                    .unwrap();
                vec![*king.current_position(), *rook.current_position()]
            })
            .to(|_board| vec![Point::new(3, 1), Point::new(4, 1)]);
    }

    #[test]
    fn it_does_not_allow_another_castle() {
        expectation()
            .to_change(|board| {
                board
                    .moves_of(&PieceId::new(1, &Color::White))
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
