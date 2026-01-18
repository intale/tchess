#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::piece_move::PieceMove;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&knight).to_vec(),
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
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&knight).to_vec(),
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
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&knight).to_vec(),
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
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
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
        &board.moves_of(&knight).to_vec(),
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
    let mut board = board_default_4x4();
    let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(2, 2));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&knight).to_vec(),
        &vec![],
    );
}

mod when_there_are_void_squares_on_the_way {
    use super::*;

    #[test]
    fn it_ignores_them() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '¤', '▓', '¤', '▓'],
                vec!['░', '¤', '¤', '¤', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&knight).to_vec(),
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

    #[test]
    fn it_does_not_include_them() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '¤', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '▓', '¤'],
                vec!['▓', '░', '▓', '¤', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let knight = board.add_piece("Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&knight).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 1)),
                &PieceMove::Point(Point::new(1, 2)),
                &PieceMove::Point(Point::new(1, 4)),
                &PieceMove::Point(Point::new(2, 5)),
            ],
        );
    }
}
