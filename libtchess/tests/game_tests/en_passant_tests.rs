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
use support::*;
use support::{
    expect::Expect, expect_not_to_change_to::ExpectNotToChange,
    expect_to_change_to::ExpectToChangeTo,
};

mod en_passant_after_long_move {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
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
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::LongMove(Point::new(3, 2)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(1, &Color::White),
                PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_captures_via_en_passant() {
        expectation()
            .to_change(|board| {
                board
                    .active_pieces(&Color::Black)
                    .keys()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .to(|_board| vec![]);
    }

    #[test]
    fn it_places_the_en_passant_pawn_properly() {
        expectation()
            .to_change(|board| {
                let ally_pawn = board
                    .find_piece_by_id(&PieceId::new(1, &Color::White))
                    .unwrap();
                *ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }
}

mod when_enemy_piece_crosses_attack_point_of_ally_pawn {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        add_piece(
            &mut board,
            "Rook",
            Color::Black,
            vec![],
            vec![],
            Point::new(3, 4),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::Point(Point::new(3, 1)),
            );
            println!("{}", board.pp());

            assert!(
                !board.move_piece(
                    &PieceId::new(1, &Color::White),
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for white pawn!"
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant() {
        expectation()
            .not_to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect());
    }
}

mod when_enemy_pawn_steps_from_attack_point_of_ally_pawn {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.pass_turn(&Color::Black);
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        add_piece(
            &mut board,
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 3),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(|board| {
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::Point(Point::new(3, 2)),
            );
            println!("{}", board.pp());

            assert!(
                !board.move_piece(
                    &PieceId::new(1, &Color::White),
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for white pawn!"
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant() {
        expectation()
            .not_to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect());
    }
}

mod when_ally_pawn_does_not_utilize_en_passant_on_ally_turn {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        add_piece(
            &mut board,
            "Bishop",
            Color::White,
            vec![],
            vec![],
            Point::new(1, 3),
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
                PieceId::new(1, &Color::Black),
                PieceMove::LongMove(Point::new(3, 2)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(2, &Color::White),
                PieceMove::Point(Point::new(2, 4)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(2, &Color::Black),
                PieceMove::Point(Point::new(1, 2)),
            );
            println!("{}", board.pp());

            assert!(
                !board.move_piece(
                    &PieceId::new(1, &Color::White),
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for white pawn!"
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant_after_turn_passes() {
        expectation()
            .not_to_change(|board| board.active_pieces(&Color::Black).keys().copied().collect());
    }
}

mod en_passant_for_two_pawns {
    use super::*;

    fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        // ID#1
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(2, 2),
        );
        // ID#2
        add_piece(
            &mut board,
            "Pawn",
            Color::White,
            vec![],
            vec![],
            Point::new(4, 2),
        );

        // ID#3
        add_piece(
            &mut board,
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 4),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>(piece_id: usize) -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
        let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
        expectation.expect(move |board| {
            move_piece(
                board,
                PieceId::new(1, &Color::Black),
                PieceMove::LongMove(Point::new(3, 2)),
            );
            println!("{}", board.pp());

            move_piece(
                board,
                PieceId::new(piece_id as isize, &Color::White),
                PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2)),
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_is_possible_to_capture_with_the_first_pawn() {
        expectation::<Vec<Point>>(1)
            .to_change(|board| {
                board
                    .active_pieces(&Color::White)
                    .iter()
                    .map(|(_, piece)| *piece.current_position())
                    .collect()
            })
            .to(|_board| vec![Point::new(3, 3), Point::new(4, 2)]);
    }

    #[test]
    fn it_places_the_first_pawn_properly() {
        expectation(1)
            .to_change(|board| {
                let ally_pawn = board
                    .find_piece_by_id(&PieceId::new(1, &Color::White))
                    .unwrap();
                *ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }

    #[test]
    fn it_is_possible_to_capture_with_the_second_pawn() {
        expectation::<Vec<Point>>(2)
            .to_change(|board| {
                board
                    .active_pieces(&Color::White)
                    .iter()
                    .map(|(_, piece)| *piece.current_position())
                    .collect()
            })
            .to(|_board| vec![Point::new(3, 3), Point::new(2, 2)]);
    }

    #[test]
    fn it_places_the_second_pawn_properly() {
        expectation(2)
            .to_change(|board| {
                let ally_pawn = board
                    .find_piece_by_id(&PieceId::new(2, &Color::White))
                    .unwrap();
                *ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }
}
