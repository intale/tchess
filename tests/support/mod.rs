pub mod traits;
pub mod expect;
pub mod expect_to_change_to;
pub mod expect_not_to_change_to;

use std::fmt::Debug;
use tchess::board::*;
use tchess::buff::Buff;
use tchess::color::Color;
use tchess::debuff::Debuff;
use tchess::dimension::Dimension;
use tchess::point::Point;
use tchess::vector::Vector;
use tchess::vector::line_vector::LineVector;
use tchess::vector_points::VectorPoints;

#[allow(unused)]
pub fn compare<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> Result<String, String>
where
    T: Debug + PartialEq,
{
    let lh_rest = vec1
        .iter()
        .filter(|effect| !vec2.contains(effect))
        .collect::<Vec<_>>();
    let rh_rest = vec2
        .iter()
        .filter(|effect| !vec1.contains(effect))
        .collect::<Vec<_>>();
    if lh_rest.len() == 0 && rh_rest.len() == 0 && vec1.len() == vec2.len() {
        return Ok("Empty arrays".to_string());
    }

    if lh_rest.len() > 0 && rh_rest.len() > 0 {
        let err = format!(
            r#"
              Expected {vec1:?} to match {vec2:?}. Missing elements: {rh_rest:?}.
              Extra elements: {lh_rest:?}.
            "#
        );
        return Err(err);
    }
    if lh_rest.len() > 0 {
        let err = format!("Expected {vec1:?} to match {vec2:?}. Extra elements: {lh_rest:?}.");
        return Err(err);
    }
    if rh_rest.len() > 0 {
        let err = format!("Expected {vec1:?} to match {vec2:?}. Missing elements: {rh_rest:?}.");
        return Err(err);
    }
    if vec1.len() != vec2.len() {
        let err = format!("Expected {vec1:?} to match {vec2:?}.");
        return Err(err);
    }
    Ok("Arrays match".to_string())
}

#[allow(unused)]
pub fn draw_box(dimension: Dimension) -> Vec<Point> {
    let box_points = [
        *dimension.min_point(),
        Point::new(
            *dimension.min_point().x().value(),
            *dimension.max_point().y().value(),
        ),
        *dimension.max_point(),
        Point::new(
            *dimension.max_point().x().value(),
            *dimension.min_point().y().value(),
        ),
    ];
    let mut points: Vec<Point> = vec![];
    for (index, start_point) in box_points.iter().enumerate() {
        let end_point = if index == box_points.len() - 1 {
            box_points[0]
        } else {
            box_points[index + 1]
        };
        let vector = LineVector::calc_direction(start_point, &end_point).unwrap();
        let vector_points =
            VectorPoints::with_initial(*start_point, dimension, Vector::Line(vector));
        for point in vector_points {
            if point == end_point {
                break;
            }
            points.push(point)
        }
    }
    points
}

#[allow(unused)]
pub fn create_box_of(
    board: &mut Board,
    name: &str,
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    dimension: Dimension,
) {
    for point in draw_box(dimension) {
        let buffs = buffs.iter().map(|buff| *buff).collect::<Vec<_>>();
        let debuffs = debuffs.iter().map(|debuff| *debuff).collect::<Vec<_>>();
        board.add_piece(name, color, buffs, debuffs, point);
    }
}
