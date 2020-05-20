use std::sync::Arc;
use crate::types::{PieceType};
use crate::types::bitboard::Bitboard;
use crate::position::castle_rights::CastleRights;
use crate::types::chess_move::Move;

//Properties that are hard to recover from a Move
#[derive(Clone)]
pub struct PositionProperties {
    pub move_played: Option<Move>,
    castling_rights: CastleRights,
    pub in_check: bool,
    //EP square (square behind a double pawn push)
    pub ep_square: Option<u8>,
    //Tuple (owner, PieceType) of the last piece captured, if any
    pub captured_piece: Option<(u8, PieceType)>,
    pub prev_properties: Option<Arc<PositionProperties>>,
}

impl PositionProperties {
    pub fn default() -> PositionProperties {
        PositionProperties{
            castling_rights: CastleRights::new(),
            move_played: None,
            prev_properties: None,
            in_check: false,
            ep_square: None,
            captured_piece: None,
        }
    }

    pub fn get_prev(&self) -> Option<Arc<PositionProperties>> {
        self.prev_properties.as_ref().cloned()
    }

}