use crate::types::bitboard::Bitboard;
use crate::types::chess_move::Move;

/// MovementPattern describes how a custom piece can move
/// Each sliding field expects a vec of vec, with the inner vec represnting a new "run"
pub struct MovementPattern {
    // Places where this piece can promote, as well as char codes for the promotion pieces
    pub promotion_squares: Option<Bitboard>,
    pub promo_vals: Option<Vec<char>>,

    // Ways the piece can capture (but not move without capturing)
    pub attack_sliding_deltas: Vec<Vec<(u8, u8)>>,
    pub attack_jump_deltas: Vec<(u8, u8)>,
    pub attack_north: bool,
    pub attack_south: bool,
    pub attack_east: bool,
    pub attack_west: bool,
    pub attack_northeast: bool,
    pub attack_northwest: bool,
    pub attack_southeast: bool,
    pub attack_southwest: bool,

    //Ways the piece can move (but not capture)
    pub translate_jump_deltas: Vec<(u8, u8)>,
    pub translate_sliding_deltas: Vec<Vec<(u8, u8)>>,
    pub translate_north: bool,
    pub translate_south: bool,
    pub translate_east: bool,
    pub translate_west: bool,
    pub translate_northeast: bool,
    pub translate_northwest: bool,
    pub translate_southeast: bool,
    pub translate_southwest: bool,
}

impl MovementPattern {
    pub fn promotion_at(&self, index:usize) -> bool {
        if let Some(bb) = &self.promotion_squares {
            return bb.bit(index).unwrap()
        }
        false
    }
}


