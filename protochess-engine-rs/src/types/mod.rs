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

pub enum PieceType {
    KING,
    QUEEN,
    KNIGHT,
    BISHOP,
    ROOK,
    PAWN,
    CUSTOM
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
