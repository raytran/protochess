use crate::position::Position;
use std::collections::HashMap;

use crate::position::piece_set::PieceSet;

use crate::move_generator::MoveGenerator;
use crate::types::PieceType;
use crate::position::piece::Piece;
use crate::MovementPattern;
use crate::types::bitboard::{Bitboard, from_index};

//Scores are in centipawns
const KING_SCORE:isize = 99999;
const QUEEN_SCORE:isize = 900;
const ROOK_SCORE:isize = 500;
const BISHOP_SCORE:isize = 300;
const KNIGHT_SCORE:isize = 300;
const PAWN_SCORE:isize = 100;
const CHECKMATED_SCORE:isize = -99999;
const MOVE_SCORE:isize = 5;
//Multiplier for the piece square table
const PST_MULTIPLIER:isize = 5;

/// Assigns a score to a given position
pub(crate) struct Evaluator {
    //Piece values for pieces,
    //Hard coded for builtin pieces,
    //generated dynamically based on the piece's movement pattern
    custom_piece_value_table: HashMap<PieceType, isize>,
    //Piece-square values for all pieces, done as a function of movement possibilities
    //Generated dynamically for all pieces
    piece_square_table: HashMap<PieceType, Vec<isize>>
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            custom_piece_value_table: HashMap::new(),
            piece_square_table:HashMap::new()
        }
    }
    //Retrieves the score for the player to move (position.whos_turn)
    pub fn evaluate(&mut self, position: &mut Position, movegen: &MoveGenerator) -> isize {
        let mut score = 0;
        let player_num = position.whos_turn;
        for ps in position.pieces.iter() {
            let side_multiplier = if ps.player_num == player_num { 1 } else {-1};
            let material_score = self.get_material_score_for_pieceset(position, ps);
            score += side_multiplier * self.get_positional_score(position, ps,movegen);

            score += side_multiplier * material_score;
        }

        score += self.get_mobility_score(position, movegen);
        score
    }

    fn get_material_score_for_pieceset(&mut self, position:&Position, piece_set:&PieceSet) -> isize{
        let mut material_score = 0;
        material_score += piece_set.king.bitboard.count_ones() as isize * KING_SCORE;
        material_score += piece_set.queen.bitboard.count_ones() as isize * QUEEN_SCORE;
        material_score += piece_set.rook.bitboard.count_ones() as isize * ROOK_SCORE;
        material_score += piece_set.knight.bitboard.count_ones() as isize * KNIGHT_SCORE;
        material_score += piece_set.bishop.bitboard.count_ones() as isize * BISHOP_SCORE;
        material_score += piece_set.pawn.bitboard.count_ones() as isize * PAWN_SCORE;

        for custom in &piece_set.custom {
            if self.custom_piece_value_table.contains_key(&custom.piece_type){
                let score = *self.custom_piece_value_table.get(&custom.piece_type).unwrap();
                material_score += custom.bitboard.count_ones() as isize * score;
            }else{
                let mp = position.get_movement_pattern(&custom.piece_type);
                let score = Evaluator::score_movement_pattern(mp);
                self.custom_piece_value_table.insert(custom.piece_type.to_owned(), score);
                material_score += custom.bitboard.count_ones() as isize * score;
            }
        }
        material_score
    }

    /// Returns a score value for a movement pattern
    fn score_movement_pattern(mp:&MovementPattern) -> isize{
        let mut score:isize = 0;
        //With all cardinal directions for attacking and translating, score = 960, which is
        //a little greater than a queen (900)
        if mp.attack_north {score += 60};
        if mp.translate_north {score += 60};
        if mp.attack_east {score += 60};
        if mp.translate_east {score += 60};
        if mp.attack_south {score += 60};
        if mp.translate_south {score += 60};
        if mp.attack_west {score += 60};
        if mp.translate_west {score += 60};
        if mp.attack_northeast {score += 60};
        if mp.translate_northeast {score += 60};
        if mp.attack_northwest {score += 60};
        if mp.translate_northwest {score += 60};
        if mp.attack_southeast {score += 60};
        if mp.translate_southeast {score += 60};
        if mp.attack_southwest {score += 60};
        if mp.translate_southwest {score += 60};

        score += (mp.translate_jump_deltas.len() * 18) as isize;
        score += (mp.attack_jump_deltas.len() * 18) as isize;
        for d in mp.translate_sliding_deltas.iter().chain(mp.attack_sliding_deltas.iter()){
            score += (d.len() * 18) as isize;
        }

        score
    }

    fn get_positional_score(&mut self, position: &Position, piece_set:&PieceSet, movegen: &MoveGenerator) -> isize {
        let mut score = 0;
        for p in piece_set.get_piece_refs() {
            //Don't mess with the king ( don't want the king to move to the center)
            if p.piece_type == PieceType::King || p.piece_type == PieceType::Pawn {
                continue;
            }

            //Add piecetype if not in table
            if !self.piece_square_table.contains_key(&p.piece_type) {
                //New entry
                let score_vec = Evaluator::get_positional_score_vec(position, p, movegen);

                self.piece_square_table.insert((&p.piece_type).to_owned(),
                                               score_vec);
            }
            //Calculate score for these pieces
            let mut bb_copy = (&p.bitboard).to_owned();
            let score_table = self.piece_square_table.get(&p.piece_type).unwrap();
            while !bb_copy.is_zero() {
                let index = bb_copy.lowest_one().unwrap();
                score += score_table[index] * PST_MULTIPLIER;
                bb_copy.set_bit(index, false);
            }
        }
        score
    }

    //Returns Vec of size 256, each with an integer representing # of moves possible at that
    // location
    fn get_positional_score_vec(position: &Position, piece:&Piece, movegen: &MoveGenerator) -> Vec<isize> {
        let mut return_vec = Vec::with_capacity(256);
        let mut total_entries = 0;
        let mut sum = 0;
        for i in 0..=255 {
            let (x, y) = from_index(i);
            let num_moves = movegen.get_num_moves_on_empty_board(i as u8, position, piece, &position.bounds) as isize;
            if position.xy_in_bounds(x, y) {
                total_entries += 1;
                sum += num_moves;
            }
            return_vec.push(num_moves);
        }

        let mean = sum / total_entries;
        //Center the dataset to give a bonus towards squares with higher moves
        //And a penalty for squares with fewer moves
        for i in 0..=255 {
            return_vec[i] -= mean;
        }

        return_vec
    }

    fn get_mobility_score(&self, position: &mut Position, movegen: &MoveGenerator) -> isize {
        let mut positional_score = 0;
        positional_score += movegen.count_legal_moves(position) as isize;
        if positional_score == 0 {
            return CHECKMATED_SCORE as isize;
        }
        positional_score * MOVE_SCORE
    }
}

#[cfg(test)]
mod eval_test {
    use crate::evaluator::Evaluator;
    use crate::position::Position;
    use crate::move_generator::MoveGenerator;

    #[test]
    fn test() {
        let mut eval = Evaluator::new();
        let movegen = MoveGenerator::new();
        let mut pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 1".parse().unwrap());
        println!("{}", eval.evaluate(&mut pos, &movegen));
    }
}
