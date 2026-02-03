pub mod expect;
pub mod expect_not_to_change_to;
pub mod expect_to_change_to;
pub mod scored_moves;
pub mod test_heat_map;
pub mod test_squares_map;
pub mod traits;

use super::scored_moves::ScoredMoves;
use super::traits::{CloneMoves, ToVecRef};
use libtchess::board::*;
use libtchess::board_config::BoardConfig;
use libtchess::buff::Buff;
use libtchess::castle_x_points::{CastleXPoints, KingCastleXPoint, RookCastleXPoint};
use libtchess::color::Color;
use libtchess::debuff::Debuff;
use libtchess::dimension::Dimension;
use libtchess::piece::Piece;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::player::Player;
use libtchess::point::Point;
use libtchess::vector::Vector;
use libtchess::vector::line_vector::LineVector;
use libtchess::vector_points::VectorPoints;
use std::env;
use std::fmt::{Debug, Display};
use test_heat_map::TestHeatMap;
use test_squares_map::TestSquaresMap;

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
            let str_vec = obj
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>();
            format!("{:?}", str_vec)
        }
    };
    let ref_formatter = |obj: Vec<&T>| {
        let debug = env::var("DEBUG").is_ok();
        if debug {
            format!("{:#?}", obj)
        } else {
            let str_vec = obj
                .iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>();
            format!("{:?}", str_vec)
        }
    };
    if lh_rest.len() > 0 && rh_rest.len() > 0 {
        let err = format!(
            r#"
              Expected {} to match {}. Missing elements: {}.
              Extra elements: {}.
            "#,
            formatter(vec1),
            formatter(vec2),
            ref_formatter(rh_rest),
            ref_formatter(lh_rest)
        );
        return Err(err);
    }
    if lh_rest.len() > 0 {
        let err = format!(
            "Expected {} to match {}. Extra elements: {}.",
            formatter(vec1),
            formatter(vec2),
            ref_formatter(lh_rest)
        );
        return Err(err);
    }
    if rh_rest.len() > 0 {
        let err = format!(
            "Expected {} to match {}. Missing elements: {}.",
            formatter(vec1),
            formatter(vec2),
            ref_formatter(rh_rest)
        );
        return Err(err);
    }
    if vec1.len() != vec2.len() {
        let err = format!("Expected {} to match {}.", formatter(vec1), formatter(vec2));
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
        }
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
    board: &mut Board<TestHeatMap, TestSquaresMap>,
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
pub fn board_config(dimension: Dimension, squares_map: TestSquaresMap) -> BoardConfig<TestHeatMap, TestSquaresMap> {
    BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Human,
        Player::Human,
    )
}

#[allow(unused)]
pub fn board_default_3x3() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_3x3_white_computer() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(3, 3));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Computer,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_default_4x4() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_4x4_white_computer() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Computer,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn board_default_5x5() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(4, 4));
    let squares_map = TestSquaresMap::from_dimension(&dimension);
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        TestHeatMap::init(),
        squares_map,
        dimension,
        Player::Human,
        Player::Human,
    );
    Board::empty(config)
}

#[allow(unused)]
pub fn scored_moves_of(board: &Board<TestHeatMap, TestSquaresMap>, pieces: Vec<&Piece>) -> Vec<ScoredMoves> {
    let mut res = vec![];

    for &piece in pieces.iter() {
        let scores = board.move_scores(piece.color());
        for score in scores {
            if let Some(scored_moves) = board.moves_by_score(piece.color(), score) {
                if let Some(piece_moves) = scored_moves.get(piece.id()) {
                    res.push(ScoredMoves::new(
                        piece.name(),
                        *piece.current_position(),
                        *score,
                        piece_moves.to_vec().clone_moves(),
                    ))
                }
            }
        }
    }
    res
}

#[allow(unused)]
pub fn all_moves(board: &Board<TestHeatMap, TestSquaresMap>, color: &Color) -> Vec<ScoredMoves> {
    let mut pieces = vec![];
    for score in board.move_scores(color) {
        pieces.append(
            &mut board
                .moves_by_score(color, score)
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
        )
    }
    let pieces = pieces
        .iter()
        .map(|piece_id| board.find_piece_by_id(piece_id).unwrap())
        .collect::<Vec<_>>();
    scored_moves_of(board, pieces)
}

pub struct PieceRepr {
    pub id: PieceId,
    pub color: Color,
    pub current_position: Point,
}

impl PieceRepr {
    pub fn id(&self) -> &PieceId {
        &self.id
    }

    pub fn current_position(&self) -> &Point {
        &self.current_position
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}

#[allow(unused)]
pub fn add_piece(
    board: &mut Board<TestHeatMap, TestSquaresMap>,
    piece_name: &str,
    color: Color,
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
    position: Point,
) -> PieceRepr {
    let piece_id = board.add_piece(piece_name, color, buffs, debuffs, position);
    let piece = board.find_piece_by_id(&piece_id).unwrap();

    PieceRepr {
        id: piece_id,
        color: *piece.color(),
        current_position: *piece.current_position(),
    }
}

#[allow(unused)]
pub fn move_piece(board: &mut Board<TestHeatMap, TestSquaresMap>, piece_id: PieceId, piece_move: PieceMove) {
    assert!(
        board.move_piece(&piece_id, &piece_move),
        "Failed to move {} on {} position",
        board
            .find_piece_by_id(&piece_id)
            .expect(format!("Could not find piece by id {}", piece_id).as_str()),
        piece_move.destination(),
    );
}
