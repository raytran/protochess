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
//Iterator that yields pseudo-legal moves from a position
pub(crate) struct MoveGenerator {
    attack_tables: AttackTables,
}
impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator{
            attack_tables: AttackTables::new(),
        }
    }

    pub fn get_psuedo_moves(&self, position:&Position) -> impl Iterator<Item=Move> {
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
                iters.push(BitboardMoves{
                    enemies: (&enemies).to_owned(),
                    moves: raw_attacks,
                    source_index: index,
                });
                pieceset.set_bit(index as usize, false);
            }
        };

        apply_to_each((&my_pieces.king).to_owned(), AttackTables::get_king_attack);
        apply_to_each((&my_pieces.queen).to_owned(), AttackTables::get_queen_attack);
        apply_to_each((&my_pieces.rook).to_owned(), AttackTables::get_rook_attack);
        apply_to_each((&my_pieces.bishop).to_owned(), AttackTables::get_bishop_attack);
        apply_to_each((&my_pieces.knight).to_owned(), AttackTables::get_knight_attack);

        let mut ep_moves = Vec::new();
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
            iters.push(BitboardMoves{
                enemies: (&enemies).to_owned(),
                moves: raw_attacks,
                source_index: index,
            });
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
                    let move_ = Move::new(index, ep_sq,  to_index(cap_x,cap_y) as u8, MoveType::CAPTURE);
                    ep_moves.push(move_);
                }
            }
            p_copy.set_bit(index as usize, false);
        }

        //Castling
        if let Some(king_index) = my_pieces.king.lowest_one() {
            if position.properties.castling_rights.can_player_castle_kingside(position.whos_turn) {
            }
            if position.properties.castling_rights.can_player_castle_queenside(position.whos_turn) {
            }
        }
        //Flatten our vector of iterators and combine with ep moves
        iters.into_iter().flatten().chain(ep_moves.into_iter())
    }

    //Checks if a move is legal
    pub fn is_move_legal(&self, move_:&Move, position:&mut Position) -> bool{
        let my_player_num = position.whos_turn;
        position.make_move(move_.to_owned());
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

        let mut legality = true;
        //Pawn
        let patt = {
            if my_player_num == 0 {
                self.attack_tables.get_north_pawn_attack_masked(loc_index, &position.occupied, &enemies)
            }else{
                self.attack_tables.get_south_pawn_attack_masked(loc_index, &position.occupied, &enemies)
            }
        };

        if legality && !(patt & enemy_pawns).is_zero() {
            legality = false;
        };

        //Knight
        let natt = self.attack_tables.get_knight_attack(loc_index, &position.occupied, &enemies);
        if legality && !(natt & enemy_knights).is_zero() {
            legality = false;
        };
        //King
        let katt = self.attack_tables.get_king_attack(loc_index, &position.occupied, &enemies);
        if legality && !(katt & enemy_kings).is_zero() {
            legality = false;
        };

        //Rook & Queen
        let ratt = self.attack_tables.get_rook_attack(loc_index, &position.occupied, &enemies);
        if legality && (!(&ratt & enemy_queens).is_zero() || !(&ratt & enemy_rooks).is_zero()){
            legality = false;
        };
        //Bishop & Queen
        let batt = self.attack_tables.get_bishop_attack(loc_index, &position.occupied, &enemies);
        if legality && (!(&batt & enemy_queens).is_zero() || !(&batt & enemy_bishops).is_zero()) {
            legality = false;
        };
        position.unmake_move();
        legality
    }
}

