use crate::types::PieceType;
use crate::position::movement_pattern::MovementPattern;
use crate::types::bitboard::Bitboard;

pub struct Piece {
    pub char_rep: char,
    pub piece_type: PieceType,
    //Movement pattern for this piece, if applicable
    pub movement_pattern: Option<MovementPattern>,
    pub bitboard: Bitboard
}

impl Piece {
    pub fn blank_custom(char_rep: char, mp: MovementPattern) -> Piece {
       Piece {
           char_rep,
           piece_type: PieceType::Custom(char_rep),
           movement_pattern: Some(mp),
           bitboard: Bitboard::zero()
       }
    }

    pub fn blank_pawn() -> Piece{
        Piece {
            char_rep: 'p',
            piece_type: PieceType::Pawn,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_knight() -> Piece{
        Piece {
            char_rep: 'n',
            piece_type: PieceType::Knight,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_king() -> Piece{
        Piece {
            char_rep: 'k',
            piece_type: PieceType::King,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_rook() -> Piece{
        Piece {
            char_rep: 'r',
            piece_type: PieceType::Rook,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_bishop() -> Piece{
        Piece {
            char_rep: 'b',
            piece_type: PieceType::Bishop,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_queen() -> Piece{
        Piece {
            char_rep: 'q',
            piece_type: PieceType::Queen,
            movement_pattern: None,
            bitboard: Bitboard::zero()
        }
    }
}