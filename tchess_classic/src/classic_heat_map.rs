use libtchess::color::Color;
use libtchess::heat_map::HeatMap;
use libtchess::piece::Piece;
use libtchess::point::Point;

const BISHOP_P0_MAP: [[i16; 8]; 8] = [
    [324, 325, 325, 325, 325, 325, 325, 324],
    [325, 328, 328, 328, 328, 328, 328, 325],
    [325, 328, 332, 333, 333, 332, 328, 325],
    [325, 328, 333, 336, 336, 333, 328, 325],
    [325, 328, 333, 336, 336, 333, 328, 325],
    [325, 328, 332, 333, 333, 332, 328, 325],
    [325, 328, 328, 328, 328, 328, 328, 325],
    [324, 325, 325, 325, 325, 325, 325, 324],
];
const BISHOP_P1_MAP: [[i16; 8]; 8] = [
    [325, 326, 326, 326, 326, 326, 326, 325],
    [326, 328, 328, 328, 328, 328, 328, 326],
    [326, 328, 331, 333, 333, 331, 328, 326],
    [326, 328, 333, 335, 335, 333, 328, 326],
    [326, 328, 333, 335, 335, 333, 328, 326],
    [326, 328, 331, 333, 333, 331, 328, 326],
    [326, 328, 328, 328, 328, 328, 328, 326],
    [325, 326, 326, 326, 326, 326, 326, 325],
];

const KING_P0_MAP: [[i16; 8]; 8] = [
    [10, 12, 8, 0, 0, 8, 12, 10],
    [10, 10, 5, -4, -4, 5, 10, 10],
    [7, 5, 0, -8, -8, 0, 5, 7],
    [4, 1, -5, -12, -12, -5, 1, 4],
    [2, -2, -8, -16, -16, -8, -2, 2],
    [1, -4, -10, -18, -18, -10, -4, 1],
    [1, -4, -10, -18, -18, -10, -4, 1],
    [1, -4, -10, -18, -18, -10, -4, 1],
];
const KING_P1_MAP: [[i16; 8]; 8] = [
    [-4, -2, 1, 3, 3, 1, -2, -4],
    [-2, 1, 4, 6, 6, 4, 1, -2],
    [1, 4, 7, 9, 9, 7, 4, 1],
    [3, 6, 9, 11, 11, 9, 6, 3],
    [3, 6, 9, 11, 11, 9, 6, 3],
    [1, 4, 7, 9, 9, 7, 4, 1],
    [-2, 1, 4, 6, 6, 4, 1, -2],
    [-4, -2, 1, 3, 3, 1, -2, -4],
];

const KNIGHT_P0_MAP: [[i16; 8]; 8] = [
    [310, 312, 313, 313, 313, 313, 312, 310],
    [312, 316, 318, 318, 318, 318, 316, 312],
    [313, 318, 323, 325, 325, 323, 318, 313],
    [313, 318, 325, 329, 329, 325, 318, 313],
    [313, 318, 325, 330, 330, 325, 318, 313],
    [313, 318, 323, 325, 325, 323, 318, 313],
    [312, 316, 318, 318, 318, 318, 316, 312],
    [310, 312, 313, 313, 313, 313, 312, 310],
];
const KNIGHT_P1_MAP: [[i16; 8]; 8] = [
    [312, 314, 315, 315, 315, 315, 314, 312],
    [314, 317, 318, 319, 319, 318, 317, 314],
    [315, 318, 322, 324, 324, 322, 318, 315],
    [315, 319, 324, 327, 327, 324, 319, 315],
    [315, 319, 324, 327, 327, 324, 319, 315],
    [315, 318, 322, 324, 324, 322, 318, 315],
    [314, 317, 318, 319, 319, 318, 317, 314],
    [312, 314, 315, 315, 315, 315, 314, 312],
];

const PAWN_P0_MAP: [[i16; 8]; 8] = [
    [100, 100, 100, 100, 100, 100, 100, 100],
    [110, 112, 114, 118, 118, 114, 112, 110],
    [107, 109, 111, 115, 115, 111, 109, 107],
    [104, 106, 108, 112, 112, 108, 106, 104],
    [102, 103, 105, 109, 109, 105, 103, 102],
    [101, 102, 103, 105, 105, 103, 102, 101],
    [100, 100, 100, 97, 97, 100, 100, 100],
    [100, 100, 100, 100, 100, 100, 100, 100],
];
const PAWN_P1_MAP: [[i16; 8]; 8] = [
    [100, 100, 100, 100, 100, 100, 100, 100],
    [118, 120, 122, 126, 126, 122, 120, 118],
    [112, 114, 116, 120, 120, 116, 114, 112],
    [107, 109, 111, 115, 115, 111, 109, 107],
    [104, 105, 107, 110, 110, 107, 105, 104],
    [102, 103, 104, 106, 106, 104, 103, 102],
    [100, 100, 100, 97, 97, 100, 100, 100],
    [100, 100, 100, 100, 100, 100, 100, 100],
];

const QUEEN_P0_MAP: [[i16; 8]; 8] = [
    [894, 895, 896, 896, 896, 896, 895, 894],
    [895, 897, 898, 898, 898, 898, 897, 895],
    [896, 898, 900, 901, 901, 900, 898, 896],
    [896, 898, 901, 903, 903, 901, 898, 896],
    [896, 898, 901, 903, 903, 901, 898, 896],
    [896, 898, 900, 901, 901, 900, 898, 896],
    [895, 897, 898, 898, 898, 898, 897, 895],
    [894, 895, 896, 896, 896, 896, 895, 894],
];
const QUEEN_P1_MAP: [[i16; 8]; 8] = [
    [896, 897, 898, 898, 898, 898, 897, 896],
    [897, 898, 900, 900, 900, 900, 898, 897],
    [898, 900, 902, 903, 903, 902, 900, 898],
    [898, 900, 903, 905, 905, 903, 900, 898],
    [898, 900, 903, 905, 905, 903, 900, 898],
    [898, 900, 902, 903, 903, 902, 900, 898],
    [897, 898, 900, 900, 900, 900, 898, 897],
    [896, 897, 898, 898, 898, 898, 897, 896],
];

const ROOK_P0_MAP: [[i16; 8]; 8] = [
    [506, 507, 507, 508, 508, 507, 507, 506],
    [506, 507, 507, 508, 508, 507, 507, 506],
    [502, 503, 503, 504, 504, 503, 503, 502],
    [500, 501, 501, 502, 502, 501, 501, 500],
    [500, 501, 501, 502, 502, 501, 501, 500],
    [499, 500, 500, 501, 501, 500, 500, 499],
    [498, 499, 499, 500, 500, 499, 499, 498],
    [497, 498, 499, 500, 500, 499, 498, 497],
];

const ROOK_P1_MAP: [[i16; 8]; 8] = [
    [508, 509, 509, 510, 510, 509, 509, 508],
    [507, 508, 508, 509, 509, 508, 508, 507],
    [504, 505, 505, 506, 506, 505, 505, 504],
    [502, 503, 503, 504, 504, 503, 503, 502],
    [501, 502, 502, 503, 503, 502, 502, 501],
    [500, 501, 501, 502, 502, 501, 501, 500],
    [499, 500, 500, 501, 501, 500, 500, 499],
    [498, 499, 499, 500, 500, 499, 499, 498],
];

pub struct ClassicHeatMap {
    phase_ratio: f32,
}

impl ClassicHeatMap {
    pub fn init() -> Self {
        Self { phase_ratio: 0.0 }
    }

    pub fn update_phase_ratio(&mut self, ratio: f32) {
        self.phase_ratio = ratio;
    }
}

impl HeatMap for ClassicHeatMap {
    // The actual implementation will be presented later. For now just put stub values here.
    fn positional_value(&self, piece: &Piece, position: &Point) -> i16 {
        let (x, y) = match piece.color() {
            Color::White => (
                *position.x().value() as usize - 1,
                8 - *position.y().value() as usize,
            ),
            Color::Black => (
                8 - *position.x().value() as usize,
                *position.y().value() as usize - 1,
            ),
        };

        match piece {
            Piece::Bishop(_) => ((1.0 - self.phase_ratio) * BISHOP_P0_MAP[y][x] as f32
                + self.phase_ratio * BISHOP_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::King(_) => ((1.0 - self.phase_ratio) * KING_P0_MAP[y][x] as f32
                + self.phase_ratio * KING_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::Knight(_) => ((1.0 - self.phase_ratio) * KNIGHT_P0_MAP[y][x] as f32
                + self.phase_ratio * KNIGHT_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::Pawn(_) => ((1.0 - self.phase_ratio) * PAWN_P0_MAP[y][x] as f32
                + self.phase_ratio * PAWN_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::Queen(_) => ((1.0 - self.phase_ratio) * QUEEN_P0_MAP[y][x] as f32
                + self.phase_ratio * QUEEN_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::Rook(_) => ((1.0 - self.phase_ratio) * ROOK_P0_MAP[y][x] as f32
                + self.phase_ratio * ROOK_P1_MAP[y][x] as f32)
                .round() as i16,
            Piece::UnknownPiece(_) => panic!("Unknown piece can't be evaluated!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libtchess::piece::PieceInit;
    use libtchess::piece::king::King;
    use libtchess::piece::pawn::Pawn;
    use libtchess::piece_id::PieceId;

    #[test]
    fn value_of_white_king_king_side_position() {
        let heat_map = ClassicHeatMap::init();
        let king = Piece::King(King::new(
            Color::White,
            vec![],
            vec![],
            Point::new(1, 1),
            PieceId::new(1, &Color::White),
        ));
        assert_eq!(heat_map.positional_value(&king, &Point::new(7, 1)), -4,);
    }

    #[test]
    fn value_of_black_king_king_side_position() {
        let heat_map = ClassicHeatMap::init();
        let king = Piece::King(King::new(
            Color::Black,
            vec![],
            vec![],
            Point::new(1, 1),
            PieceId::new(1, &Color::Black),
        ));
        assert_eq!(heat_map.positional_value(&king, &Point::new(7, 8)), -4,);
    }

    #[test]
    fn value_of_white_king_queen_side_position() {
        let heat_map = ClassicHeatMap::init();
        let king = Piece::King(King::new(
            Color::White,
            vec![],
            vec![],
            Point::new(1, 1),
            PieceId::new(1, &Color::White),
        ));
        assert_eq!(heat_map.positional_value(&king, &Point::new(3, 1)), -10,);
    }

    #[test]
    fn value_of_black_king_queen_side_position() {
        let heat_map = ClassicHeatMap::init();
        let king = Piece::King(King::new(
            Color::Black,
            vec![],
            vec![],
            Point::new(1, 1),
            PieceId::new(1, &Color::Black),
        ));
        assert_eq!(heat_map.positional_value(&king, &Point::new(3, 8)), -10,);
    }

    #[test]
    fn value_of_white_pawn_e4() {
        let heat_map = ClassicHeatMap::init();
        let pawn = Piece::Pawn(Pawn::new(
            Color::White,
            vec![],
            vec![],
            Point::new(5, 4),
            PieceId::new(1, &Color::White),
        ));
        assert_eq!(heat_map.positional_value(&pawn, &Point::new(5, 4)), 109,);
    }

    #[test]
    fn value_of_black_pawn_e5() {
        let heat_map = ClassicHeatMap::init();
        let pawn = Piece::Pawn(Pawn::new(
            Color::Black,
            vec![],
            vec![],
            Point::new(5, 5),
            PieceId::new(1, &Color::Black),
        ));
        assert_eq!(heat_map.positional_value(&pawn, &Point::new(5, 5)), 109,);
    }
}
