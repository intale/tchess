#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
pub enum PromotePiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotePiece {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Knight,
            Self::Bishop,
            Self::Rook,
            Self::Queen,
        ]
    }

    pub fn name(&self) -> String {
        match self {
            Self::Knight => String::from("Knight"),
            Self::Bishop => String::from("Bishop"),
            Self::Rook => String::from("Rook"),
            Self::Queen => String::from("Queen"),
        }
    }
}
