use crate::types::bitboard::{Bitboard};
use crate::types::chess_move::{Move, MoveType};

/// Iterator that converts a Bitboard of move possibilities to Moves
pub struct BitboardMoves {
    pub(crate) enemies: Bitboard,     //Enemies
    pub(crate) moves:Bitboard,        //moveset for source piece
    pub(crate) source_index: u8,       //Source piece index
    pub(crate) promotion_squares: Option<Bitboard>, //Optional promotion squares for this piece
    pub(crate) promo_vals: Option<Vec<char>>,   //Optional promotable values for this piece
    current_promo_vals: Option<Vec<char>>, //Internal; used as a copy of promovals for each sq
}

impl BitboardMoves {
    pub fn new(enemies:Bitboard,
               moves:Bitboard,
               source_index:u8,
               promotion_squares:Option<Bitboard>,
               promo_vals:Option<Vec<char>>) -> BitboardMoves{
        BitboardMoves{
            enemies,
            moves,
            source_index,
            promotion_squares,
            promo_vals,
            current_promo_vals:None,
        }
    }
}

impl Iterator for BitboardMoves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(to) = self.moves.lowest_one() {
            let promo_here = {
                if let Some(promo_sqs) = &self.promotion_squares {
                    promo_sqs.bit(to).unwrap()
                } else {
                    false
                }
            };
            let capture_here = { self.enemies.bit(to).unwrap() };
            let move_type = {
                match (capture_here, promo_here) {
                    (true, true) => { MoveType::PromotionCapture },
                    (true, false) => { MoveType::Capture },
                    (false, true) => { MoveType::Promotion },
                    (false, false) => { MoveType::Quiet },
                }
            };
            let target = {if capture_here {to} else {0}};
            let promo_char = {
                if promo_here {
                    //promotion, do not go next until we run out of promo options
                    let promo_options = {
                        if let Some(pv) = &mut self.current_promo_vals {
                            pv
                        }else{
                            self.current_promo_vals = (&self.promo_vals).to_owned();
                            self.current_promo_vals.as_mut().unwrap()
                        }
                    };

                    let next_char = promo_options.pop().unwrap();
                    //If we run out of promo options, we can go to the next square
                    if promo_options.len() == 0 {
                        //Reset current_promo_vals for the next time
                        self.current_promo_vals = None;
                        self.moves.set_bit(to, false);
                    }
                    //Unwrap intentionally here; want to panic if this goes wrong
                    Some(next_char)
                }else{
                    //No promotion chars left, go to next after this
                    self.moves.set_bit(to, false);
                    None
                }
            };
            Some(Move::new(self.source_index as u8, to as u8, Some(target as u8), move_type, promo_char))
        } else {
            None
        }
    }
}


