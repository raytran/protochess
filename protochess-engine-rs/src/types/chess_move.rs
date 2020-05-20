pub enum MoveType {
    QUIET,
    CAPTURE,
    CASTLE,
    PROMOTION,
}
#[derive(Copy, Clone)]
pub struct Move(u32);

//Stores a move in a u32
//0-7:   from index:u8
//8-15:  to index:u8
//16-23: target index:u8
//24-25 : movetype
// 00 = quiet
// 01 = capture
// 10 = castle
// 11 = promotion

impl Move {
    pub fn new(from:u8, to:u8, target_loc: u8, move_type:MoveType) -> Move{
        Move(
            (from as u32)
                | (to as u32) << 8u32
                | (target_loc as u32) << 16u32
                | match move_type {
                MoveType::QUIET => {0}
                MoveType::CAPTURE => {1u32 << 24}
                MoveType::CASTLE => {2u32 << 24}
                MoveType::PROMOTION => {3u32 << 24}
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

    pub fn get_target(&self) -> u8 {
        ((self.0 >> 16) & 255u32) as u8
    }
}
