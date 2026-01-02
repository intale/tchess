use std::fmt::{Debug, Display};
use super::{expect::Expect, compare};

pub trait ExpectNotToChange<TT> {
    type Item;

    #[allow(unused)]
    fn not_to_change<F>(&mut self, change_fn: F)
    where
        F: Fn(&mut TT) -> Self::Item + 'static;
}

impl<T: PartialEq + Debug + Display, TT> ExpectNotToChange<TT> for Expect<Vec<T>, TT> {
    type Item = Vec<T>;

    fn not_to_change<F>(&mut self, change_fn: F)
    where
        F: Fn(&mut TT) -> Self::Item + 'static
    {
        let mut setup = self.setup_fn()();
        let initial_value = change_fn(&mut setup);
        self.subject_fn().unwrap()(&mut setup);
        let final_value = change_fn(&mut setup);

        let result = compare(&initial_value, &final_value);
        match result {
            Ok(_) => (),
            Err(_) => {
                panic!(
                    "Expect subject not to change {:?}, but it changed to {:?}.",
                    initial_value, final_value
                );
            }
        }
    }
}
