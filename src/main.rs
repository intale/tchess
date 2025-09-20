mod pieces;
mod color;
mod board;
mod utils;
mod point;
mod cell;
mod point_to_piece_association;
mod diagonal_vector;
mod line_vector;
mod jump_vector;

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
    println!("{}", board.pp());
}
