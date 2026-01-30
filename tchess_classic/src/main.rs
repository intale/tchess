use libtchess::utils::pretty_print::PrettyPrint;
use crate::classic_game::ClassicGame;

mod board_position;
mod classic_game;
mod classic_square_map;
mod game_stats;
mod classic_heat_map;
mod game_result;
mod move_result;

fn main() {
    let classic_board = ClassicGame::default();
    println!("{}", classic_board.board().pp());
}
