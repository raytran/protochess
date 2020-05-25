use crate::position::Position;
use std::collections::HashMap;

use crate::position::piece_set::PieceSet;

use crate::move_generator::MoveGenerator;
use crate::types::PieceType;

const KING_SCORE:isize = 9999;
const QUEEN_SCORE:isize = 90;
const ROOK_SCORE:isize = 50;
const BISHOP_SCORE:isize = 30;
const KNIGHT_SCORE:isize = 30;
const PAWN_SCORE:isize = 10;
const CHECKMATED_SCORE:isize = -10000;
const MOVE_SCORE:isize = 2;

/// Assigns a score to a given position
pub(crate) struct Evaluator {
    //Piece values for pieces, generated dynamically
    custom_score_table: HashMap<PieceType, usize>,
    //Piece-square values for all pieces, done as a function of movement possibilities
    piece_square_table: HashMap<PieceType, Vec<u32>>
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            custom_score_table: HashMap::new(),
            piece_square_table:HashMap::new()
        }
    }
    //Retrieves the score for the player to move (position.whos_turn)
    pub fn evaluate(&self, position: &mut Position, movegen: &MoveGenerator) -> isize {
        let mut score = 0;
        let player_num = position.whos_turn;
        for (i, ps) in position.pieces.iter().enumerate() {
            let side_multiplier = if i as u8 == player_num { 1 } else {-1};
            score += side_multiplier * self.get_material_score_for_pieceset(ps);
        }
        score += self.get_positional_score(position, movegen);
        score
    }

    fn get_material_score_for_pieceset(&self, piece_set:&PieceSet) -> isize{
        let mut material_score = 0;
        material_score += piece_set.king.bitboard.count_ones() as isize * KING_SCORE;
        material_score += piece_set.queen.bitboard.count_ones() as isize * QUEEN_SCORE;
        material_score += piece_set.rook.bitboard.count_ones() as isize * ROOK_SCORE;
        material_score += piece_set.knight.bitboard.count_ones() as isize * KNIGHT_SCORE;
        material_score += piece_set.bishop.bitboard.count_ones() as isize * BISHOP_SCORE;
        material_score += piece_set.pawn.bitboard.count_ones() as isize * PAWN_SCORE;
        material_score
    }

    fn get_positional_score(&self, position: &mut Position, movegen: &MoveGenerator) -> isize {
        let mut positional_score = 0;
        positional_score += movegen.count_legal_moves(position) as isize;
        if positional_score == 0 {
            return CHECKMATED_SCORE as isize;
        }
        positional_score * MOVE_SCORE
    }
}