pub mod traits;
pub mod expect;
pub mod expect_to_change_to;
pub mod expect_not_to_change_to;
pub mod test_squares_map;
pub mod test_heat_map;
pub mod scored_moves;

use std::env;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use tchess::board::*;
use tchess::board_config::{BoardConfig};
use tchess::buff::Buff;
use tchess::castle_x_points::{CastleXPoints, KingCastleXPoint, RookCastleXPoint};
use tchess::color::Color;
use tchess::debuff::Debuff;
use tchess::dimension::Dimension;
use tchess::piece::Piece;
use tchess::player::Player;
use tchess::point::Point;
use tchess::static_piece_weights::StaticPieceWeights;
use tchess::vector::Vector;
use tchess::vector::line_vector::LineVector;
use tchess::vector_points::VectorPoints;
use test_squares_map::TestSquaresMap;
use test_heat_map::TestHeatMap;
use super::scored_moves::ScoredMoves;
use super::traits::{CloneMoves, ToVecRef};

#[allow(unused)]
pub fn compare<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> Result<String, String>
where
    T: Display + Debug + PartialEq,
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

    let formatter = |obj: &Vec<T>| {
        let debug = env::var("DEBUG").is_ok();
        if debug {
            format!("{:#?}", obj)
        } else {
            let str_vec = obj.iter().map(|item| format!("{}", item)).collect::<Vec<_>>();
            format!("{:?}", str_vec)
        }
    };
    let ref_formatter = |obj: Vec<&T>| {
        let debug = env::var("DEBUG").is_ok();
        if debug {
            format!("{:#?}", obj)
        } else {
            let str_vec = obj.iter().map(|item| format!("{}", item)).collect::<Vec<_>>();
            format!("{:?}", str_vec)
        }
    };
    if lh_rest.len() > 0 && rh_rest.len() > 0 {
        let err = format!(
            r#"
              Expected {} to match {}. Missing elements: {}.
              Extra elements: {}.
            "#,
            formatter(vec1), formatter(vec2), ref_formatter(rh_rest), ref_formatter(lh_rest)
        );
        return Err(err);
    }
    if lh_rest.len() > 0 {
        let err = format!(
            "Expected {} to match {}. Extra elements: {}.",
            formatter(vec1), formatter(vec2), ref_formatter(lh_rest)
        );
        return Err(err);
    }
    if rh_rest.len() > 0 {
        let err = format!(
            "Expected {} to match {}. Missing elements: {}.",
            formatter(vec1), formatter(vec2), ref_formatter(rh_rest)
        );
        return Err(err);
    }
    if vec1.len() != vec2.len() {
        let err = format!(
            "Expected {} to match {}.",
            formatter(vec1), formatter(vec2)
        );
        return Err(err);
    }
    Ok("Arrays match".to_string())
}

pub fn compare_and_assert<T>(vec1: &Vec<T>, vec2: &Vec<T>)
where
    T: Display + Debug + PartialEq,
{
    let result = compare::<T>(vec1, vec2);
    match result {
        Ok(_) => (),
        Err(msg) => {
            panic!("{}", msg);
        },
    };
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

#[allow(unused)]
pub fn board_config(dimension: Dimension, squares_map: TestSquaresMap) -> BoardConfig {
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Human,
        Player::Human,
    )
}

#[allow(unused)]
pub fn board_default_3x3() -> Board {
    let dimension = Dimension::new(Point::new(1,1), Point::new(3, 3));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_3x3_white_computer() -> Board {
    let dimension = Dimension::new(Point::new(1,1), Point::new(3, 3));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Computer,
        Player::Human,
    );
    Board::empty(config)
}


#[allow(unused)]
pub fn board_default_4x4() -> Board {
    let dimension = Dimension::new(Point::new(1,1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_4x4_white_computer() -> Board {
    let dimension = Dimension::new(Point::new(1,1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Computer,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_default_5x5() -> Board {
    let dimension = Dimension::new(Point::new(1,1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let static_weights = StaticPieceWeights {
        bishop: 3,
        king: 0,
        knight: 3,
        pawn: 1,
        queen: 10,
        rook: 5,
    };
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(TestHeatMap::init()),
        Box::new(squares_map),
        dimension,
        static_weights,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn scored_moves_of(board: &Board, pieces: Vec<&Rc<Piece>>) -> Vec<ScoredMoves> {
    let mut res = vec![];

    for piece in pieces.iter() {
        let scores = board.move_scores(piece.color());
        for score in scores {
            if board.moves_by_score(piece, score).is_none() {
                continue
            }
            res.push(
                ScoredMoves::new(
                    piece.name(),
                    piece.current_position(),
                    *score,
                    board.moves_by_score(piece, score).to_vec().clone_moves(),
                )
            )
        }
    }
    res
}
