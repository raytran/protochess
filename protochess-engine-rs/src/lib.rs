#[macro_use]
extern crate lazy_static;
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
use std::collections::HashMap;
use crate::evaluator::Evaluator;
use crate::position::movement_pattern::MovementPattern;
pub use crate::types::PieceType;
pub use crate::types::chess_move::Move;
use crate::searcher::Searcher;
pub use crate::position::movement_pattern::MovementPatternExternal;
use crate::types::bitboard::Bitboard;


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

    pub fn set_bounds(&mut self, width: u8, height: u8, valid_squares:Vec<(u8,u8)>){
        let mut bounds = Bitboard::zero();
        for square in valid_squares {
            bounds.set_bit(to_index(square.0, square.1), true);
        }
        self.current_position.set_bounds(Dimensions{ width, height }, bounds);
    }

    fn each_owner_contains_k(vec: &Vec<(u8,u8,u8,char)>) -> bool{
        let mut num_players:u8 = 0;
        for (owner, x, y, pce) in vec {
            if *owner >= num_players {
                num_players = owner + 1;
            }
        }
        let mut has_k = vec![false; num_players as usize];
        for (owner, x, y, pce_char) in vec {
            if pce_char.to_ascii_lowercase() == 'k' {
                has_k[*owner as usize] = true;
            }
        }
        for k in has_k {
            if !k { return false; }
        }
        true
    }
    pub fn set_state(&mut self, movement_patterns: HashMap<char, MovementPatternExternal>,
                     valid_squares:Vec<(u8, u8)>, pieces: Vec<(u8, u8, u8, char)>){
        assert!(Game::each_owner_contains_k(&pieces));
        let mut width = 0;
        let mut height = 0;
        let mut bounds = Bitboard::zero();
        for sq in valid_squares {
            if sq.0 >= width { width = sq.0 + 1; }
            if sq.1 >= height { height = sq.1 + 1; }
            bounds.set_bit(to_index(sq.0, sq.1), true);
        }

        let pieces =
            pieces.into_iter()
                .map(|(owner, x, y, pce_chr)|
                    (owner, to_index(x, y) as u8, PieceType::from_char(pce_chr)))
                .collect();
        self.current_position = Position::custom(Dimensions{width, height},
                                                 bounds,
                                                 movement_patterns, pieces
        )

    }


    pub fn get_width(&self) -> u8 {
        self.current_position.dimensions.width
    }

    pub fn get_height(&self) -> u8 {
        self.current_position.dimensions.height
    }

    pub fn to_string(&mut self) -> String {
        self.current_position.to_string()
    }

    pub fn get_zobrist(&self) -> u64 {
        self.current_position.get_zobrist()
    }

    /// Performs a move from (x1, y1) to (x2, y2) on the current board position
    pub fn make_move(&mut self, move_generator:&MoveGenerator, x1:u8, y1:u8, x2:u8, y2: u8) -> bool {
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
    pub current_position: Position,
    pub move_generator: MoveGenerator,
    pub(crate) evaluator: Evaluator,
    pub(crate) searcher: Searcher,
}

impl Engine {
    /// Initializes a new engine
    pub fn default() -> Engine{
        let dims = Dimensions{width:8,height:8};
        Engine{
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
    pub fn register_piecetype(&mut self, char_rep:char, mpe: MovementPatternExternal) {
        self.current_position.register_piecetype(char_rep, mpe);
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
        Engine{
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

    ///Calculates and plays the best move found up to a given depth
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

    ///Returns (fromx,fromy,tox,toy)
    pub fn get_best_move(&mut self, depth:u8) -> Option<(u8, u8, u8, u8)> {
        if let Some(best) = self.searcher.get_best_move(&mut self.current_position,
                                                        &mut self.evaluator,
                                                        &self.move_generator,
                                                        depth){
            let (x1, y1) = from_index(best.get_from() as usize);
            let (x2, y2) = from_index(best.get_to() as usize);
            Some((x1, y1, x2, y2))
        }else{
            None
        }
    }

    ///Calculates and plays the best move found
    pub fn play_best_move_timeout(&mut self, max_sec:u64) -> (bool, u8) {
        if let Some((best, depth)) = self.searcher.get_best_move_timeout(&mut self.current_position,
                                                                         &mut self.evaluator,
                                                                         &self.move_generator,
                                                                         max_sec){
            let (x1, y1) = from_index(best.get_from() as usize);
            let (x2, y2) = from_index(best.get_to() as usize);
            (self.make_move(x1, y1, x2, y2), depth)
        }else{
            (false, 0)
        }
    }

    ///Returns ((fromX,fromY,toX,toY), depth)
    pub fn get_best_move_timeout(&mut self, max_sec: u64) -> Option<((u8, u8, u8, u8), u8)> {
        if let Some((best, depth)) = self.searcher.get_best_move_timeout(&mut self.current_position,
                                                                         &mut self.evaluator,
                                                                         &self.move_generator,
                                                                         max_sec){
            let (x1, y1) = from_index(best.get_from() as usize);
            let (x2, y2) = from_index(best.get_to() as usize);
            Some(((x1, y1, x2, y2), depth))
        }else{
            None
        }
    }

    pub fn moves_from(&mut self, x:u8, y:u8) -> Vec<(u8, u8)>{
        let moves = self.move_generator.get_legal_moves_as_tuples(&mut self.current_position);
        let mut possible_moves = Vec::new();
        for (from, to) in moves{
            if from == (x, y){
                possible_moves.push(to);
            }
        }
        possible_moves
    }

    pub fn to_move_in_check(&mut self) -> bool {
        self.move_generator.in_check(&mut self.current_position)
    }

    pub fn set_state(&mut self, movement_patterns: HashMap<char, MovementPatternExternal>,
                     valid_squares:Vec<(u8, u8)>, pieces: Vec<(u8, u8, u8, char)>){
        assert!(Game::each_owner_contains_k(&pieces));
        let mut width = 0;
        let mut height = 0;
        let mut bounds = Bitboard::zero();
        for sq in valid_squares {
            if sq.0 >= width { width = sq.0 + 1; }
            if sq.1 >= height { height = sq.1 + 1; }
            bounds.set_bit(to_index(sq.0, sq.1), true);
        }

        let pieces =
            pieces.into_iter()
                .map(|(owner, x, y, pce_chr)|
                    (owner, to_index(x, y) as u8, PieceType::from_char(pce_chr)))
                .collect();
        self.current_position = Position::custom(Dimensions{width, height},
                                                 bounds,
                                                 movement_patterns, pieces
        )
    }
}

