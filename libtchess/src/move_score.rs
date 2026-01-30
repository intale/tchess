use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone)]
pub enum MoveScore {
    WeightDelta(i16),
}

impl MoveScore {
    pub fn variant_weight(&self) -> i8 {
        match self {
            Self::WeightDelta(_) => 0,
        }
    }

    pub fn score(&self) -> &i16 {
        match self {
            Self::WeightDelta(w) => w,
        }
    }
}

impl PartialOrd for MoveScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MoveScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.variant_weight().cmp(&other.variant_weight()).then(self.score().cmp(other.score()))
    }
}

impl Display for MoveScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Self::WeightDelta(d) => write!(f, "Δ{d}")
        }
    }
}
