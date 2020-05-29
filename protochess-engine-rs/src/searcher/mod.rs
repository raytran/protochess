use std::collections::HashMap;
use crate::types::chess_move::Move;
use crate::position::Position;
use crate::move_generator::MoveGenerator;
use std::cmp;
use crate::evaluator::Evaluator;
use crate::types::bitboard::from_index;

//An entry in the transposition table
enum EntryFlag{
    UPPER,
    EXACT,
    LOWER,
}

struct Entry {
    pub flag: EntryFlag,
    pub value: i64,
    pub depth: u8,
}

pub(crate) struct Searcher {
    transposition_table: HashMap<u64, Entry>
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher{
            transposition_table: HashMap::with_capacity(1000)
        }
    }

    pub fn get_best_move(&mut self, evaluator: &mut Evaluator, movegen: &MoveGenerator, position: &mut Position, depth:u8) -> (u8,u8,u8,u8) {
        let mut alpha = i64::MIN + 1;
        let beta = i64::MAX;

        let moves = movegen.get_pseudo_moves(position);
        let mut best_move = (0,0,0,0);
        let mut moves_considered = 0;
        let mut best_value = i64::MIN + 1;
        for move_ in moves  {
            if !movegen.is_move_legal(&move_, position) {
                continue;
            }
            moves_considered += 1;

            position.make_move(move_);

            let value = self.negamax_with_memory(evaluator,
                                                 movegen,
                                                 position,
                                                 depth - 1,
                                                 beta.wrapping_neg(),
                                                 alpha.wrapping_neg()
            ).wrapping_neg();

            position.unmake_move();

            if value >= best_value {
                best_value = value;
                let (x1, y1) = from_index(move_.get_from() as usize);
                let (x2, y2) = from_index(move_.get_to() as usize);
                best_move = (x1, y1, x2, y2);
            }

            alpha = cmp::max(alpha, best_value);
            if alpha >= beta {
                break;
            }
        }
        println!("moves considered: {}, best score: {}", moves_considered, alpha);
        println!("best move: {} {} {} {}", best_move.0, best_move.1, best_move.2, best_move.3);
        best_move
    }
    pub fn negamax_with_memory(&mut self, evaluator: &mut Evaluator, movegen: &MoveGenerator, position: &mut Position, depth:u8, mut alpha:i64, mut beta:i64) -> i64 {
        let alpha_original = alpha;
        if let Some(entry) = self.transposition_table.get(&position.get_zobrist()){
            if entry.depth >= depth {
                match entry.flag {
                    EntryFlag::EXACT => {
                        return entry.value;
                    }
                    EntryFlag::LOWER => {
                        alpha = cmp::max(alpha, entry.value);
                    }
                    EntryFlag::UPPER => {
                        beta = cmp::min(beta, entry.value);
                    }
                }
                if alpha >= beta {
                    return entry.value;
                }
            }
        };


        if depth == 0 {
            return evaluator.evaluate(position, movegen) as i64;
        }


        let moves = movegen.get_pseudo_moves(position);
        let mut value = i64::MIN + 1;
        for move_ in moves {
            if !movegen.is_move_legal(&move_, position) {
                continue;
            }
            position.make_move(move_);
            value = cmp::max(value,
                             self.negamax_with_memory(evaluator,
                                                      movegen,
                                                      position,
                                                      depth - 1,
                                                      beta.wrapping_neg(),
                                                      alpha.wrapping_neg()
                             ).wrapping_neg()
            );

            alpha = cmp::max(alpha, value);
            position.unmake_move();

            if alpha >= beta {
                break;
            }
        }

        let mut new_entry = Entry {
            value,
            depth,
            flag: EntryFlag::UPPER,
        };

        if value <= alpha_original {
            new_entry.flag = EntryFlag::UPPER;
        }else if value >= beta {
            new_entry.flag = EntryFlag::LOWER;
        }else{
            new_entry.flag = EntryFlag::EXACT;
        }
        self.transposition_table.insert(position.get_zobrist(), new_entry);
        value

    }

}