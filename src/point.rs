mod x_point_t;
mod y_point_t;

use std::hash::Hasher;
use x_point_t::XPointT;
use y_point_t::YPointT;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: XPointT,
    y: YPointT,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        let x = XPointT::new(x);
        let y = YPointT::new(y);
        Self { x, y }
    }

    pub fn get_x(&self) -> XPointT {
        self.x
    }

    pub fn get_y(&self) -> YPointT {
        self.y
    }
}

// https://docs.rs/nohash-hasher/0.2.0/nohash_hasher/
impl std::hash::Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let high_bits = (self.x.get_value() as u16) << 8;
        hasher.write_u16(high_bits + self.y.get_value() as u16)
    }
}

impl PrettyPrint for Point {
    fn pp(&self) -> String {
        let mut output = String::new();
        output.push_str(self.x.pp().as_str());
        output.push_str(self.y.pp().as_str());
        output
    }
}

impl nohash_hasher::IsEnabled for Point {}
