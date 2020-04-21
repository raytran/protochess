use crate::types::bitboard::Bitboard;
use arrayvec::ArrayVec;
use crate::types::{Dimensions, bitboard, AttackDirection};

use bitboard::to_index;

pub struct MaskHandler {
    dimensions: Dimensions,
    boundary: Bitboard,
    north: ArrayVec<[Bitboard;256]>,
    east: ArrayVec<[Bitboard;256]>,
    south: ArrayVec<[Bitboard;256]>,
    west: ArrayVec<[Bitboard;256]>,
    northeast: ArrayVec<[Bitboard;256]>,
    northwest: ArrayVec<[Bitboard;256]>,
    southeast: ArrayVec<[Bitboard;256]>,
    southwest: ArrayVec<[Bitboard;256]>,
    knight: ArrayVec<[Bitboard;256]>,
    king: ArrayVec<[Bitboard;256]>,
    left_masks: ArrayVec<[Bitboard;16]>,
    right_masks: ArrayVec<[Bitboard;16]>,
    zero: Bitboard
}

impl MaskHandler {
    pub fn new_from_width_height(width:u8,height:u8) -> MaskHandler{
        MaskHandler::new(&Dimensions{width,height})
    }

    pub fn new(dims:&Dimensions) -> MaskHandler{
        let mut left_masks = ArrayVec::<[Bitboard;16]>::new();
        let mut right_masks = ArrayVec::<[Bitboard;16]>::new();
        //Initialize left & right file masks
        let mut cumulative_left = Bitboard::zero();
        let mut cumulative_right = Bitboard::zero();
        for i in 0..dims.width {
            let mut new_left = Bitboard::zero();
            let mut new_right = Bitboard::zero();
            new_left |= &cumulative_left;
            new_right |= &cumulative_right;

            for j in 0..dims.height{
                new_left.set_bit(bitboard::to_index(i,j,dims.width),true);
                new_right.set_bit(bitboard::to_index(dims.width-i-1,j,dims.width),true);
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
        let mut knight = ArrayVec::<[Bitboard;256]>::new();
        let mut king = ArrayVec::<[Bitboard;256]>::new();
        let mut bounds = Bitboard::zero();

        for i in 0..256 {
            knight.push(Bitboard::zero());
            king.push(Bitboard::zero());
            north.push(Bitboard::zero());
            east.push(Bitboard::zero());
            west.push(Bitboard::zero());
            south.push(Bitboard::zero());
            northeast.push(Bitboard::zero());
            northwest.push(Bitboard::zero());
            southeast.push(Bitboard::zero());
            southwest.push(Bitboard::zero());
        }

        for x in 0..dims.width as i8 {
            for y in 0..dims.height as i8 {
                let index:usize = to_index(x as u8, y as u8, dims.width) as usize;


                //KING LOOKUP TABLE
                let king_deltas = [(0,  1), (0,  -1), (-1, 0), (1,  0),
                    (1,  1), (1,  -1), (-1, 1), (-1, -1)];

                for &delta in &king_deltas {
                    let x2 = delta.0 + x;
                    let y2 = delta.1 + y;
                    if x2 >= 0 && x2 < dims.width as i8 && y2 >=0 && y2 < dims.height as i8 {
                        king[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width),true);
                    }
                }


                //KNIGHT LOOKUP TABLE
                let knight_deltas = [(2,  1), (2,  -1), (-2, 1), (-2, -1),
                    (1,  2), (1,  -2), (-1, 2), (-1, -2)];

                for &delta in &knight_deltas {
                    let x2 = delta.0 + x;
                    let y2 = delta.1 + y;
                    if x2 >= 0 && x2 < dims.width as i8 && y2 >=0 && y2 < dims.height as i8 {
                        knight[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width),true);
                    }
                }

                bounds.set_bit(index as usize, true);

                //NORTH LOOKUP TABLE
                for j in y + 1..dims.height as i8 {
                    north[index].set_bit(to_index(x as u8, j as u8, dims.width), true);
                }
                //SOUTH LOOKUP TABLE
                for j in 0..y {
                    south[index].set_bit(to_index(x as u8, j as u8, dims.width), true);
                }
                //EAST LOOKUP TABLE
                for j in x + 1..dims.width as i8 {
                    east[index].set_bit(to_index(j as u8, y as u8, dims.width), true);
                }
                //WEST LOOKUP TABLE
                for j in 0..x {
                    west[index].set_bit(to_index(j as u8, y as u8, dims.width), true);
                }
                //NORTHEAST LOOKUP TABLE
                let mut x2:i8 = (x + 1) as i8;
                let mut y2:i8 = (y + 1) as i8;
                while x2 < dims.width as i8 && y2 < dims.height as i8 {
                    northeast[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width), true);
                    x2 +=1;
                    y2 +=1;
                }

                //NORTHWEST LOOKUP TABLE
                x2 = (x - 1) as i8;
                y2 = (y + 1) as i8;
                while x2 >= 0 && y2 < dims.height as i8 {
                    northwest[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width), true);
                    x2 -= 1;
                    y2 += 1;
                }
                //SOUTHEAST LOOKUP TABLE
                x2 = (x + 1) as i8;
                y2 = (y - 1) as i8;
                while x2 < dims.width as i8 && y2 >= 0 {
                    southeast[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width), true);
                    x2 += 1;
                    y2 -= 1;
                }
                //SOUTHWEST LOOKUP TABLE
                x2 = (x - 1) as i8;
                y2 = (y - 1) as i8;
                while x2 >= 0 && y2 >= 0 {
                    southwest[index].set_bit(to_index(x2 as u8, y2 as u8, dims.width), true);
                    x2 -= 1;
                    y2 -= 1;
                }
            }
        }

        MaskHandler{
            dimensions: Dimensions{width: dims.width, height:dims.height},
            boundary: bounds,
            knight,
            north,
            east,
            king,
            south,
            west,
            northeast,
            northwest,
            southeast,
            southwest,
            left_masks,
            right_masks,
            zero: Bitboard::zero()
        }
    }

    pub fn get_attack(&self, dir:&AttackDirection, index:usize) -> &Bitboard {
        match dir {
            AttackDirection::NORTH => &self.north[index],
            AttackDirection::EAST => &self.east[index],
            AttackDirection::SOUTH => &self.south[index],
            AttackDirection::WEST => &self.west[index],
            AttackDirection::NORTHEAST => &self.northeast[index],
            AttackDirection::NORTHWEST => &self.northwest[index],
            AttackDirection::SOUTHEAST => &self.southeast[index],
            AttackDirection::SOUTHWEST => &self.southwest[index],
            AttackDirection::KNIGHT => &self.knight[index],
            AttackDirection::KING => &self.king[index],
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

    //bitboard must be same dimensions
    pub fn shift_north(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        bitboard << (amt * self.dimensions.width)
    }

    pub fn shift_south(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        bitboard >> (amt * self.dimensions.width)
    }

    pub fn shift_east(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        (bitboard << amt) & (!self.get_left_mask(amt as usize))
    }

    pub fn shift_west(&self, amt:u8, bitboard: &Bitboard) -> Bitboard {
        (bitboard >> amt) & (!self.get_right_mask(amt as usize))
    }
}