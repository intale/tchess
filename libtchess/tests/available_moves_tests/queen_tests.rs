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
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&queen).to_vec(),
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
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&queen).to_vec(),
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
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(2, 2));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&queen).to_vec(),
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
    let mut board = board_default_4x4();
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(1, 1));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(4, 4)),
        ],
    );
}

#[test]
fn when_queen_is_pinned_by_line() {
    let mut board = board_default_4x4();
    let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(2, 3));
    board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 1));
    board.add_piece("Rook", Color::Black, vec![], vec![], Point::new(2, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board.moves_of(&queen).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(2, 2)),
        ],
    );
}

mod when_there_are_void_squares_on_the_way {
    use super::*;

    #[test]
    fn it_does_not_include_them() {
        let squares_map = TestSquaresMap::from_chars(
            vec![
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '▓', '░', '¤', '░'],
                vec!['▓', '░', '▓', '¤', '▓'],
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
        let queen = board.add_piece("Queen", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board.moves_of(&queen).to_vec(),
            &vec![
                &PieceMove::Point(Point::new(1, 3)),
                &PieceMove::Point(Point::new(1, 5)),
                &PieceMove::Point(Point::new(2, 3)),
                &PieceMove::Point(Point::new(2, 4)),
                &PieceMove::Point(Point::new(3, 4)),
                &PieceMove::Point(Point::new(3, 5)),
            ],
        );
    }
}
