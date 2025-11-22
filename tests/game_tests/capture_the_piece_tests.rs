#[path = "../support/mod.rs"]
mod support;

use std::rc::Rc;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::to_vec::{ToVecRef, ClonePieces};
use tchess::piece_move::PieceMove;
use tchess::utils::pretty_print::PrettyPrint;
use support::Expect;
use tchess::pieces::Piece;

// 4 ▓▓▓ ░░░ ▓▓▓ ░♔░
// 3 ░░░ ▓▓▓ ░♗░ ▓▓▓
// 2 ▓▓▓ ░♝░ ▓▓▓ ░░░
// 1 ░░░ ▓▓▓ ░░░ ▓▓▓
fn setup_board() -> Board {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(2, 2)
    );
    board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(1, 1)
    );

    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(4, 4)
    );
    println!("{}", board.pp());
    board
}

fn expectation() -> Expect<Vec<Rc<Piece>>, Board> {
    let mut expectation: Expect<Vec<Rc<Piece>>, Board> = Expect::setup(setup_board);
    expectation.expect(|board| {
        let ally_bishop = Rc::clone(board.piece_at(&Point::new(2, 2)).unwrap());
        assert!(
            board.move_piece(&ally_bishop, &PieceMove::Point(Point::new(3, 3))),
            "Unable to move {:?} on c3", ally_bishop
        );
        println!("{}", board.pp());
    });
    expectation
}

#[test]
fn it_removes_captured_piece_from_x_ray_pieces_list() {
    expectation().to_change(|board| {
        board.x_ray_pieces(&Color::Black).pieces().clone_pieces()
    }).to(|_board| {
        Vec::<Rc<Piece>>::new()
    });
}

#[test]
fn it_removes_captured_piece_from_available_moves_list() {
    expectation().to_change(|board| {
        board.moves(&Color::Black).all_pieces().clone_pieces()
    }).to(|board| {
        vec![Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap())]
    });
}

#[test]
fn it_removes_captured_piece_attack_points_list() {
    expectation().to_change(|board| {
        board.attack_points(&Color::Black).get_all_pieces().clone_pieces()
    }).to(|board| {
        vec![Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap())]
    });
}

#[test]
fn it_removes_captured_piece_defensive_points_list() {
    expectation().to_change(|board| {
        board.defensive_points(&Color::Black).get_all_pieces().clone_pieces()
    }).to(|_board| {
        vec![]
    });
}

#[test]
fn it_removes_captured_piece_from_the_board_cell() {
    expectation().to_change(|board| {
        board.to_vec().clone_pieces()
    }).to(|board| {
        vec![
            Rc::clone(board.piece_at(&Point::new(4, 4)).unwrap()),
            Rc::clone(board.piece_at(&Point::new(3, 3)).unwrap()),
            Rc::clone(board.piece_at(&Point::new(1, 1)).unwrap()),
        ]
    });
}
