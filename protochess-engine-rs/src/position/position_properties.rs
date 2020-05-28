use std::sync::Arc;
use crate::types::{PieceType};

use crate::position::castle_rights::CastleRights;
use crate::types::chess_move::Move;

/// Properties that are hard to recover from a Move
#[derive(Clone)]
pub struct PositionProperties {
    pub zobrist_key: u64,
    pub move_played: Option<Move>,
    //If the last move was a promotion, promote_from is the previous piecetype
    pub promote_from: Option<PieceType>,
    pub castling_rights: CastleRights,
    //EP square (square behind a double pawn push)
    pub ep_square: Option<u8>,
    //Tuple (owner, PieceType) of the last piece captured, if any
    pub captured_piece: Option<(u8, PieceType)>,
    pub prev_properties: Option<Arc<PositionProperties>>,
}

impl PositionProperties {
    pub fn default() -> PositionProperties {
        PositionProperties{
            zobrist_key: 0,
            castling_rights: CastleRights::new(),
            move_played: None,
            prev_properties: None,
            promote_from: None,
            ep_square: None,
            captured_piece: None,
        }
    }

    pub fn get_prev(&self) -> Option<Arc<PositionProperties>> {
        self.prev_properties.as_ref().cloned()
    }

}