#[derive(Copy, Clone)]
pub struct KingCastleXPoint(pub i16);

#[derive(Copy, Clone)]
pub struct RookCastleXPoint(pub i16);

#[derive(Copy, Clone)]
pub struct CastleXPoints(pub KingCastleXPoint, pub RookCastleXPoint);

impl CastleXPoints {
    pub fn king_x(&self) -> &i16 {
        &self.0.0
    }

    pub fn rook_x(&self) -> &i16 {
        &self.1.0
    }
}
