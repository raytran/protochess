use crate::types::{Dimensions, bitboard};
pub use crate::position::Position;
pub use crate::move_generator::MoveGenerator;
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
mod transposition_table;

use crate::evaluator::Evaluator;
pub use crate::position::movement_pattern::MovementPattern;
pub use crate::types::PieceType;
pub use crate::types::chess_move::Move;
use crate::searcher::Searcher;

/// Simple game without an AI engine
pub struct Game {
    pub current_position: Position,
}

impl Game {
    pub fn default() -> Game {
        Game {
            current_position: Position::default(),
        }
    }

    pub fn to_string(&mut self) -> String {
        self.current_position.to_string()
    }

    pub fn get_zobrist(&self) -> u64 {
        self.current_position.get_zobrist()
    }

    /// Performs a move from (x1, y1) to (x2, y2) on the current board position
    pub fn make_move(&mut self, move_generator:&MoveGenerator, x1:u8, y1:u8, x2:u8, y2: u8) -> bool{
        let from = bitboard::to_index(x1,y1) as u8;
        let to = bitboard::to_index(x2,y2) as u8;

        let moves = move_generator.get_pseudo_moves(&mut self.current_position);
        for move_ in moves {
            if !move_generator.is_move_legal(&move_, &mut self.current_position) {
                continue;
            }
            if move_.get_from() == from && move_.get_to() == to {
                self.current_position.make_move(move_);
                return true
            }
        }
        false
    }

    /// Undoes the most recent move on the current board position
    pub fn undo(&mut self) {
        self.current_position.unmake_move();
    }

    pub fn get_whos_turn(&self) -> u8 {
        self.current_position.whos_turn
    }

}


/// Starting point for the engine
pub struct Engine {
    pub(crate) dimensions: Dimensions,
    pub(crate) current_position: Position,
    pub(crate) move_generator: MoveGenerator,
    pub(crate) evaluator: Evaluator,
    pub(crate) searcher: Searcher,
}

impl Engine {
    /// Initializes a new engine
    pub fn default() -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
            dimensions:dims,
            move_generator: MoveGenerator::new(),
            current_position: Position::default(),
            evaluator: Evaluator::new(),
            searcher: Searcher::new(),
        }
    }

    /// Returns the zobrist hash key for the current position
    pub fn get_zobrist(&self) -> u64 {
        self.current_position.get_zobrist()
    }

    /// Returns the score of the current position for the side to move
    pub fn get_score(&mut self) -> isize{
        self.evaluator.evaluate(&mut self.current_position, &self.move_generator)
    }

    /// Registers a custom piecetype for the current position
    pub fn register_piecetype(&mut self, char_rep:char, mp: MovementPattern) {
        self.current_position.register_piecetype(char_rep, mp);
    }

    /// Adds a new piece on the board
    pub fn add_piece(&mut self, owner:usize, piece_type:PieceType, x: u8, y:u8) {
        self.current_position.add_piece(0, PieceType::Custom('a'), to_index(x,y) as u8);
    }

    /// Removes a piece on the board, if it exists
    pub fn remove_piece(&mut self, index:u8) {
        self.current_position.remove_piece(index);
    }

    /// Performs a move from (x1, y1) to (x2, y2) on the current board position
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

    /// Undoes the most recent move on the current board position
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

    /// Returns the number of possible moves from a board position up to a given depth
    /// See https://www.chessprogramming.org/Perft
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

    /// Like perft, but prints the moves at the first ply
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

    ///Calculates and plays the best move found
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

