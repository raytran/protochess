mod mask_handler;

use arrayvec::ArrayVec;
use crate::types::bitboard::{Bitboard, to_index, from_index};
use crate::move_generator::attack_tables::mask_handler::MaskHandler;

/// Holds pre-calculated attack tables for the pieces, assuming a 16x16 size board
/// Only for classical set of pieces
pub struct AttackTables {
    slider_attacks: Vec<Vec<u16>>,
    knight_attacks: ArrayVec<[Bitboard;256]>,
    king_attacks: ArrayVec<[Bitboard;256]>,
    north_pawn_attacks: ArrayVec<[Bitboard;256]>,
    north_pawn_single_push: ArrayVec<[Bitboard;256]>,
    north_pawn_double_push: ArrayVec<[Bitboard;256]>,
    south_pawn_attacks: ArrayVec<[Bitboard;256]>,
    south_pawn_single_push: ArrayVec<[Bitboard;256]>,
    south_pawn_double_push: ArrayVec<[Bitboard;256]>,
    pub masks: MaskHandler
}

impl AttackTables {
    pub(crate) fn new() -> AttackTables {
        let mut knight_attacks = ArrayVec::<[Bitboard;256]>::new();
        let mut king_attacks = ArrayVec::<[Bitboard;256]>::new();
        let mut north_pawn_attacks = ArrayVec::<[Bitboard;256]>::new();
        let mut north_pawn_single_push = ArrayVec::<[Bitboard;256]>::new();
        let mut north_pawn_double_push = ArrayVec::<[Bitboard;256]>::new();
        let mut south_pawn_attacks = ArrayVec::<[Bitboard;256]>::new();
        let mut south_pawn_single_push = ArrayVec::<[Bitboard;256]>::new();
        let mut south_pawn_double_push = ArrayVec::<[Bitboard;256]>::new();
        for _i in 0..256 {
            knight_attacks.push(Bitboard::zero());
            king_attacks.push(Bitboard::zero());
            north_pawn_attacks.push(Bitboard::zero());
            north_pawn_single_push.push(Bitboard::zero());
            north_pawn_double_push.push(Bitboard::zero());
            south_pawn_attacks.push(Bitboard::zero());
            south_pawn_single_push.push(Bitboard::zero());
            south_pawn_double_push.push(Bitboard::zero());
        }

        for x in 0..16 as i8 {
            for y in 0..16 as i8 {
                let index:usize = to_index(x as u8, y as u8) as usize;
                //PAWN
                if y != 15 {
                    north_pawn_single_push[index].set_bit(to_index(x as u8,(y + 1) as u8),true);
                    north_pawn_double_push[index].set_bit(to_index(x as u8,(y + 1) as u8),true);
                    if y + 2 < 16 {
                        north_pawn_double_push[index].set_bit(to_index(x as u8,(y + 2) as u8),true);
                    }
                    if x + 1 < 16 {
                        north_pawn_attacks[index].set_bit(to_index((x + 1) as u8,(y + 1) as u8),true);
                    }
                    if x - 1 >= 0 {
                        north_pawn_attacks[index].set_bit(to_index((x - 1) as u8,(y + 1) as u8),true);
                    }
                }

                if y != 0 {
                    south_pawn_single_push[index].set_bit(to_index(x as u8,(y - 1) as u8),true);
                    south_pawn_double_push[index].set_bit(to_index(x as u8,(y - 1) as u8),true);
                    if y - 2 >= 0 {
                        south_pawn_double_push[index].set_bit(to_index(x as u8,(y - 2) as u8),true);
                    }
                    if x + 1 < 16 {
                        south_pawn_attacks[index].set_bit(to_index((x + 1) as u8,(y - 1) as u8),true);
                    }
                    if x - 1 >= 0 {
                        south_pawn_attacks[index].set_bit(to_index((x - 1) as u8,(y - 1) as u8),true);
                    }
                }

                //KING LOOKUP TABLE
                let king_deltas = [(0,  1), (0,  -1), (-1, 0), (1,  0),
                    (1,  1), (1,  -1), (-1, 1), (-1, -1)];

                for delta in &king_deltas {
                    let x2 = delta.0 + x;
                    let y2 = delta.1 + y;
                    if x2 >= 0 && x2 < 16 as i8 && y2 >=0 && y2 < 16 as i8 {
                        king_attacks[index].set_bit(to_index(x2 as u8, y2 as u8), true);
                    }
                }
                //KNIGHT LOOKUP TABLE
                let knight_deltas = [(2,  1), (2,  -1), (-2, 1), (-2, -1),
                    (1,  2), (1,  -2), (-1, 2), (-1, -2)];

                for delta in &knight_deltas {
                    let x2 = delta.0 + x;
                    let y2 = delta.1 + y;
                    if x2 >= 0 && x2 < 16 as i8 && y2 >=0 && y2 < 16 as i8 {
                        knight_attacks[index].set_bit(to_index(x2 as u8, y2 as u8),true);
                    }
                }
            }
        }

        //16 * 2^16 possible states; 16 squares in 1 rank, 2^16 possible occupancies per rank
        let mut slider_attacks = vec![vec![0; 65536]; 16];
        //16 squares in 1 rank
        for i in 0..16 {
            //2^16 = 65536 possible occupancies
            for occ in 0..=65535 {
                //Square from index
                let sq = 1u16 << i;
                //Classical approach to generate the table
                fn get_left_attack(src:u16) -> u16 {
                    if src == 0 {
                        0
                    } else {
                        src - 1u16
                    }
                }
                fn get_right_attack(src:u16) -> u16 {
                    !src & !get_left_attack(src)
                }

                let mut left_attack = get_left_attack(sq);
                let left_blockers = occ & left_attack;

                if left_blockers != 0 {
                    let msb_blockers = 1u16 << (15 - left_blockers.leading_zeros());
                    left_attack ^= get_left_attack(msb_blockers);
                }

                let mut right_attack = get_right_attack(sq);
                let right_blockers = occ & right_attack;
                if right_blockers != 0 {
                    let lsb_blockers = 1u16 << right_blockers.trailing_zeros();
                    right_attack ^= get_right_attack(lsb_blockers);
                }
                slider_attacks[i as usize][occ as usize] = right_attack ^ left_attack;
            }
        }

        AttackTables{
            slider_attacks,
            king_attacks,
            knight_attacks,
            north_pawn_attacks,
            north_pawn_single_push,
            north_pawn_double_push,
            south_pawn_attacks,
            south_pawn_single_push,
            south_pawn_double_push,
            masks: MaskHandler::new()
        }
    }

    pub fn get_rank_attack(&self, loc_index:u8, occ: &Bitboard) -> Bitboard {
        let (x, y) = from_index(loc_index as usize);
        //Isolate the rank
        let rank_only = self.masks.shift_south(y, occ);
        let first_byte = rank_only.byte(0).unwrap();
        let second_byte = rank_only.byte(1).unwrap();
        //Looup the occupancy rank in our table
        let occ_index = (second_byte as u16) << 8 ^ (first_byte as u16);
        let attack = self.slider_attacks[x as usize][occ_index as usize];
        //Shift attack back to rank
        let mut return_bb = Bitboard::zero();
        return_bb ^= attack;
        self.masks.shift_north(y, &return_bb)
    }

    pub fn get_file_attack(&self, loc_index:u8, occ: &Bitboard) -> Bitboard {
        //First map the file to a rank
        let (x, y) = from_index(loc_index as usize);
        //mask rank only and shift to A file
        let a_shifted = self.masks.shift_west(x, occ) & self.masks.get_file(0);
        let first_rank = self.masks.shift_south(15,&(a_shifted.overflowing_mul(self.masks.get_main_diagonal()).0));
        //Lookup the attack in our table
        let occ_index = (first_rank.byte(0).unwrap() as u16) ^ ((first_rank.byte(1).unwrap() as u16) << 8);
        let rank_index = 15 - y;
        let attack = self.slider_attacks[rank_index as usize][occ_index as usize];
        //Map the attable back into the file
        let mut return_bb = Bitboard::zero();
        return_bb ^= attack;
        //Shift the rank back into the corresponding file
        let last_file = self.masks.get_right_mask(1) & (return_bb.overflowing_mul(self.masks.get_main_diagonal()).0);
        self.masks.shift_west(15 - x, &last_file)

    }

    pub fn get_diagonal_attack(&self, loc_index:u8, occ: &Bitboard) -> Bitboard {
        let (x, _y) = from_index(loc_index as usize);
        //Map the diagonal to the first rank
        let masked_diag = occ & self.masks.get_diagonal(loc_index);
        let last_rank_with_garbage = masked_diag.overflowing_mul(self.masks.get_file(0)).0;
        let first_rank = self.masks.shift_south(15,&last_rank_with_garbage);
        //Lookup the attack for the first rank
        let occ_index = (first_rank.byte(0).unwrap() as u16) ^ ((first_rank.byte(1).unwrap() as u16) << 8);
        let attack = self.slider_attacks[x as usize][occ_index as usize];
        //Map attack back to diagonal
        let mut return_bb = Bitboard::zero();
        return_bb ^= attack;
        return_bb.overflowing_mul(self.masks.get_file(0)).0 & self.masks.get_diagonal(loc_index)
    }

    pub fn get_antidiagonal_attack(&self, loc_index:u8, occ: &Bitboard) -> Bitboard {
        let (x, _y) = from_index(loc_index as usize);
        //Map the diagonal to the first rank
        let masked_diag = occ & self.masks.get_antidiagonal(loc_index);
        let last_rank_with_garbage = masked_diag.overflowing_mul(self.masks.get_file(0)).0;
        let first_rank = self.masks.shift_south(15,&last_rank_with_garbage);
        //Lookup the attack for the first rank
        let occ_index = (first_rank.byte(0).unwrap() as u16) ^ ((first_rank.byte(1).unwrap() as u16) << 8);
        let attack = self.slider_attacks[x as usize][occ_index as usize];
        //Map attack back to diagonal
        let mut return_bb = Bitboard::zero();
        return_bb ^= attack;
        return_bb.overflowing_mul(self.masks.get_file(0)).0 & self.masks.get_antidiagonal(loc_index)
    }

    pub fn get_knight_attack(&self, loc_index:u8, _occ: &Bitboard, _enemies: &Bitboard) -> Bitboard {
        (&self.knight_attacks[loc_index as usize]).to_owned()
    }

    pub fn get_king_attack(&self, loc_index:u8, _occ: &Bitboard, _enemies: &Bitboard) -> Bitboard {
        (&self.king_attacks[loc_index as usize]).to_owned()
    }

    pub fn get_north_pawn_attack(&self, loc_index:u8, occ: &Bitboard, enemies: &Bitboard) -> Bitboard {
        let (x, y) = from_index(loc_index as usize);
        let return_bb = {
            //Double push
            if y == 1 && !occ.bit(to_index(x, y+1)).unwrap() {
                &self.north_pawn_double_push[loc_index as usize] & !occ
            }else{
                &self.north_pawn_single_push[loc_index as usize] & !occ
            }
        };
        return_bb ^ (&self.north_pawn_attacks[loc_index as usize] & enemies)
    }

    pub fn get_south_pawn_attack(&self, loc_index:u8, occ: &Bitboard, enemies: &Bitboard) -> Bitboard {
        let (x, y) = from_index(loc_index as usize);
        let return_bb = {
            //Double push
            if y == 6 && !occ.bit(to_index(x, y-1)).unwrap() {
                &self.south_pawn_double_push[loc_index as usize] & !occ
            }else{
                &self.south_pawn_single_push[loc_index as usize] & !occ
            }
        };
        return_bb ^ (&self.south_pawn_attacks[loc_index as usize] & enemies)
    }

    pub fn get_south_pawn_attack_masked(&self, loc_index:u8, _occ: &Bitboard, enemies: &Bitboard) -> Bitboard {
        &self.south_pawn_attacks[loc_index as usize] & enemies
    }

    pub fn get_north_pawn_attack_masked(&self, loc_index:u8, _occ: &Bitboard, enemies: &Bitboard) -> Bitboard {
        &self.north_pawn_attacks[loc_index as usize] & enemies
    }

    pub fn get_north_pawn_attack_raw(&self, loc_index:u8) -> &Bitboard {
        &self.north_pawn_attacks[loc_index as usize]
    }

    pub fn get_south_pawn_attack_raw(&self, loc_index:u8) -> &Bitboard {
        &self.south_pawn_attacks[loc_index as usize]
    }

    /// Returns a bitboard of the sliding piece moves
    pub fn get_sliding_moves_bb(&self,
                                loc_index:u8,
                                occ: &Bitboard,
                                north: bool,
                                east: bool,
                                south: bool,
                                west: bool,
                                northeast: bool,
                                northwest: bool,
                                southeast:bool,
                                southwest:bool,
    ) -> Bitboard {
        let mut raw_attacks = Bitboard::zero();
        if north || south {
            raw_attacks |= self.get_file_attack(loc_index, occ);
            if !north {
                raw_attacks &= !self.masks.get_north(loc_index);
            }else if !south {
                raw_attacks &= !self.masks.get_south(loc_index);
            }
        }

        if east || west {
            raw_attacks |= self.get_rank_attack(loc_index, occ);
            if !east {
                raw_attacks &= !self.masks.get_east(loc_index);
            }else if !west {
                raw_attacks &= !self.masks.get_west(loc_index);
            }
        }

        if northeast || southwest {
            raw_attacks |= self.get_diagonal_attack(loc_index, occ);
            if !northeast {
                raw_attacks &= !self.masks.get_northeast(loc_index);
            }else if !southwest {
                raw_attacks &= !self.masks.get_southwest(loc_index);
            }
        }

        if northwest || southeast {
            raw_attacks |= self.get_antidiagonal_attack(loc_index, occ);
            if !northwest {
                raw_attacks &= !self.masks.get_northwest(loc_index);
            }else if !southeast {
                raw_attacks &= !self.masks.get_southeast(loc_index);
            }
        }

        raw_attacks
    }

    pub fn get_rook_attack(&self, loc_index:u8, occ: &Bitboard, _enemies: &Bitboard) -> Bitboard {
        self.get_file_attack(loc_index, occ)
            ^ self.get_rank_attack(loc_index, occ)
    }

    pub fn get_bishop_attack(&self, loc_index:u8, occ: &Bitboard, _enemies: &Bitboard) -> Bitboard {
        self.get_diagonal_attack(loc_index, occ)
            ^ self.get_antidiagonal_attack(loc_index, occ)
    }

    pub fn get_queen_attack(&self, loc_index:u8, occ: &Bitboard, enemies: &Bitboard) -> Bitboard {
        self.get_rook_attack(loc_index, occ, enemies)
            ^ self.get_bishop_attack(loc_index, occ, enemies)
    }

}

#[cfg(test)]
mod tests {
    use crate::move_generator::attack_tables::AttackTables;
    use crate::types::bitboard::Bitboard;
    use crate::types::bitboard::to_string;


    #[test]
    fn test() {
        let _attacktb = AttackTables::new();
        let mut bb = Bitboard::zero();
        bb |= 9252345218324798u64;



        println!("occ \n{}", to_string(&bb));
        //let rankatt = attacktb.get_rank_attack(2,&bb);
        //println!("{}", to_string(&rankatt));


    }
}
