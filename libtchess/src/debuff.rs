use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Debuff {
    Captured,
    Check,
    Checkmate,
    Pin(Vector),
}
