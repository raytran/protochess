use crate::types::{Dimensions, bitboard};
use crate::types::{Move, LineAttackType, AttackDirection };
use crate::types::bitboard::{Bitboard, to_string};
use crate::position::Position;
use crate::position::piece_set::PieceSet;
use crate::move_generator::attack_tables::AttackTables;
use crate::move_generator::bitboard_moves::BitboardMoves;

mod attack_tables;
mod bitboard_moves;
//Iterator that yields possible moves from a position
pub(crate) struct MoveGenerator {
    attack_tables: AttackTables,
}
impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator{
            attack_tables: AttackTables::new(),
        }
    }


    pub fn get_moves(&self, position:&Position) -> Vec<Move> {
        let my_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;

        //create a vector of iterators
        let mut movelist:Vec<Move> = Vec::with_capacity(250);


        let mut apply_to_each = |mut pieceset:Bitboard, func: fn(&AttackTables, u8, &Bitboard, &Bitboard)-> Bitboard| {
            while !pieceset.is_zero() {
                let index = pieceset.lowest_one().unwrap() as u8;
                let mut raw_attacks = func(&self.attack_tables,index, &position.occupied, &enemies);
                //Do not attack ourselves
                raw_attacks &= !&my_pieces.occupied;
                //Keep only in bounds
                raw_attacks &= &position.bounds;

                while !raw_attacks.is_zero() {
                    let to = raw_attacks.lowest_one().unwrap();
                    raw_attacks.set_bit(to, false);
                    if enemies.bit(to).unwrap() {
                        movelist.push(Move::new(index, to as u8,true));
                    }else{
                        movelist.push(Move::new(index, to as u8,false));
                    }
                }

                pieceset.set_bit(index as usize, false);
            }
        };

        apply_to_each((&my_pieces.king).to_owned(), AttackTables::get_king_attack);
        apply_to_each((&my_pieces.queen).to_owned(), AttackTables::get_queen_attack);
        apply_to_each((&my_pieces.rook).to_owned(), AttackTables::get_rook_attack);
        apply_to_each((&my_pieces.bishop).to_owned(), AttackTables::get_bishop_attack);
        apply_to_each((&my_pieces.knight).to_owned(), AttackTables::get_knight_attack);
        if position.whos_turn == 0 {
            apply_to_each((&my_pieces.pawn).to_owned(), AttackTables::get_north_pawn_attack);
        }else{
            apply_to_each((&my_pieces.pawn).to_owned(), AttackTables::get_south_pawn_attack);
        }

        movelist
    }
    /*
    pub fn get_moves(&self, position:&Position) -> impl Iterator<Item=Move> {
        let my_pieces: &PieceSet = &position.pieces[position.whos_turn as usize];
        let enemies = &position.occupied & !&my_pieces.occupied;

        //create a vector of iterators
        let mut iters:Vec<BitboardMoves> = Vec::new();


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
        if position.whos_turn == 0 {
            apply_to_each((&my_pieces.pawn).to_owned(), AttackTables::get_north_pawn_attack);
        }else{
            apply_to_each((&my_pieces.pawn).to_owned(), AttackTables::get_south_pawn_attack);
        }

        //Flatten our vector of iterators
        iters.into_iter().flatten()
    }
     */
}

