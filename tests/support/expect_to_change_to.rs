use std::fmt::{Debug, Display};
use tchess::point::Point;
use super::{expect::Expect, compare, compare_and_assert};

pub trait ExpectToChangeTo<TT> {
    type Item;

    #[allow(unused)]
    fn to<ExpF>(&mut self, expectation: ExpF)
    where
        ExpF: Fn(&mut TT) -> Self::Item;
}

impl<T: Display + PartialEq + Debug, TT> ExpectToChangeTo<TT> for Expect<Vec<T>, TT> {
    type Item = Vec<T>;
    #[allow(unused)]
    fn to<ExpF>(&mut self, expectation: ExpF)
    where
        ExpF: Fn(&mut TT) -> Vec<T>,
    {
        let mut setup = self.setup_fn()();
        let change_fn = self.change_fn().unwrap();
        let initial_value = change_fn(&mut setup);
        self.subject_fn().unwrap()(&mut setup);
        let final_value = change_fn(&mut setup);

        if let Ok(_) = compare(&initial_value, &final_value) {
            panic!("Expect subject to change {:?}, but didn't.", initial_value);
        }
        compare_and_assert(&final_value, &expectation(&mut setup));
    }
}

impl<TT> ExpectToChangeTo<TT> for Expect<Point, TT> {
    type Item = Point;

    #[allow(unused)]
    fn to<ExpF>(&mut self, expectation: ExpF)
    where
        ExpF: Fn(&mut TT) -> Point,
    {
        let mut setup = self.setup_fn()();
        let change_fn = self.change_fn().unwrap();
        let initial_value = change_fn(&mut setup);
        self.subject_fn().unwrap()(&mut setup);
        let final_value = change_fn(&mut setup);

        assert_ne!(
            initial_value, final_value,
            "Expect subject to change {:?}, but didn't.",
            initial_value
        );
        assert_eq!(final_value, expectation(&mut setup));
    }
}
