use crate::types::Dimensions;
pub type Bitboard = numext_fixed_uint::U256;

pub fn to_index(x:u8, y:u8, width: u8) -> usize{
    (width * y + x) as usize
}

pub fn from_index(index:usize, width: u8) -> (u8, u8) {
    (index as u8 % width , index as u8 / width )
}

pub fn to_string(bitboard:&Bitboard, dimensions: &Dimensions) -> String {
    let mut return_str = String::new();
    for y in (0..dimensions.height).rev() {
        for x in 0..dimensions.width {
            if bitboard.bit(to_index(x, y, dimensions.width)).unwrap(){
                return_str.push('1');
            }else{
                return_str.push('.');
            }
            return_str.push(' ');
        }
        return_str.push('\n');
    }
    return_str
}



