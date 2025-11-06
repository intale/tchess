#[path = "../support/mod.rs"]
mod support;

use tchess::board::Board;
use tchess::color::Color;
use tchess::point::Point;
use support::compare;
use support::to_vec::ToVecRef;
use tchess::buff::Buff;
use tchess::castle_points::{CastlePoints};
use tchess::piece_move::PieceMove;

#[test]
fn when_there_are_no_pieces_around() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_an_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Pawn", Color::Black, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_a_protected_enemy_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Pawn", Color::Black, vec![], vec![], Point::new(4, 4)
    );
    board.add_piece(
        "Bishop", Color::Black, vec![], vec![], Point::new(5, 5)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_there_is_an_ally_piece_on_the_way() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(5, 5));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(3, 3)
    );
    board.add_piece(
        "Bishop", Color::White, vec![], vec![], Point::new(4, 4)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(2, 2)),
            &PieceMove::Point(Point::new(2, 3)),
            &PieceMove::Point(Point::new(2, 4)),
            &PieceMove::Point(Point::new(3, 4)),
            &PieceMove::Point(Point::new(4, 3)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(3, 2)),
        ],
    );
}

#[test]
fn when_move_points_are_under_attack() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(4, 4));
    let king = board.add_piece(
        "King", Color::White, vec![], vec![], Point::new(2, 2)
    );
    board.add_piece(
        "King", Color::Black, vec![], vec![], Point::new(2, 4)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(1, 2)),
            &PieceMove::Point(Point::new(3, 2)),
            &PieceMove::Point(Point::new(1, 1)),
            &PieceMove::Point(Point::new(2, 1)),
            &PieceMove::Point(Point::new(3, 1)),
        ],
    );
}

#[test]
fn when_castle_is_available_for_king_only() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let white_king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );
    board.add_piece(
        "Rook", Color::White, vec![], vec![], Point::new(1, 1)
    );

    let black_king = board.add_piece(
        "King", Color::Black, vec![Buff::Castle], vec![], Point::new(5, 8)
    );
    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(1, 8)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&white_king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(4, 1)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(6, 1)),
        ],
    );
    compare(
        &board,
        &board.moves(&Color::Black).moves(&black_king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(4, 8)),
            &PieceMove::Point(Point::new(4, 7)),
            &PieceMove::Point(Point::new(5, 7)),
            &PieceMove::Point(Point::new(6, 7)),
            &PieceMove::Point(Point::new(6, 8)),
        ],
    );
}

#[test]
fn when_castle_is_available_for_king_and_one_rook() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let white_king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );
    let white_rook = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(1, 1)
    );

    let black_king = board.add_piece(
        "King", Color::Black, vec![Buff::Castle], vec![], Point::new(5, 8)
    );
    let black_rook = board.add_piece(
        "Rook", Color::Black, vec![Buff::Castle], vec![], Point::new(1, 8)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&white_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 1), Point::new(4, 1),
                    white_king.current_position(), white_rook.current_position()
                )
            ),
            &PieceMove::Point(Point::new(4, 1)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(6, 1)),
        ],
    );
    compare(
        &board,
        &board.moves(&Color::Black).moves(&black_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 8), Point::new(4, 8),
                    black_king.current_position(), black_rook.current_position()
                )
            ),
            &PieceMove::Point(Point::new(4, 8)),
            &PieceMove::Point(Point::new(4, 7)),
            &PieceMove::Point(Point::new(5, 7)),
            &PieceMove::Point(Point::new(6, 7)),
            &PieceMove::Point(Point::new(6, 8)),
        ],
    );
}


#[test]
fn when_castle_is_available_for_king_and_two_rooks() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let white_king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );
    let white_rook1 = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(1, 1)
    );
    let white_rook2 = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(8, 1)
    );

    let black_king = board.add_piece(
        "King", Color::Black, vec![Buff::Castle], vec![], Point::new(5, 8)
    );
    let black_rook1 = board.add_piece(
        "Rook", Color::Black, vec![Buff::Castle], vec![], Point::new(1, 8)
    );
    let black_rook2 = board.add_piece(
        "Rook", Color::Black, vec![Buff::Castle], vec![], Point::new(8, 8)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&white_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 1), Point::new(4, 1),
                    white_king.current_position(), white_rook1.current_position()
                )
            ),
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(7, 1), Point::new(6, 1),
                    white_king.current_position(), white_rook2.current_position()
                )
            ),
            &PieceMove::Point(Point::new(4, 1)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(6, 1)),
        ],
    );
    compare(
        &board,
        &board.moves(&Color::Black).moves(&black_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 8), Point::new(4, 8),
                    black_king.current_position(), black_rook1.current_position()
                )
            ),
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(7, 8), Point::new(6, 8),
                    black_king.current_position(), black_rook2.current_position()
                )
            ),
            &PieceMove::Point(Point::new(4, 8)),
            &PieceMove::Point(Point::new(4, 7)),
            &PieceMove::Point(Point::new(5, 7)),
            &PieceMove::Point(Point::new(6, 7)),
            &PieceMove::Point(Point::new(6, 8)),
        ],
    );
}

#[test]
fn when_castle_is_available_for_king_and_two_rooks_for_non_classic_position() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let white_king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(4, 1)
    );
    let white_rook1 = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(3, 1)
    );
    let white_rook2 = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );
    // Add two white pawns so they cover castle way of both sides
    board.add_piece(
        "Pawn", Color::White, vec![Buff::Castle], vec![], Point::new(3, 2)
    );
    board.add_piece(
        "Pawn", Color::White, vec![Buff::Castle], vec![], Point::new(5, 2)
    );

    let black_king = board.add_piece(
        "King", Color::Black, vec![Buff::Castle], vec![], Point::new(4, 8)
    );
    let black_rook1 = board.add_piece(
        "Rook", Color::Black, vec![Buff::Castle], vec![], Point::new(3, 8)
    );
    let black_rook2 = board.add_piece(
        "Rook", Color::Black, vec![Buff::Castle], vec![], Point::new(5, 8)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&white_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 1), Point::new(4, 1),
                    white_king.current_position(), white_rook1.current_position()
                )
            ),
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(7, 1), Point::new(6, 1),
                    white_king.current_position(), white_rook2.current_position()
                )
            ),
            &PieceMove::Point(Point::new(4, 2)),
        ],
    );
    compare(
        &board,
        &board.moves(&Color::Black).moves(&black_king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 8), Point::new(4, 8),
                    black_king.current_position(), black_rook1.current_position()
                )
            ),
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(7, 8), Point::new(6, 8),
                    black_king.current_position(), black_rook2.current_position()
                )
            ),
            &PieceMove::Point(Point::new(3, 7)),
            &PieceMove::Point(Point::new(4, 7)),
            &PieceMove::Point(Point::new(5, 7)),
        ],
    );
}

#[test]
fn when_king_castle_point_is_under_attack() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );
    board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(1, 1)
    );

    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(3, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(4, 1)),
            &PieceMove::Point(Point::new(4, 2)),
            &PieceMove::Point(Point::new(5, 2)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(6, 1)),
        ],
    );
}

#[test]
fn when_king_castle_way_is_under_attack() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(7, 1)
    );
    board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(5, 1)
    );

    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(5, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(6, 1)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(7, 2)),
            &PieceMove::Point(Point::new(8, 2)),
            &PieceMove::Point(Point::new(8, 1)),
        ],
    );
}

#[test]
fn when_rook_castle_way_is_under_attack() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(7, 1)
    );
    let white_rook = board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(1, 1)
    );

    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(2, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Castle(
                CastlePoints::new(
                    Point::new(3, 1), Point::new(4, 1),
                    king.current_position(), white_rook.current_position()
                )
            ),
            &PieceMove::Point(Point::new(6, 1)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(7, 2)),
            &PieceMove::Point(Point::new(8, 2)),
            &PieceMove::Point(Point::new(8, 1)),
        ],
    );
}

#[test]
fn when_king_is_under_check() {
    let mut board = Board::empty(Point::new(1, 1), Point::new(8, 8));
    let king = board.add_piece(
        "King", Color::White, vec![Buff::Castle], vec![], Point::new(7, 1)
    );
    board.add_piece(
        "Rook", Color::White, vec![Buff::Castle], vec![], Point::new(1, 1)
    );

    board.add_piece(
        "Rook", Color::Black, vec![], vec![], Point::new(7, 3)
    );

    compare(
        &board,
        &board.moves(&Color::White).moves(&king).to_vec(),
        &vec![
            &PieceMove::Point(Point::new(6, 1)),
            &PieceMove::Point(Point::new(6, 2)),
            &PieceMove::Point(Point::new(8, 2)),
            &PieceMove::Point(Point::new(8, 1)),
        ],
    );
}
