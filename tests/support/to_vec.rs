use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use tchess::board::Board;
use tchess::pieces::Piece;

#[allow(unused)]
pub trait ClonePieces {
    fn clone_pieces(&self) -> Vec<Rc<Piece>>;
}

#[allow(unused)]
pub trait ToVecCopy {
    type Item: Clone + Copy;
    fn to_vec(&self) -> Vec<Self::Item>;
}

#[allow(unused)]
pub trait ToVecRef {
    type Item;

    fn to_vec(&self) -> Vec<&Self::Item>;
}

impl ClonePieces for Vec<&Rc<Piece>> {
    fn clone_pieces(&self) -> Vec<Rc<Piece>> {
        self.iter().map(|piece| Rc::clone(piece)).collect()
    }
}

impl<T: Copy> ToVecCopy for RefCell<Vec<T>> {
    type Item = T;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.borrow()[..].iter().map(|&v| v).collect::<Vec<_>>()
    }
}

impl<T, B> ToVecRef for HashSet<T, B> {
    type Item = T;

    fn to_vec(&self) -> Vec<&Self::Item> {
        self.iter().collect()
    }
}

impl<T, B> ToVecRef for Option<&HashSet<T, B>> {
    type Item = T;

    fn to_vec(&self) -> Vec<&Self::Item> {
        match self {
            Some(hash) => hash.to_vec(),
            None => vec![],
        }
    }
}

impl ToVecRef for Board {
    type Item = Rc<Piece>;

    fn to_vec(&self) -> Vec<&Self::Item> {
        self.get_board()
            .iter()
            .filter_map(|(_, cell)| cell.get_piece().as_ref())
            .collect::<Vec<_>>()
    }
}
