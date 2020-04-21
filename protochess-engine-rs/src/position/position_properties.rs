use std::sync::Arc;
use crate::types::{Move, PieceType};
use crate::types::bitboard::Bitboard;

//Properties that are hard to recover from a Move
#[derive(Clone)]
pub struct PositionProperties {
    pub move_played: Option<Move>,
    castling_rights:u8,
    pub in_check: bool,
    //Tuple (owner, PieceType) of the last piece captured, if any
    pub captured_piece: Option<(u8, PieceType)>,
    pub prev_properties: Option<Arc<PositionProperties>>,
}

impl PositionProperties {
    pub fn default() -> PositionProperties {
        PositionProperties{
            castling_rights: (1 << 2) | 1,
            move_played: None,
            prev_properties: None,
            in_check: false,
            captured_piece: None,
        }
    }

    pub fn get_prev(&self) -> Option<Arc<PositionProperties>> {
        self.prev_properties.as_ref().cloned()
    }

    pub fn can_white_castle(&self) -> bool {
        (self.castling_rights & 1)  != 0
    }

    pub fn can_black_castle(&self) -> bool{
        (self.castling_rights & (1 << 2))  != 0
    }
}