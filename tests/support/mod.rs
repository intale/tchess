pub mod traits;

use std::fmt::Debug;
use tchess::utils::pretty_print::PrettyPrint;
use tchess::board::*;
use tchess::buff::Buff;
use tchess::color::Color;
use tchess::debuff::Debuff;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::vector::line_vector::LineVector;
use tchess::vector::Vector;
use tchess::vector_points::VectorPoints;

#[allow(unused)]
pub fn compare<T>(board: &Board, vec1: &Vec<T>, vec2: &Vec<T>)
where T: Debug + PartialEq {
    println!("{}", board.pp());
    let lh_rest = vec1.iter().filter(|effect| !vec2.contains(effect)).collect::<Vec<_>>();
    let rh_rest = vec2.iter().filter(|effect| !vec1.contains(effect)).collect::<Vec<_>>();
    if lh_rest.len() == 0 && rh_rest.len() == 0 && vec1.len() == vec2.len() {
        return
    }

    if lh_rest.len() > 0 && rh_rest.len() > 0 {
        panic!(r#"
              Expected {vec1:?} to match {vec2:?}. Missing elements: {rh_rest:?}.
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

#[allow(unused)]
pub fn draw_box(dimension: Dimension) -> Vec<Point> {
    let box_points = [
        *dimension.min_point(),
        Point::new(*dimension.min_point().x().value(), *dimension.max_point().y().value()),
        *dimension.max_point(),
        Point::new(*dimension.max_point().x().value(), *dimension.min_point().y().value()),
    ];
    let mut points: Vec<Point> = vec![];
    for (index, start_point) in box_points.iter().enumerate() {
        let end_point = if index == box_points.len() - 1 {
            box_points[0]
        } else {
            box_points[index + 1]
        };
        let vector = LineVector::calc_direction(start_point, &end_point).unwrap();
        let vector_points = VectorPoints::with_initial(
            *start_point, dimension, Vector::Line(vector)
        );
        for point in vector_points {
            if point == end_point {
                break
            }
            points.push(point)
        }
    }
    points
}

#[allow(unused)]
pub fn create_box_of(board: &mut Board, name: &str, color: Color, buffs: Vec<Buff>,
                     debuffs: Vec<Debuff>, dimension: Dimension) {
    for point in draw_box(dimension) {
        let buffs = buffs.iter().map(|buff| *buff).collect::<Vec<_>>();
        let debuffs = debuffs.iter().map(|debuff| *debuff).collect::<Vec<_>>();
        board.add_piece(name, color, buffs, debuffs, point);
    }
}

#[allow(unused)]
pub struct Expect<T, TT> {
    setup: Box<dyn Fn() -> TT>,
    subject_fn: Option<Box<dyn Fn(&mut TT)>>,
    change_fn: Option<Box<dyn Fn(&mut TT) -> T>>,
}

impl<T: PartialEq + Debug, TT> Expect<T, TT> {
    #[allow(unused)]
    pub fn setup<F>(setup_fn: F) -> Self
    where F: Fn() -> TT + 'static
    {
        Self { setup: Box::new(setup_fn), subject_fn: None, change_fn: None }
    }

    #[allow(unused)]
    pub fn expect<F>(&mut self, subject_fn: F) -> &mut Self
    where F: Fn(&mut TT) + 'static

    {
        self.subject_fn = Some(Box::new(subject_fn));
        self
    }

    #[allow(unused)]
    pub fn to_change<F>(&mut self, change_fn: F) -> &mut Self
    where F: Fn(&mut TT) -> T + 'static
    {
        self.change_fn = Some(Box::new(change_fn));
        self
    }

    #[allow(unused)]
    pub fn not_to_change<F>(&mut self, change_fn: F)
    where F: Fn(&mut TT) -> T + 'static
    {
        let mut setup = self.setup.as_ref()();
        let initial_value = change_fn(&mut setup);
        self.subject_fn.as_ref().unwrap()(&mut setup);
        let final_value = change_fn(&mut setup);

        assert_eq!(
            initial_value, final_value,
            "Expect subject not to change {:?}, but it changed to {:?}.", initial_value, final_value
        );
    }

    #[allow(unused)]
    pub fn to<ExpF>(&mut self, expectation: ExpF)
    where ExpF: Fn(&mut TT) -> T
    {
        let mut setup = self.setup.as_ref()();
        let change_fn = self.change_fn.as_ref().unwrap();
        let initial_value = change_fn(&mut setup);
        self.subject_fn.as_ref().unwrap()(&mut setup);
        let final_value = change_fn(&mut setup);

        assert_ne!(
            initial_value, final_value,
            "Expect subject to change {:?}, but didn't.", initial_value
        );
        assert_eq!(final_value, expectation(&mut setup));
    }
}
