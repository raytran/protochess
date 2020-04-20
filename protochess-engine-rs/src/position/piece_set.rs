//Pieces that a player has
use crate::types::bitboard::Bitboard;
pub struct PieceSet {
    pub occupied: Bitboard,
    pub king: Bitboard,
    pub queen: Bitboard,
    pub bishop: Bitboard,
    pub knight: Bitboard,
    pub rook: Bitboard,
    pub pawn: Bitboard,
    pub custom: Vec<(char,Bitboard)>,
}

impl PieceSet {
    pub fn new() -> PieceSet {
        PieceSet {
            occupied: Bitboard::zero(),
            king: Bitboard::zero(),
            queen: Bitboard::zero(),
            bishop: Bitboard::zero(),
            knight: Bitboard::zero(),
            rook: Bitboard::zero(),
            pawn: Bitboard::zero(),
            custom: Vec::new(),
        }
    }

    pub fn piece_at(&self, index: usize) -> Option<char>{
        if self.king.bit(index).unwrap(){
            Some('k')
        }else if self.queen.bit(index).unwrap(){
            Some('q')
        }else if self.bishop.bit(index).unwrap(){
            Some('b')
        }else if self.knight.bit(index).unwrap(){
            Some('n')
        }else if self.rook.bit(index).unwrap(){
            Some('r')
        }else if self.pawn.bit(index).unwrap(){
            Some('p')
        }else{
            for (c, b) in self.custom.iter(){
                if b.bit(index).unwrap(){
                    return Some(*c);
                }
            }
            None
        }
    }
}