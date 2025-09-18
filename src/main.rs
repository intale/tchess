mod pieces;
mod color;
mod board;
mod utils;
mod point;
mod cell;
mod moves_map;

use pieces::*;
use color::*;
use board::*;
use crate::utils::pretty_print::PrettyPrint;

fn main() {
    let board = Board::classic_chess_board();
    println!("White attack moves");
    println!("{}", board.get_white_attack_moves().pp_pieces());
    println!("\nBlack attack moves");
    println!("{}", board.get_black_attack_moves().pp_pieces());
    println!("{}", board.pp());
}
