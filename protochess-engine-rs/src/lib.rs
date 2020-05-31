use crate::types::{Dimensions, bitboard};
use crate::position::Position;
use crate::move_generator::MoveGenerator;
use crate::rankfile::to_rank_file;

use crate::types::bitboard::{to_index, from_index};

//Private modules
mod constants;
mod move_generator;
mod types;
mod position;
mod evaluator;
mod searcher;
mod rankfile;

use crate::evaluator::Evaluator;
pub use crate::position::movement_pattern::MovementPattern;
pub use crate::types::PieceType;
use crate::searcher::Searcher;


/// Starting point for the engine
pub struct Engine {
    pub(crate) dimensions: Dimensions,
    pub(crate) current_position: Position,
    pub(crate) move_generator: MoveGenerator,
    pub(crate) evaluator: Evaluator,
    pub(crate) searcher: Searcher
}

impl Engine {
    pub fn default() -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            dimensions:dims,
            move_generator: MoveGenerator::new(),
            current_position: Position::default(),
            evaluator: Evaluator::new(),
            searcher: Searcher::new()
        }
    }

    pub fn get_zobrist(&self) -> u64 {
        self.current_position.get_zobrist()
    }

    pub fn get_score(&mut self) -> isize{
        self.evaluator.evaluate(&mut self.current_position, &self.move_generator)
    }

    pub fn register_piecetype(&mut self, char_rep:char, mp: MovementPattern) {
        self.current_position.register_piecetype(char_rep, mp);
    }

    pub fn add_piece(&mut self, owner:usize, piece_type:PieceType, x: u8, y:u8) {
        self.current_position.add_piece(0, PieceType::Custom('a'), to_index(x,y) as u8);
    }

    pub fn remove_piece(&mut self, index:u8) {
        self.current_position.remove_piece(index);
    }

    pub fn make_move(&mut self, x1:u8, y1:u8, x2:u8, y2: u8) -> bool{
        let from = bitboard::to_index(x1,y1) as u8;
        let to = bitboard::to_index(x2,y2) as u8;

        let moves = self.move_generator.get_pseudo_moves(&mut self.current_position);
        for move_ in moves {
            if !self.move_generator.is_move_legal(&move_, &mut self.current_position) {
                continue;
            }
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

    pub fn to_string(&mut self) -> String {
        self.current_position.to_string()
    }

    pub fn from_fen(fen:String) -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            dimensions:dims,
            move_generator: MoveGenerator::new(),
            evaluator: Evaluator::new(),
            searcher: Searcher::new(),
            current_position: Position::from_fen(fen),
        }
    }

    pub fn perft(&mut self,depth:u8) -> u64 {
        let mut nodes = 0u64;

        let moves = self.move_generator.get_pseudo_moves(&mut self.current_position);

        if depth == 1 {
            return self.move_generator.count_legal_moves(&mut self.current_position);
        }
        for move_ in moves{
            if !self.move_generator.is_move_legal(&move_, &mut self.current_position) {
                continue;
            }
            self.current_position.make_move(move_);
            nodes += self.perft(depth - 1);
            self.current_position.unmake_move();
        }
        nodes
    }

    pub fn perft_divide(&mut self,depth:u8) -> u64 {
        let mut nodes = 0u64;

        let moves = self.move_generator.get_pseudo_moves(&mut self.current_position);
        if depth == 1 {
            return self.move_generator.count_legal_moves(&mut self.current_position);
        }
        let mut printing = Vec::new();
        for move_ in moves{
            if !self.move_generator.is_move_legal(&move_, &mut self.current_position) {
                continue;
            }

            let (x,y) = bitboard::from_index(move_.get_from() as usize);
            let (x2,y2) = bitboard::from_index(move_.get_to() as usize);
            self.current_position.make_move(move_);
            let plus = self.perft(depth - 1);
            nodes += plus;
            self.current_position.unmake_move();
            //Print nodes
            printing.push(format!("{}{}: {}", to_rank_file(x,y), to_rank_file(x2,y2), plus));
        }
        printing.sort();
        for s in printing {
            println!("{}",s);
        }
        nodes
    }

    pub fn play_best_move(&mut self, depth:u8) -> bool {
        if let Some(best) = self.searcher.get_best_move(&mut self.current_position,
                                               &mut self.evaluator,
                                               &self.move_generator,
                                               depth){
            let (x1, y1) = from_index(best.get_from() as usize);
            let (x2, y2) = from_index(best.get_to() as usize);
            self.make_move(x1, y1, x2, y2)
        }else{
            false
        }
    }
}

