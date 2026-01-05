use std::rc::Rc;
use tchess::board::*;
use tchess::board_config::{BoardConfig, CastleXPoints, KingCastleXPoint, RookCastleXPoint};
use tchess::board_square::{BoardSquare, Square};
use tchess::buff::Buff;
use tchess::color::Color;
use tchess::dimension::Dimension;
use tchess::heat_map::HeatMap;
use tchess::piece::Piece;
use tchess::point::Point;
use tchess::squares_map::SquaresMap;
use tchess::utils::pretty_print::PrettyPrint;

fn main() {
    let board = classic_chess_board();
    println!("White attack points");
    println!("{}", board.attack_points(&Color::White).pp_pieces());
    println!("\nBlack attack points");
    println!("{}", board.attack_points(&Color::Black).pp_pieces());
    println!("White pieces attacks");
    println!("{}", board.attack_points(&Color::White).pp_points());
    println!("\nBlack pieces attacks");
    println!("{}", board.attack_points(&Color::Black).pp_points());
    println!("White pieces defenses");
    println!("{}", board.defensive_points(&Color::White).pp_points());
    println!("\nBlack pieces defenses");
    println!("{}", board.defensive_points(&Color::Black).pp_points());
    println!("{}", board.pp());
}

// Temporary put entities for classic board here. It should be extracted in its own crate later.
struct ClassicSquaresMap;

impl ClassicSquaresMap {
    pub fn init() -> Self {
        Self {}
    }
}

impl SquaresMap for ClassicSquaresMap {
    fn square(&self, point: &Point) -> Option<BoardSquare> {
        let color = if point.x().value().wrapping_add(*point.y().value()) % 2 == 0 {
            Color::Black
        } else {
            Color::White
        };
        Some(BoardSquare::Square(Square::new(color, None)))
    }
}

struct HeatMapStub;

impl HeatMapStub {
    pub fn empty() -> Self {
        Self {}
    }
}

impl HeatMap for HeatMapStub {
    fn positional_value(&self, piece: &Rc<Piece>, _position: &Point) -> i16 {
        match &**piece {
            Piece::Bishop(_) => 300,
            Piece::King(_) => 0,
            Piece::Knight(_) => 300,
            Piece::Pawn(_) => 100,
            Piece::Queen(_) => 1000,
            Piece::Rook(_) => 500,
        }
    }
}

fn classic_chess_board() -> Board {
    let dimension = Dimension::new(Point::new(1, 1), Point::new(8, 8));
    let config = BoardConfig::new(
        CastleXPoints(KingCastleXPoint(7), RookCastleXPoint(6)),
        CastleXPoints(KingCastleXPoint(3), RookCastleXPoint(4)),
        Box::new(HeatMapStub::empty()),
        Box::new(ClassicSquaresMap::init()),
        dimension,
    );
    let mut board = Board::empty(config);

    for y in board.dimension().get_rows_range() {
        for x in board.dimension().get_columns_range() {
            let point = Point::new(x, y);
            match (y, x) {
                // White pieces
                (1, 1) | (1, 8) => {
                    board.add_piece(
                        "Rook", Color::White, vec![Buff::Castle], vec![], point,
                    );
                    ()
                },
                (1, 2) | (1, 7) => {
                    board.add_piece(
                        "Knight", Color::White, vec![], vec![], point,
                    );
                    ()
                },
                (1, 3) | (1, 6) => {
                    board.add_piece(
                        "Bishop", Color::White, vec![], vec![], point,
                    );
                    ()
                },
                (1, 4) => {
                    board.add_piece(
                        "Queen", Color::White, vec![], vec![], point,
                    );
                    ()
                },
                (1, 5) => {
                    board.add_piece(
                        "King", Color::White, vec![Buff::Castle], vec![], point,
                    );
                    ()
                },
                (2, _) => {
                    board.add_piece(
                        "Pawn", Color::White, vec![Buff::AdditionalPoint], vec![], point,
                    );
                    ()
                },
                // Black pieces
                (8, 1) | (8, 8) => {
                    board.add_piece(
                        "Rook", Color::Black, vec![Buff::Castle], vec![], point,
                    );
                    ()
                },
                (8, 2) | (8, 7) => {
                    board.add_piece(
                        "Knight", Color::Black, vec![], vec![], point,
                    );
                    ()
                },
                (8, 3) | (8, 6) => {
                    board.add_piece(
                        "Bishop", Color::Black, vec![], vec![], point,
                    );
                    ()
                },
                (8, 5) => {
                    board.add_piece(
                        "King", Color::Black, vec![Buff::Castle], vec![], point,
                    );
                    ()
                },
                (8, 4) => {
                    board.add_piece(
                        "Queen", Color::Black, vec![], vec![], point,
                    );
                    ()
                },
                (7, _) => {
                    board.add_piece(
                        "Pawn", Color::Black, vec![Buff::AdditionalPoint], vec![], point,
                    );
                    ()
                },
                _ => ()
            };
        }
    }
    board
}