#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum PromotePiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

impl PromotePiece {
    pub fn all_variants() -> Vec<Self> {
        vec![
            Self::Bishop,
            Self::Knight,
            Self::Queen,
            Self::Rook,
        ]
    }

    pub fn name(&self) -> String {
        match self {
            Self::Bishop => String::from("Bishop"),
            Self::Knight => String::from("Knight"),
            Self::Queen => String::from("Queen"),
            Self::Rook => String::from("Rook"),
        }
    }
}
