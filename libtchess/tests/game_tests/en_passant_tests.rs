#[path = "../support/mod.rs"]
mod support;

use libtchess::board::Board;
use libtchess::buff::Buff;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::piece::Piece;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;
use std::rc::Rc;
use support::test_squares_map::TestSquaresMap;
use support::traits::{ClonePieces, ToVecRef};
use support::*;
use support::{
    expect::Expect, expect_not_to_change_to::ExpectNotToChange,
    expect_to_change_to::ExpectToChangeTo,
};

mod en_passant_after_long_move {
    use super::*;
    use libtchess::piece::PieceId;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        // ID#1
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        // ID#2
        board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 4),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let enemy_pawn = Rc::clone(board.piece_at(&Point::new(3, 4)).unwrap());
            assert!(
                board.move_piece(&enemy_pawn, &PieceMove::LongMove(Point::new(3, 2))),
                "Unable to move {:?} on c2",
                enemy_pawn
            );
            println!("{}", board.pp());

            let ally_pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
            assert!(
                board.move_piece(
                    &ally_pawn,
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "Unable to move {:?} on c3",
                ally_pawn
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_captures_via_en_passant() {
        expectation()
            .to_change(|board| board.to_vec().clone_pieces())
            .to(|board| vec![Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap())]);
    }

    #[test]
    fn it_places_the_en_passant_pawn_properly() {
        expectation()
            .to_change(|board| {
                let ally_pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }
}

mod when_enemy_piece_crosses_attack_point_of_ally_pawn {
    use super::*;

    fn setup_board() -> Board {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(3, 4));
        println!("{}", board.pp());
        board
    }

    fn expectation() -> Expect<Vec<Rc<Piece>>, Board> {
        let mut expectation: Expect<Vec<Rc<Piece>>, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let enemy_rook = Rc::clone(board.piece_at(&Point::new(3, 4)).unwrap());
            assert!(
                board.move_piece(&enemy_rook, &PieceMove::Point(Point::new(3, 1))),
                "Unable to move {:?} on c1",
                enemy_rook
            );
            println!("{}", board.pp());

            let ally_pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
            assert!(
                !board.move_piece(
                    &ally_pawn,
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for {:?}",
                ally_pawn
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant() {
        expectation().not_to_change(|board| board.to_vec().clone_pieces());
    }
}

mod when_enemy_pawn_steps_from_attack_point_of_ally_pawn {
    use super::*;

    fn setup_board() -> Board {
        let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
        let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
        let mut board = Board::empty(config);
        board.pass_turn(&Color::Black);
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 3),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation() -> Expect<Vec<Rc<Piece>>, Board> {
        let mut expectation: Expect<Vec<Rc<Piece>>, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let enemy_pawn = Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap());
            assert!(
                board.move_piece(&enemy_pawn, &PieceMove::Point(Point::new(3, 2))),
                "Unable to move {:?} on c2",
                enemy_pawn
            );
            println!("{}", board.pp());

            let ally_pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
            assert!(
                !board.move_piece(
                    &ally_pawn,
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for {:?}",
                ally_pawn
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant() {
        expectation().not_to_change(|board| board.to_vec().clone_pieces());
    }
}

mod when_ally_pawn_does_not_utilize_en_passant_on_ally_turn {
    use super::*;

    fn setup_board() -> Board {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(1, 3));

        board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 4),
        );
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 1));
        println!("{}", board.pp());
        board
    }

    fn expectation() -> Expect<Vec<Rc<Piece>>, Board> {
        let mut expectation: Expect<Vec<Rc<Piece>>, Board> = Expect::setup(setup_board);
        expectation.expect(|board| {
            let enemy_pawn = Rc::clone(board.piece_at(&Point::new(3, 4)).unwrap());
            assert!(
                board.move_piece(&enemy_pawn, &PieceMove::LongMove(Point::new(3, 2))),
                "Unable to move {:?} on c2",
                enemy_pawn
            );
            println!("{}", board.pp());

            let ally_bishop = Rc::clone(board.piece_at(&Point::new(1, 3)).unwrap());
            assert!(
                board.move_piece(&ally_bishop, &PieceMove::Point(Point::new(2, 4))),
                "Unable to move {:?} on b4",
                ally_bishop
            );
            println!("{}", board.pp());

            let enemy_bishop = Rc::clone(board.piece_at(&Point::new(2, 1)).unwrap());
            assert!(
                board.move_piece(&enemy_bishop, &PieceMove::Point(Point::new(1, 2))),
                "Unable to move {:?} on a2",
                enemy_bishop
            );
            println!("{}", board.pp());

            let ally_pawn = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
            assert!(
                !board.move_piece(
                    &ally_pawn,
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "En passant must not be possible for {:?}",
                ally_pawn
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_does_not_allow_en_passant_after_turn_passes() {
        expectation().not_to_change(|board| board.to_vec().clone_pieces());
    }
}

mod en_passant_for_two_pawns {
    use super::*;
    use libtchess::piece::PieceId;
    use std::fmt::Debug;

    fn setup_board() -> Board {
        let mut board = board_default_4x4();
        board.pass_turn(&Color::Black);
        // ID#1
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        // ID#2
        board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(4, 2));

        // ID#3
        board.add_piece(
            "Pawn",
            Color::Black,
            vec![Buff::AdditionalPoint],
            vec![],
            Point::new(3, 4),
        );
        println!("{}", board.pp());
        board
    }

    fn expectation<T: PartialEq + Debug>(piece_id: usize) -> Expect<T, Board> {
        let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
        expectation.expect(move |board| {
            let enemy_pawn = Rc::clone(board.piece_at(&Point::new(3, 4)).unwrap());
            assert!(
                board.move_piece(&enemy_pawn, &PieceMove::LongMove(Point::new(3, 2))),
                "Unable to move {:?} on c2",
                enemy_pawn
            );
            println!("{}", board.pp());

            let pawn = Rc::clone(
                board
                    .find_piece_by_id(&Color::White, &PieceId(piece_id))
                    .unwrap(),
            );
            assert!(
                board.move_piece(
                    &pawn,
                    &PieceMove::EnPassant(Point::new(3, 3), Point::new(3, 2))
                ),
                "Unable to move {:?} on c3",
                pawn
            );
            println!("{}", board.pp());
        });
        expectation
    }

    #[test]
    fn it_is_possible_to_capture_with_the_first_pawn() {
        expectation(1)
            .to_change(|board| board.to_vec().clone_pieces())
            .to(|board| {
                vec![
                    Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap()),
                    Rc::clone(board.piece_at(&Point::new(4, 2)).unwrap()),
                ]
            });
    }

    #[test]
    fn it_places_the_first_pawn_properly() {
        expectation(1)
            .to_change(|board| {
                let ally_pawn = board.find_piece_by_id(&Color::White, &PieceId(1)).unwrap();
                ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }

    #[test]
    fn it_is_possible_to_capture_with_the_second_pawn() {
        expectation(2)
            .to_change(|board| board.to_vec().clone_pieces())
            .to(|board| {
                vec![
                    Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap()),
                    Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap()),
                ]
            });
    }

    #[test]
    fn it_places_the_second_pawn_properly() {
        expectation(2)
            .to_change(|board| {
                let ally_pawn = board.find_piece_by_id(&Color::White, &PieceId(2)).unwrap();
                ally_pawn.current_position()
            })
            .to(|_board| Point::new(3, 3));
    }
}
