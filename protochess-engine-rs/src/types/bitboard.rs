use crate::types::*;
use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Not};

pub struct Bitboard {
    a: u128,
    b: u128,
}

impl From<u128> for Bitboard{
    fn from(a_: u128) -> Self {
        Bitboard{a:a_,b:0}
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard{a: self.a | rhs.a, b: self.b | rhs.b}
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.a |= rhs.a;
        self.b |= rhs.b;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard{a: self.a & rhs.a, b: self.b & rhs.b}
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.a &= rhs.a;
        self.b &= rhs.b;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard{ a:self.a ^ rhs.a, b: self.b ^ rhs.b}
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.a ^= rhs.a;
        self.b ^= rhs.b;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard{a: !self.a, b: !self.b}
    }
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard{a:0,b:0}
    }

    pub fn bit_set(&mut self, index: u8) {
        if index < 128 {
            self.a |= 1u128 << (index as u128);
        }else{
            self.b |= 1u128 << ((index-128) as u128);
        }
    }

    pub fn bit_unset(&mut self, index:u8){
        if index < 128 {
            self.a &= !(1u128 << (index as u128));
        }else{
            self.b &= !(1u128 << ((index-128) as u128));
        }
    }

    pub fn bit_test(&self, index: u8) -> bool{
        if index < 128 {
            self.a & (1u128 << (index as u128)) != 0
        }else{
            self.b & (1u128 << ((index - 128) as u128)) != 0
        }
    }


    pub fn to_string(&self, dimensions: &Dimensions) -> String {
        let mut return_str = String::new();
        for y in (0..dimensions.height).rev() {
            for x in 0..dimensions.width {
                if self.bit_test(to_index(x, y, dimensions.width)){
                    return_str.push('1');
                }else{
                    return_str.push('.');
                }
            }
            return_str.push('\n');
        }
        return_str
    }
}

pub fn to_index(x:u8, y:u8, width: u8) -> u8{
    width * y + x
}
