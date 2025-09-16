use std::cmp::Ordering;
use std::ops::{Add, Deref};
use crate::utils::pretty_print::PrettyPrint;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct XPointT(i16);

impl XPointT {
    pub fn new(value: i16) -> Self {
        Self(value)
    }
    
    pub fn get_value(&self) -> i16 {
        self.0
    }
}

impl Add<i16> for XPointT {
    type Output = i16;

    fn add(self, rhs: i16) -> Self::Output {
        self.0 + rhs
    }
}

impl PartialEq<i16> for XPointT {
    fn eq(&self, other: &i16) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<i16> for XPointT {
    fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Deref for XPointT {
    type Target = i16;

    fn deref(&self) -> &i16 {
        &self.0
    }
}

impl PrettyPrint for XPointT {
    fn pp(&self) -> String {
        if self.0 < 0 {
            return "".to_string()
        }
        char::from_u32((self.0 + 97i16) as u32).unwrap().to_string()
    }
}
