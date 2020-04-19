//Pieces that a player has
use crate::types::bitboard::Bitboard;
pub struct PieceSet {
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
            king: Bitboard::new(),
            queen: Bitboard::new(),
            bishop: Bitboard::new(),
            knight: Bitboard::new(),
            rook: Bitboard::new(),
            pawn: Bitboard::new(),
            custom: Vec::new(),
        }
    }

    pub fn char_at(&self, index: u8) -> char{
        if self.king.bit_test(index){
            'k'
        }else if self.queen.bit_test(index){
            'q'
        }else if self.bishop.bit_test(index){
            'b'
        }else if self.knight.bit_test(index){
            'n'
        }else if self.rook.bit_test(index){
            'r'
        }else if self.pawn.bit_test(index){
            'p'
        }else{
            for (c, b) in self.custom.iter(){
                if b.bit_test(index){
                    return *c
                }
            }
            '.'
        }
    }
}