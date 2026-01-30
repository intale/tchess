use libtchess::color::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameResult {
    Stalemate(Color),
    Checkmate(Color),
    InsufficientMaterialDraw,
    FiftyMoveRuleDraw,
    DrawByRepetition,
}
