pub mod bitboard;
pub mod chess_move;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
        match c.to_ascii_lowercase() {
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
