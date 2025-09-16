use std::hash::BuildHasherDefault;
use std::rc::Rc;
use multimap::MultiMap;
use nohash_hasher::NoHashHasher;
use crate::color::Color;
use crate::pieces::Piece;
use crate::point::Point;
use crate::utils::pretty_print::PrettyPrint;

type PointToPieceMapT = MultiMap<Point, Rc<Piece>, BuildHasherDefault<NoHashHasher<Point>>>;

#[derive(Debug)]
pub struct AttacksMap {
    attacks: PointToPieceMapT,
    color: Color
}

impl AttacksMap {
    pub fn empty(color: Color) -> Self {
        let attacks = MultiMap::with_hasher(BuildHasherDefault::default());
        Self { color, attacks }
    }

    pub fn add_attacks(&mut self, points: Vec<Point>, piece: &Rc<Piece>) {
        for point in points.into_iter() {
            self.attacks.insert(point, Rc::clone(piece))
        }
    }
}

pub struct MovesMap {
    moves: PointToPieceMapT,
    color: Color
}

impl MovesMap {
    pub fn empty(color: Color) -> Self {
        let moves = MultiMap::with_hasher(BuildHasherDefault::default());
        Self { color, moves }
    }
}

impl PrettyPrint for AttacksMap {
    fn pp(&self) -> String {
        let mut output = String::new();
        for (point, pieces) in &self.attacks {
            output.push_str(point.pp().as_str());
            output.push_str(" ");
        }
        output.to_string()
    }
}
