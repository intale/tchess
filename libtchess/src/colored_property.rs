use std::ops::{Index, IndexMut};
use crate::color::Color;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ColoredProperty<T: Clone>(pub [T; 2]);

impl<T: Clone> ColoredProperty<T> {
    pub fn collection(&self) -> &[T; 2] {
        &self.0
    }

    pub fn collection_mut(&mut self) -> &mut [T; 2] {
        &mut self.0
    }
}

impl<T: Clone> Index<&Color> for ColoredProperty<T> {
    type Output = T;

    fn index(&self, index: &Color) -> &Self::Output {
        match index {
            Color::White => &self.0[0],
            Color::Black => &self.0[1],
        }
    }
}

impl<T: Clone> IndexMut<&Color> for ColoredProperty<T> {
    fn index_mut(&mut self, index: &Color) -> &mut T {
        match index {
            Color::White => &mut self.0[0],
            Color::Black => &mut self.0[1],
        }
    }
}
