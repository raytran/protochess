use arrayvec::ArrayVec;
use crate::types::*;
use crate::constants::fen;
use crate::position::piece_set::PieceSet;
use crate::types::bitboard::{Bitboard, to_index, from_index, to_string};
use std::sync::Arc;

use position_properties::PositionProperties;
use crate::types::chess_move::Move;

mod position_properties;
mod castle_rights;
pub mod piece_set;

pub struct Position {
    pub dimensions: Dimensions,
    pub bounds: Bitboard, //Bitboard representing the boundaries
    pub num_players: u8,
    pub whos_turn: u8,
    pub pieces:ArrayVec<[PieceSet;4]>, //pieces[0] = white's pieces, pieces[1] black etc
    pub occupied: Bitboard,
    //Properties relating only to the current position
    // Typically hard-to-recover properties, like castling
    //Similar to state in stockfish
    pub properties: Arc<PositionProperties>,
}

impl Position {
    pub fn default() -> Position{
        Position::from_fen(String::from(fen::STARTING_POS))
    }

    //Modifies the position to make the move
    pub fn make_move(&mut self, move_: Move) {
        let my_player_num = self.whos_turn;
        self.whos_turn = (self.whos_turn + 1) % self.num_players;

        let mut new_props:PositionProperties = (*self.properties).clone();
        //Remove captured piece, if any
        if move_.get_is_capture() {
            let capt_index = move_.get_target() as usize;
            new_props.captured_piece = self.piece_at(capt_index);
            let capd_bb:&mut Bitboard = self.piece_bb_at(capt_index).unwrap();
            capd_bb.set_bit(capt_index, false);
        }

        let from= move_.get_from();
        let to = move_.get_to();
        let from_piece_type = self.piece_at(from as usize).unwrap().1;

        //Pawn en-passant
        //Check for a pawn double push to set ep square
        let (x1, y1) = from_index(from as usize);
        let (x2, y2) = from_index(to as usize);
        if from_piece_type == PieceType::PAWN
            && (y2 as i8 - y1 as i8).abs() == 2
            && x1 == x2 {
            new_props.ep_square = Some(
                if y2 > y1 {
                    to_index(x1, y2 - 1) as u8
                } else {
                    to_index(x1, y2 + 1) as u8
                }
            );
        } else {
            new_props.ep_square = None;
        }


        //Castling
        //Disable rights if applicable
        if new_props.castling_rights.can_player_castle(my_player_num) {
            if from_piece_type == PieceType::KING {
                new_props.castling_rights.disable_kingside_castle(my_player_num);
                new_props.castling_rights.disable_queenside_castle(my_player_num);
            }else if from_piece_type == PieceType::ROOK {
                //King side
                if x1 >= self.dimensions.width/2 {
                    new_props.castling_rights.disable_kingside_castle(my_player_num);
                }else{
                    new_props.castling_rights.disable_queenside_castle(my_player_num);
                }
            }
        }

        //Move piece to location
        self.move_piece(from, to);

        //Update props
        new_props.move_played = Some(move_);
        new_props.prev_properties = Some(Arc::clone(&self.properties));
        self.properties = Arc::new(new_props);
        //Update occupied bbs for future calculations
        self.update_occupied();
    }

    //Undo the most recent move
    pub fn unmake_move(&mut self) {

        if self.whos_turn == 0 {
            self.whos_turn = self.num_players -1;
        }else{
            self.whos_turn = (self.whos_turn - 1) % self.num_players;
        }

        let move_ = self.properties.move_played.unwrap();
        let from = move_.get_from();
        let to= move_.get_to();

        //Undo move piece to location
        //Remove piece here
        self.move_piece(to, from);

        //Undo move capture
        if move_.get_is_capture() {
            let capt = move_.get_target();
            let (owner, pt) = self.properties.captured_piece.as_ref().unwrap();
            match pt {
                PieceType::KING => {self.pieces[*owner as usize].king.set_bit(capt as usize, true);},
                PieceType::QUEEN => {self.pieces[*owner as usize].queen.set_bit(capt as usize, true);},
                PieceType::ROOK => {self.pieces[*owner as usize].rook.set_bit(capt as usize, true);},
                PieceType::BISHOP => {self.pieces[*owner as usize].bishop.set_bit(capt as usize, true);},
                PieceType::KNIGHT => {self.pieces[*owner as usize].knight.set_bit(capt as usize, true);},
                PieceType::PAWN => {self.pieces[*owner as usize].pawn.set_bit(capt as usize, true);},
                PieceType::CUSTOM(ptc) => {
                    for (c, bb) in self.pieces[*owner as usize].custom.iter_mut() {
                        if *ptc == *c {
                            bb.set_bit(to as usize,true);
                            break;
                        }
                    }
                },
            }
        }

        //Update props
        //Consume prev props; never to return again
        self.properties = self.properties.get_prev().unwrap();

        //Update occupied bbs for future calculations
        self.update_occupied();
    }

    pub fn to_string(&self) -> String {
        let mut return_str= String::new();
        for y in (0..self.dimensions.height).rev() {
            for x in 0..self.dimensions.width {

                if let Some((i, pt)) = self.piece_at(bitboard::to_index(x,y)){
                    match pt {
                        PieceType::KING => if i == 0 {return_str.push('K')} else {return_str.push('k')},
                        PieceType::QUEEN => if i == 0 {return_str.push('Q')} else {return_str.push('q')},
                        PieceType::ROOK => if i == 0 {return_str.push('R')} else {return_str.push('r')},
                        PieceType::BISHOP => if i == 0 {return_str.push('B')} else {return_str.push('b')},
                        PieceType::KNIGHT => if i == 0 {return_str.push('N')} else {return_str.push('n')},
                        PieceType::PAWN => if i == 0 {return_str.push('P')} else {return_str.push('p')},
                        PieceType::CUSTOM(c) => return_str.push(c),
                    }
                }else{
                    return_str.push('.');
                }
                return_str.push(' ');
            }
            return_str.push('\n');
        }
        return_str
    }

    pub fn from_fen(fen: String) -> Position{
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

                    let index = bitboard::to_index(x, y);
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

        let mut bounds = Bitboard::zero();
        for x in 0..8{
            for y in 0..8{
                bounds.set_bit(to_index(x,y),true);
            }
        }
        Position{
            whos_turn: 0,
            num_players: 2,
            dimensions: dims,
            pieces: wb_pieces,
            occupied,
            bounds,
            properties: Arc::new(PositionProperties::default())
        }
    }

    //Returns tuple (player_num, PieceType)
    pub fn piece_at(&self,index:usize) -> Option<(u8, PieceType)> {
        for (i, ps) in self.pieces.iter().enumerate() {
            if let Some(c) = ps.piecetype_at(index) {
                return Some((i as u8, c));
            }
        }
        None
    }

    //Returns bitoard of piece at index
    pub fn piece_bb_at(&mut self,index:usize) -> Option<&mut Bitboard> {
        for (i, ps) in self.pieces.iter_mut().enumerate() {
            if let Some(b) = ps.piece_bb_at(index) {
                return Some(b);
            }
        }
        None
    }

    fn update_occupied(&mut self){
        self.occupied = Bitboard::zero();
        for (i, ps) in self.pieces.iter_mut().enumerate() {
            ps.update_occupied();
            self.occupied |= &ps.occupied;
        }
    }

    fn move_piece(&mut self, from:u8, to:u8){
        if let Some(source_bb) = self.piece_bb_at(from as usize){
            source_bb.set_bit(from as usize, false);
            source_bb.set_bit(to as usize, true);
        }else{
            println!("nothing to move??");
            println!("from {} {}", from_index(from as usize).0, from_index(from as usize).1);
            println!("to {} {}", from_index(to as usize).0, from_index(to as usize).1);
            println!("==");
        }
    }

}

