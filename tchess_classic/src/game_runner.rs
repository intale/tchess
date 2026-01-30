use std::rc::Rc;
use libtchess::board::Board;
use libtchess::piece::Piece;
use libtchess::piece_move::PieceMove;
use crate::game_result::GameResult;

struct GameRunner<'a> {
    board: &'a Board,
    game_result: Option<GameResult>,
}

impl GameRunner<'_> {
    
}
