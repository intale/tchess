use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use libtchess::piece_move::PieceMove;

#[allow(unused)]
pub trait CloneMoves {
    fn clone_moves(&self) -> Vec<PieceMove>;
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

impl CloneMoves for Vec<&PieceMove> {
    fn clone_moves(&self) -> Vec<PieceMove> {
        self.iter().map(|&piece_move| piece_move.clone()).collect()
    }
}

impl<T: Copy> ToVecCopy for RefCell<Vec<T>> {
    type Item = T;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.borrow()[..].iter().map(|&v| v).collect::<Vec<_>>()
    }
}

impl<T: Copy> ToVecCopy for BTreeSet<T> {
    type Item = T;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.iter().map(|&v| v).collect::<Vec<_>>()
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
