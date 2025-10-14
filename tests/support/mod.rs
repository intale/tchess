pub mod to_vec;

use std::collections::HashSet;
use std::fmt::Debug;

use tchess::utils::pretty_print::PrettyPrint;
use tchess::board::*;

pub fn compare<T>(board: &Board, vec1: &Vec<T>, vec2: &Vec<T>)
where T: Debug + PartialEq {
    let lh_rest = vec1.iter().filter(|debuff| !vec2.contains(debuff)).collect::<Vec<_>>();
    let rh_rest = vec2.iter().filter(|debuff| !vec1.contains(debuff)).collect::<Vec<_>>();
    if lh_rest.len() == 0 && rh_rest.len() == 0 && vec1.len() == vec2.len() {
        return
    }

    println!("{}", board.pp());
    if lh_rest.len() > 0 && rh_rest.len() > 0 {
        panic!(r#"
              Expected {vec1:?} to match {vec2:?}. Missing elements: {rh_rest:?}. \
              Extra elements: {lh_rest:?}.
            "#);
    }
    if lh_rest.len() > 0 {
        panic!("Expected {vec1:?} to match {vec2:?}. Extra elements: {lh_rest:?}.")
    }
    if rh_rest.len() > 0 {
        panic!("Expected {vec1:?} to match {vec2:?}. Missing elements: {rh_rest:?}.")
    }
    if vec1.len() != vec2.len() {
        panic!("Expected {vec1:?} to match {vec2:?}.")
    }
}
