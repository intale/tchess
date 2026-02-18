pub mod expect;
pub mod expect_not_to_change_to;
pub mod expect_to_change_to;
pub mod piece_moves_by_score;
pub mod test_heat_map;
pub mod test_squares_map;
pub mod traits;

use super::piece_moves_by_score::PieceMovesByScore;
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
use rustc_hash::FxHashMap;
use libtchess::move_score::MoveScore;
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
pub fn classic_8x8_prefilled() -> Board<TestHeatMap, TestSquaresMap> {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
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
    let mut board = Board::empty(config);

    for y in board.dimension().get_rows_range() {
        for x in board.dimension().get_columns_range() {
            let point = Point::new(x, y);
            match (y, x) {
                // White pieces
                (1, 1) | (1, 8) => {
                    board.add_piece("Rook", Color::White, vec![Buff::Castle], vec![], point);
                    ()
                }
                (1, 2) | (1, 7) => {
                    board.add_piece("Knight", Color::White, vec![], vec![], point);
                    ()
                }
                (1, 3) | (1, 6) => {
                    board.add_piece("Bishop", Color::White, vec![], vec![], point);
                    ()
                }
                (1, 4) => {
                    board.add_piece("Queen", Color::White, vec![], vec![], point);
                    ()
                }
                (1, 5) => {
                    board.add_piece("King", Color::White, vec![Buff::Castle], vec![], point);
                    ()
                }
                (2, _) => {
                    board.add_piece(
                        "Pawn",
                        Color::White,
                        vec![Buff::AdditionalPoint],
                        vec![],
                        point,
                    );
                    ()
                }
                // Black pieces
                (8, 1) | (8, 8) => {
                    board.add_piece("Rook", Color::Black, vec![Buff::Castle], vec![], point);
                    ()
                }
                (8, 2) | (8, 7) => {
                    board.add_piece("Knight", Color::Black, vec![], vec![], point);
                    ()
                }
                (8, 3) | (8, 6) => {
                    board.add_piece("Bishop", Color::Black, vec![], vec![], point);
                    ()
                }
                (8, 5) => {
                    board.add_piece("King", Color::Black, vec![Buff::Castle], vec![], point);
                    ()
                }
                (8, 4) => {
                    board.add_piece("Queen", Color::Black, vec![], vec![], point);
                    ()
                }
                (7, _) => {
                    board.add_piece(
                        "Pawn",
                        Color::Black,
                        vec![Buff::AdditionalPoint],
                        vec![],
                        point,
                    );
                    ()
                }
                _ => (),
            };
        }
    }
    board
}

#[allow(unused)]
pub fn scored_moves_of(board: &Board<TestHeatMap, TestSquaresMap>, pieces: Vec<&Piece>) -> Vec<PieceMovesByScore> {
    let mut res = vec![];

    for &piece in pieces.iter() {
        let moves_with_score = all_moves(board, piece.color());
        for move_with_score in moves_with_score {
            if &move_with_score.current_position == piece.current_position() && piece.name() == move_with_score.piece_name {
                res.push(move_with_score)
            }
        }
    }
    res
}

#[allow(unused)]
pub fn all_moves(board: &Board<TestHeatMap, TestSquaresMap>, color: &Color) -> Vec<PieceMovesByScore> {
    let mut moves_by_score: FxHashMap<(PieceId, MoveScore), Vec<PieceMove>>  = FxHashMap::default();
    for (move_score, piece_to_moves) in board.score_to_moves(color) {
        for (piece_id, piece_moves) in piece_to_moves {
            let key = (*piece_id, *move_score);
            if !moves_by_score.contains_key(&key) {
                moves_by_score.insert(key, vec![]);
            }
            let moves = moves_by_score.get_mut(&key).unwrap();
            for piece_move in piece_moves {
                moves.push(*piece_move);
            }
        }
    }

    moves_by_score.iter().map(|((piece_id, move_score), piece_moves)| {
        let piece = board.find_piece_by_id(piece_id).unwrap();
        PieceMovesByScore::new(
            piece.name(),
            *piece.current_position(),
            *move_score,
            piece_moves.clone(),
        )
    }).collect::<Vec<_>>()
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
        board.move_piece(&piece_id, &piece_move).is_some(),
        "Failed to move {} on {} position",
        board
            .find_piece_by_id(&piece_id)
            .expect(format!("Could not find piece by id {}", piece_id).as_str()),
        piece_move.destination(),
    );
}

#[allow(unused)]
pub fn move_piece_at(board: &mut Board<TestHeatMap, TestSquaresMap>, point: Point, piece_move: PieceMove) {
    let piece_id = *board.piece_id_at(&point).unwrap();
    assert!(
        board.move_piece(&piece_id, &piece_move).is_some(),
        "Failed to move {} on {} position",
        board
            .find_piece_by_id(&piece_id)
            .expect(format!("Could not find piece by id {}", piece_id).as_str()),
        piece_move.destination(),
    );
}
