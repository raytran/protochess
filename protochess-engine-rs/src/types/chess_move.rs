pub enum MoveType {
    Quiet,
    Capture,
    QueensideCastle,
    KingsideCastle,
    Promotion,
    PromotionCapture,
}
#[derive(Copy, Clone)]
pub struct Move(u32);

//Stores a move in a u32
//0-7:   from index:u8
//8-15:  to index:u8
//16-23: target index:u8
//24-26 : movetype
// 000 = quiet
// 001 = capture
// 010 = castle
// 011 = promotion
// 100 = promotion-capture

impl Move {
    pub fn new(from:u8, to:u8, target_loc: u8, move_type:MoveType) -> Move{
        Move(
            (from as u32)
                | (to as u32) << 8u32
                | (target_loc as u32) << 16u32
                | match move_type {
                MoveType::Quiet => {0}
                MoveType::Capture => {1u32 << 24}
                MoveType::KingsideCastle => {2u32 << 24}
                MoveType::QueensideCastle => {3u32 << 24}
                MoveType::Promotion => {4u32 << 24}
                MoveType::PromotionCapture => {5u32 << 24}
            }
        )
    }

    pub fn get_from(&self) -> u8{
        (self.0 & 255u32) as u8
    }

    pub fn get_to(&self) -> u8{
        ((self.0 >> 8) & 255u32) as u8
    }

    pub fn get_is_capture(&self) -> bool{
        ((self.0 >> 24) & 1u32) != 0u32
    }

    pub fn get_move_type(&self) -> MoveType {
        match &self.0 >> 24 & 7u32 {
            0 => { MoveType::Quiet }
            1 => { MoveType::Capture }
            2 => { MoveType::KingsideCastle }
            3 => { MoveType::QueensideCastle }
            4 => { MoveType::Promotion }
            5 => { MoveType::PromotionCapture }
            _ => { MoveType::Quiet }
        }
    }

    pub fn get_target(&self) -> u8 {
        ((self.0 >> 16) & 255u32) as u8
    }
}
