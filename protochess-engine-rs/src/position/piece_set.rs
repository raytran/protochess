//Pieces that a player has
use crate::types::bitboard::Bitboard;
use crate::types::{PieceType};
use crate::position::piece::Piece;

/// Represents a set of pieces for a player
/// custom is a vec of custom piece types
/// holding tuples mapping Bitboards to their char representation and MovementPattern
/// chars should not contain k, q, r, p, or n
pub struct PieceSet {
    pub occupied: Bitboard,
    pub king: Piece,
    pub queen: Piece,
    pub bishop: Piece,
    pub knight: Piece,
    pub rook: Piece,
    pub pawn: Piece,
    pub custom: Vec<Piece>,
}

impl PieceSet {
    pub fn new() -> PieceSet {
        PieceSet {
            occupied: Bitboard::zero(),
            king: Piece::blank_king(),
            queen: Piece::blank_queen(),
            bishop: Piece::blank_bishop(),
            knight: Piece::blank_knight(),
            rook: Piece::blank_rook(),
            pawn: Piece::blank_pawn(),
            custom: Vec::new(),
        }
    }

    pub fn piece_bb_at(&mut self, index: usize) -> Option<&mut Bitboard>{
        if self.king.bitboard.bit(index).unwrap(){
            Some(&mut self.king.bitboard)
        }else if self.queen.bitboard.bit(index).unwrap(){
            Some(&mut self.queen.bitboard)
        }else if self.bishop.bitboard.bit(index).unwrap(){
            Some(&mut self.bishop.bitboard)
        }else if self.knight.bitboard.bit(index).unwrap(){
            Some(&mut self.knight.bitboard)
        }else if self.rook.bitboard.bit(index).unwrap(){
            Some(&mut self.rook.bitboard)
        }else if self.pawn.bitboard.bit(index).unwrap(){
            Some(&mut self.pawn.bitboard)
        }else{
            for p in self.custom.iter_mut(){
                if p.bitboard.bit(index).unwrap(){
                    return Some(&mut p.bitboard);
                }
            }
            None
        }
    }

    pub fn piecetype_at(&self, index: usize) -> Option<PieceType>{
        if self.king.bitboard.bit(index).unwrap(){
            Some(PieceType::King)
        }else if self.queen.bitboard.bit(index).unwrap(){
            Some(PieceType::Queen)
        }else if self.bishop.bitboard.bit(index).unwrap(){
            Some(PieceType::Bishop)
        }else if self.knight.bitboard.bit(index).unwrap(){
            Some(PieceType::Knight)
        }else if self.rook.bitboard.bit(index).unwrap(){
            Some(PieceType::Rook)
        }else if self.pawn.bitboard.bit(index).unwrap(){
            Some(PieceType::Pawn)
        }else{
            for p in self.custom.iter(){
                if p.bitboard.bit(index).unwrap(){
                    return Some(p.piece_type.to_owned());
                }
            }
            None
        }
    }

    //Recomputes occupied bb
    pub fn update_occupied(&mut self){
        self.occupied = Bitboard::zero();
        self.occupied |= &self.king.bitboard;
        self.occupied |= &self.queen.bitboard;
        self.occupied |= &self.bishop.bitboard;
        self.occupied |= &self.knight.bitboard;
        self.occupied |= &self.rook.bitboard;
        self.occupied |= &self.pawn.bitboard;
        for p in &self.custom {
            self.occupied |= &p.bitboard;
        }
    }
}