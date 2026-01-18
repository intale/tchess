// This information is used for static evaluation of the material on the board. For example, it is
// used to calculate insufficient material rule.
pub struct StaticPieceWeights {
    pub bishop: u8,
    pub king: u8,
    pub knight: u8,
    pub pawn: u8,
    pub queen: u8,
    pub rook: u8,
}
