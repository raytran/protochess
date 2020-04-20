use crate::types::{new_move, Move, LineType, Dimensions, bitboard};
use crate::types::bitboard::Bitboard;
use crate::Engine;
use crate::position::Position;
use crate::position::piece_set::PieceSet;

impl Engine {
    pub(crate) fn generate_moves(&self, position: &Position, whos_turn:u8) -> Vec<Move> {
        let mut moves = Vec::new();
        let pieces = &position.pieces[whos_turn as usize];
        let enemies = &position.occupied & !&pieces.occupied;

        //Generate all pawn moves at once
        self.generate_all_pawn_moves(position, &mut moves, &pieces.pawn, &enemies, whos_turn);




        let q_copy = &mut pieces.queen.to_owned();
        //Queen moves:
        while !q_copy.is_zero(){
            let index = q_copy.lowest_one().unwrap();
            self.generate_queen_moves(&mut moves, position, pieces, &enemies, index);
            q_copy.set_bit(index,false);
        }

        let r_copy = &mut pieces.rook.to_owned();
        //Rook moves:
        while !r_copy.is_zero(){
            let index = r_copy.lowest_one().unwrap();
            self.generate_rook_moves(&mut moves, position, pieces, &enemies, index);
            r_copy.set_bit(index,false);
        }



        let b_copy = &mut pieces.bishop.to_owned();
        //Bishop moves:
        while !b_copy.is_zero(){
            let index = b_copy.lowest_one().unwrap();
            self.generate_rook_moves(&mut moves, position, pieces, &enemies, index);
            b_copy.set_bit(index,false);
        }
        //TODO custom pieces using classical approach

        moves
    }




    fn generate_knight_moves(&self, movelist: &mut Vec<Move>, position: &Position, team: &PieceSet, enemies:&Bitboard, source_index:usize) {
        //TODO

    }

    fn generate_queen_moves(&self, movelist: &mut Vec<Move>, position: &Position, team: &PieceSet, enemies:&Bitboard, source_index:usize) {
        self.generate_rook_moves(movelist, position, team, enemies, source_index);
        self.generate_bishop_moves(movelist, position, team, enemies, source_index);
    }

    fn generate_rook_moves(&self, movelist: &mut Vec<Move>, position: &Position, team: &PieceSet, enemies:&Bitboard, source_index:usize) {
        self.line_attacks(movelist, position, team, enemies, source_index, &LineType::RANK);
        self.line_attacks(movelist, position, team, enemies, source_index, &LineType::FILE);
    }

    fn generate_bishop_moves(&self, movelist: &mut Vec<Move>, position: &Position, team: &PieceSet, enemies:&Bitboard, source_index:usize) {
        self.line_attacks(movelist, position, team, enemies, source_index, &LineType::ANTIDIAGONAL);
        self.line_attacks(movelist, position, team, enemies, source_index, &LineType::DIAGONAL);
    }

    fn line_attacks(&self, movelist: &mut Vec<Move>, position: &Position, team: &PieceSet, enemies:&Bitboard, source_index:usize, line_type:&LineType) {
        let occ = &position.occupied;

        let full_mask = self.masks.get_attack(&line_type.get_lower(), source_index) | self.masks.get_attack(&line_type.get_upper(), source_index);
        let occ_lower = self.masks.get_attack(&line_type.get_lower(), source_index) & occ;
        let occ_upper = self.masks.get_attack(&line_type.get_upper(), source_index) & occ;

        let mut lsb_upper = Bitboard::zero();
        if let Some(i) = &occ_upper.lowest_one() {
            lsb_upper.set_bit(*i,true);
        }

        let mut msb_lower = Bitboard::zero();
        if let Some(i) = &occ_lower.highest_one(){
            msb_lower.set_bit(*i,true);
        }else{
            msb_lower.set_bit(0,true);
        }

        let twice_lsb_upper:Bitboard = lsb_upper << 1;

        let mut attacks = twice_lsb_upper.overflowing_sub(&msb_lower).0 & full_mask;

        //Don't attack ourselves
        attacks &= !&team.occupied;

        while !attacks.is_zero() {
            let to = attacks.lowest_one().unwrap();
            if enemies.bit(to).unwrap() {
                movelist.push(new_move(source_index as u8, to as u8,true));
            }else{
                movelist.push(new_move(source_index as u8, to as u8,false));
            }
            attacks.set_bit(to, false);
        }
    }

    fn generate_all_pawn_moves(&self, position: &Position, movelist: &mut Vec<Move>, pawns: &Bitboard, enemies: &Bitboard, whos_turn: u8) {
        if whos_turn == 0 {
            self.generate_all_pawn_pushes(position, movelist, pawns, true);
            self.generate_all_pawn_captures(position, movelist, pawns, true, enemies);
        }else{
            self.generate_all_pawn_pushes(position, movelist, pawns, false);
            self.generate_all_pawn_captures(position, movelist, pawns, false, enemies);
        }
    }

    fn generate_all_pawn_captures(&self, position: &Position, movelist: &mut Vec<Move>, pawns: &Bitboard, northfacing: bool, enemies: &Bitboard) {
        if northfacing {
            let north_one = self.masks.shift_north(1,pawns);
            let mut ne_attacks = self.masks.shift_east(1,&north_one) & enemies;
            let mut nw_attacks = self.masks.shift_west(1,&north_one) & enemies;

            while !ne_attacks.is_zero() {
                let to = ne_attacks.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width - 1;
                movelist.push(new_move(from, to,true));
                ne_attacks.set_bit(to as usize, false);
            }
            while !nw_attacks.is_zero() {
                let to = nw_attacks.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width + 1;
                movelist.push(new_move(from, to,true));
                nw_attacks.set_bit(to as usize, false);
            }
        }else{
            let south_one = self.masks.shift_south(1,pawns);
            let mut se_attacks = self.masks.shift_east(1,&south_one) & enemies;
            let mut sw_attacks = self.masks.shift_west(1,&south_one) & enemies;

            while !se_attacks.is_zero() {
                let to = se_attacks.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width - 1;
                movelist.push(new_move(from, to,true));
                se_attacks.set_bit(to as usize, false);
            }
            while !sw_attacks.is_zero() {
                let to = sw_attacks.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width + 1;
                movelist.push(new_move(from, to,true));
                sw_attacks.set_bit(to as usize, false);
            }
        }
    }

    fn generate_all_pawn_pushes(&self, position: &Position, movelist: &mut Vec<Move>, pawns: &Bitboard, northfacing: bool) {
        if northfacing {
            //Single pawn moves
            let empty:Bitboard = !&position.occupied;
            let mut north_one = self.masks.shift_north(1, pawns) & &empty;
            //Double pawn moves
            let mut north_two = self.masks.shift_north(1, &north_one) & &empty;
            while !north_one.is_zero() {
                let to = north_one.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width;
                movelist.push(new_move(from, to,false));
                north_one.set_bit(to as usize, false);
            }

            while !north_two.is_zero() {
                let to = north_two.lowest_one().unwrap() as u8;
                let from = to - 2 * position.dimensions.width;
                movelist.push(new_move(from, to,false));
                north_two.set_bit(to as usize, false);
            }
        }else{
            //Single pawn moves
            let empty:Bitboard = !&position.occupied;
            let mut south_one = self.masks.shift_south(1, pawns) & &empty;
            //Double pawn moves
            let mut south_two = self.masks.shift_south(1, &south_one) & &empty;
            while !south_one.is_zero() {
                let to = south_one.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width;
                movelist.push(new_move(from, to,false));
                south_one.set_bit(to as usize, false);
            }

            while !south_two.is_zero() {
                let to = south_two.lowest_one().unwrap() as u8;
                let from = to + 2 * position.dimensions.width;
                movelist.push(new_move(from, to,false));
                south_two.set_bit(to as usize, false);
            }
        }
    }
}
