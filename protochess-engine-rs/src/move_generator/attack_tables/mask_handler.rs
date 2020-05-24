use arrayvec::ArrayVec;
use crate::types::bitboard::{Bitboard, to_index};
use crate::types::bitboard;

/// General bitboard masks for use with attack tables
pub struct MaskHandler {
    north: ArrayVec<[Bitboard;256]>,
    east: ArrayVec<[Bitboard;256]>,
    south: ArrayVec<[Bitboard;256]>,
    west: ArrayVec<[Bitboard;256]>,
    northeast: ArrayVec<[Bitboard;256]>,
    northwest: ArrayVec<[Bitboard;256]>,
    southeast: ArrayVec<[Bitboard;256]>,
    southwest: ArrayVec<[Bitboard;256]>,
    diagonals: ArrayVec<[Bitboard;256]>,
    antidiagonals: ArrayVec<[Bitboard;256]>,
    left_masks: ArrayVec<[Bitboard;16]>,
    right_masks: ArrayVec<[Bitboard;16]>,
    files: ArrayVec<[Bitboard;16]>,
    ranks: ArrayVec<[Bitboard;16]>,
    main_diagonal: Bitboard,
    zero: Bitboard,
}

impl MaskHandler {
    pub fn new() -> MaskHandler{
        let mut left_masks = ArrayVec::<[Bitboard;16]>::new();
        let mut right_masks = ArrayVec::<[Bitboard;16]>::new();
        //Initialize left & right file masks
        let mut cumulative_left = Bitboard::zero();
        let mut cumulative_right = Bitboard::zero();

        for i in 0..16 {
            let mut new_left = Bitboard::zero();
            let mut new_right = Bitboard::zero();
            new_left |= &cumulative_left;
            new_right |= &cumulative_right;

            for j in 0..16 {
                new_left.set_bit(bitboard::to_index(i,j),true);
                new_right.set_bit(bitboard::to_index(16-i-1,j),true);
            }

            cumulative_left |= &new_left;
            cumulative_right |= &new_right;
            right_masks.push(new_right);
            left_masks.push(new_left);
        }

        let mut north = ArrayVec::<[Bitboard;256]>::new();
        let mut east = ArrayVec::<[Bitboard;256]>::new();
        let mut west = ArrayVec::<[Bitboard;256]>::new();
        let mut south = ArrayVec::<[Bitboard;256]>::new();
        let mut northeast = ArrayVec::<[Bitboard;256]>::new();
        let mut northwest = ArrayVec::<[Bitboard;256]>::new();
        let mut southeast = ArrayVec::<[Bitboard;256]>::new();
        let mut southwest = ArrayVec::<[Bitboard;256]>::new();
        let mut diagonals = ArrayVec::<[Bitboard;256]>::new();
        let mut antidiagonals = ArrayVec::<[Bitboard;256]>::new();
        for _i in 0..256 {
            north.push(Bitboard::zero());
            east.push(Bitboard::zero());
            west.push(Bitboard::zero());
            south.push(Bitboard::zero());
            northeast.push(Bitboard::zero());
            northwest.push(Bitboard::zero());
            southeast.push(Bitboard::zero());
            southwest.push(Bitboard::zero());
            diagonals.push(Bitboard::zero());
            antidiagonals.push(Bitboard::zero());
        }

        for x in 0..16 as i8 {
            for y in 0..16 as i8 {
                let index:usize = to_index(x as u8, y as u8) as usize;

                //NORTH LOOKUP TABLE
                for j in y + 1..16 as i8 {
                    north[index].set_bit(to_index(x as u8, j as u8), true);
                }
                //SOUTH LOOKUP TABLE
                for j in 0..y {
                    south[index].set_bit(to_index(x as u8, j as u8), true);
                }
                //EAST LOOKUP TABLE
                for j in x + 1..16 as i8 {
                    east[index].set_bit(to_index(j as u8, y as u8), true);
                }
                //WEST LOOKUP TABLE
                for j in 0..x {
                    west[index].set_bit(to_index(j as u8, y as u8), true);
                }
                //NORTHEAST LOOKUP TABLE
                let mut x2:i8 = (x + 1) as i8;
                let mut y2:i8 = (y + 1) as i8;
                while x2 < 16 as i8 && y2 < 16 as i8 {
                    northeast[index].set_bit(to_index(x2 as u8, y2 as u8), true);
                    x2 +=1;
                    y2 +=1;
                }

                //NORTHWEST LOOKUP TABLE
                x2 = (x - 1) as i8;
                y2 = (y + 1) as i8;
                while x2 >= 0 && y2 < 16 as i8 {
                    northwest[index].set_bit(to_index(x2 as u8, y2 as u8), true);
                    x2 -= 1;
                    y2 += 1;
                }
                //SOUTHEAST LOOKUP TABLE
                x2 = (x + 1) as i8;
                y2 = (y - 1) as i8;
                while x2 < 16 as i8 && y2 >= 0 {
                    southeast[index].set_bit(to_index(x2 as u8, y2 as u8), true);
                    x2 += 1;
                    y2 -= 1;
                }
                //SOUTHWEST LOOKUP TABLE
                x2 = (x - 1) as i8;
                y2 = (y - 1) as i8;
                while x2 >= 0 && y2 >= 0 {
                    southwest[index].set_bit(to_index(x2 as u8, y2 as u8), true);
                    x2 -= 1;
                    y2 -= 1;
                }

                diagonals[index] = &northeast[index] ^ &southwest[index];
                antidiagonals[index] = &northwest[index] ^ &southeast[index];
            }
        }

        let mut main_diagonal = Bitboard::one();
        main_diagonal ^= &northeast[0];

        let mut ranks = ArrayVec::<[Bitboard;16]>::new();
        let mut files = ArrayVec::<[Bitboard;16]>::new();
        for i in 0..16 {

            let mut file = Bitboard::zero();
            for y in 0..16 {
                file.set_bit(to_index(i, y), true);
            }
            files.push(file);

            let mut rank = Bitboard::zero();
            for x in 0..16 {
                rank.set_bit(to_index(x, i), true);
            }

            ranks.push(rank);
        }

        MaskHandler {
            north,
            east,
            south,
            west,
            northeast,
            northwest,
            southeast,
            southwest,
            left_masks,
            right_masks,
            diagonals,
            antidiagonals,
            main_diagonal,
            ranks,
            files,
            zero: Bitboard::zero()
        }
    }

    pub fn get_right_mask(&self, num_cols:usize) -> &Bitboard {
        if num_cols == 0 {
            return &self.zero;
        }
        &self.right_masks[num_cols - 1]
    }

    pub fn get_left_mask(&self, num_cols:usize) -> &Bitboard {
        if num_cols == 0 {
            return &self.zero;
        }
        &self.left_masks[num_cols - 1]
    }

    pub fn get_main_diagonal(&self) -> &Bitboard {
        &self.main_diagonal
    }

    pub fn get_diagonal(&self, index:u8) -> &Bitboard{
        &self.diagonals[index as usize]
    }

    pub fn get_north(&self, index:u8) -> &Bitboard{
        &self.north[index as usize]
    }

    pub fn get_south(&self, index:u8) -> &Bitboard{
        &self.south[index as usize]
    }

    pub fn get_east(&self, index:u8) -> &Bitboard{
        &self.east[index as usize]
    }

    pub fn get_west(&self, index:u8) -> &Bitboard{
        &self.west[index as usize]
    }

    pub fn get_northwest(&self, index:u8) -> &Bitboard{
        &self.northwest[index as usize]
    }

    pub fn get_northeast(&self, index:u8) -> &Bitboard{
        &self.northeast[index as usize]
    }

    pub fn get_southeast(&self, index:u8) -> &Bitboard{
        &self.southeast[index as usize]
    }

    pub fn get_southwest(&self, index:u8) -> &Bitboard{
        &self.southwest[index as usize]
    }

    pub fn get_antidiagonal(&self, index:u8) -> &Bitboard{
        &self.antidiagonals[index as usize]
    }

    pub fn get_file(&self, n:u8) -> &Bitboard {
        &self.files[n as usize]
    }

    pub fn get_rank(&self, n:u8) -> &Bitboard {
        &self.ranks[n as usize]
    }

    //bitboard must be same dimensions
    pub fn shift_north(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        bitboard << (amt * 16)
    }

    pub fn shift_south(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        bitboard >> (amt * 16)
    }

    pub fn shift_east(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        (bitboard << amt) & (!self.get_left_mask(amt as usize))
    }

    pub fn shift_west(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        (bitboard >> amt) & (!self.get_right_mask(amt as usize))
    }

}
