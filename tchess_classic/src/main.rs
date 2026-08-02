use std::fmt::{Debug, Formatter};
use im_rc::HashMap;
use rustc_hash::FxBuildHasher;
use libtchess::color::Color;
use libtchess::colored_property::ColoredProperty;
use libtchess::piece_id::PieceId;
use libtchess::piece_move::PieceMove;
use libtchess::utils::pretty_print::PrettyPrint;
use crate::classic_game::ClassicGame;
use crate::classic_heat_map::ClassicHeatMap;
use crate::classic_square_map::ClassicSquaresMap;
use crate::move_result::MoveResult;

mod classic_game;
mod classic_square_map;
mod classic_heat_map;
mod game_result;
mod move_result;
mod board_positions;

fn main() {
    let mut classic_game = ClassicGame::classic_board();
    // for (piece_id, piece) in classic_game.board().active_pieces(&Color::White) {
    //     println!("{}: {}", piece, piece_id)
    // };
    // for (piece_id, piece) in classic_game.board().active_pieces(&Color::Black) {
    //     println!("{}: {}", piece, piece_id)
    // };
    // return;
    // println!("{}", classic_game.board().pp());
    // let mut find_move = FindMove::new(classic_game.clone(), Color::White, 100);
    // let found_moves = find_move.next();
    //
    // println!("{:?}", found_moves);
    // for (piece_id, piece_move, _, _) in found_moves {
    //     classic_game.move_piece(&piece_id, &piece_move);
    //     println!("{}", classic_game.board().pp());
    // }
    let result;
    let mut moves_calculated = 0;
    let start_time = std::time::SystemTime::now();
    println!("Benchmark started");
    result = find_move(classic_game.clone(), 2, &mut moves_calculated);
    let time_took = start_time.elapsed().unwrap().as_millis();
    println!("Moves calculated: {}. Time took: {:?} ms. Speed: {}", moves_calculated, time_took, (moves_calculated as f32 / time_took as f32) * 1000f32);
    println!("{:?}", result);
    for (piece_id, piece_move) in result.1.iter() {
        classic_game.move_piece(piece_id, piece_move);
        println!("{}", classic_game.board().pp());
    }
}

struct SearchResult {}

// #[derive(Clone)]
// struct FindMove {
//     game: ClassicGame<ClassicHeatMap, ClassicSquaresMap>,
//     side: Color,
//     depth: usize,
//     moves_made: Vec<(PieceId, PieceMove)>,
//     current_score: MoveScore,
//     current_piece_id: PieceId,
//     current_move_idx: usize,
//     current_evaluation: i32,
// }
//
// impl FindMove {
//     fn new(game: ClassicGame<ClassicHeatMap, ClassicSquaresMap>, side: Color, depth: usize) -> Self {
//         let (move_score, piece_to_moves) = game.board().score_to_moves(game.board().current_turn()).iter().rev().next().unwrap();
//         let current_score = *move_score;
//         let current_piece_id = *piece_to_moves.iter().next().unwrap().0;
//         let current_evaluation = game.current_evaluation();
//         Self {
//             game,
//             side,
//             depth,
//             moves_made: vec![],
//             current_score,
//             current_piece_id,
//             current_move_idx: 0,
//             current_evaluation,
//         }
//     }
//
//     pub fn next(&mut self) -> Vec<(PieceId, PieceMove, i32, Option<FindMove>)> {
//         let mut moves_sequence: Vec<(PieceId, PieceMove, i32, Option<FindMove>)> = vec![];
//         self.find_sequence(&mut moves_sequence);
//         // println!("{:?}", moves_sequence);
//
//         moves_sequence
//     }
//
//     fn find_sequence(&mut self, moves_sequence: &mut Vec<(PieceId, PieceMove, i32, Option<FindMove>)>) {
//         // println!("{}", self.depth);
//         // println!("{:?}", moves_sequence);
//
//         let piece_to_moves = self.game.board().score_to_moves(self.game.board().current_turn()).get(&self.current_score).unwrap();
//         let piece_moves = piece_to_moves.get(&self.current_piece_id).unwrap();
//
//         if self.current_move_idx == piece_moves.len() {
//             if let Some((piece_id, _)) = piece_to_moves.get_next(&self.current_piece_id) {
//                 self.current_move_idx = 0;
//                 self.current_piece_id = *piece_id;
//             } else {
//                 if let Some((move_score, piece_to_moves)) = self.game.board().score_to_moves(self.game.board().current_turn()).get_prev(&self.current_score) {
//                     self.current_move_idx = 0;
//                     self.current_score = *move_score;
//                     self.current_piece_id = *piece_to_moves.iter().next().unwrap().0;
//                 };
//             }
//         }
//         let piece_move = piece_moves.get(self.current_move_idx);
//         if piece_move.is_none() {
//             return
//         }
//         let piece_move = piece_move.unwrap();
//         self.current_move_idx += 1;
//
//         // println!("current_score: {:?}; current_piece_id: {:?}, current_move_idx: {:?}, piece_moves.len(): {:?}", self.current_score, self.current_piece_id, self.current_move_idx, piece_moves.len());
//         // println!("asdasd");
//         let mut next_pos_game = self.game.clone();
//         next_pos_game.move_piece(&self.current_piece_id, piece_move);
//         moves_sequence.push((self.current_piece_id, *piece_move, next_pos_game.current_evaluation(), Some(self.clone())));
//
//         if next_pos_game.game_result().is_some() || self.depth == 0 {
//             return
//         }
//         let mut find_move = FindMove::new(next_pos_game, self.side, self.depth - 1);
//         find_move.find_sequence(moves_sequence);
//     }
//
//     // fn find_sequence2(&mut self, moves_sequence: &mut Vec<(PieceId, PieceMove, i32, Option<FindMove>)>) -> i32 {
//     //     let current_turn = self.game.board().current_turn();
//     //     let max_outcome = if current_turn == &Color::White {
//     //         i32::MAX
//     //     } else {
//     //         i32::MIN
//     //     };
//     //     for (move_score, piece_to_moves) in self.game.board().score_to_moves(current_turn).iter().rev() {
//     //         for (piece_id, piece_moves) in piece_to_moves.iter() {
//     //             for piece_move in piece_moves.iter() {
//     //                 let mut next_pos_game = self.game.clone();
//     //                 next_pos_game.move_piece(&piece_id, piece_move);
//     //                 // find_sequence2
//     //                 if next_pos_game.game_result().is_some() || self.depth == 0 {
//     //                     return
//     //                 }
//     //             }
//     //         }
//     //     };
//     // }
// }

// impl Debug for FindMove {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "FindMove<depth: {}, current_evaluation: {}>", self.depth, self.current_evaluation)
//
//     }
// }

fn find_move(game: ClassicGame<ClassicHeatMap, ClassicSquaresMap>, turn_depth: usize, moves_calculated: &mut usize) -> (ColoredProperty<i32>, Vec<(PieceId, PieceMove)>) {
    // println!("{}", turn_depth);
    // println!("{:?}", moves_made);
    let current_turn = game.board().current_turn();
    let prev_turn = &current_turn.inverse();
    let mut outcome = ColoredProperty([0, 0]);
    outcome[prev_turn] = game.current_evaluation();
    let mut max_outcome = if current_turn == &Color::White {
        i32::MIN
    } else {
        i32::MAX
    };

    let mut best_moves_made = vec![];

    for (_, piece_to_moves) in game.board().score_to_moves(current_turn).iter().rev() {
        for (piece_id, piece_moves) in piece_to_moves.iter() {
            for piece_move in piece_moves.iter() {
                let mut moves_made: Vec<(PieceId, PieceMove)> = vec![];
                let mut next_pos_game = game.clone();
                // println!("{}", next_pos_game.board().pp());
                // println!("{:?}", (piece_id, piece_move));
                let result = next_pos_game.move_piece(piece_id, piece_move);
                match result {
                    MoveResult::IllegalMove => panic!("Illegal move: {:?}", (piece_id, piece_move)),
                    _ => ()
                }
                *moves_calculated += 1;
                moves_made.push((*piece_id, *piece_move));

                if next_pos_game.game_result().is_some() || turn_depth == 0 {
                    let eval = game.current_evaluation();
                    // println!("current_turn: {:?}, eval: {}, best_moves_made: {:?}", current_turn, eval, best_moves_made);
                    if current_turn == &Color::White {
                        if eval >= max_outcome {
                            max_outcome = eval;
                            best_moves_made = moves_made;
                        }
                    } else {
                        if eval <= max_outcome {
                            max_outcome = eval;
                            best_moves_made = moves_made;
                        }
                    }
                } else {
                    let mut find_move = find_move(next_pos_game, turn_depth - 1, moves_calculated);
                    moves_made.append(&mut find_move.1);
                    // println!("a{:?}", moves_made);
                    if current_turn == &Color::White {
                        if find_move.0[current_turn] >= max_outcome {
                            max_outcome = find_move.0[current_turn];
                            best_moves_made = moves_made;
                        }
                    } else {
                        if find_move.0[current_turn] <= max_outcome {
                            max_outcome = find_move.0[current_turn];
                            best_moves_made = moves_made;
                        }
                    }
                }
            }
        }
    }

    outcome[current_turn] = max_outcome;
    (outcome, best_moves_made)
}
