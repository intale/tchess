use std::rc::Rc;
use crate::piece::Piece;

#[derive(Debug)]
pub enum LastBoardChanges {
    PieceAdded(Rc<Piece>),
    PieceRemoved(Rc<Piece>),
    PiecePositionChanged(Rc<Piece>),    
    EnPassantChanged(Rc<Piece>),
    CastleChanged(Rc<Piece>),
}
