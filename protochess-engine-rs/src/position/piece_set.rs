//Pieces that a player has
use crate::types::bitboard::Bitboard;
use crate::position::piece::Piece;

/// Represents a set of pieces for a player
/// custom is a vec of custom piece
pub struct PieceSet {
    pub occupied: Bitboard,
    pub king: Piece,
    pub queen: Piece,
    pub bishop: Piece,
    pub knight: Piece,
    pub rook: Piece,
    pub pawn: Piece,
    pub custom: Vec<Piece>,
    pub player_num: u8
}

impl PieceSet {
    pub fn new(player_num:u8) -> PieceSet {
        PieceSet {
            occupied: Bitboard::zero(),
            king: Piece::blank_king(player_num),
            queen: Piece::blank_queen(player_num),
            bishop: Piece::blank_bishop(player_num),
            knight: Piece::blank_knight(player_num),
            rook: Piece::blank_rook(player_num),
            pawn: Piece::blank_pawn(player_num),
            custom: Vec::new(),
            player_num
        }
    }

    pub fn piece_at(&mut self, index:usize) -> Option<&mut Piece> {
        if self.king.bitboard.bit(index).unwrap(){
            Some(&mut self.king)
        }else if self.queen.bitboard.bit(index).unwrap(){
            Some(&mut self.queen)
        }else if self.bishop.bitboard.bit(index).unwrap(){
            Some(&mut self.bishop)
        }else if self.knight.bitboard.bit(index).unwrap(){
            Some(&mut self.knight)
        }else if self.rook.bitboard.bit(index).unwrap(){
            Some(&mut self.rook)
        }else if self.pawn.bitboard.bit(index).unwrap(){
            Some(&mut self.pawn)
        }else{
            for p in self.custom.iter_mut(){
                if p.bitboard.bit(index).unwrap(){
                    return Some(p);
                }
            }
            None
        }
    }

    pub fn get_piece_refs(&self) -> Vec<&Piece> {
        let mut return_vec = Vec::with_capacity(6);
        return_vec.push(&self.king);
        return_vec.push(&self.queen);
        return_vec.push(&self.bishop);
        return_vec.push(&self.knight);
        return_vec.push(&self.rook);
        return_vec.push(&self.pawn);
        for p in &self.custom {
            return_vec.push(p);
        }
        return_vec
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