use crate::types::bitboard::{Bitboard};
use crate::types::{Dimensions, bitboard, get_from, get_to, get_capture};
use crate::position::Position;
use crate::mask_handler::MaskHandler;


//Private modules
mod mask_handler;
mod movegen;
mod types;
mod position;
mod fen;
mod rankfile;


pub struct Engine {
    dimensions: Dimensions,
    masks: MaskHandler,
    current_position: Position,
}

impl Engine {
    pub fn default() -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            masks:MaskHandler::new(&dims),
            dimensions:dims,
            current_position: Position::default(),
        }
    }

    pub fn from_fen(fen:String) -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            masks:MaskHandler::new(&dims),
            dimensions:dims,
            current_position: Position::from_FEN(fen),
        }
    }

    pub fn perft(&self,depth:u8) -> u64 {
        let moves = self.generate_moves(&self.current_position, 0);
        for move_ in &moves{

            let (x1, y1) = bitboard::from_index(get_from(&move_) as usize, self.dimensions.width);
            let (x2, y2) = bitboard::from_index(get_to(&move_) as usize, self.dimensions.width);
            println!("from: {}, {} , to: {},{}",x1,y1,x2,y2);

            if get_capture(move_){
                println!("CAPTURE!");
            }

        }

        moves.len() as u64
    }
}

