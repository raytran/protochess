use crate::types::{Dimensions, bitboard, Move};
use crate::position::Position;
use crate::move_generator::MoveGenerator;
//Private modules
mod constants;

mod move_generator;
mod types;
mod position;
mod rankfile;

pub struct Engine {
    dimensions: Dimensions,
    current_position: Position,
    move_generator: MoveGenerator,
}

impl Engine {
    pub fn default() -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            dimensions:dims,
            move_generator: MoveGenerator::new(),
            current_position: Position::default(),
        }
    }

    pub fn make_move(&mut self, x1:u8, y1:u8, x2:u8, y2: u8, whos_turn:u8) -> bool{
        if whos_turn != self.current_position.whos_turn {
            return false
        }
        let from = bitboard::to_index(x1,y1) as u8;
        let to = bitboard::to_index(x2,y2) as u8;

        //let moves = self.move_generator.generate_moves(&self.current_position);
        //for move_ in moves {
        //    if move_.get_from() == from && move_.get_to() == to {
        //        self.current_position.make_move(move_);
        //        return true
        //    }
        //}
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
            dimensions:dims,
            move_generator: MoveGenerator::new(),
            current_position: Position::from_fen(fen),
        }
    }

    pub fn perft(&mut self,depth:u8) -> u64 {
        let mut nodes = 0u64;

        let moves = self.move_generator.get_moves(&self.current_position);
        if depth == 1 {
            return moves.len() as u64;
        }
        for move_ in moves{

            let (x,y) = bitboard::from_index(move_.get_from() as usize);

            self.current_position.make_move(move_);
            nodes += self.perft(depth - 1);
            self.current_position.unmake_move();
        }
        nodes
    }
}

