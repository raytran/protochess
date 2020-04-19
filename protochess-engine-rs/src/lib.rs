use crate::types::bitboard::{Bitboard};
use crate::types::Dimensions;
use crate::position::Position;

mod types;
mod position;
mod fen;

pub fn example() {
    let mut bb= Bitboard::new();
    bb.bit_set(3);
    bb.bit_set(8);
    bb.bit_set(128);
    bb.bit_set(255);
    println!("{}",bb.to_string(&Dimensions{width:16, height:16}));
    bb.bit_unset(255);
    println!("{}",bb.to_string(&Dimensions{width:16, height:16}));


    let mut pos = Position::default();
    println!("{}",pos.to_string());
}

