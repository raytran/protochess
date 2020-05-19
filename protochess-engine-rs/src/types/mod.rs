pub mod bitboard;

#[derive(Clone, PartialEq)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    CUSTOM(char),
}
pub struct Dimensions {
    pub width:u8,
    pub height:u8,
}

#[derive(Copy, Clone)]
pub struct Move(u32);

//Stores a move in a u32
//0-7:   from index:u8
//8-15:  to index:u8
//16 :   capture flag:bool
//17-24: capture index:u8

impl Move {
    pub fn new(from:u8, to:u8, capture:bool, capture_loc: u8) -> Move{
        Move(
            (from as u32)
            | (to as u32) << 8u32
            | {if capture { 1u32 << 16} else {0u32}}
            | (capture_loc as u32) << 17u32
        )
    }

    pub fn get_from(&self) -> u8{
        (self.0 & 255u32) as u8
    }

    pub fn get_to(&self) -> u8{
        ((self.0 >> 8) & 255u32) as u8
    }

    pub fn get_is_capture(&self) -> bool{
        ((self.0 >> 16) & 1u32) != 0u32
    }

    pub fn get_capture(&self) -> u8 {
        ((self.0 >> 17) & 255u32) as u8
    }
}

//Direction for an attack
pub enum AttackDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
    KNIGHT,
    KING,
}

//Upper, lower
pub enum LineAttackType {
    DIAGONAL,
    ANTIDIAGONAL,
    RANK,
    FILE,
}

impl LineAttackType {
    pub fn get_upper(&self) -> AttackDirection {
        match self{
            LineAttackType::DIAGONAL => AttackDirection::NORTHEAST,
            LineAttackType::ANTIDIAGONAL => AttackDirection::NORTHWEST,
            LineAttackType::RANK => AttackDirection::EAST,
            LineAttackType::FILE => AttackDirection::NORTH,
        }
    }

    pub fn get_lower(&self) -> AttackDirection {
        match self {
            LineAttackType::DIAGONAL => AttackDirection::SOUTHWEST,
            LineAttackType::ANTIDIAGONAL => AttackDirection::SOUTHEAST,
            LineAttackType::RANK => AttackDirection::WEST,
            LineAttackType::FILE => AttackDirection::SOUTH,
        }
    }
}
