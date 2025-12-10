#[path = "../support/mod.rs"]
mod support;
use support::compare;
use support::traits::ToVecRef;
use tchess::board::Board;
use tchess::color::Color;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare(
        &board.moves(&Color::White).moves_of(&rook).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(3, 5)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(5, 3)),
            &PieceMove::Point(Point::new(3, 1)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_a_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 2));

    println!("{}", board.pp());
    compare(
        &board.moves(&Color::White).moves_of(&rook).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(2, 1)),
        ],
    )
    .unwrap();
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 1));

    println!("{}", board.pp());
    compare(
        &board.moves(&Color::White).moves_of(&rook).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
        ],
    )
    .unwrap();
}

#[test]
fn when_rook_is_pinned_by_a_diagonal() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let rook = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare(
        &board.moves(&Color::White).moves_of(&rook).to_vec(),
        &vec![],
    )
    .unwrap();
}

#[test]
fn when_rook_is_pinned_by_line() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let queen = board.add_piece("Rook", Color::White, vec![], vec![], Point::new(2, 3));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 4));

    println!("{}", board.pp());
    compare(
        &board.moves(&Color::White).moves_of(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(2, 2)),
        ],
    )
    .unwrap();
}
