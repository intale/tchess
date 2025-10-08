use crate::vector::Vector;

#[derive(Debug)]
pub enum Debuff {
    Captured,
    Check,
    Checkmate,
    Pin(Vector),
}
