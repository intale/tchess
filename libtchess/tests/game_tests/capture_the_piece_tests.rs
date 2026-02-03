#[path = "../support/mod.rs"]
mod support;

use im_rc::HashSet;
use libtchess::board::Board;
use libtchess::color::Color;
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

// 4 ▓▓▓ ░░░ ▓▓▓ ░♔░
// 3 ░░░ ▓▓▓ ░♗░ ▓▓▓
// 2 ▓▓▓ ░♝░ ▓▓▓ ░░░
// 1 ░░░ ▓▓▓ ░░░ ▓▓▓
fn setup_board() -> Board<TestHeatMap, TestSquaresMap> {
    let mut board = board_default_4x4();
    add_piece(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Point::new(2, 2),
    );
    add_piece(
        &mut board,
        "King",
        Color::White,
        vec![],
        vec![],
        Point::new(1, 1),
    );

    add_piece(
        &mut board,
        "Bishop",
        Color::Black,
        vec![],
        vec![],
        Point::new(3, 3),
    );
    add_piece(
        &mut board,
        "King",
        Color::Black,
        vec![],
        vec![],
        Point::new(4, 4),
    );
    println!("{}", board.pp());
    board
}

fn expectation<T: PartialEq + Debug>() -> Expect<T, Board<TestHeatMap, TestSquaresMap>> {
    let mut expectation: Expect<T, Board<TestHeatMap, TestSquaresMap>> = Expect::setup(setup_board);
    expectation.expect(|board| {
        move_piece(
            board,
            PieceId::new(1, &Color::White),
            PieceMove::Point(Point::new(3, 3)),
        );
        println!("{}", board.pp());
    });
    expectation
}

#[test]
fn it_removes_captured_piece_from_x_ray_pieces_list() {
    expectation()
        .to_change(|board| board.x_ray_pieces(&Color::Black).pieces_owned())
        .to(|_board| Vec::new());
}

#[test]
fn it_removes_captured_piece_from_active_pieces_list() {
    expectation()
        .to_change(|board| {
            board
                .active_pieces(&Color::Black)
                .keys()
                .copied()
                .collect::<Vec<_>>()
        })
        .to(|_board| vec![PieceId::new(2, &Color::Black)]);
}

#[test]
fn it_removes_captured_piece_strategy_points_list() {
    expectation()
        .to_change(|board| {
            board
                .strategy_points(&Color::Black)
                .get_points(&PieceId::new(1, &Color::Black))
                .unwrap_or(&HashSet::default())
                .iter()
                .copied()
                .collect::<Vec<_>>()
        })
        .to(|_board| vec![]);
}

#[test]
fn it_removes_available_moves_of_captured_pieces() {
    expectation()
        .to_change(|board| {
            board
                .moves_of(&PieceId::new(1, &Color::Black))
                .to_vec()
                .clone_moves()
        })
        .to(|_board| vec![]);
}
