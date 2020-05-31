use std::fmt;
use crate::rankfile::to_rank_file;
use crate::types::bitboard::from_index;

#[derive(PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
    QueensideCastle,
    KingsideCastle,
    Promotion,
    PromotionCapture,
    Null,
}

/// Stores a move in a u32
///0-7:   from index:u8
///8-15:  to index:u8
///16-23: target index:u8
///24-26 : movetype
/// 000 = quiet
/// 001 = capture
/// 010 = castle
/// 011 = promotion
/// 100 = promotion-capture
#[derive(PartialEq, Copy, Clone)]
pub struct Move(u32, Option<char>);

impl Move {
    pub fn new(from:u8, to:u8, target_loc: Option<u8>, move_type:MoveType, promo:Option<char>) -> Move{
        Move(
            (from as u32)
                | (to as u32) << 8u32
                |
                {
                    if let Some(tl) = target_loc {
                        (tl as u32) << 16u32
                    } else {
                        0
                    }
                }
                |
                match move_type {
                    MoveType::Quiet => {0}
                    MoveType::Capture => {1u32 << 24}
                    MoveType::KingsideCastle => {2u32 << 24}
                    MoveType::QueensideCastle => {3u32 << 24}
                    MoveType::Promotion => {4u32 << 24}
                    MoveType::PromotionCapture => {5u32 << 24}
                    MoveType::Null => {6u32 << 24}
                },
            promo
        )
    }

    pub fn null() -> Move {
        Move::new(0,0,None,MoveType::Null, None)
    }

    pub fn get_from(&self) -> u8{
        (self.0 & 255u32) as u8
    }

    pub fn get_to(&self) -> u8{
        ((self.0 >> 8) & 255u32) as u8
    }

    pub fn get_is_capture(&self) -> bool{
        ((self.0 >> 24) & 1u32) != 0u32
    }

    pub fn get_move_type(&self) -> MoveType {
        match &self.0 >> 24 & 7u32 {
            0 => { MoveType::Quiet }
            1 => { MoveType::Capture }
            2 => { MoveType::KingsideCastle }
            3 => { MoveType::QueensideCastle }
            4 => { MoveType::Promotion }
            5 => { MoveType::PromotionCapture }
            6 => { MoveType::Null }
            _ => { MoveType::Quiet }
        }
    }

    pub fn get_promotion_char(&self) -> Option<char> {
        self.1
    }

    pub fn get_target(&self) -> u8 {
        ((self.0 >> 16) & 255u32) as u8
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x1, y1) = from_index(self.get_from() as usize);
        let (x2, y2) = from_index(self.get_to() as usize);
        write!(f, "(from: {}, to:{})", to_rank_file(x1, y1),to_rank_file(x2, y2))
    }
}
