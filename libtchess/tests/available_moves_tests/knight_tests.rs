#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::dimension::Dimension;
use libtchess::piece_move::PieceMove;
use libtchess::point::Point;
use libtchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(knight.id()).to_vec(),
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
    let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));
    add_piece(&mut board,"Bishop", Color::Black, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(knight.id()).to_vec(),
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
    let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));
    add_piece(&mut board,"Bishop", Color::White, vec![], vec![], Point::new(4, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(knight.id()).to_vec(),
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
    let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));

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
        &board.moves_of(knight.id()).to_vec(),
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
    let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(2, 2));
    add_piece(&mut board,"King", Color::White, vec![], vec![], Point::new(2, 1));
    add_piece(&mut board,"Rook", Color::Black, vec![], vec![], Point::new(2, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(knight.id()).to_vec(),
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
        let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(knight.id()).to_vec(),
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
        let knight = add_piece(&mut board,"Knight", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(knight.id()).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(2, 1)),
                &PieceMove::Point(Point::new(1, 2)),
                &PieceMove::Point(Point::new(1, 4)),
                &PieceMove::Point(Point::new(2, 5)),
            ],
        );
    }
}
