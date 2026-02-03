use crate::piece_id::PieceId;
use crate::point::Point;
use crate::strategy_point::StrategyPoint;
use im_rc::{HashMap, HashSet};
use rustc_hash::FxBuildHasher;

#[derive(Debug, Clone)]
pub struct StrategyPoints {
    point_to_pieces: HashMap<StrategyPoint, HashSet<PieceId, FxBuildHasher>, FxBuildHasher>,
    piece_to_points: HashMap<PieceId, HashSet<StrategyPoint, FxBuildHasher>, FxBuildHasher>,
}

impl StrategyPoints {
    pub fn empty() -> Self {
        let point_to_pieces = HashMap::default();
        let piece_to_points = HashMap::default();
        Self {
            point_to_pieces,
            piece_to_points,
        }
    }

    fn get_pieces_mut(&mut self, point: &StrategyPoint) -> &mut HashSet<PieceId, FxBuildHasher> {
        if !self.point_to_pieces.contains_key(point) {
            self.point_to_pieces.insert(*point, HashSet::default());
        }
        self.point_to_pieces.get_mut(point).unwrap()
    }

    fn get_points_mut(&mut self, piece_id: &PieceId) -> &mut HashSet<StrategyPoint, FxBuildHasher> {
        if !self.piece_to_points.contains_key(piece_id) {
            self.piece_to_points.insert(*piece_id, HashSet::default());
        }
        self.piece_to_points.get_mut(piece_id).unwrap()
    }

    pub fn has_pieces(&self, point: &StrategyPoint) -> bool {
        if let Some(pieces) = self.get_pieces(point) {
            !pieces.is_empty()
        } else {
            false
        }
    }

    pub fn get_points(&self, piece_id: &PieceId) -> Option<&HashSet<StrategyPoint, FxBuildHasher>> {
        self.piece_to_points.get(piece_id)
    }

    pub fn get_pieces(&self, point: &StrategyPoint) -> Option<&HashSet<PieceId, FxBuildHasher>> {
        self.point_to_pieces.get(point)
    }

    pub fn add_association(&mut self, point: StrategyPoint, piece_id: &PieceId) {
        self.get_pieces_mut(&point).insert(*piece_id);
        self.get_points_mut(piece_id).insert(point);
    }

    pub fn remove_piece(&mut self, piece_id: &PieceId) {
        let points = self.piece_to_points.remove(piece_id);
        if let Some(points) = points {
            for point in points.iter() {
                if let Some(pieces) = self.point_to_pieces.get_mut(point) {
                    pieces.remove(piece_id);
                    if pieces.is_empty() {
                        self.point_to_pieces.remove(point);
                    }
                }
            }
        }
    }

    pub fn is_under_attack(&self, point: &Point) -> bool {
        self.has_pieces(&StrategyPoint::Attack(*point))
    }

    pub fn is_under_enemy_defense(&self, point: &Point) -> bool {
        self.has_pieces(&StrategyPoint::Defense(*point))
    }
}
