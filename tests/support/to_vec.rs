use std::cell::RefCell;
use std::collections::HashSet;
use tchess::buff::{Buff, BuffsCollection};
use tchess::debuff::{Debuff, DebuffsCollection};

pub trait ToVecCopy {
    type Item: Clone + Copy;
    fn to_vec(&self) -> Vec<Self::Item>;
}

pub trait ToVecRef {
    type Item;

    fn to_vec(&self) -> Vec<&Self::Item>;
}

impl<T: Copy> ToVecCopy for RefCell<Vec<T>> {
    type Item = T;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.borrow()[..].iter().map(|&v| v).collect::<Vec<_>>()
    }
}

impl ToVecCopy for DebuffsCollection {
    type Item = Debuff;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.to_ref_cell().to_vec()
    }
}

impl ToVecCopy for BuffsCollection {
    type Item = Buff;

    fn to_vec(&self) -> Vec<Self::Item> {
        self.to_ref_cell().to_vec()
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
