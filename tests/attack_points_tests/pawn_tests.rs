#[path = "../support/mod.rs"]
mod support;

use support::compare;
use support::traits::ToVecRef;
use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;

#[cfg(test)]
mod white_pawn {
    use super::*;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3), &Point::new(3, 3)],
        )
        .unwrap();
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3), &Point::new(3, 3)],
        )
        .unwrap();
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
        let pawn = board.add_piece("Pawn", Color::White, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 3));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::White)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 3)],
        )
        .unwrap();
    }
}

#[cfg(test)]
mod black_pawn {
    use super::*;
    use tchess::utils::pretty_print::PrettyPrint;

    #[test]
    fn when_there_are_no_pieces_around() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 1), &Point::new(3, 1)],
        )
        .unwrap();
    }

    #[test]
    fn when_there_is_a_an_enemy_piece_on_an_attack_point() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(3, 3));
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(2, 2));
        board.add_piece("Bishop", Color::White, vec![], vec![], Point::new(3, 1));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(1, 1), &Point::new(3, 1)],
        )
        .unwrap();
    }

    #[test]
    fn when_there_is_an_ally_piece_on_an_attack_point() {
        let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
        board.set_pov(Color::Black);
        let pawn = board.add_piece("Pawn", Color::Black, vec![], vec![], Point::new(3, 3));
        board.add_piece("Bishop", Color::Black, vec![], vec![], Point::new(2, 2));

        println!("{}", board.pp());
        compare(
            &board
                .attack_points(&Color::Black)
                .get_points(&pawn)
                .to_vec(),
            &vec![&Point::new(4, 2)],
        )
        .unwrap();
    }
}
