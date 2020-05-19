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

impl Move {
    pub fn new(from:u8, to:u8, capture:bool) -> Move{
        //println!("{}",capture);
        Move((from as u32) | (to as u32) << 8u32 | {if capture { 1u32 << 17} else {0u32}})
    }

    pub fn get_from(&self) -> u8{
        (self.0 & 255u32) as u8
    }

    pub fn get_to(&self) -> u8{
        ((self.0 >> 8) & 255u32) as u8
    }

    pub fn get_capture(&self) -> bool{
        ((self.0 >> 17) & 1u32) != 0u32
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
