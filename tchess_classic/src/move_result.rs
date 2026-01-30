use crate::game_result::GameResult;

#[derive(Eq, PartialEq, Debug)]
pub enum MoveResult {
    PieceMoved,
    IllegalMove,
    GameEnded(GameResult),
}
