use crate::types::PieceType;
use crate::position::movement_pattern::MovementPattern;
use crate::types::bitboard::Bitboard;

pub struct Piece {
    pub char_rep: char,
    pub piece_type: PieceType,
    pub bitboard: Bitboard
}

impl Piece {
    pub fn blank_custom(char_rep: char) -> Piece {
       Piece {
           char_rep,
           piece_type: PieceType::Custom(char_rep),
           bitboard: Bitboard::zero()
       }
    }

    pub fn blank_pawn() -> Piece{
        Piece {
            char_rep: 'p',
            piece_type: PieceType::Pawn,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_knight() -> Piece{
        Piece {
            char_rep: 'n',
            piece_type: PieceType::Knight,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_king() -> Piece{
        Piece {
            char_rep: 'k',
            piece_type: PieceType::King,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_rook() -> Piece{
        Piece {
            char_rep: 'r',
            piece_type: PieceType::Rook,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_bishop() -> Piece{
        Piece {
            char_rep: 'b',
            piece_type: PieceType::Bishop,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_queen() -> Piece{
        Piece {
            char_rep: 'q',
            piece_type: PieceType::Queen,
            bitboard: Bitboard::zero()
        }
    }
}