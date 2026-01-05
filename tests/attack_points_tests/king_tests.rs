#[path = "../support/mod.rs"]
mod support;

use support::test_squares_map::TestSquaresMap;
use support::traits::ToVecRef;
use support::*;
use tchess::board::Board;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::utils::pretty_print::PrettyPrint;

#[test]
fn when_there_are_no_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .attack_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .attack_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_a_protected_enemy_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(4, 4));
    board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(5, 5));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .attack_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(5, 5));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));
    board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(4, 4));

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .attack_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(2, 2),
            &Point::new(2, 3),
            &Point::new(2, 4),
            &Point::new(3, 4),
            &Point::new(4, 3),
            &Point::new(4, 2),
            &Point::new(3, 2),
        ],
    );
}

#[test]
fn when_there_are_enemy_pieces_around() {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let config = board_config(dimension, TestSquaresMap::from_dimension(&dimension));
    let mut board = Board::empty(config);
    let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(2, 2));

    // A box of enemy knights around the king
    create_box_of(
        &mut board,
        "Knight",
        Color::Black,
        vec![],
        vec![],
        Dimension::new(Point::new(1, 1), Point::new(3, 3)),
    );

    println!("{}", board.pp());
    compare_and_assert(
        &board
            .attack_points(&Color::White)
            .get_points(&king)
            .to_vec(),
        &vec![
            &Point::new(1, 1),
            &Point::new(1, 2),
            &Point::new(1, 3),
            &Point::new(2, 3),
            &Point::new(3, 3),
            &Point::new(3, 2),
            &Point::new(3, 1),
            &Point::new(2, 1),
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
                vec!['▓', '░', '▓', '░', '▓'],
                vec!['░', '¤', '░', '▓', '░'],
                vec!['▓', '░', '▓', '░', '▓'],
            ],
            &Color::White,
        );
        let config = board_config(
            Dimension::new(Point::new(1, 1), Point::new(5, 5)),
            squares_map,
        );
        let mut board = Board::empty(config);
        let king = board.add_piece("King", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare_and_assert(
            &board
                .attack_points(&Color::White)
                .get_points(&king)
                .to_vec(),
            &vec![
                &Point::new(2, 3),
                &Point::new(2, 4),
                &Point::new(3, 4),
                &Point::new(4, 3),
                &Point::new(4, 2),
                &Point::new(3, 2),
            ],
        );
    }
}
