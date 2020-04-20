use arrayvec::ArrayVec;
use crate::types::*;
use crate::fen;
use crate::position::piece_set::PieceSet;
use crate::types::bitboard::Bitboard;

pub mod piece_set;

pub struct Position{
    pub dimensions: Dimensions,
    pub whos_turn: u8,
    pub pieces:ArrayVec<[PieceSet;4]>, //pieces[0] = white's pieces, pieces[1] black etc
    pub occupied: Bitboard,
}

impl Position {
    pub fn default() -> Position{
        Position::from_FEN(String::from(fen::STARTING_POS))
    }

    pub fn to_string(&self) -> String {
        let mut return_str= String::new();
        for y in (0..self.dimensions.height).rev() {
            for x in 0..self.dimensions.width {

                if let Some(c) = self.piece_at(bitboard::to_index(x,y,self.dimensions.width)){
                    return_str.push(c);
                }else{
                    return_str.push('.');
                }
                return_str.push(' ');
            }
            return_str.push('\n');
        }
        return_str
    }

    pub fn from_FEN(fen: String) -> Position{
        let dims = Dimensions{width:8,height:8};

        let mut wb_pieces = ArrayVec::<[_;4]>::new();
        let mut w_pieces = PieceSet::new();
        let mut b_pieces = PieceSet::new();

        let mut x:u8 =0;
        let mut y :u8 = 7;
        let mut field = 0;

        for c in fen.chars(){
            if c == ' ' {
                field += 1;
            }
            match field{
                0 => {
                    if c == '/' {
                        x = 0;
                        y -= 1;
                        continue;
                    }else if c.is_numeric() {
                        x += c.to_digit(10).expect("Not a digit!") as u8;
                        continue;
                    }

                    let index = bitboard::to_index(x, y, dims.width);
                    let bitboard: &mut Bitboard = match c.to_ascii_lowercase() {
                        'k' => {
                            if c.is_uppercase() { &mut w_pieces.king } else { &mut b_pieces.king }
                        },
                        'q' => {
                            if c.is_uppercase() { &mut w_pieces.queen } else { &mut b_pieces.queen }
                        },
                        'r' => {
                            if c.is_uppercase() { &mut w_pieces.rook } else { &mut b_pieces.rook }
                        },
                        'b' => {
                            if c.is_uppercase() { &mut w_pieces.bishop } else { &mut b_pieces.bishop }
                        },
                        'n' => {
                            if c.is_uppercase() { &mut w_pieces.knight } else { &mut b_pieces.knight }
                        },
                        'p' => {
                            if c.is_uppercase() { &mut w_pieces.pawn } else { &mut b_pieces.pawn }
                        },
                        _ => continue,
                    };

                    bitboard.set_bit(index, true);
                    if c.is_uppercase() {w_pieces.occupied.set_bit(index,true)} else {b_pieces.occupied.set_bit(index, true)};
                    x += 1;
                }
                _ => continue,
            }
        }

        let mut occupied = Bitboard::zero();
        occupied |= &w_pieces.occupied;
        occupied |= &b_pieces.occupied;

        wb_pieces.push(w_pieces);
        wb_pieces.push(b_pieces);

        Position{whos_turn: 0, dimensions: dims, pieces: wb_pieces, occupied}
    }

    pub fn piece_at(&self,index:usize) -> Option<char> {
        for ps in &self.pieces {
            if let Some(c) = ps.piece_at(index) {
                return Some(c);
            }
        }
        None
    }
}

