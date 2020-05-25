use crate::types::bitboard::Bitboard;

/// MovementPattern describes how a piece can move
pub struct MovementPattern {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub northeast: bool,
    pub northwest: bool,
    pub southeast: bool,
    pub southwest: bool,

    // Places where this piece can promote, as well as char codes for the promotion pieces
    pub promotion_squares: Option<Bitboard>,
    pub promo_vals: Option<Vec<char>>,
    // Ways the piece can capture
    pub attack_sliding_deltas: Vec<(u8, u8)>,
    pub attack_jump_deltas: Vec<(u8, u8)>,

    //Ways the piece can move (but not capture)
    pub move_jump_deltas: Vec<(u8, u8)>,
    pub move_sliding_deltas: Vec<(u8, u8)>,
}

impl MovementPattern {
    pub fn new() -> MovementPattern {
        MovementPattern {
            north: false,
            south: false,
            east: false,
            west: false,
            northeast: false,
            northwest: false,
            southeast: false,
            southwest: false,
            promotion_squares: None,
            promo_vals: None,
            attack_sliding_deltas: vec![],
            attack_jump_deltas: vec![],
            move_jump_deltas: vec![],
            move_sliding_deltas: vec![]
        }
    }
}


