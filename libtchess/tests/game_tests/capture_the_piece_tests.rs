#[path = "../support/mod.rs"]
mod support;

use std::fmt::Debug;
use std::rc::Rc;
use rustc_hash::FxHashSet;
use support::traits::{ClonePieces, ToVecRef};
use support::*;
use support::{expect::Expect, expect_to_change_to::ExpectToChangeTo};
use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::piece::{Piece, PieceId};
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;

// 4 ▓▓▓ ░░░ ▓▓▓ ░♔░
// 3 ░░░ ▓▓▓ ░♗░ ▓▓▓
// 2 ▓▓▓ ░♝░ ▓▓▓ ░░░
// 1 ░░░ ▓▓▓ ░░░ ▓▓▓
fn setup_board() -> Board {
    let mut board = board_default_4x4();
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));

    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));
    board.add_piece("King", Color::Black, vec![], vec![], Point::new(4, 4));
    println!("{}", board.pp());
    board
}

fn expectation<T: PartialEq + Debug>() -> Expect<T, Board> {
    let mut expectation: Expect<T, Board> = Expect::setup(setup_board);
    expectation.expect(|board| {
        let ally_bishop = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
        assert!(
            board.move_piece(&ally_bishop, &PieceMove::Point(Point::new(3, 3))),
            "Unable to move {:?} on c3",
            ally_bishop
        );
        println!("{}", board.pp());
    });
    expectation
}

#[test]
fn it_removes_captured_piece_from_x_ray_pieces_list() {
    expectation()
        .to_change(|board| board.x_ray_pieces(&Color::Black).pieces().clone_pieces())
        .to(|_board| Vec::<Rc<Piece>>::new());
}

#[test]
fn it_removes_captured_piece_from_available_moves_list() {
    expectation()
        .to_change(|board| board.active_pieces(&Color::Black).clone_pieces())
        .to(|board| vec![Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap())]);
}

#[test]
fn it_removes_captured_piece_strategy_points_list() {
    expectation()
        .to_change(|board| {
            board
                .strategy_points(&Color::Black)
                .get_points(&PieceId(3))
                .unwrap_or(&FxHashSet::default())
                .iter().map(|p| *p)
                .collect::<Vec<_>>()
        })
        .to(|_board| vec![]);
}

#[test]
fn it_removes_captured_piece_from_the_board_square() {
    expectation()
        .to_change(|board| board.to_vec().clone_pieces())
        .to(|board| {
            vec![
                Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap()),
                Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap()),
                Rc::clone(board.piece_at(&Point::new(1, 1)).unwrap()),
            ]
        });
}
