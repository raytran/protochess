use crate::types::Dimensions;
use crate::types::{Move, LineAttackType, AttackDirection };
use crate::types::bitboard::Bitboard;
use crate::position::Position;
use crate::move_generator::mask_handler::MaskHandler;

mod mask_handler;
pub struct MoveGenerator {
    masks: MaskHandler,
    dimensions: Dimensions,
}

impl MoveGenerator {
    pub(crate) fn new(dimensions:&Dimensions) -> MoveGenerator{
        MoveGenerator {
            masks: MaskHandler::new(&dimensions),
            dimensions: Dimensions{width: dimensions.width, height:dimensions.height},
        }
    }

    pub(crate) fn generate_moves(&self, position: &Position, whos_turn:u8) -> Vec<Move> {
        let mut moves = Vec::new();
        self.generate_all_pawn_moves(position, &mut moves, whos_turn);
        self.generate_all_queen_moves(position,&mut moves, whos_turn);
        self.generate_all_rook_moves(position,&mut moves, whos_turn);
        self.generate_all_bishop_moves(position,&mut moves, whos_turn);
        self.generate_all_knight_moves(position, &mut moves, whos_turn);
        self.generate_all_king_moves(position, &mut moves, whos_turn);
        //TODO custom pieces using classical approach
        moves
    }

    //generate_all_* methods:
    //Applies generate_*_move for every * in position
    //Applies generate_queen_move for every queen in the position
    fn generate_all_queen_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn: u8){
        let pieces = &position.pieces[whos_turn as usize];
        let q_copy = (&pieces.queen).to_owned();
        self.generate_all_piece_moves(position,movelist,whos_turn,q_copy,
                                      MoveGenerator::generate_queen_moves);
    }

    fn generate_all_rook_moves(&self, position: &Position, movelist:&mut Vec<Move>, whos_turn:u8){
        let pieces = &position.pieces[whos_turn as usize];
        let r_copy = (&pieces.rook).to_owned();
        self.generate_all_piece_moves(position,movelist,whos_turn,r_copy,
                                      MoveGenerator::generate_rook_moves);
    }

    fn generate_all_bishop_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn:u8){
        let pieces = &position.pieces[whos_turn as usize];
        let b_copy = (&pieces.bishop).to_owned();

        self.generate_all_piece_moves(position,movelist,whos_turn,b_copy,
                                      MoveGenerator::generate_bishop_moves);
    }

    fn generate_all_knight_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn:u8){
        let pieces = &position.pieces[whos_turn as usize];
        let n_copy = (&pieces.knight).to_owned();
        self.generate_all_piece_moves(position,movelist,whos_turn,n_copy,
                                      MoveGenerator::generate_knight_moves);
    }

    fn generate_all_king_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn:u8){
        let pieces = &position.pieces[whos_turn as usize];
        let k_copy = (&pieces.king).to_owned();
        self.generate_all_piece_moves(position,movelist,whos_turn,k_copy,
                                      MoveGenerator::generate_king_moves);
    }

    //Generates all moves for a given piece bitboard using a generate function
    fn generate_all_piece_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn:u8,
                                mut all_pieces_of_type: Bitboard,
                                generate_func: fn(&MoveGenerator, &Position,&mut Vec<Move>, u8, usize)){
        //Bishop moves:
        while !all_pieces_of_type.is_zero(){
            let index = all_pieces_of_type.lowest_one().unwrap();
            generate_func(self, &position,  movelist, whos_turn, index);
            all_pieces_of_type.set_bit(index,false);
        }
    }

    fn generate_all_pawn_moves(&self, position: &Position, movelist: &mut Vec<Move>, whos_turn: u8) {
        //Pawns are different in that we can calculate moves for the whole set at once
        if whos_turn == 0 {
            self.generate_all_pawn_pushes(position, movelist, whos_turn, true);
            self.generate_all_pawn_captures(position, movelist, whos_turn, true );
        }else{
            self.generate_all_pawn_pushes(position, movelist, whos_turn, false);
            self.generate_all_pawn_captures(position, movelist, whos_turn, false);
        }
    }

    //Appends to movelist moves for a queen at source_index
    fn generate_knight_moves(&self, position: &Position, movelist: &mut Vec<Move>,
                             whos_turn:u8, source_index:usize) {
        let enemies = &position.occupied & !&position.pieces[whos_turn as usize].occupied;

        let mut moves = self.masks.get_attack(&AttackDirection::KNIGHT,source_index).to_owned();
        //Don't attack ourselves
        moves &= !&position.pieces[whos_turn as usize].occupied;
        MoveGenerator::bitboard_to_moves(movelist, moves, &enemies, source_index);
    }

    fn generate_king_moves(&self, position: &Position, movelist: &mut Vec<Move>,
                           whos_turn:u8, source_index:usize) {
        let enemies = &position.occupied & !&position.pieces[whos_turn as usize].occupied;
        let mut moves = self.masks.get_attack(&AttackDirection::KING,source_index).to_owned();
        //Don't attack ourselves
        moves &= !&position.pieces[whos_turn as usize].occupied;
        MoveGenerator::bitboard_to_moves(movelist, moves, &enemies, source_index);
    }

    //Appends to movelist moves for a queen at source_index
    fn generate_queen_moves(&self, position: &Position, movelist: &mut Vec<Move>,
                            whos_turn:u8, source_index:usize) {
        self.generate_rook_moves(position ,movelist, whos_turn, source_index);
        self.generate_bishop_moves(position, movelist, whos_turn, source_index);
    }

    //Appends to movelist moves for a rook at source_index
    fn generate_rook_moves(&self, position: &Position, movelist: &mut Vec<Move>,
                           whos_turn:u8, source_index:usize) {
        self.line_attacks(position, movelist, whos_turn, source_index, &LineAttackType::RANK);
        self.line_attacks(position, movelist, whos_turn, source_index, &LineAttackType::FILE);
    }

    //Appends to movelist moves for a bishop at source_index
    fn generate_bishop_moves(&self, position: &Position, movelist: &mut Vec<Move>,
                             whos_turn:u8, source_index:usize) {
        self.line_attacks(position, movelist, whos_turn, source_index, &LineAttackType::DIAGONAL);
        self.line_attacks(position, movelist, whos_turn, source_index, &LineAttackType::ANTIDIAGONAL);
    }

    //Appends to movelist moves for a single piece at source_index that can move in a LineType
    fn line_attacks(&self, position: &Position, movelist: &mut Vec<Move>,
                    whos_turn:u8, source_index:usize, line_type:&LineAttackType) {
        let occ = &position.occupied;

        let full_mask = self.masks.get_attack(&line_type.get_lower(), source_index)
            | self.masks.get_attack(&line_type.get_upper(), source_index);
        let occ_lower = self.masks.get_attack(&line_type.get_lower(), source_index) & occ;
        let occ_upper = self.masks.get_attack(&line_type.get_upper(), source_index) & occ;

        let mut lsb_upper = Bitboard::zero();
        if let Some(i) = &occ_upper.lowest_one() {
            lsb_upper.set_bit(*i,true);
        }

        let mut msb_lower = Bitboard::zero();
        if let Some(i) = &occ_lower.highest_one(){
            msb_lower.set_bit(*i,true);
        }else{
            msb_lower.set_bit(0,true);
        }

        let twice_lsb_upper:Bitboard = lsb_upper << 1;

        let mut attacks = twice_lsb_upper.overflowing_sub(&msb_lower).0 & full_mask;

        //Don't attack ourselves
        attacks &= !&position.pieces[whos_turn as usize].occupied;

        let enemies = &position.occupied & !&position.pieces[whos_turn as usize].occupied;
        MoveGenerator::bitboard_to_moves(movelist, attacks, &enemies, source_index);
    }

    fn generate_all_pawn_captures(&self, position: &Position, movelist: &mut Vec<Move>,
                                  whos_turn:u8, northfacing: bool) {
        let pawns = &position.pieces[whos_turn as usize].pawn;
        let enemies = &position.occupied & !&position.pieces[whos_turn as usize].occupied;

        if northfacing {
            let north_one = self.masks.shift_north(1,pawns);
            let mut ne_attacks = self.masks.shift_east(1,&north_one) & &enemies;
            let mut nw_attacks = self.masks.shift_west(1,&north_one) & &enemies;

            while !ne_attacks.is_zero() {
                let to = ne_attacks.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width - 1;
                movelist.push(Move::new(from, to,true));
                ne_attacks.set_bit(to as usize, false);
            }
            while !nw_attacks.is_zero() {
                let to = nw_attacks.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width + 1;
                movelist.push(Move::new(from, to,true));
                nw_attacks.set_bit(to as usize, false);
            }
        }else{
            let south_one = self.masks.shift_south(1,pawns);
            let mut se_attacks = self.masks.shift_east(1,&south_one) & &enemies;
            let mut sw_attacks = self.masks.shift_west(1,&south_one) & &enemies;

            while !se_attacks.is_zero() {
                let to = se_attacks.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width - 1;
                movelist.push(Move::new(from, to,true));
                se_attacks.set_bit(to as usize, false);
            }
            while !sw_attacks.is_zero() {
                let to = sw_attacks.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width + 1;
                movelist.push(Move::new(from, to,true));
                sw_attacks.set_bit(to as usize, false);
            }
        }
    }

    fn generate_all_pawn_pushes(&self, position: &Position, movelist: &mut Vec<Move>,
                                whos_turn: u8,northfacing: bool) {
        let pawns = &position.pieces[whos_turn as usize].pawn;
        if northfacing {
            //Single pawn moves
            let empty:Bitboard = !&position.occupied;
            let mut north_one = self.masks.shift_north(1, pawns) & &empty;
            //Double pawn moves
            let mut north_two = self.masks.shift_north(1, &north_one) & &empty;
            while !north_one.is_zero() {
                let to = north_one.lowest_one().unwrap() as u8;
                let from = to - position.dimensions.width;
                movelist.push(Move::new(from, to,false));
                north_one.set_bit(to as usize, false);
            }

            while !north_two.is_zero() {
                let to = north_two.lowest_one().unwrap() as u8;
                let from = to - 2 * position.dimensions.width;
                movelist.push(Move::new(from, to,false));
                north_two.set_bit(to as usize, false);
            }
        }else{
            //Single pawn moves
            let empty:Bitboard = !&position.occupied;
            let mut south_one = self.masks.shift_south(1, pawns) & &empty;
            //Double pawn moves
            let mut south_two = self.masks.shift_south(1, &south_one) & &empty;
            while !south_one.is_zero() {
                let to = south_one.lowest_one().unwrap() as u8;
                let from = to + position.dimensions.width;
                movelist.push(Move::new(from, to,false));
                south_one.set_bit(to as usize, false);
            }

            while !south_two.is_zero() {
                let to = south_two.lowest_one().unwrap() as u8;
                let from = to + 2 * position.dimensions.width;
                movelist.push(Move::new(from, to,false));
                south_two.set_bit(to as usize, false);
            }
        }
    }

    //Converts a bitboard to Moves
    fn bitboard_to_moves(movelist: &mut Vec<Move>, mut moves:Bitboard, enemies: &Bitboard, source_index: usize){
        while !moves.is_zero() {
            let to = moves.lowest_one().unwrap();
            if enemies.bit(to).unwrap() {
                movelist.push(Move::new(source_index as u8, to as u8,true));
            }else{
                movelist.push(Move::new(source_index as u8, to as u8,false));
            }
            moves.set_bit(to, false);
        }
    }
}
