use crate::types::PieceType;
use crate::types::bitboard::Bitboard;

pub struct Piece {
    pub char_rep: char,
    //Player num for the owner of this piece
    pub player_num: u8,
    pub piece_type: PieceType,
    pub bitboard: Bitboard
}

impl Piece {
    pub fn blank_custom(player_num:u8, char_rep: char) -> Piece {
       Piece {
           player_num,
           char_rep,
           piece_type: PieceType::Custom(char_rep),
           bitboard: Bitboard::zero()
       }
    }

    pub fn blank_pawn(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'p',
            piece_type: PieceType::Pawn,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_knight(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'n',
            piece_type: PieceType::Knight,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_king(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'k',
            piece_type: PieceType::King,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_rook(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'r',
            piece_type: PieceType::Rook,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_bishop(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'b',
            piece_type: PieceType::Bishop,
            bitboard: Bitboard::zero()
        }
    }

    pub fn blank_queen(player_num:u8) -> Piece{
        Piece {
            player_num,
            char_rep: 'q',
            piece_type: PieceType::Queen,
            bitboard: Bitboard::zero()
        }
    }
}