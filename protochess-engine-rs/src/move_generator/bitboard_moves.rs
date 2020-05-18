use crate::types::bitboard::Bitboard;
use crate::types::Move;

//Iterator that converts a Bitboard to Moves
pub struct BitboardMoves {
    pub(crate) enemies: Bitboard, //Enemies
    pub(crate) moves:Bitboard,        //moveset for source piece
    pub(crate) source_index: u8       //Source piece index
}
impl Iterator for BitboardMoves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.moves.is_zero() {
            None
        }else{
            let to = self.moves.lowest_one().unwrap();
            self.moves.set_bit(to, false);
            if self.enemies.bit(to).unwrap() {
                Some(Move::new(self.source_index as u8, to as u8,true))
            }else{
                Some(Move::new(self.source_index as u8, to as u8,false))
            }
        }
    }
}


