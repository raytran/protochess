use crate::position::Position;
use std::collections::HashMap;

use crate::position::piece_set::PieceSet;

use crate::move_generator::MoveGenerator;
use crate::types::PieceType;
use crate::position::piece::Piece;
use crate::MovementPattern;
use crate::types::bitboard::from_index;
use crate::types::chess_move::Move;

// Scores are in centipawns
const KING_SCORE:isize = 9999;
const QUEEN_SCORE:isize = 900;
const ROOK_SCORE:isize = 500;
const BISHOP_SCORE:isize = 350;
const KNIGHT_SCORE:isize = 300;
const PAWN_SCORE:isize = 100;
const CHECKMATED_SCORE:isize = -99999;
const CASTLING_BONUS:isize = 400;
//Multiplier for the piece square table
const PST_MULTIPLIER:isize = 5;

/// Assigns a score to a given position
pub(crate) struct Evaluator {
    //Piece values for pieces,
    //Hard coded for builtin pieces,
    //generated dynamically based on the piece's movement pattern
    custom_piece_value_table: HashMap<PieceType, isize, ahash::RandomState>,
    //Piece-square values for all pieces, done as a function of movement possibilities
    //Generated dynamically for all pieces
    piece_square_table: HashMap<PieceType, Vec<isize>, ahash::RandomState>
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            custom_piece_value_table: HashMap::with_hasher(ahash::RandomState::new()),
            piece_square_table:HashMap::with_hasher(ahash::RandomState::new())
        }
    }
    /// Retrieves the score for the player to move (position.whos_turn)
    pub fn evaluate(&mut self, position: &mut Position, movegen: &MoveGenerator) -> isize {
        let mut score = 0;
        let player_num = position.whos_turn;
        //Material score
        let mut total_material_score = 0;
        for ps in position.pieces.iter() {
            let side_multiplier = if ps.player_num == player_num { 1 } else { -1 };
            let material_score = self.get_material_score_for_pieceset(position, ps);
            score += side_multiplier * material_score;
            total_material_score += material_score;
        }

        //Positional score
        let is_endgame = total_material_score < 2 * KING_SCORE + 2 * QUEEN_SCORE + 2 * ROOK_SCORE;
        for ps in position.pieces.iter(){
            let side_multiplier = if ps.player_num == player_num { 1 } else { -1 };
            let positional_score = self.get_positional_score(is_endgame, position, ps,movegen);
            //Castling bonus
            if position.properties.castling_rights.did_player_castle(ps.player_num) && !is_endgame {
                score += CASTLING_BONUS;
            }
            score += side_multiplier * positional_score;
        }

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
                let option_mp = position.get_movement_pattern(&custom.piece_type);
                let score = {
                    if let Some(mp) = option_mp {
                        Evaluator::score_movement_pattern(mp)
                    }else{
                        0
                    }
                };
                self.custom_piece_value_table.insert(custom.piece_type.to_owned(), score);
                material_score += custom.bitboard.count_ones() as isize * score;
            }
        }
        material_score
    }

    /// Scores a move on a position
    pub fn score_move(&mut self, depth:u8, history_moves: &[[u8;256];256], killer_moves: &[[Move;2];64], position: &mut Position, move_:&Move) -> usize {
        if !move_.get_is_capture() {
            return if move_ == &killer_moves[depth as usize][0] || move_ == &killer_moves[depth as usize][1] {
                9000
            } else {
                history_moves[move_.get_from() as usize][move_.get_to() as usize] as usize
            }
        }
        let attacker:PieceType = (&position.piece_at(move_.get_from() as usize).unwrap().1.piece_type).to_owned();
        let victim:PieceType = (&position.piece_at(move_.get_target() as usize).unwrap().1.piece_type).to_owned();

        let attack_score = self.get_material_score(attacker, position);
        let victim_score = self.get_material_score(victim, position);

        (KING_SCORE + (victim_score - attack_score)) as usize
    }

    /// Returns the current material score for a given Position
    pub fn get_material_score(&mut self, piece_type:PieceType, position:&Position) -> isize {
        match piece_type {
            PieceType::Pawn => { PAWN_SCORE }
            PieceType::Knight => { KNIGHT_SCORE }
            PieceType::Bishop => { BISHOP_SCORE }
            PieceType::Rook => { ROOK_SCORE }
            PieceType::Queen => { QUEEN_SCORE }
            PieceType::King => { KING_SCORE }
            PieceType::Custom(c) => {
                if self.custom_piece_value_table.contains_key(&piece_type){
                    *self.custom_piece_value_table.get(&piece_type).unwrap()
                }else{
                    let option_mp = position.get_movement_pattern(&piece_type);
                    let score = {
                        if let Some(mp) = option_mp {
                            Evaluator::score_movement_pattern(mp)
                        }else{
                            0
                        }
                    };
                    self.custom_piece_value_table.insert((&piece_type).to_owned(), score);
                    score
                }
            }
        }
    }

    /// Determines whether or not null move pruning can be performed for a Position
    pub fn can_do_null_move(&mut self, position:&Position) -> bool {
        self.get_material_score_for_pieceset(&position, &position.pieces[position.whos_turn as usize])
            > KING_SCORE + ROOK_SCORE
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

    fn get_positional_score(&mut self, is_endgame: bool, position: &Position, piece_set:&PieceSet, movegen: &MoveGenerator) -> isize {
        let mut score = 0;


        for p in piece_set.get_piece_refs() {

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
                //If it is the king then limit moves (encourage moving away from the center)
                if  p.piece_type == PieceType::King {
                    if !is_endgame {
                        score += -score_table[index] * PST_MULTIPLIER;
                    }else{
                        score += score_table[index] * PST_MULTIPLIER;
                    }
                }else{
                    score += score_table[index] * PST_MULTIPLIER;
                }

                bb_copy.set_bit(index, false);
            }
        }
        score
    }

    /// Returns Vec of size 256, each with an integer representing # of moves possible at that
    /// location
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
