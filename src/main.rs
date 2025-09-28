mod pieces;
mod color;
mod board;
mod utils;
mod point;
mod cell;
mod point_to_piece_association;
mod vector;
mod vector_points;
mod dimension;

use pieces::*;
use color::*;
use board::*;
use crate::utils::pretty_print::PrettyPrint;

fn main() {
    let board = Board::classic_chess_board();
    println!("White attack points");
    println!("{}", board.get_white_attack_points().pp_pieces());
    println!("\nBlack attack points");
    println!("{}", board.get_black_attack_points().pp_pieces());
    println!("White pieces attacks");
    println!("{}", board.get_white_attack_points().pp_points());
    println!("\nBlack pieces attacks");
    println!("{}", board.get_black_attack_points().pp_points());
    println!("White pieces defenses");
    println!("{}", board.get_white_defensive_points().pp_points());
    println!("\nBlack pieces defenses");
    println!("{}", board.get_black_defensive_points().pp_points());
    println!("{}", board.pp());
}
