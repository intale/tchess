#[path = "../support/mod.rs"]
mod support;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::board_square_builder::{
    BoardSquareBuilder, default_square_builder::DefaultSquareBuilder,
};
use tchess::color::Color;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&bishop).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(3, 3)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_a_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&bishop).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(3, 3)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&bishop).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(1, 3)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_bishop_is_pinned_by_one_of_its_diagonals() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&bishop).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(4, 4)),
        ],
    );
}

#[test]
fn when_bishop_is_pinned_by_line() {
    let mut board = Board::empty(
        Point::new(1, 1),
        Point::new(4, 4),
        DefaultSquareBuilder::init(),
    );
    let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves(&Color::White).moves_of(&bishop).to_vec(),
        &vec![],
    );
}

mod when_there_are_different_color_squares_on_the_diagonal {
    use super::*;
    use support::init_square_builder_from;

    #[test]
    fn it_does_not_include_them() {
        let builder = init_square_builder_from(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '░', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '░', '░', '▓', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White
        );

        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(5, 5),
            builder,
        );
        let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .moves(&Color::White)
                .moves_of(&bishop)
                .to_vec(),
            &vec![
                &PieceMove::Point(Point::new(1, 5)),
                &PieceMove::Point(Point::new(2, 4)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 1)),
            ],
        );
    }
}

mod when_there_are_void_squares_on_the_diagonal {
    use super::*;
    use support::init_square_builder_from;

    #[test]
    fn it_does_not_include_them() {
        let builder = init_square_builder_from(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '¤', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '░', '▓', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White
        );

        let mut board = Board::empty(
            Point::new(1, 1),
            Point::new(5, 5),
            builder,
        );
        let bishop = board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .moves(&Color::White)
                .moves_of(&bishop)
                .to_vec(),
            &vec![
                &PieceMove::Point(Point::new(1, 5)),
                &PieceMove::Point(Point::new(2, 4)),
                &PieceMove::Point(Point::new(4, 2)),
                &PieceMove::Point(Point::new(5, 1)),
            ],
        );
    }
}
