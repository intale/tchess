use tchess::board::*;
use tchess::color::Color;
use tchess::utils::pretty_print::PrettyPrint;

fn main() {
    let board = Board::classic_chess_board();
    println!("White attack points");
    println!("{}", board.attack_points(&Color::White).pp_pieces());
    println!("\nBlack attack points");
    println!("{}", board.attack_points(&Color::Black).pp_pieces());
    println!("White pieces attacks");
    println!("{}", board.attack_points(&Color::White).pp_points());
    println!("\nBlack pieces attacks");
    println!("{}", board.attack_points(&Color::Black).pp_points());
    println!("White pieces defenses");
    println!("{}", board.defensive_points(&Color::White).pp_points());
    println!("\nBlack pieces defenses");
    println!("{}", board.defensive_points(&Color::Black).pp_points());
    println!("{}", board.pp());
}
