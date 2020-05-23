use crate::types::{Dimensions, bitboard, PieceType};
use crate::types::{LineAttackType, AttackDirection };
use crate::types::bitboard::{Bitboard, to_string, from_index, to_index};
use crate::position::Position;
use crate::position::piece_set::PieceSet;
use crate::move_generator::attack_tables::AttackTables;
use crate::move_generator::bitboard_moves::BitboardMoves;
use std::iter;
use crate::types::chess_move::{Move, MoveType};

mod attack_tables;
mod bitboard_moves;
pub(crate) struct MoveGenerator {
    attack_tables: AttackTables,
}
impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator{
            attack_tables: AttackTables::new(),
        }
    }

    /// Iterator that yields pseudo-legal moves from a position
    pub fn get_pseudo_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        self.get_classical_pseudo_moves(position)
            .chain(self.get_custom_psuedo_moves(position))
    }

    /// Iterator that yields pseudo-legal moves from a position
    /// Considering only the classical piece set
    pub fn get_classical_pseudo_moves(&self, position:&mut Position) -> impl Iterator<Item=Move> {
        let my_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;

        //create a vector of iterators
        let mut iters:Vec<BitboardMoves> = Vec::with_capacity(6);

        let mut apply_to_each = |mut pieceset:Bitboard, func: fn(&AttackTables, u8, &Bitboard, &Bitboard)-> Bitboard| {
            while !pieceset.is_zero() {
                let index = pieceset.lowest_one().unwrap() as u8;
                let mut raw_attacks = func(&self.attack_tables,index, &position.occupied, &enemies);
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

        apply_to_each((&my_pieces.king).to_owned(), AttackTables::get_king_attack);
        apply_to_each((&my_pieces.queen).to_owned(), AttackTables::get_queen_attack);
        apply_to_each((&my_pieces.rook).to_owned(), AttackTables::get_rook_attack);
        apply_to_each((&my_pieces.bishop).to_owned(), AttackTables::get_bishop_attack);
        apply_to_each((&my_pieces.knight).to_owned(), AttackTables::get_knight_attack);

        let mut extra_moves = Vec::new();
        let mut p_copy = (&my_pieces.pawn).to_owned();
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
                let mut attack_only = {
                    if position.whos_turn == 0 {
                        self.attack_tables.get_north_pawn_attack_raw(index) & !(&my_pieces.occupied)
                    } else {
                        self.attack_tables.get_south_pawn_attack_raw(index) & !(&my_pieces.occupied)
                    }
                };
                if attack_only.bit(ep_sq as usize).unwrap() {
                    let (mut cap_x, mut cap_y) = from_index(ep_sq as usize);

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
        if let Some(king_index) = my_pieces.king.lowest_one() {
            let (mut kx, mut ky) = from_index(king_index);
            if position.properties.castling_rights.can_player_castle_kingside(position.whos_turn) {
                let rook_index = to_index(position.dimensions.width - 1,ky) as u8;
                if let Some((owner, pt)) = position.piece_at(rook_index as usize) {
                    if owner == position.whos_turn && pt == PieceType::Rook {
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
                    if owner == position.whos_turn && pt == PieceType::Rook {
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
        let enemies = &position.occupied & !&my_pieces.occupied;

        let mut iters:Vec<BitboardMoves> = Vec::new();
        let mut moves = Vec::new();

        for (c, bb, movement) in &my_pieces.custom {
            let mut bb_copy = bb.to_owned();
            while !bb_copy.is_zero() {
                let index = bb_copy.lowest_one().unwrap() as u8;
                // Sliding moves along ranks or files
                let mut raw_attacks = Bitboard::zero();
                if movement.north || movement.south {
                    raw_attacks |= self.attack_tables.get_file_attack(index, &position.occupied);
                    if !movement.north {
                       raw_attacks &= !self.attack_tables.masks.get_north(index);
                    }else if !movement.south {
                       raw_attacks &= !self.attack_tables.masks.get_south(index);
                    }
                }
                if movement.east || movement.west {
                    raw_attacks |= self.attack_tables.get_rank_attack(index, &position.occupied);
                    if !movement.east {
                        raw_attacks &= !self.attack_tables.masks.get_east(index);
                    }else if !movement.west {
                        raw_attacks &= !self.attack_tables.masks.get_west(index);
                    }
                }
                if movement.northeast || movement.southwest {
                    raw_attacks |= self.attack_tables.get_diagonal_attack(index, &position.occupied);
                    if !movement.northeast {
                        raw_attacks &= !self.attack_tables.masks.get_northeast(index);
                    }else if !movement.southwest {
                        raw_attacks &= !self.attack_tables.masks.get_southwest(index);
                    }
                }
                if movement.northwest || movement.southeast {
                    raw_attacks |= self.attack_tables.get_antidiagonal_attack(index, &position.occupied);
                    if !movement.northwest {
                        raw_attacks &= !self.attack_tables.masks.get_northwest(index);
                    }else if !movement.southeast {
                        raw_attacks &= !self.attack_tables.masks.get_southeast(index);
                    }
                }
                //Do not attack ourselves
                raw_attacks &= !&my_pieces.occupied;
                //Keep only in bounds
                raw_attacks &= &position.bounds;
                iters.push(BitboardMoves::new(
                    (&enemies).to_owned(),
                    raw_attacks,
                    index,
                    movement.promotion_squares.to_owned(),
                    movement.promo_vals.to_owned(),
                ));

                // Delta based moves (sliding, non sliding)
                let (x, y) = from_index(index as usize);
                for (dx, dy) in &movement.move_jump_deltas {
                    let (x2, y2) = (x + *dx, y + *dy);
                    let to = to_index(x2, y2);
                    if position.xy_in_bounds(x2, y2) && !position.occupied.bit(to).unwrap() {
                        moves.push(Move::new(index, to as u8, None, MoveType::Quiet, None));
                    }
                }

                let (x, y) = from_index(index as usize);
                for (dx, dy) in &movement.attack_jump_deltas {
                    let (x2, y2) = (x + *dx, y + *dy);
                    let to = to_index(x2, y2);
                    if enemies.bit(to).unwrap() {
                        moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::Capture, None));
                    }
                }

                let (x, y) = from_index(index as usize);
                for (dx, dy) in &movement.attack_sliding_deltas {
                    let (x2, y2) = (x + *dx, y + *dy);
                    let to = to_index(x2, y2);
                    //Out of bounds, next sliding moves can be ignored
                    if !position.xy_in_bounds(x2, y2) {
                        break;
                    }
                    //If there is an enemy here, we can add an attack move
                    if enemies.bit(to).unwrap() {
                        moves.push(Move::new(index, to as u8, Some(to as u8), MoveType::Capture, None));
                        break;
                    }
                    //Occupied by own team
                    if position.occupied.bit(to).unwrap() {
                        break;
                    }
                }

                let (x, y) = from_index(index as usize);
                for (dx, dy) in &movement.move_sliding_deltas {
                    let (x2, y2) = (x + *dx, y + *dy);
                    let to = to_index(x2, y2);
                    //If the point is out of bounds or there is another piece here, we cannot go any
                    //farther
                    if !position.xy_in_bounds(x2, y2) || position.occupied.bit(to).unwrap() {
                        break;
                    }
                    moves.push(Move::new(index, to as u8, None, MoveType::Quiet, None));
                }

                bb_copy.set_bit(index as usize, false);
            }
        }
        iters.into_iter().flatten().chain(moves.into_iter())
    }

    /// Returns whether or not a player is in check for a given position
    pub fn is_in_check(&self, position: &Position, my_player_num: u8) -> bool {
        let my_pieces: &PieceSet = &position.pieces[my_player_num as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;

        //Calculate enemies piece sets
        let enemy_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        //TODO generalize for >2 players
        let enemy_pawns = &enemy_pieces.pawn;
        let enemy_knights = &enemy_pieces.knight;
        let enemy_bishops = &enemy_pieces.bishop;
        let enemy_queens = &enemy_pieces.queen;
        let enemy_rooks = &enemy_pieces.rook;
        let enemy_kings = &enemy_pieces.king;

        let loc_index = my_pieces.king.lowest_one().unwrap() as u8;

        //Pawn
        let patt = {
            if my_player_num == 0 {
                self.attack_tables.get_north_pawn_attack_masked(loc_index, &position.occupied, &enemies)
            }else{
                self.attack_tables.get_south_pawn_attack_masked(loc_index, &position.occupied, &enemies)
            }
        };

        if !(patt & enemy_pawns).is_zero() {
            return true;
        };

        //Knight
        let natt = self.attack_tables.get_knight_attack(loc_index, &position.occupied, &enemies);
        if !(natt & enemy_knights).is_zero() {
            return true;
        };
        //King
        let katt = self.attack_tables.get_king_attack(loc_index, &position.occupied, &enemies);
        if !(katt & enemy_kings).is_zero() {
            return true;
        };

        //Rook & Queen
        let ratt = self.attack_tables.get_rook_attack(loc_index, &position.occupied, &enemies);
        if (!(&ratt & enemy_queens).is_zero() || !(&ratt & enemy_rooks).is_zero()){
            return true;
        };
        //Bishop & Queen
        let batt = self.attack_tables.get_bishop_attack(loc_index, &position.occupied, &enemies);
        if (!(&batt & enemy_queens).is_zero() || !(&batt & enemy_bishops).is_zero()) {
            return true;
        };

        false
    }

    ///Checks if a move is legal
    pub fn is_move_legal(&self, move_:&Move, position:&mut Position) -> bool{
        //You cannot capture kings
        if move_.get_move_type() == MoveType::PromotionCapture || move_.get_move_type() == MoveType::Capture {
            if position.piece_at(move_.get_target() as usize).unwrap().1 == PieceType::King {
                return false;
            }
        }
        let my_player_num = position.whos_turn;
        let mut legality = true;
        position.make_move(move_.to_owned());
        if self.is_in_check(position, my_player_num) {
            legality = false;
        }
        position.unmake_move();
        legality
    }
}

