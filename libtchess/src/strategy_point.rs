use std::fmt::{Display, Formatter};
use crate::point::Point;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum StrategyPoint {
    Attack(Point),
    Defense(Point),
    Move(Point),
    BlockedMove(Point),
    DeadEnd(Point),
}

impl Display for StrategyPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attack(point) => write!(f, "⚔{}", point),
            Self::Move(point) => write!(f, "⇧{}", point),
            Self::Defense(point) => write!(f, "⛨{}", point),
            Self::BlockedMove(point) => write!(f, "🛇{}", point),
            Self::DeadEnd(point) => write!(f, "🧱{}", point),
        }
    }
}
