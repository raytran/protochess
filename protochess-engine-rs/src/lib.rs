use crate::types::{Dimensions, bitboard, Move};
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

    pub fn make_move(&mut self, x1:u8, y1:u8, x2:u8, y2: u8, whos_turn:u8) -> bool{
        let from = bitboard::to_index(x1,y1,self.current_position.dimensions.width) as u8;
        let to = bitboard::to_index(x2,y2,self.current_position.dimensions.width) as u8;

        let moves = self.generate_moves(&self.current_position, whos_turn);
        for move_ in moves {
            if move_.get_from() == from && move_.get_to() == to {
                self.current_position.make_move(move_);
                return true
            }
        }
        false
    }

    pub fn undo(&mut self){
        self.current_position.unmake_move();
    }

    pub fn to_string(&self) -> String {
        self.current_position.to_string()
    }

    pub fn from_fen(fen:String) -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            masks:MaskHandler::new(&dims),
            dimensions:dims,
            current_position: Position::from_fen(fen),
        }
    }

    pub fn perft(&self,depth:u8) -> u64 {
        let moves = self.generate_moves(&self.current_position, 0);
        for move_ in &moves{

            let (x1, y1) = bitboard::from_index(move_.get_from() as usize, self.dimensions.width);
            let (x2, y2) = bitboard::from_index(move_.get_to() as usize, self.dimensions.width);
            println!("from: {}, {} , to: {},{}",x1,y1,x2,y2);

            if move_.get_capture(){
                println!("CAPTURE!");
            }

        }

        moves.len() as u64
    }
}

