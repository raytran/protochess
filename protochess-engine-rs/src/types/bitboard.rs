
pub type Bitboard = numext_fixed_uint::U256;

pub fn to_index(x:u8, y:u8) -> usize{
    (16 * y + x) as usize
}

pub fn from_index(index:usize) -> (u8, u8) {
    (index as u8 % 16 , index as u8 / 16 )
}

pub fn to_string(bitboard:&Bitboard) -> String {
    let mut return_str = String::new();
    for y in (0..16).rev() {
        for x in 0..16 {
            if bitboard.bit(to_index(x, y)).unwrap(){
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



