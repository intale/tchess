mod x_point_t;
mod y_point_t;

use std::fmt::{Display, Formatter};
use x_point_t::XPointT;
use y_point_t::YPointT;
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
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

    pub fn x(&self) -> &XPointT {
        &self.x
    }

    pub fn y(&self) -> &YPointT {
        &self.y
    }

    pub fn to_tuple(&self) -> (&i16, &i16) {
        (self.x().value(), self.y().value())
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

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", *self.x, *self.y)
    }
}
