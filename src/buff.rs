use std::cell::RefCell;
use std::rc::Rc;
use crate::pieces::Piece;
use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PromotePiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Buff {
    Castle,
    EnPassant(Point),
    Promote(PromotePiece),
    AdditionalPoint, // A pawn buff to allow going one additional point further
}

#[derive(Debug)]
struct BuffsList {
    pub castle: Option<Buff>,
    pub en_passant: Option<Buff>,
    pub promote: Option<Buff>,
    pub additional_point: Option<Buff>,
}

impl BuffsList {
    pub fn empty() -> Self {
        Self {
            castle: None,
            en_passant: None,
            promote: None,
            additional_point: None,
        }
    }

    pub fn add(&mut self, buff: Buff) {
        match buff {
            Buff::Castle => { self.castle = Some(buff) },
            Buff::EnPassant(_) => { self.en_passant = Some(buff) },
            Buff::Promote(_) => { self.promote = Some(buff) },
            Buff::AdditionalPoint => { self.additional_point = Some(buff) },
        }
    }
}

#[derive(Debug)]
pub struct BuffsCollection {
    buffs: RefCell<BuffsList>,
}

impl BuffsCollection {
    pub fn new(buffs: Vec<Buff>) -> Self {
        let mut list = BuffsList::empty();
        for buff in buffs {
            list.add(buff)
        }
        Self { buffs: RefCell::new(list) }
    }

    pub fn add(&self, buff: Buff) {
        self.buffs.borrow_mut().add(buff)
    }

    pub fn has_additional_point(&self) -> bool {
        self.buffs.borrow().additional_point.is_some()
    }

    pub fn has_castle(&self) -> bool {
        self.buffs.borrow().castle.is_some()
    }

    pub fn en_passant(&self) -> Option<Point> {
        if let Some(en_passant) = self.buffs.borrow().en_passant.as_ref() {
            match en_passant {
                Buff::EnPassant(p) => {
                    Some(*p)
                },
                _ => panic!("Invalid EnPassant buff {:?}!", en_passant)
            }
        } else {
            None
        }
    }

    pub fn to_vec(&self) -> Vec<Buff> {
        let list = self.buffs.borrow();
        let buffs = [
            list.castle.as_ref(),
            list.en_passant.as_ref(),
            list.promote.as_ref(),
            list.additional_point.as_ref(),
        ];
        buffs
            .iter()
            .filter(|buff| buff.is_some())
            .map(|buff| buff.unwrap().clone())
            .collect::<Vec<_>>()
    }
}
