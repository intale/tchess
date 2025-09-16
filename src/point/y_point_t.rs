use std::cmp::Ordering;
use std::ops::{Add, Deref};
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct YPointT(i16);


impl YPointT {
    pub fn new(value: i16) -> Self {
        Self(value)
    }

    pub fn get_value(&self) -> i16 {
        self.0
    }
}

impl Add<i16> for YPointT {
    type Output = i16;

    fn add(self, rhs: i16) -> Self::Output {
        self.0 + rhs
    }
}

impl PartialEq<i16> for YPointT {
    fn eq(&self, other: &i16) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<i16> for YPointT {
    fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Deref for YPointT {
    type Target = i16;

    fn deref(&self) -> &i16 {
        &self.0
    }
}

impl PrettyPrint for YPointT {
    fn pp(&self) -> String {
        if self.0 < 0 {
            return "".to_string()
        }
        (self.0 + 1i16).to_string()
    }
}
