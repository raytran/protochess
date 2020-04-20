pub mod bitboard;
pub struct Dimensions {
    pub width:u8,
    pub height:u8,
}
pub type Move = u32;

pub fn new_move(from:u8, to:u8, capture:bool) -> Move{
    //println!("{}",capture);
    (from as u32) | (to as u32) << 8u32 | {if capture { 1u32 << 17} else {0u32}}
}

pub fn get_from(move_: &Move) -> u8{
    (move_ & 255u32) as u8
}

pub fn get_to(move_: &Move) -> u8{
    ((move_ >> 8) & 255u32) as u8
}

pub fn get_capture(move_: &Move) -> bool{
    ((move_ >> 17) & 1u32) != 0u32
}

//Direction for an attack
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}

//Upper, lower
pub enum LineType{
    DIAGONAL,
    ANTIDIAGONAL,
    RANK,
    FILE,
}

impl LineType {
    pub fn get_upper(&self) -> Direction{
        match self{
            LineType::DIAGONAL => Direction::NORTHEAST,
            LineType::ANTIDIAGONAL => Direction::NORTHWEST,
            LineType::RANK => Direction::EAST,
            LineType::FILE => Direction::NORTH,
        }
    }

    pub fn get_lower(&self) -> Direction {
        match self {
            LineType::DIAGONAL => Direction::SOUTHWEST,
            LineType::ANTIDIAGONAL => Direction::SOUTHEAST,
            LineType::RANK => Direction::WEST,
            LineType::FILE => Direction::SOUTH,
        }
    }
}
