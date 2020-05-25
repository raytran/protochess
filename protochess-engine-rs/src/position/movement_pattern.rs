use crate::types::bitboard::Bitboard;
use crate::types::chess_move::Move;

/// MovementPattern describes how a custom piece can move
pub struct MovementPattern {
    // Places where this piece can promote, as well as char codes for the promotion pieces
    pub promotion_squares: Option<Bitboard>,
    pub promo_vals: Option<Vec<char>>,

    // Ways the piece can capture (but not move without capturing)
    pub attack_sliding_deltas: Vec<(u8, u8)>,
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
    pub move_jump_deltas: Vec<(u8, u8)>,
    pub move_sliding_deltas: Vec<(u8, u8)>,
    pub move_north: bool,
    pub move_south: bool,
    pub move_east: bool,
    pub move_west: bool,
    pub move_northeast: bool,
    pub move_northwest: bool,
    pub move_southeast: bool,
    pub move_southwest: bool,
}

impl MovementPattern {
    pub fn null() -> MovementPattern {
        MovementPattern {
            promotion_squares: None,
            promo_vals: None,
            attack_sliding_deltas: vec![],
            attack_jump_deltas: vec![],
            attack_north: false,
            attack_south: false,
            attack_east: false,
            attack_west: false,
            attack_northeast: false,
            attack_northwest: false,
            attack_southeast: false,
            attack_southwest: false,
            move_jump_deltas: vec![],
            move_sliding_deltas: vec![],
            move_north: false,
            move_south: false,
            move_east: false,
            move_west: false,
            move_northeast: false,
            move_northwest: false,
            move_southeast: false,
            move_southwest: false
        }
    }
}


