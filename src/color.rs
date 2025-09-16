#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl PartialEq<&Color> for Color {
    fn eq(&self, other: &&Color) -> bool {
        match (self, other) {
            (Color::White, Color::White) => true,
            (Color::Black, Color::Black) => true,
            _ => false,
        }
    }
}
