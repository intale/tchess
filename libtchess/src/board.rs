use crate::board_config::BoardConfig;
use crate::board_map::BoardMap;
use crate::board_square::BoardSquare;
use crate::board_stats::BoardStats;
use crate::board_summary::BoardSummary;
use crate::buff::Buff;
use crate::buffs_map::BuffsMap;
use crate::color::Color;
use crate::colored_property::ColoredProperty;
use crate::debuff::Debuff;
use crate::debuffs_map::DebuffsMap;
use crate::dimension::Dimension;
use crate::heat_map::HeatMap;
use crate::ids_generator::IdsGenerator;
use crate::move_score::MoveScore;
use crate::moves_map::MovesMap;
use crate::piece::Piece;
use crate::piece_id::PieceId;
use crate::piece_move::PieceMove;
use crate::point::Point;
use crate::squares_map::SquaresMap;
use crate::strategy_point::StrategyPoint;
use crate::strategy_points::StrategyPoints;
use crate::utils::pretty_print::PrettyPrint;
use crate::vector::Vector;
use crate::vector::line_vector::LineVector;
use crate::vector_points::VectorPoints;
use crate::x_ray_pieces::XRayPieces;
use im_rc::{HashMap, HashSet, OrdMap};
use rustc_hash::FxBuildHasher;

// Invert colors of chess symbols so they look more meaningful in the terminal window with black
// background. Debugging purpose only.
pub const INVERT_COLORS: bool = true;

#[derive(Clone)]
pub struct Board<HT: HeatMap, SQ: SquaresMap> {
    board_map: BoardMap,
    strategy_points: ColoredProperty<StrategyPoints>,
    x_ray_pieces: ColoredProperty<XRayPieces>,
    pawns_with_en_passant: ColoredProperty<HashSet<PieceId, FxBuildHasher>>,
    moves_map: ColoredProperty<MovesMap>,
    general_constraints: ColoredProperty<Option<MovesMap>>,
    ids_generator: ColoredProperty<IdsGenerator>,
    debuffs_map: ColoredProperty<DebuffsMap>,
    buffs_map: ColoredProperty<BuffsMap>,
    current_turn: Color,
    // Determines board's point of view. Debugging purpose only.
    pov: Color,
    config: BoardConfig<HT, SQ>,
    board_summary: BoardSummary,
}

impl<HT: HeatMap, SQ: SquaresMap> Board<HT, SQ> {
    pub fn active_pieces(&self, color: &Color) -> &HashMap<PieceId, Piece, FxBuildHasher> {
        self.board_map.active_pieces(color)
    }

    pub fn config(&self) -> &BoardConfig<HT, SQ> {
        &self.config
    }

    pub fn dimension(&self) -> &Dimension {
        self.config.dimension()
    }

    pub fn strategy_points(&self, color: &Color) -> &StrategyPoints {
        &self.strategy_points[color]
    }

    pub fn find_piece_by_id(&self, piece_id: &PieceId) -> Option<&Piece> {
        self.board_map.maybe_find_piece_by_id(piece_id)
    }

    pub fn x_ray_pieces(&self, color: &Color) -> &XRayPieces {
        &self.x_ray_pieces[color]
    }

    fn evaluate_move(
        config: &BoardConfig<HT, SQ>,
        board_map: &BoardMap,
        piece_move: &PieceMove,
        piece: &Piece,
    ) -> MoveScore {
        let destination_point = piece_move.destination();
        let new_position_score;
        let current_position_score;
        match piece_move {
            PieceMove::Promote(_, promote_piece) => {
                let theoretically_promoted_piece = Piece::init_piece_by_name(
                    &promote_piece.name(),
                    *piece.color(),
                    *destination_point,
                    PieceId::new(1, piece.color()),
                );
                // Calculate promote move value based on the promoted piece instead of a value of
                // a pawn to promote
                new_position_score = config
                    .heat_map()
                    .positional_value(&theoretically_promoted_piece, &destination_point);
                current_position_score = config
                    .heat_map()
                    .positional_value(piece, &piece.current_position());
            }
            PieceMove::Castle(castle_points) => {
                let rook = board_map
                    .piece_at(castle_points.initial_rook_point())
                    .unwrap();
                let old_king_score = config
                    .heat_map()
                    .positional_value(piece, castle_points.initial_king_point());
                let old_rook_score = config
                    .heat_map()
                    .positional_value(rook, castle_points.initial_rook_point());
                let new_king_score = config
                    .heat_map()
                    .positional_value(piece, castle_points.king_point());
                let new_rook_score = config
                    .heat_map()
                    .positional_value(rook, castle_points.rook_point());
                new_position_score = new_king_score + new_rook_score;
                current_position_score = old_king_score + old_rook_score;
            }
            _ => {
                new_position_score = config
                    .heat_map()
                    .positional_value(piece, &destination_point);
                current_position_score = config
                    .heat_map()
                    .positional_value(piece, &piece.current_position());
            }
        };

        MoveScore::WeightDelta(new_position_score - current_position_score)
    }

    pub fn king(&self, color: &Color) -> Option<&Piece> {
        self.board_map.king(color)
    }

    pub fn pawns_with_en_passant(&self, color: &Color) -> &HashSet<PieceId, FxBuildHasher> {
        &self.pawns_with_en_passant[color]
    }

    pub fn empty(config: BoardConfig<HT, SQ>) -> Self {
        let mut board = Self {
            board_map: BoardMap::empty(),
            strategy_points: ColoredProperty([StrategyPoints::empty(), StrategyPoints::empty()]),
            x_ray_pieces: ColoredProperty([XRayPieces::empty(), XRayPieces::empty()]),
            pawns_with_en_passant: ColoredProperty([HashSet::default(), HashSet::default()]),
            moves_map: ColoredProperty([MovesMap::empty(), MovesMap::empty()]),
            general_constraints: ColoredProperty([None, None]),
            ids_generator: ColoredProperty([IdsGenerator::init(), IdsGenerator::init()]),
            debuffs_map: ColoredProperty([DebuffsMap::empty(), DebuffsMap::empty()]),
            buffs_map: ColoredProperty([BuffsMap::empty(), BuffsMap::empty()]),
            current_turn: Color::White,
            pov: Color::White,
            config,
            board_summary: BoardSummary::new(),
        };
        for y in board.dimension().get_rows_range() {
            for x in board.dimension().get_columns_range() {
                board.init_square(Point::new(x, y));
            }
        }
        board
    }

    fn init_square(&mut self, point: Point) {
        if let Some(square) = self.config.squares_map().square(&point) {
            self.board_map.add_square(point, square);
        }
    }

    // Pins points and castle points are not taken into account here. They require more complex
    // computations than a simple look up.
    fn pieces_to_recalculate(
        &mut self,
        point: &Point,
        caused_by_color: &Color,
    ) -> HashSet<PieceId> {
        let mut pieces: HashSet<PieceId> = HashSet::default();
        let inverse_color = caused_by_color.inverse();

        if let Some(piece_ids) =
            self.strategy_points[&inverse_color].get_pieces(&StrategyPoint::Attack(*point))
        {
            for piece_id in piece_ids {
                pieces.insert(*piece_id);
            }
        }
        if let Some(piece_ids) =
            self.strategy_points[&inverse_color].get_pieces(&StrategyPoint::Move(*point))
        {
            for piece_id in piece_ids {
                pieces.insert(*piece_id);
            }
        }

        if let Some(piece_ids) =
            self.strategy_points[caused_by_color].get_pieces(&StrategyPoint::Defense(*point))
        {
            for piece_id in piece_ids {
                pieces.insert(*piece_id);
            }
        }

        // BlockedMove source cause can originate from both sides. Thus, calculate it for both
        // colors
        for color in [Color::White, Color::Black].iter() {
            if let Some(piece_ids) =
                self.strategy_points[color].get_pieces(&StrategyPoint::BlockedMove(*point))
            {
                for piece_id in piece_ids {
                    pieces.insert(*piece_id);
                }
            }
        }

        for piece_id in self.pawns_with_en_passant[&inverse_color].iter() {
            pieces.insert(*piece_id);
        }
        pieces
    }

    fn calculate_strategy_points(
        piece: &Piece,
        board_map: &BoardMap,
        cbuffs_map: &ColoredProperty<BuffsMap>,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &mut ColoredProperty<StrategyPoints>,
    ) {
        let strategy_points = &mut cstrategy_points[piece.color()];
        strategy_points.remove_piece(piece.id());
        let dimension = config.dimension();
        let add_strategy_point = |strategy_point| {
            strategy_points.add_association(strategy_point, piece.id());
        };

        match piece {
            Piece::Pawn(p) => {
                p.calculate_strategy_points(board_map, cbuffs_map, dimension, add_strategy_point)
            }
            Piece::Rook(p) => p.calculate_strategy_points(board_map, dimension, add_strategy_point),
            Piece::Knight(p) => {
                p.calculate_strategy_points(board_map, dimension, add_strategy_point)
            }
            Piece::Bishop(p) => {
                p.calculate_strategy_points(board_map, dimension, add_strategy_point)
            }
            Piece::Queen(p) => {
                p.calculate_strategy_points(board_map, dimension, add_strategy_point)
            }
            Piece::King(p) => p.calculate_strategy_points(board_map, dimension, add_strategy_point),
            Piece::UnknownPiece(_) => {
                panic!("Can't calculate strategy points for an unknown piece!")
            }
        }
    }

    // Calculate whether the given piece is facing the opposite king
    fn calculate_x_ray(
        piece: &Piece,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &mut ColoredProperty<MovesMap>,
        cbuffs_map: &mut ColoredProperty<BuffsMap>,
        cdebuffs_map: &mut ColoredProperty<DebuffsMap>,
        cx_ray_pieces: &mut ColoredProperty<XRayPieces>,
    ) {
        let opposite_king = match board_map.king(&piece.color().inverse()) {
            Some(piece) => piece,
            None => return,
        };

        if let Some(direction) = Self::x_ray_direction(piece, &opposite_king) {
            // Get current piece which occupies this direction
            let current_piece = cx_ray_pieces[piece.color()].piece_by_direction(&direction);
            let current_piece = match current_piece {
                Some(piece_id) => Some(board_map.find_piece_by_id(piece_id)),
                None => None,
            };

            // Add or try to replace the direction by the given piece
            let new_piece =
                cx_ray_pieces[piece.color()].add_x_ray_vector(&direction, current_piece, piece);

            // The direction was replaced by the new piece. We have to remove a pin of the old
            // piece if any.
            if let Some(current_piece) = current_piece
                && current_piece != new_piece
            {
                Self::remove_x_ray_piece(
                    current_piece.id(),
                    board_map,
                    config,
                    cstrategy_points,
                    cmoves_map,
                    cbuffs_map,
                    cdebuffs_map,
                    cx_ray_pieces,
                );
            }

            if new_piece == piece {
                // We are calculating pins for the piece which already occupies the given
                // direction or have just occupied it. In this case we need to recalculate moves
                // of possible pinned piece.
                Self::clear_existing_pins(
                    piece.id(),
                    board_map,
                    config,
                    cstrategy_points,
                    cmoves_map,
                    cbuffs_map,
                    cdebuffs_map,
                    cx_ray_pieces,
                );
                Self::add_pins(
                    &opposite_king,
                    piece,
                    board_map,
                    config,
                    cstrategy_points,
                    cmoves_map,
                    cbuffs_map,
                    cdebuffs_map,
                    cx_ray_pieces,
                );
            } else {
                // The given direction was not replaced by the given piece. This means that
                // there is already another piece, positioned closer to the opposite king than
                // the given one.
                Self::remove_x_ray_piece(
                    piece.id(),
                    board_map,
                    config,
                    cstrategy_points,
                    cmoves_map,
                    cbuffs_map,
                    cdebuffs_map,
                    cx_ray_pieces,
                );
            }
        } else {
            // The given piece does not have any connections to the opposite king - we have to
            // remove previously existing pin if any.
            Self::remove_x_ray_piece(
                piece.id(),
                board_map,
                config,
                cstrategy_points,
                cmoves_map,
                cbuffs_map,
                cdebuffs_map,
                cx_ray_pieces,
            );
        }
    }

    fn calculate_moves_for(
        piece: &Piece,
        board_map: &BoardMap,
        cbuffs_map: &ColoredProperty<BuffsMap>,
        cdebuffs_map: &ColoredProperty<DebuffsMap>,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &mut ColoredProperty<MovesMap>,
    ) {
        cmoves_map[piece.color()].remove_piece(piece.id());

        let add_move = |piece_move: PieceMove| {
            let move_score = Self::evaluate_move(config, board_map, &piece_move, piece);
            cmoves_map[piece.color()].add(piece.id(), piece_move, move_score);
        };
        match piece {
            Piece::King(k) => {
                k.calculate_moves(
                    board_map,
                    cbuffs_map,
                    cdebuffs_map,
                    config.dimension(),
                    config,
                    &cstrategy_points[&piece.color().inverse()],
                    add_move,
                );
            }
            Piece::Bishop(p) => {
                p.calculate_moves(board_map, cdebuffs_map, config.dimension(), add_move)
            }
            Piece::Knight(p) => {
                p.calculate_moves(board_map, cdebuffs_map, config.dimension(), add_move)
            }
            Piece::Pawn(p) => p.calculate_moves(
                board_map,
                cbuffs_map,
                cdebuffs_map,
                config.dimension(),
                add_move,
            ),
            Piece::Queen(p) => {
                p.calculate_moves(board_map, cdebuffs_map, config.dimension(), add_move)
            }
            Piece::Rook(p) => {
                p.calculate_moves(board_map, cdebuffs_map, config.dimension(), add_move)
            }
            Piece::UnknownPiece(_) => panic!("Unknown piece does not have any moves!"),
        }
    }

    fn calculate_general_constraints(
        king: &Piece,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &ColoredProperty<MovesMap>,
        cgeneral_constraints: &mut ColoredProperty<Option<MovesMap>>,
    ) {
        let mut constraints: Vec<PieceMove> = vec![];
        let pieces_caused_check = cstrategy_points[&king.color().inverse()]
            .get_pieces(&StrategyPoint::Attack(*king.current_position()))
            .unwrap();
        // When the king is in check by more than one piece, no legal moves can be made by
        // any piece except the king itself.
        if pieces_caused_check.len() == 1 {
            let piece_caused_check =
                board_map.find_piece_by_id(pieces_caused_check.iter().next().unwrap());
            // Add the position of the piece caused check.
            constraints.push(PieceMove::Point(*piece_caused_check.current_position()));

            let direction = Vector::calc_direction(
                &piece_caused_check.current_position(),
                &king.current_position(),
            )
            .unwrap();
            match direction {
                // Jump checks can't be blocked. The only way to remove it is to eliminate the
                // knight.
                Vector::Jump(_) => (),
                _ => {
                    let vector_points = VectorPoints::without_initial(
                        *piece_caused_check.current_position(),
                        *config.dimension(),
                        direction,
                    );
                    for point in vector_points {
                        // Exclude king's position
                        if &point == king.current_position() {
                            break;
                        }
                        constraints.push(PieceMove::Point(point));
                    }
                }
            }
        }

        for piece_move in constraints {
            Self::add_general_constraints(
                king.color(),
                piece_move,
                board_map,
                config,
                cmoves_map,
                cstrategy_points,
                cgeneral_constraints,
            );
        }
    }

    fn add_king_moves_to_general_constraints(
        king: &Piece,
        cmoves_map: &ColoredProperty<MovesMap>,
        cgeneral_constraints: &mut ColoredProperty<Option<MovesMap>>,
    ) {
        let general_constraints = cgeneral_constraints[king.color()]
            .as_mut()
            .expect("Logical error: constraints MovesMap must be initialized at this point!");
        if let Some(moves) = cmoves_map[king.color()].moves_of(king.id()) {
            for (piece_move, move_score) in moves.iter() {
                general_constraints.add(king.id(), *piece_move, *move_score);
            }
        }
    }

    pub fn has_no_moves(&self, color: &Color) -> bool {
        if let Some(general_constraints) = &self.general_constraints[color] {
            general_constraints.is_empty()
        } else {
            self.moves_map[color].is_empty()
        }
    }

    pub fn moves_of(
        &self,
        piece_id: &PieceId,
    ) -> Option<&HashMap<PieceMove, MoveScore, FxBuildHasher>> {
        if let Some(general_constraints) = &self.general_constraints[&piece_id.color()] {
            general_constraints.moves_of(piece_id)
        } else {
            self.moves_map[&piece_id.color()].moves_of(piece_id)
        }
    }

    pub fn score_to_moves(
        &self,
        color: &Color,
    ) -> &OrdMap<MoveScore, OrdMap<PieceId, im_rc::Vector<PieceMove>>> {
        if let Some(general_constraints) = &self.general_constraints[color] {
            general_constraints.score_to_moves()
        } else {
            self.moves_map[color].score_to_moves()
        }
    }

    fn add_general_constraints(
        color: &Color,
        piece_move: PieceMove,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cmoves_map: &ColoredProperty<MovesMap>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cgeneral_constraints: &mut ColoredProperty<Option<MovesMap>>,
    ) {
        let destination_to_cover = piece_move.destination();
        let constraints = cgeneral_constraints[color]
            .as_mut()
            .expect("Logical error: constraints MovesMap must be initialized at this point!");
        let mut calc_pieces = |piece_ids: &HashSet<PieceId, FxBuildHasher>| {
            for piece_id in piece_ids {
                let piece = board_map.find_piece_by_id(piece_id);
                if let Some(moves) = cmoves_map[color].moves_of(piece_id) {
                    match piece {
                        Piece::Pawn(_) => {
                            for (piece_move, _) in moves {
                                if piece_move.destination() != destination_to_cover {
                                    continue;
                                }
                                let move_score =
                                    Self::evaluate_move(&config, board_map, piece_move, piece);
                                constraints.add(piece_id, *piece_move, move_score);
                            }
                        }
                        _ => {
                            // Not taking into account other move variants as it is not possible to
                            // cover from check with castle move for example
                            if let Some((defensive_move, _)) =
                                moves.get_key_value(&PieceMove::Point(*destination_to_cover))
                            {
                                let move_score =
                                    Self::evaluate_move(&config, board_map, defensive_move, piece);
                                constraints.add(piece_id, *defensive_move, move_score);
                            };
                        }
                    }
                }
            }
        };
        if let Some(piece_ids) =
            cstrategy_points[color].get_pieces(&StrategyPoint::Attack(*destination_to_cover))
        {
            calc_pieces(piece_ids)
        }
        if let Some(piece_ids) =
            cstrategy_points[color].get_pieces(&StrategyPoint::Move(*destination_to_cover))
        {
            calc_pieces(piece_ids)
        }
    }

    fn calc_en_passant(
        position: &Point,
        caused_by_color: &Color,
        board_map: &BoardMap,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cbuffs_map: &mut ColoredProperty<BuffsMap>,
        board_summary: &mut BoardSummary,
        cpawns_with_en_passant: &mut ColoredProperty<HashSet<PieceId, FxBuildHasher>>,
    ) {
        let en_passant_position = match caused_by_color {
            Color::White => Point::new(*position.x().value(), *position.y().value() - 1),
            Color::Black => Point::new(*position.x().value(), *position.y().value() + 1),
        };

        let mut pawns: Vec<&PieceId> = vec![];
        if let Some(piece_ids) = cstrategy_points[&caused_by_color.inverse()]
            .get_pieces(&StrategyPoint::Attack(en_passant_position))
        {
            for piece_id in piece_ids {
                let piece = board_map.find_piece_by_id(piece_id);
                match piece {
                    Piece::Pawn(_) => {
                        cbuffs_map[&piece_id.color()]
                            .add(piece_id, Buff::EnPassant(en_passant_position, *position));
                        pawns.push(piece.id());
                        board_summary.update_piece_en_passant(piece.id(), true);
                    }
                    _ => (),
                }
            }
        }
        for pawn_id in pawns {
            cpawns_with_en_passant[&caused_by_color.inverse()].insert(*pawn_id);
        }
    }

    fn clear_en_passant(&mut self) {
        for pawn_id in self.pawns_with_en_passant[&Color::White].iter() {
            self.buffs_map[&pawn_id.color()].remove_en_passant(pawn_id);
            self.board_summary.update_piece_en_passant(pawn_id, false);
        }
        self.pawns_with_en_passant[&Color::White].clear();
        for pawn_id in self.pawns_with_en_passant[&Color::Black].iter() {
            self.buffs_map[&pawn_id.color()].remove_en_passant(pawn_id);
            self.board_summary.update_piece_en_passant(pawn_id, false);
        }
        self.pawns_with_en_passant[&Color::Black].clear();
    }

    pub fn pass_turn(&mut self, color: &Color) {
        self.current_turn = *color;
    }

    pub fn piece_at(&self, point: &Point) -> Option<&Piece> {
        self.board_map.piece_at(point)
    }

    pub fn piece_id_at(&self, point: &Point) -> Option<&PieceId> {
        self.board_map.piece_id_at(point)
    }

    pub fn board_square(&self, point: &Point) -> &BoardSquare {
        self.board_map.board_square(point)
    }

    fn add_pins(
        pin_to: &Piece,
        pinned_by: &Piece,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &mut ColoredProperty<MovesMap>,
        cbuffs_map: &mut ColoredProperty<BuffsMap>,
        cdebuffs_map: &mut ColoredProperty<DebuffsMap>,
        cx_ray_pieces: &mut ColoredProperty<XRayPieces>,
    ) -> bool {
        let points = cstrategy_points[pinned_by.color()].get_points(pinned_by.id());
        if let Some(points) = points {
            if points.contains(&StrategyPoint::Attack(*pin_to.current_position())) {
                // No need to calculate pinned pieces, because pin_to piece is directly attacked by
                // the given pinned_by piece
                return false;
            }
        }

        let enemy_color = pinned_by.color();
        let x_ray_direction = Self::x_ray_direction(pinned_by, pin_to).unwrap_or_else(|| {
            panic!(
                "Logical mistake: {:?} must have a connection to {:?} at this point!",
                pinned_by, pin_to
            )
        });

        let mut current_piece_on_the_way: Option<&Piece> = None;
        let vector_points = VectorPoints::without_initial(
            *pinned_by.current_position(),
            *config.dimension(),
            x_ray_direction,
        );
        for point in vector_points {
            if let Some(piece) = board_map.piece_at(&point) {
                // Enemy piece meets his ally
                if piece.color() == enemy_color {
                    break;
                }
                match current_piece_on_the_way {
                    Some(p) => {
                        if piece == pin_to {
                            // Current piece is pin_to. We have a bound!
                            cdebuffs_map[p.color()].add(p.id(), Debuff::Pin(x_ray_direction));
                            Self::calculate_moves_for(
                                p,
                                board_map,
                                cbuffs_map,
                                cdebuffs_map,
                                config,
                                cstrategy_points,
                                cmoves_map,
                            );
                            cx_ray_pieces[pinned_by.color()].add_pin(&p, pinned_by);
                            return true;
                        }
                        break;
                    }
                    None => current_piece_on_the_way = Some(piece),
                }
            }
        }
        false
    }

    pub fn add_piece(
        &mut self,
        name: &str,
        color: Color,
        buffs: Vec<Buff>,
        debuffs: Vec<Debuff>,
        position: Point,
    ) -> PieceId {
        if !self.board_square(&position).is_empty_square() {
            panic!(
                "Can't add {} piece. Position {:?} is not empty!",
                name, position
            )
        }
        self.add_piece_unchecked(name, color, buffs, debuffs, position, true)
    }

    fn add_piece_unchecked(
        &mut self,
        name: &str,
        color: Color,
        buffs: Vec<Buff>,
        debuffs: Vec<Debuff>,
        position: Point,
        calculate_mechanics: bool,
    ) -> PieceId {
        let id = self.ids_generator[&color].next_val(&color);
        let piece = Piece::init_piece_by_name(name, color, position, id);
        let buffs_map = &self.buffs_map[piece.color()];
        self.board_summary.add_piece(
            &piece,
            buffs_map.has_castle(piece.id()),
            buffs_map.has_en_passant(piece.id()),
        );
        self.board_map.add_piece(piece, position);
        self.buffs_map[&id.color()].add_from_vec(&id, buffs);
        self.debuffs_map[&id.color()].add_from_vec(&id, debuffs);
        if calculate_mechanics {
            self.recalculate_connected_positions(&position, &color, true);
            self.recalculate_connected_positions(&position, &color.inverse(), false);
            self.recalculate_king_mechanics(&color);
            self.recalculate_king_mechanics(&color.inverse());
        }
        id
    }

    pub fn move_piece(&mut self, piece_id: &PieceId, piece_move: &PieceMove) -> Option<MoveScore> {
        if self.current_turn != piece_id.color() {
            return None;
        }

        if let Some(&move_score) =
            self.moves_map[&piece_id.color()].move_score(piece_id, piece_move)
        {
            self.move_piece_unchecked(piece_id, piece_move, true);
            self.pass_turn(&piece_id.color().inverse());
            self.board_summary.next_turn();
            Some(move_score)
        } else {
            None
        }
    }

    fn move_piece_unchecked(
        &mut self,
        piece_id: &PieceId,
        piece_move: &PieceMove,
        calculate_king: bool,
    ) {
        self.clear_en_passant();
        {
            let piece = self.board_map.find_piece_by_id(piece_id);
            match piece {
                Piece::King(_) | Piece::Rook(_) => {
                    let buffs_map_mut = &mut self.buffs_map[piece.color()];
                    if buffs_map_mut.has_castle(piece.id()) {
                        buffs_map_mut.remove_castle(piece.id());
                        self.board_summary.update_piece_castle(piece.id(), false);
                    }
                }
                Piece::Pawn(_) => {
                    self.board_summary.pawn_moved();
                    self.buffs_map[piece.color()].remove_additional_point(piece.id());
                    match piece_move {
                        PieceMove::LongMove(new_position) => {
                            Self::calc_en_passant(
                                new_position,
                                &piece.color(),
                                &self.board_map,
                                &self.strategy_points,
                                &mut self.buffs_map,
                                &mut self.board_summary,
                                &mut self.pawns_with_en_passant,
                            );
                        }
                        _ => (),
                    }
                }
                _ => (),
            };
        }
        match piece_move {
            PieceMove::Point(new_position) | PieceMove::LongMove(new_position) => {
                let enemy_piece_id = self.board_map.piece_id_at(new_position).copied();
                self.perform_move(piece_id, &new_position, enemy_piece_id);
            }
            PieceMove::EnPassant(new_position, enemy_position) => {
                let captured_pawn_id = self.board_map.piece_id_at(enemy_position).unwrap_or_else(|| {
                    panic!(
                        "Logical mistake: enemy pawn should be present at {:?} during en passant",
                        enemy_position
                    );
                });
                self.perform_move(piece_id, &new_position, Some(*captured_pawn_id));
            }
            PieceMove::Castle(castle_points) => {
                let &king_id = self.board_map.piece_id_at(
                    castle_points.initial_king_point()
                ).expect(
                    format!(
                        "Logical mistake: expect the king to be present at {} position during the castle move.",
                        castle_points.initial_king_point()
                    ).as_str()
                );
                let &rook_id = self.board_map.piece_id_at(
                    castle_points.initial_rook_point()
                ).expect(
                    format!(
                        "Logical mistake: expect the rook to be present at {} position during the castle move.",
                        castle_points.initial_rook_point()
                    ).as_str()
                );
                // In chess960 king or rook may keep staying on their places during the castling
                if castle_points.initial_rook_point() != castle_points.rook_point() {
                    self.move_piece_unchecked(
                        &rook_id,
                        &PieceMove::Point(*castle_points.rook_point()),
                        false,
                    );
                }
                if castle_points.initial_king_point() != castle_points.king_point() {
                    self.move_piece_unchecked(
                        &king_id,
                        &PieceMove::Point(*castle_points.king_point()),
                        false,
                    );
                }
            }
            PieceMove::Promote(point, promote_piece) => {
                let pawn = self.remove_piece(piece_id);
                let promoted_piece_id = self.add_piece_unchecked(
                    &promote_piece.name(),
                    piece_id.color(),
                    vec![],
                    vec![],
                    *pawn.current_position(),
                    false,
                );
                self.board_summary.piece_promoted();
                self.move_piece_unchecked(&promoted_piece_id, &PieceMove::Point(*point), false);
            }
        }
        if calculate_king {
            self.recalculate_king_mechanics(&piece_id.color());
            self.recalculate_king_mechanics(&piece_id.color().inverse());
        }
    }

    fn perform_move(
        &mut self,
        piece_id_to_move: &PieceId,
        new_position: &Point,
        enemy_piece_id: Option<PieceId>,
    ) {
        if let Some(piece_id) = enemy_piece_id {
            let piece = self.remove_piece(&piece_id);
            self.board_summary.piece_captured(piece);
        }
        let old_position = self
            .board_map
            .change_piece_position(new_position, piece_id_to_move);
        self.board_summary
            .update_piece_position(piece_id_to_move, new_position);
        self.recalculate_connected_positions(&old_position, &piece_id_to_move.color(), false);
        self.recalculate_connected_positions(&new_position, &piece_id_to_move.color(), true);
        self.recalculate_connected_positions(
            &new_position,
            &piece_id_to_move.color().inverse(),
            false,
        );
    }

    // Not every piece removal from the board is capturing. For example, when promoting a pawn - we
    // need to remove it from the board without any other potential actions
    fn remove_piece(&mut self, piece_id: &PieceId) -> Piece {
        self.strategy_points[&piece_id.color()].remove_piece(piece_id);
        self.moves_map[&piece_id.color()].remove_piece(piece_id);
        if let Some(debuff) = self.debuffs_map[&piece_id.color()].pin(piece_id) {
            let vector = match debuff {
                Debuff::Pin(v) => v,
                _ => panic!("Logical error. Expected pin debuff, got: {:?}", debuff),
            };
            self.x_ray_pieces[&piece_id.color().inverse()].remove_pinned_piece(vector);
        }
        let piece = self.board_map.remove_piece(piece_id);
        Self::remove_x_ray_piece(
            piece_id,
            &self.board_map,
            &self.config,
            &self.strategy_points,
            &mut self.moves_map,
            &mut self.buffs_map,
            &mut self.debuffs_map,
            &mut self.x_ray_pieces,
        );
        self.board_summary.remove_piece(piece_id);
        piece
    }

    fn recalculate_connected_positions(
        &mut self,
        point: &Point,
        caused_by_color: &Color,
        include_piece_at_position: bool,
    ) {
        let mut pieces_to_recalculate = self.pieces_to_recalculate(point, caused_by_color);

        if include_piece_at_position && let Some(piece_id) = self.board_map.piece_id_at(point) {
            pieces_to_recalculate.insert(*piece_id);
        }

        let pieces_to_recalculate = pieces_to_recalculate
            .iter()
            .map(|piece_id| self.board_map.find_piece_by_id(piece_id))
            .collect::<Vec<_>>();

        for piece in pieces_to_recalculate.iter() {
            Self::calculate_strategy_points(
                piece,
                &self.board_map,
                &self.buffs_map,
                &self.config,
                &mut self.strategy_points,
            );
        }

        for piece in pieces_to_recalculate.iter() {
            match piece {
                Piece::King(_) => {
                    // King moves are calculated separately
                }
                _ => {
                    Self::calculate_moves_for(
                        piece,
                        &self.board_map,
                        &self.buffs_map,
                        &self.debuffs_map,
                        &self.config,
                        &self.strategy_points,
                        &mut self.moves_map,
                    );
                }
            }
        }

        // This covers the case when an ally piece moves from the x-ray direction, thus causing
        // another ally piece, standing in front of it, be pinned. Example, white bishop(or black -
        // if you are using light theme in your IDE/editor) causes white queen be pinned by moving
        // to a2:
        // 3 ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓
        // 2 ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░
        // 1 ░♚░ ▓♝▓ ░♛░ ▓▓▓ ░░░ ▓▓▓ ░♖░ ▓♔▓
        //    a   b   c   d   e   f   g   h
        // 3 ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓
        // 2 ▓♝▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░ ▓▓▓ ░░░
        // 1 ░♚░ ▓▓▓ ░♛░ ▓▓▓ ░░░ ▓▓▓ ░♖░ ▓♔▓
        //    a   b   c   d   e   f   g   h
        for piece_id in self.x_ray_pieces[&caused_by_color.inverse()].pieces_owned() {
            let piece = self.board_map.find_piece_by_id(&piece_id);
            if let Some(x_ray_direction) = self
                .x_ray_pieces(&caused_by_color.inverse())
                .direction(&piece_id)
            {
                if let Some(direction) = Vector::calc_direction(&piece.current_position(), point) {
                    if x_ray_direction == &direction {
                        Self::calculate_x_ray(
                            &piece,
                            &self.board_map,
                            &self.config,
                            &self.strategy_points,
                            &mut self.moves_map,
                            &mut self.buffs_map,
                            &mut self.debuffs_map,
                            &mut self.x_ray_pieces,
                        );
                    }
                }
            }
        }

        // Re-calculate x-rays of x-ray pieces, explicitly affected by the current position
        for piece in pieces_to_recalculate {
            match &*piece {
                Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => {
                    Self::calculate_x_ray(
                        &piece,
                        &self.board_map,
                        &self.config,
                        &self.strategy_points,
                        &mut self.moves_map,
                        &mut self.buffs_map,
                        &mut self.debuffs_map,
                        &mut self.x_ray_pieces,
                    );
                }
                _ => (),
            }
        }
    }

    fn recalculate_king_mechanics(&mut self, color: &Color) {
        if let Some(king) = self.board_map.king(color) {
            self.debuffs_map[king.color()].remove_check(king.id());
            self.general_constraints[king.color()] = None;
            if self.strategy_points[&king.color().inverse()]
                .is_under_attack(&king.current_position())
            {
                self.debuffs_map[king.color()].add(king.id(), Debuff::Check);
                self.general_constraints[king.color()] = Some(MovesMap::empty());
            }
            Self::calculate_moves_for(
                king,
                &self.board_map,
                &self.buffs_map,
                &self.debuffs_map,
                &self.config,
                &self.strategy_points,
                &mut self.moves_map,
            );
            if self.debuffs_map[king.color()].has_check(king.id()) {
                Self::calculate_general_constraints(
                    king,
                    &self.board_map,
                    &self.config,
                    &self.strategy_points,
                    &self.moves_map,
                    &mut self.general_constraints,
                );
                // Copy king moves into general constraints
                Self::add_king_moves_to_general_constraints(
                    &king,
                    &self.moves_map,
                    &mut self.general_constraints,
                );
            }
        }
    }

    pub fn set_pov(&mut self, color: Color) {
        self.pov = color;
    }

    pub fn current_turn(&self) -> &Color {
        &self.current_turn
    }

    pub fn stats(&'_ self) -> BoardStats<'_> {
        self.board_summary.stats()
    }

    pub fn buffs(&self, color: &Color) -> &BuffsMap {
        &self.buffs_map[color]
    }

    pub fn debuffs(&self, color: &Color) -> &DebuffsMap {
        &self.debuffs_map[color]
    }

    fn x_ray_direction(piece: &Piece, opposite_king: &Piece) -> Option<Vector> {
        match piece {
            Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => {
                piece.attack_vector(&piece.current_position(), &opposite_king.current_position())
            }
            _ => None,
        }
    }

    fn remove_x_ray_piece(
        piece_id: &PieceId,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &mut ColoredProperty<MovesMap>,
        cbuffs_map: &mut ColoredProperty<BuffsMap>,
        cdebuffs_map: &mut ColoredProperty<DebuffsMap>,
        cx_ray_pieces: &mut ColoredProperty<XRayPieces>,
    ) {
        Self::clear_existing_pins(
            piece_id,
            board_map,
            config,
            cstrategy_points,
            cmoves_map,
            cbuffs_map,
            cdebuffs_map,
            cx_ray_pieces,
        );
        cx_ray_pieces[&piece_id.color()].remove_piece(piece_id);
    }

    fn clear_existing_pins(
        piece_id: &PieceId,
        board_map: &BoardMap,
        config: &BoardConfig<HT, SQ>,
        cstrategy_points: &ColoredProperty<StrategyPoints>,
        cmoves_map: &mut ColoredProperty<MovesMap>,
        cbuffs_map: &mut ColoredProperty<BuffsMap>,
        cdebuffs_map: &mut ColoredProperty<DebuffsMap>,
        cx_ray_pieces: &ColoredProperty<XRayPieces>,
    ) {
        if let Some(pinned_id) = cx_ray_pieces[&piece_id.color()].pinned_piece(piece_id) {
            let pinned = board_map.find_piece_by_id(pinned_id);
            cdebuffs_map[pinned.color()].remove_pin(pinned.id());
            Self::calculate_moves_for(
                pinned,
                board_map,
                cbuffs_map,
                cdebuffs_map,
                config,
                cstrategy_points,
                cmoves_map,
            );
        }
    }
}

impl<HT: HeatMap, SQ: SquaresMap> PrettyPrint for Board<HT, SQ> {
    fn pp(&self) -> String {
        let mut output = String::new();
        let mut buf: Vec<String> = vec![];

        let y_range: Vec<i16> = if self.pov == Color::White {
            (*self.dimension().min_point().y().value()..=*self.dimension().max_point().y().value())
                .rev()
                .collect()
        } else {
            (*self.dimension().min_point().y().value()..=*self.dimension().max_point().y().value())
                .collect()
        };
        let x_range: Vec<i16> = if self.pov == Color::White {
            (*self.dimension().min_point().x().value()..=*self.dimension().max_point().x().value())
                .collect()
        } else {
            (*self.dimension().min_point().x().value()..=*self.dimension().max_point().x().value())
                .rev()
                .collect()
        };

        for y in y_range {
            for x in x_range.clone() {
                let point = Point::new(x, y);
                let square = self.board_square(&point);
                if (self.pov == Color::White && point.x() == self.dimension().min_point().x())
                    || (self.pov == Color::Black && point.x() == self.dimension().max_point().x())
                {
                    output.push_str(point.y().pp().as_str());
                    output.push_str(" ");
                }
                let square_str = match square.get_piece_id() {
                    Some(piece_id) => {
                        let piece = self.find_piece_by_id(piece_id).unwrap();
                        square
                            .pp()
                            .replace(&piece_id.to_string(), piece.pp().as_str())
                    }
                    None => square.pp(),
                };
                output.push_str(square_str.as_str());
                output.push(' ');
                if (self.pov == Color::White && point.x() == self.dimension().max_point().x())
                    || (self.pov == Color::Black && point.x() == self.dimension().min_point().x())
                {
                    output.push_str("\n");
                    buf.push(output.clone());
                    output = String::new();
                }
            }
        }
        output.push_str("  ");

        let vector_points = if self.pov == Color::White {
            VectorPoints::with_initial(
                Point::new(
                    *self.dimension().min_point().x().value(),
                    *self.dimension().max_point().y().value(),
                ),
                *self.dimension(),
                Vector::Line(LineVector::Right),
            )
        } else {
            VectorPoints::with_initial(
                Point::new(
                    *self.dimension().max_point().x().value(),
                    *self.dimension().max_point().y().value(),
                ),
                *self.dimension(),
                Vector::Line(LineVector::Left),
            )
        };
        for point in vector_points {
            output.push_str(" ");
            output.push_str(point.x().pp().as_str());
            output.push_str("  ");
        }
        buf.push(output);
        buf.join("")
    }
}

#[cfg(test)]
mod tests {}
