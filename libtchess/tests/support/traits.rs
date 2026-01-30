use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use rustc_hash::FxHashSet;
use libtchess::board::Board;
use libtchess::color::Color;
use libtchess::piece_move::PieceMove;
use libtchess::piece::Piece;

#[allow(unused)]
pub trait ClonePieces {
    fn clone_pieces(&self) -> Vec<Rc<Piece>>;
}

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

impl ClonePieces for Vec<&Rc<Piece>> {
    fn clone_pieces(&self) -> Vec<Rc<Piece>> {
        self.iter().map(|piece| Rc::clone(piece)).collect()
    }
}

impl ClonePieces for FxHashSet<Rc<Piece>> {
    fn clone_pieces(&self) -> Vec<Rc<Piece>> {
        self.iter().map(|piece| Rc::clone(piece)).collect()
    }
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

impl ToVecRef for Board {
    type Item = Rc<Piece>;

    fn to_vec(&self) -> Vec<&Self::Item> {
        let mut pieces = self.active_pieces(&Color::White).to_vec();
        pieces.append(&mut self.active_pieces(&Color::Black).to_vec());
        pieces
    }
}

