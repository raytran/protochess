use arrayvec::ArrayVec;
use crate::types::*;
use crate::fen;
use crate::position::piece_set::PieceSet;
use crate::types::bitboard::Bitboard;

mod piece_set;

pub struct Position{
    dimensions: Dimensions,
    whos_turn: u8,
    pieces:ArrayVec<[PieceSet;4]>, //pieces[0] = white's pieces, pieces[1] black etc
}

impl Position {
    pub fn default() -> Position{
        Position::from_FEN(String::from(fen::STARTING_POS))
    }

    pub fn to_string(&self) -> String {
        let mut return_str= String::new();
        for y in (0..self.dimensions.height).rev() {
            for x in 0..self.dimensions.width {
                let mut c = '.';
                for piece_set in &self.pieces {
                    c = piece_set.char_at(bitboard::to_index(x,y,self.dimensions.width));
                    if c != '.'{
                        break;
                    }
                }
                return_str.push(c);
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

        println!("{}",fen);
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

                    bitboard.bit_set(index);
                    x += 1;
                }
                _ => continue,
            }
        }
        wb_pieces.push(w_pieces);
        wb_pieces.push(b_pieces);
        Position{whos_turn: 0, dimensions: dims, pieces: wb_pieces}
    }
}

