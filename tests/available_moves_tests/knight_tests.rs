#[path = "../support/mod.rs"]
mod support;

use support::create_box_of;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::board_square_builders::{
    BoardSquareBuilder, default_square_builder::DefaultSquareBuilder,
};
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(5, 5),
        DefaultSquareBuilder::init(),
    );
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(5, 5),
        DefaultSquareBuilder::init(),
    );
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_an_attack_point() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(5, 5),
        DefaultSquareBuilder::init(),
    );
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_there_are_ally_pieces_between_the_knight_and_an_enemy_piece() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(5, 5),
        DefaultSquareBuilder::init(),
    );
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    // A box of bishops around the knight
    create_box_of(
        &mut board,
        "Bishop",
        Color::White,
        vec![],
        vec![],
        Dimension::new(Point::new(2, 2), Point::new(4, 4)),
    );

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&knight).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(1, 4)),
            &PieceMove::Point(Point::new(2, 5)),
            &PieceMove::Point(Point::new(4, 5)),
            &PieceMove::Point(Point::new(5, 4)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(4, 1)),
        ],
    );
}

#[test]
fn when_knight_is_pinned() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&knight).to_vec(),
        &vec![],
    );
}

mod when_there_are_void_squares_on_the_way {
    use super::*;
    use support::init_square_builder_from;

    #[test]
    fn it_ignores_them() {
        let builder = init_square_builder_from(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '¤', '▓', '¤', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White
        );

        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(5, 5),
            builder,
        );
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .moves(&Color::White)
                .moves_of(&knight)
                .to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 1)),
                &PieceMove::Point(Point::new(1, 2)),
                &PieceMove::Point(Point::new(1, 4)),
                &PieceMove::Point(Point::new(2, 5)),
                &PieceMove::Point(Point::new(4, 5)),
                &PieceMove::Point(Point::new(5, 4)),
                &PieceMove::Point(Point::new(5, 2)),
                &PieceMove::Point(Point::new(4, 1)),
            ],
        );
    }
}

mod when_there_are_void_squares_on_move_points {
    use super::*;
    use support::init_square_builder_from;

    #[test]
    fn it_does_not_include_them() {
        let builder = init_square_builder_from(
            vec![
                vec!['▓', '░', '▓', '¤', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '¤', '▓'],
            ],
            &Color::White
        );

        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(5, 5),
            builder,
        );
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .moves(&Color::White)
                .moves_of(&knight)
                .to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 1)),
                &PieceMove::Point(Point::new(1, 2)),
                &PieceMove::Point(Point::new(1, 4)),
                &PieceMove::Point(Point::new(2, 5)),
            ],
        );
    }
}
