use crate::position::Position;
use std::collections::HashMap;

use crate::position::piece_set::PieceSet;

use crate::move_generator::MoveGenerator;
use crate::types::PieceType;
use crate::position::piece::Piece;
use crate::MovementPattern;

//Scores are in centipawns
const KING_SCORE:isize = 9999;
const QUEEN_SCORE:isize = 900;
const ROOK_SCORE:isize = 500;
const BISHOP_SCORE:isize = 300;
const KNIGHT_SCORE:isize = 300;
const PAWN_SCORE:isize = 100;
const CHECKMATED_SCORE:isize = -10000;
const MOVE_SCORE:isize = 2;

/// Assigns a score to a given position
pub(crate) struct Evaluator {
    //Piece values for pieces,
    //Hard coded for builtin pieces,
    //generated dynamically based on the piece's movement pattern
    custom_piece_value_table: HashMap<PieceType, isize>,
    //Piece-square values for all pieces, done as a function of movement possibilities
    //Generated dynamically for all pieces
    piece_square_table: HashMap<PieceType, Vec<u32>>
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
        for (i, ps) in position.pieces.iter().enumerate() {
            let side_multiplier = if i as u8 == player_num { 1 } else {-1};
            score += side_multiplier * self.get_material_score_for_pieceset(position, ps);
        }
        score += self.get_positional_score(position, movegen);
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

    fn get_positional_score(&self, position: &Position, movegen: &MoveGenerator) -> isize {

        0
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