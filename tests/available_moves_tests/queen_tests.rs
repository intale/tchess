#[path = "../support/mod.rs"]
mod support;
use support::compare;
use support::to_vec::ToVecRef;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use tchess::piece_move::{PieceMove};

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(5, 5)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(5, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(1, 5)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(5, 1)),
            &PieceMove::Point(Point::new(3, 5)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(1, 1)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(2, 2)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
    let queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(1, 1)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(2, 2)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_queen_is_pinned_by_one_of_its_diagonals() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(1, 1)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(4, 4)),
        ],
    );
}

#[test]
fn when_queen_is_pinned_by_line() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let queen = board.add_piece(
        "Queen", Color::White, vec![], vec![], Point::new(2, 3)
    );
    board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(2, 1)
    );
    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(2, 4)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(2, 2)),
        ],
    );
}
