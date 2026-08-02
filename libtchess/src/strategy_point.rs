use std::fmt::{Display, Formatter};
use crate::point::Point;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum StrategyPoint {
    Attack(Point),
    Defense(Point),
    Move(Point),
    BlockedMove(Point),
    DeadEndMove(Point),
    DeadEndAttack(Point),
    DeadEnd(Point),
}

impl StrategyPoint {
    pub fn destination(&self) -> &Point {
        match self { 
            Self::Attack(p) => p,
            Self::Defense(p) => p,
            Self::Move(p) => p,
            Self::BlockedMove(p) => p,
            Self::DeadEndMove(p) => p,
            Self::DeadEndAttack(p) => p,
            Self::DeadEnd(p) => p,
        }
    }
}

impl Display for StrategyPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attack(point) => write!(f, "⚔{}", point),
            Self::Move(point) => write!(f, "⇧{}", point),
            Self::Defense(point) => write!(f, "⛨{}", point),
            Self::BlockedMove(point) => write!(f, "🛇{}", point),
            Self::DeadEndMove(point) => write!(f, "🧱{}", point),
            Self::DeadEndAttack(point) => write!(f, "🧱{}", point),
            Self::DeadEnd(point) => write!(f, "🧱{}", point),
        }
    }
}
