pub mod bitboard;
pub mod chess_move;

#[derive(Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Custom(char),
}

impl PieceType {
    pub fn from_char(c:char) -> PieceType {
        match c {
            'k' =>{PieceType::King}
            'q' =>{PieceType::Queen}
            'r' =>{PieceType::Rook}
            'b' =>{PieceType::Bishop}
            'n' =>{PieceType::Knight}
            'p' =>{PieceType::Pawn}
            _ => {PieceType::Custom(c)}
        }
    }
}
pub struct Dimensions {
    pub width:u8,
    pub height:u8,
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
