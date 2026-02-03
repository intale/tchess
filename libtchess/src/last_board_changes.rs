use crate::piece_id::PieceId;

#[derive(Debug, Clone)]
pub enum LastBoardChanges {
    PieceAdded(PieceId),
    PieceRemoved(PieceId),
    PiecePositionChanged(PieceId),
    EnPassantChanged(PieceId),
    CastleChanged(PieceId),
}
