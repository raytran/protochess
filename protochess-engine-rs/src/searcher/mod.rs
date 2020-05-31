use std::collections::HashMap;
use crate::types::chess_move::Move;
use crate::position::Position;
use crate::move_generator::MoveGenerator;
use std::cmp;
use crate::evaluator::Evaluator;
use crate::types::bitboard::from_index;
use crate::Engine;

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
    //transposition_table: HashMap<u64, Entry>
    transposition_table: HashMap<u64, Move, ahash::RandomState>,
    //We store two killer moves per ply,
    //indexed by killer_moves[depth][0] or killer_moves[depth][0]
    killer_moves: [[Move;2];64],
    //Indexed by history_moves[side2move][from][to]
    history_moves: [[usize;256];256],

    //Stats
    //Counter for the number of nodes searched
    nodes_searched: usize,
    nodes_fail_high_first:usize,
    nodes_fail_high: usize
}

impl Searcher {
    pub fn new() -> Searcher {
        let hasher = ahash::RandomState::new();
        Searcher{
            transposition_table: HashMap::with_capacity_and_hasher(1000, hasher),
            killer_moves: [[Move::null(); 2];64],
            history_moves: [[0;256];256],
            nodes_searched: 0,
            nodes_fail_high: 0,
            nodes_fail_high_first: 0
        }
    }


    pub fn get_best_move(&mut self, position: &mut Position, eval: &mut Evaluator, movegen: &MoveGenerator, depth: u8) -> Option<Move> {
        //Iterative deepening
        self.clear_heuristics();
        for d in 1..=depth {
            let best_score = self.alphabeta(position, eval, movegen, d, -isize::MAX, isize::MAX, true);
            //Print PV info
            let ordering_percentage:f64 = if self.nodes_fail_high != 0 { (self.nodes_fail_high_first as f64) / (self.nodes_fail_high as f64) } else { 0.0 };
            println!("score:{} depth: {}, nodes: {}, ordering: {}", best_score, d, self.nodes_searched, ordering_percentage);

            self.clear_search_stats();
            let mut moves_made = 0;
            while let Some(best_move) = self.transposition_table.get(&position.get_zobrist()) {
                print!("{}-", best_move);
                position.make_move(best_move.to_owned());
                moves_made += 1;
                if moves_made == d {
                    break;
                }
            }
            for _ in 0..moves_made {
                position.unmake_move();
            }
            print!("\n");

        }

        match self.transposition_table.get(&position.get_zobrist()){
            Some(entry) => {Some(entry.to_owned())}
            None => None
        }
    }

    fn alphabeta(&mut self, position: &mut Position, eval: &mut Evaluator, movegen: &MoveGenerator,
                     depth: u8, mut alpha: isize, mut beta: isize, do_null: bool) -> isize {
        self.nodes_searched += 1;
        if depth == 0 {
            return self.quiesce(position, eval, movegen, depth, alpha, beta);
            //return eval.evaluate(position, movegen);
        }

        let mut best_move = Move::null();
        let mut num_legal_moves = 0;
        let old_alpha = alpha;

        //Null move pruning
        if do_null {
            if depth > 3 && eval.can_do_null_move(position) && !movegen.in_check(position) {
                position.make_move(Move::null());
                let nscore = -self.alphabeta(position,eval, movegen,
                                             depth - 3, -beta, -beta + 1, false);
                position.unmake_move();
                if nscore >= beta {
                    return beta;
                }
            }
        }


        let mut moves_and_score:Vec<(usize, Move)> = movegen.get_pseudo_moves(position)
            .map(|mv| {
                (eval.score_move(depth,&self.history_moves,&self.killer_moves, position, &mv), mv)
            }).collect();

        //Assign PV move score to usize::MAX
        if let Some(best_move) = self.transposition_table.get(&position.get_zobrist()) {
            for i in 0..moves_and_score.len(){
                if moves_and_score[i].1 == *best_move {
                    moves_and_score[i] = (usize::MAX, moves_and_score[i].1);
                    break;
                }
            }
        }

        //for (score, move_) in moves_and_score {
        for i in 0..moves_and_score.len() {
            //Pick the best move
            Searcher::sort_moves(i, &mut moves_and_score);
            let move_ = moves_and_score[i].1;

            if !movegen.is_move_legal(&move_, position) {
                continue;
            }

            num_legal_moves += 1;
            position.make_move((&move_).to_owned());
            let score = -self.alphabeta(position, eval, movegen,
                                       depth - 1, -beta, -alpha, true);
            position.unmake_move();

            if score >= beta {
                if num_legal_moves == 1 {
                    self.nodes_fail_high_first += 1;
                }
                self.nodes_fail_high += 1;
                //Record new killer moves
                if !move_.get_is_capture() {
                    //Make sure we're only recording new moves
                    if move_ != self.killer_moves[depth as usize][0]
                        && move_ != self.killer_moves[depth as usize][1] {
                        self.killer_moves[depth as usize][1] = self.killer_moves[depth as usize][0];
                        self.killer_moves[depth as usize][0] = move_;
                    }
                }
                return beta;
            }
            if score > alpha {
                alpha = score;
                best_move = move_;

                //History heuristic
                if !move_.get_is_capture() {
                    self.history_moves
                        [move_.get_from() as usize]
                        [move_.get_to() as usize] += depth as usize;
                }
            }
        }
        if num_legal_moves == 0 {
            return -99999;
        }

        if alpha != old_alpha {
            //Alpha improvement, record PV
            self.transposition_table.insert(position.get_zobrist(), (&best_move).to_owned());
        }
        alpha
    }


    fn quiesce(&mut self, position: &mut Position, eval: &mut Evaluator, movegen: &MoveGenerator,
                 depth:u8, mut alpha: isize, mut beta: isize) -> isize {
        self.nodes_searched += 1;
        let mut score = eval.evaluate(position, movegen);
        if score >= beta{
            return beta;
        }
        if score > alpha {
            alpha = score;
        }

        let mut best_move = Move::null();
        let mut num_legal_moves = 0;
        let old_alpha = alpha;

        let mut moves_and_score:Vec<(usize, Move)> = movegen.get_capture_moves(position)
            .map(|mv| {
                (eval.score_move(depth, &self.history_moves, &self.killer_moves, position, &mv), mv)
            }).collect();

        //Assign PV move score to usize::MAX
        if let Some(best_move) = self.transposition_table.get(&position.get_zobrist()) {
            for i in 0..moves_and_score.len(){
                if moves_and_score[i].1 == *best_move {
                    moves_and_score[i] = (usize::MAX, moves_and_score[i].1);
                    break;
                }
            }
        }

        //for (score, move_) in moves_and_score {
        for i in 0..moves_and_score.len() {
            //Pick the best move
            Searcher::sort_moves(i, &mut moves_and_score);
            let move_ = moves_and_score[i].1;

            if !movegen.is_move_legal(&move_, position) {
                continue;
            }

            num_legal_moves += 1;
            position.make_move((&move_).to_owned());
            let score = -self.quiesce(position, eval, movegen,
                                         depth, -beta, -alpha);
            position.unmake_move();

            if score >= beta {
                if num_legal_moves == 1 {
                    self.nodes_fail_high_first += 1;
                }
                self.nodes_fail_high += 1;
                return beta;
            }
            if score > alpha {
                alpha = score;
                best_move = move_;
            }
        }

        if alpha != old_alpha {
            //Alpha improvement, record PV
            self.transposition_table.insert(position.get_zobrist(), (&best_move).to_owned());
        }

        alpha
    }

    //Selection sort, swapping at moves_seen with the next best move from moves[current_index:]
    fn sort_moves(current_index:usize, moves:&mut Vec<(usize, Move)>) {
        let mut best_score = 0;
        let mut best_score_index = current_index;
        for i in current_index..moves.len(){
            let score = moves[i].0;
            if score >= best_score {
                best_score = score;
                best_score_index = i;
            }
        }
        if current_index != best_score_index {
            moves.swap(current_index, best_score_index);
        }
    }

    //Resets heuristics
    fn clear_heuristics(&mut self) {
        for i in 0..self.killer_moves.len() {
            for j in 0..self.killer_moves[i].len() {
                self.killer_moves[i][j] = Move::null();
            }
        }
        for i in 0..self.history_moves.len() {
            for j in 0..self.history_moves[i].len() {
                    self.history_moves[i][j] = 0;
            }
        }
    }

    fn clear_search_stats(&mut self) {
        self.nodes_searched = 0;
        self.nodes_fail_high_first = 0;
        self.nodes_fail_high = 0;
    }








    /*
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
     */

}