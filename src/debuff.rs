use crate::vector::Vector;

#[derive(Debug, PartialEq)]
pub enum Debuff {
    Captured,
    Check,
    Checkmate,
    Pin(Vector),
}
