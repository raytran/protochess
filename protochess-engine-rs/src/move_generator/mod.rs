use crate::types::{PieceType};

use crate::types::bitboard::{Bitboard, from_index, to_index};
use crate::position::Position;
use crate::position::piece_set::PieceSet;
use crate::move_generator::attack_tables::AttackTables;
use crate::move_generator::bitboard_moves::BitboardMoves;

use crate::types::chess_move::{Move, MoveType};
use crate::position::piece::Piece;

mod attack_tables;
mod bitboard_moves;
pub struct MoveGenerator {
    pub attack_tables: AttackTables,
}
impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator{
            attack_tables: AttackTables::new(),
        }
    }
    pub fn get_legal_moves_as_tuples(&self, position: &mut Position) -> Vec<((u8,u8), (u8,u8))> {
        let mut legal_tuples = Vec::new();
        for move_ in self.get_pseudo_moves(position){
            if !self.is_move_legal(&move_, position){
                continue;
            }
            legal_tuples.push((from_index(move_.get_from() as usize),
                               from_index(move_.get_to() as usize)));
        }
        legal_tuples
    }

    /// Iterator that yields pseudo-legal moves from a position
    pub fn get_pseudo_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        self.get_classical_pseudo_moves(position)
            .chain(self.get_custom_psuedo_moves(position))
    }

    ///Iterator that yields only capture moves
    pub fn get_capture_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        self.get_classical_pseudo_moves(position).filter(|x| x.get_is_capture())
            .chain(self.get_custom_psuedo_moves(position).filter(|x| x.get_is_capture()))
    }

    /// Iterator that yields pseudo-legal moves from a position
    /// Considering only the classical piece set
    pub fn get_classical_pseudo_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        let my_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;

        //create a vector of iterators
        let mut iters:Vec<BitboardMoves> = Vec::with_capacity(6);
        let occ_or_not_in_bounds = &position.occupied | !&position.bounds;

        let mut apply_to_each = |mut pieceset:Bitboard, func: fn(&AttackTables, u8, &Bitboard, &Bitboard)-> Bitboard| {
            while !pieceset.is_zero() {
                let index = pieceset.lowest_one().unwrap() as u8;
                let mut raw_attacks = func(&self.attack_tables,index, &occ_or_not_in_bounds, &enemies);
                //Do not attack ourselves
                raw_attacks &= !&my_pieces.occupied;
                //Keep only in bounds
                raw_attacks &= &position.bounds;
                iters.push(BitboardMoves::new(
                    (&enemies).to_owned(),
                    raw_attacks,
                    index,
                    None,
                    None,
                ));
                pieceset.set_bit(index as usize, false);
            }
        };

        apply_to_each((&my_pieces.king.bitboard).to_owned(), AttackTables::get_king_attack);
        apply_to_each((&my_pieces.queen.bitboard).to_owned(), AttackTables::get_queen_attack);
        apply_to_each((&my_pieces.rook.bitboard).to_owned(), AttackTables::get_rook_attack);
        apply_to_each((&my_pieces.bishop.bitboard).to_owned(), AttackTables::get_bishop_attack);
        apply_to_each((&my_pieces.knight.bitboard).to_owned(), AttackTables::get_knight_attack);

        let mut extra_moves = Vec::new();
        let mut p_copy = (&my_pieces.pawn.bitboard).to_owned();
        while !p_copy.is_zero() {
            let index = p_copy.lowest_one().unwrap() as u8;
            let mut raw_attacks = {
                if position.whos_turn == 0 {
                    self.attack_tables.get_north_pawn_attack(index, &position.occupied, &enemies)
                } else {
                    self.attack_tables.get_south_pawn_attack(index, &position.occupied, &enemies)
                }
            };
            //Do not attack ourselves
            raw_attacks &= !&my_pieces.occupied;
            //Keep only in bounds
            raw_attacks &= &position.bounds;
            let promotion_squares = {
                if position.whos_turn == 0 {
                    Some(self.attack_tables.masks.get_rank(position.dimensions.height - 1).to_owned())
                } else {
                    Some(self.attack_tables.masks.get_rank(0).to_owned())
                }
            };
            let promo_vals = Some(vec!['r', 'b', 'n', 'q']);
            iters.push(BitboardMoves::new(
                (&enemies).to_owned(),
                raw_attacks,
                index,
                promotion_squares,
                promo_vals
            ));
            //Check EP
            if let Some(ep_sq) = position.properties.ep_square {
                let attack_only = {
                    if position.whos_turn == 0 {
                        self.attack_tables.get_north_pawn_attack_raw(index) & !(&my_pieces.occupied)
                    } else {
                        self.attack_tables.get_south_pawn_attack_raw(index) & !(&my_pieces.occupied)
                    }
                };
                if attack_only.bit(ep_sq as usize).unwrap() {
                    let (cap_x, mut cap_y) = from_index(ep_sq as usize);

                    if position.whos_turn == 0 {
                        cap_y -= 1;
                    } else {
                        cap_y += 1;
                    }
                    let move_ = Move::new(index, ep_sq,  Some(to_index(cap_x,cap_y) as u8), MoveType::Capture, None);
                    extra_moves.push(move_);
                }
            }
            p_copy.set_bit(index as usize, false);
        }
        //Castling
        if let Some(king_index) = my_pieces.king.bitboard.lowest_one() {
            let (kx, ky) = from_index(king_index);
            let whos_turn = position.whos_turn;
            if position.properties.castling_rights.can_player_castle_kingside(position.whos_turn) {
                let rook_index = to_index(position.dimensions.width - 1,ky) as u8;
                if let Some((owner, pt )) = position.piece_at(rook_index as usize) {
                    if owner == whos_turn && pt.piece_type == PieceType::Rook {
                        //See if the space between is clear
                        let east = self.attack_tables.masks.get_east(king_index as u8);
                        let mut occ = east & &position.occupied;
                        occ.set_bit(rook_index as usize, false);
                        if occ.is_zero() {
                            //See if we can move the king one step east without stepping into check
                            let king_one_step_indx = to_index(kx + 1, ky) as u8;
                            if self.is_move_legal(&Move::null(), position)
                                && self.is_move_legal(
                                &Move::new(king_index as u8, king_one_step_indx, None, MoveType::Quiet, None),
                                position
                            ){
                                let to_index = to_index(kx + 2, ky) as u8;
                                extra_moves.push(Move::new(king_index as u8,
                                                           to_index,
                                                           Some(rook_index),
                                                           MoveType::KingsideCastle,
                                                           None));
                            }
                        }
                    }
                }
            }
            if position.properties.castling_rights.can_player_castle_queenside(position.whos_turn) {
                let rook_index = to_index(0 ,ky) as u8;
                if let Some((owner, pt)) = position.piece_at(rook_index as usize) {
                    if owner == whos_turn && pt.piece_type == PieceType::Rook {
                        let west = self.attack_tables.masks.get_west(king_index as u8);
                        let mut occ = west & &position.occupied;
                        occ.set_bit(rook_index as usize, false);

                        if occ.is_zero() {
                            //See if we can move the king one step east without stepping into check
                            let king_one_step_indx = to_index(kx - 1, ky) as u8;
                            if self.is_move_legal(&Move::null(), position)
                                && self.is_move_legal(
                                &Move::new(king_index as u8, king_one_step_indx, None, MoveType::Quiet, None),
                                position
                            ){
                                let to_index = to_index(kx - 2, ky) as u8;
                                extra_moves.push(Move::new(king_index as u8,
                                                           to_index,
                                                           Some(rook_index),
                                                           MoveType::QueensideCastle,
                                                           None));
                            }
                        }
                    }
                }
            }
        }


        //Flatten our vector of iterators and combine with ep moves
        iters.into_iter().flatten().chain(extra_moves.into_iter())
    }

    /// Iterator that yields pseudo-legal moves from a positon
    /// Considering only custom piece types
    fn get_custom_psuedo_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        let my_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];

        let mut iters:Vec<BitboardMoves> = Vec::new();
        let mut moves = Vec::new();

        //Return early if there are no custom pieces
        if my_pieces.custom.len() == 0 {
            return iters.into_iter().flatten().chain(moves.into_iter());
        }

        let enemies = &position.occupied & !&my_pieces.occupied;
        let occ_or_not_in_bounds = &position.occupied | !&position.bounds;

        for p in &my_pieces.custom {
            //let movement = p.movement_pattern.as_ref().unwrap();
            let movement = {
                if let Some(mp) = position.get_movement_pattern(&p.piece_type){
                    mp
                }else{
                    continue;
                }
            };

            let bb = &p.bitboard;
            let mut bb_copy = bb.to_owned();
            while !bb_copy.is_zero() {
                let index = bb_copy.lowest_one().unwrap() as u8;
                // Sliding moves along ranks or files
                //Attacks!
                let mut raw_attacks = self.attack_tables.get_sliding_moves_bb(
                    index,
                    &occ_or_not_in_bounds,
                    movement.attack_north,
                    movement.attack_east,
                   movement.attack_south,
                    movement.attack_west,
                    movement.attack_northeast,
                    movement.attack_northwest,
                    movement.attack_southeast,
                    movement.attack_southwest
                );
                //Attacks ONLY
                raw_attacks &= &enemies;
                //Keep only in bounds
                raw_attacks &= &position.bounds;
                iters.push(BitboardMoves::new(
                    (&enemies).to_owned(),
                    raw_attacks,
                    index,
                    movement.promotion_squares.to_owned(),
                    movement.promo_vals.to_owned(),
                ));
                //Movements!
                let mut raw_moves = self.attack_tables.get_sliding_moves_bb(index,
                                                                            &occ_or_not_in_bounds,
                                                                            movement.translate_north,
                                                                            movement.translate_east,
                                                                            movement.translate_south,
                                                                            movement.translate_west,
                                                                            movement.translate_northeast,
                                                                            movement.translate_northwest,
                                                                            movement.translate_southeast,
                                                                            movement.translate_southwest
                );
                //Non-attacks ONLY
                raw_moves &= !&position.occupied;
                //Keep only in bounds
                raw_moves &= &position.bounds;
                iters.push(BitboardMoves::new(
                    (&enemies).to_owned(),
                    raw_moves,
                    index,
                    movement.promotion_squares.to_owned(),
                    movement.promo_vals.to_owned(),
                ));


                // Delta based moves (sliding, non sliding)
                let (x, y) = from_index(index as usize);
                for (dx, dy) in &movement.translate_jump_deltas {
                    let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                    if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                        continue;
                    }
                    let to = to_index(x2 as u8, y2 as u8);
                    if position.xy_in_bounds(x2 as u8, y2 as u8) && !position.occupied.bit(to).unwrap() {
                        //Promotion here?
                        if movement.promotion_at(to){
                            //Add all the promotion moves
                            for c in movement.promo_vals.as_ref().unwrap() {
                                moves.push(Move::new(index, to as u8, None, MoveType::Promotion, Some(*c)));
                            }
                        }else{
                            moves.push(Move::new(index, to as u8, None, MoveType::Quiet, None));
                        }
                    }
                }

                for (dx, dy) in &movement.attack_jump_deltas {

                    let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                    if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                        continue;
                    }
                    let to = to_index(x2 as u8, y2 as u8);
                    if enemies.bit(to).unwrap() {
                        //Promotion here?
                        if movement.promotion_at(to) {
                            //Add all the promotion moves
                            for c in movement.promo_vals.as_ref().unwrap() {
                                moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::PromotionCapture, Some(*c)));
                            }
                        }else{
                            moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::Capture, None));
                        }
                    }
                }

                for run in &movement.attack_sliding_deltas {
                    for (dx, dy) in run {

                        let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                        if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                            break;
                        }

                        let to = to_index(x2 as u8, y2 as u8);
                        //Out of bounds, next sliding moves can be ignored
                        if !position.xy_in_bounds(x2 as u8, y2 as u8) {
                            break;
                        }
                        //If there is an enemy here, we can add an attack move
                        if enemies.bit(to).unwrap() {
                            if movement.promotion_at(to) {
                                //Add all the promotion moves
                                for c in movement.promo_vals.as_ref().unwrap() {
                                    moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::PromotionCapture, Some(*c)));
                                }
                            }else{
                                moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::Capture, None));
                            }
                            break;
                        }
                        //Occupied by own team
                        if position.occupied.bit(to).unwrap() {
                            break;
                        }
                    }
                }


                for run in &movement.translate_sliding_deltas {
                    for (dx, dy) in run {
                        let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                        if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                            break;
                        }
                        let to = to_index(x2 as u8, y2 as u8);
                        //If the point is out of bounds or there is another piece here, we cannot go any
                        //farther
                        if !position.xy_in_bounds(x2 as u8, y2 as u8) || position.occupied.bit(to).unwrap() {
                            break;
                        }
                        if movement.promotion_at(to) {
                            //Add all the promotion moves
                            for c in movement.promo_vals.as_ref().unwrap() {
                                moves.push(Move::new(index, to as u8, None, MoveType::Quiet, Some(*c)));
                            }
                        }else {
                            moves.push(Move::new(index, to as u8, None, MoveType::Quiet, None));
                        }
                    }
                }

                bb_copy.set_bit(index as usize, false);
            }
        }
        iters.into_iter().flatten().chain(moves.into_iter())
    }

    /// Returns the number of moves of a piecetype on an otherwise empty board
    /// Useful for evaluation
    pub fn get_num_moves_on_empty_board(&self, index:u8, position:&Position, piece:&Piece, bounds: &Bitboard) -> u32 {
        let (x, y) = from_index(index as usize);
        if !position.xy_in_bounds(x, y) {
           return 0;
        }
        let zero = Bitboard::zero();
        let not_in_bounds = !&position.bounds;
        let mut moves = match piece.piece_type {
            PieceType::Queen => {self.attack_tables.get_queen_attack(index, &not_in_bounds, &zero)}
            PieceType::Bishop => {self.attack_tables.get_bishop_attack(index, &not_in_bounds, &zero)}
            PieceType::Rook => {self.attack_tables.get_rook_attack(index, &not_in_bounds, &zero)}
            PieceType::Knight => {self.attack_tables.get_knight_attack(index, &not_in_bounds, &zero)}
            PieceType::King => {self.attack_tables.get_king_attack(index, &not_in_bounds, &zero)}
            PieceType::Pawn => {self.attack_tables.get_north_pawn_attack(index, &not_in_bounds, &zero)}
            PieceType::Custom(_c) => {
                let mp = {
                    if let Some(mp) = position.get_movement_pattern(&piece.piece_type){
                        mp
                    }else{
                        return 0;
                    }
                };

                let mut slides = self.attack_tables.get_sliding_moves_bb(
                    index,
                    &not_in_bounds,
                    mp.translate_north || mp.attack_north,
                    mp.translate_east || mp.attack_east,
                    mp.translate_south || mp.attack_south,
                    mp.translate_west || mp.attack_west,
                    mp.translate_northeast || mp.attack_northeast,
                    mp.translate_northwest || mp.attack_northwest,
                    mp.translate_southeast || mp.attack_southeast,
                    mp.translate_southwest || mp.attack_southwest
                );

                // Delta based moves (sliding, non sliding)
                let (x, y) = from_index(index as usize);
                for (dx, dy) in mp.translate_jump_deltas.iter().chain(mp.attack_jump_deltas.iter()) {
                    let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                    if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                        continue;
                    }

                    let to = to_index(x2 as u8, y2 as u8);
                    if bounds.bit(to).unwrap() {
                        slides.set_bit(to, true);
                    }
                }
                for run in mp.attack_sliding_deltas.iter().chain(mp.translate_sliding_deltas.iter()) {
                    for (dx, dy) in run {
                        let (x2, y2) = (x as i8 + *dx, y as i8 + *dy);
                        if x2 < 0 || y2 < 0 || x2 > 15 || y2 > 15 {
                            break;
                        }
                        let to = to_index(x2 as u8, y2 as u8);
                        //Out of bounds, next sliding moves can be ignored
                        if !bounds.bit(to).unwrap() {
                            break;
                        }
                        slides.set_bit(to, true);
                    }
                }
                slides
            }
        };
        //Keep only in bounds
        moves &= bounds;
        moves.count_ones()
    }

    /// Returns whether or not a player is in check for a given position
    fn is_in_check_from_king(&self, position: &Position, my_player_num: u8) -> bool {
        let my_pieces: &PieceSet = &position.pieces[my_player_num as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;
        let occ_or_not_in_bounds = &position.occupied | !&position.bounds;

        //Calculate enemies piece sets
        let enemy_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        //TODO generalize for >2 players
        let enemy_pawns = &enemy_pieces.pawn.bitboard;
        let enemy_knights = &enemy_pieces.knight.bitboard;
        let enemy_bishops = &enemy_pieces.bishop.bitboard;
        let enemy_queens = &enemy_pieces.queen.bitboard;
        let enemy_rooks = &enemy_pieces.rook.bitboard;
        let enemy_kings = &enemy_pieces.king.bitboard;

        let loc_index = my_pieces.king.bitboard.lowest_one().unwrap() as u8;

        //Pawn
        let patt = {
            if my_player_num == 0 {
                self.attack_tables.get_north_pawn_attack_masked(loc_index, &occ_or_not_in_bounds, &enemies)
            }else{
                self.attack_tables.get_south_pawn_attack_masked(loc_index, &occ_or_not_in_bounds, &enemies)
            }
        };

        if !(patt & enemy_pawns).is_zero() {
            return true;
        };

        //Knight
        let natt = self.attack_tables.get_knight_attack(loc_index, &occ_or_not_in_bounds, &enemies);
        if !(natt & enemy_knights).is_zero() {
            return true;
        };
        //King
        let katt = self.attack_tables.get_king_attack(loc_index, &occ_or_not_in_bounds, &enemies);
        if !(katt & enemy_kings).is_zero() {
            return true;
        };

        //Rook & Queen
        let ratt = self.attack_tables.get_rook_attack(loc_index, &occ_or_not_in_bounds, &enemies);
        if !(&ratt & enemy_queens).is_zero() || !(&ratt & enemy_rooks).is_zero() {
            return true;
        };
        //Bishop & Queen
        let batt = self.attack_tables.get_bishop_attack(loc_index, &occ_or_not_in_bounds, &enemies);
        if !(&batt & enemy_queens).is_zero() || !(&batt & enemy_bishops).is_zero() {
            return true;
        };

        false
    }

    ///Checks if the player to move is in check
    pub fn in_check(&self, position:&mut Position) -> bool {
        let my_player_num = position.whos_turn;
        let mut in_check = false;
        position.make_move(Move::null());
        if self.is_in_check_from_king(position, my_player_num) {
            in_check = true;
        }
        //Custom pieces
        for move_ in self.get_custom_psuedo_moves(position)  {
            if move_.get_is_capture() && position.piece_at(move_.get_target() as usize).unwrap().1.piece_type == PieceType::King {
                in_check = true;
                break;
            }
        }
        position.unmake_move();
        in_check
    }

    ///Checks if a move is legal
    pub fn is_move_legal(&self, move_:&Move, position:&mut Position) -> bool{
        //You cannot capture kings
        if move_.get_move_type() == MoveType::PromotionCapture || move_.get_move_type() == MoveType::Capture {
            if position.piece_at(move_.get_target() as usize).unwrap().1.piece_type == PieceType::King {
                return false;
            }
        }
        let my_player_num = position.whos_turn;
        let mut legality = true;
        position.make_move(move_.to_owned());
        if self.is_in_check_from_king(position, my_player_num) {
            legality = false;
        }
        //Custom pieces
        for move_ in self.get_custom_psuedo_moves(position)  {
            if move_.get_is_capture() && position.piece_at(move_.get_target() as usize).unwrap().1.piece_type == PieceType::King {
                legality = false;
                break;
            }
        }
        position.unmake_move();
        legality
    }


    ///Returns the number of legal moves for a position
    pub fn count_legal_moves(&self, position: &mut Position) -> u64{
        let mut nodes = 0u64;
        for move_ in self.get_pseudo_moves(position){
            if !self.is_move_legal(&move_, position) {
                continue;
            }
            nodes += 1;
        }
        nodes
    }
}


#[cfg(test)]
mod eval_test {
    use crate::position::Position;
    use crate::move_generator::MoveGenerator;

    #[test]
    fn capture_moves(){
        let mut pos = Position::from_fen("rnb1kbnr/ppppqppp/8/8/5P2/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1".parse().unwrap());
        let movegen = MoveGenerator::new();
        println!("{}",pos.get_zobrist());
        println!("{}", movegen.in_check(&mut pos));
        println!("{}",pos.get_zobrist());
        for move_ in movegen.get_capture_moves(&mut pos){
            println!("{}", move_);
            assert!(move_.get_is_capture());
        }
    }
}
