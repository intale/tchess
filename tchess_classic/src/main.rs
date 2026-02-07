use libtchess::utils::pretty_print::PrettyPrint;
use crate::classic_game::ClassicGame;

mod classic_game;
mod classic_square_map;
mod classic_heat_map;
mod game_result;
mod move_result;
mod board_positions;
// mod game_runner;

fn main() {
    let classic_board = ClassicGame::classic_board();
    println!("{}", classic_board.board().pp());
}
