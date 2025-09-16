mod pieces;
mod color;
mod board;
mod utils;
mod point;
mod cell;
mod shadow_board;

use pieces::*;
use color::*;
use board::*;
use crate::utils::pretty_print::PrettyPrint;

fn main() {
    let board = Board::classic_chess_board();
    println!("{}", board.pp());
}
