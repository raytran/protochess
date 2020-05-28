use crate::PieceType;
use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use crate::position::castle_rights::CastleRights;
use crate::position::piece::Piece;

//Holds the random numbers used in generating zobrist keys
pub struct ZobristTable {
    //Map of piecetype -> Vec of length 256, one random # for each index for each playernum
    zobrist: HashMap<(u8, PieceType), Vec<u64>>,
    ep_zobrist: Vec<u64>,
    white_to_move: u64,
    w_q_castle: u64,
    b_q_castle: u64,
    w_k_castle: u64,
    b_k_castle: u64,
    rng: StdRng
}

impl ZobristTable {
    pub fn new() -> ZobristTable {
        let mut rng = StdRng::seed_from_u64(5435651169991665628);
        let mut ep_zobrist = Vec::with_capacity(16);
        for _ in 0..=16 {
            ep_zobrist.push(rng.gen::<u64>());
        }
        let mut table = ZobristTable{
            zobrist: Default::default(),
            ep_zobrist,
            white_to_move: rng.gen::<u64>(),
            w_q_castle: rng.gen::<u64>(),
            b_q_castle: rng.gen::<u64>(),
            w_k_castle: rng.gen::<u64>(),
            b_k_castle: rng.gen::<u64>(),
            rng
        };
        //Initialize the table with the default piece set in a repeatable way
        //Mostly for easier testing
        table.register_piecetype(0, &PieceType::King);
        table.register_piecetype(0, &PieceType::Queen);
        table.register_piecetype(0, &PieceType::Pawn);
        table.register_piecetype(0, &PieceType::Bishop);
        table.register_piecetype(0, &PieceType::Knight);
        table.register_piecetype(0, &PieceType::Rook);

        table.register_piecetype(1, &PieceType::King);
        table.register_piecetype(1, &PieceType::Queen);
        table.register_piecetype(1, &PieceType::Pawn);
        table.register_piecetype(1, &PieceType::Bishop);
        table.register_piecetype(1, &PieceType::Knight);
        table.register_piecetype(1, &PieceType::Rook);
        table
    }

    /// Zobrist for the player to move
    pub fn get_to_move_zobrist(&self, player_num: u8) -> u64 {
        self.white_to_move
    }

    pub fn get_castling_zobrist(&self, player_num: u8, kingside: bool) -> u64 {
        match (player_num, kingside) {
            (0, true) => {self.w_k_castle}
            (0, false) => {self.w_q_castle}
            (1, true) => {self.b_k_castle}
            (1, false) => {self.b_q_castle}
            _ => {0}
        }
    }

    pub fn get_zobrist_sq_from_pt(&mut self, pt: &PieceType, owner: u8, index: u8) -> u64 {
        if !self.zobrist.contains_key(&(owner, pt.to_owned())) {
            self.register_piecetype(owner, pt);
        }
        self.zobrist.get(&(owner, pt.to_owned())).unwrap()[index as usize]
    }

    pub fn get_zobrist_sq(&mut self, piece:&Piece, index:u8) -> u64 {
        if !self.zobrist.contains_key(&(piece.player_num, (&piece.piece_type).to_owned())) {
            self.register_piecetype(piece.player_num, &piece.piece_type);
        }
        self.zobrist.get(&(piece.player_num, (&piece.piece_type).to_owned())).unwrap()[index as usize]
    }

    pub fn get_ep_zobrist_file(&mut self, rank: u8) -> u64 {
        self.ep_zobrist[rank as usize]
    }

    fn register_piecetype(&mut self, player_num:u8, pt:&PieceType){
        let mut randoms = Vec::with_capacity(256);
        for i in 0..=255 {
            randoms.push(self.rng.gen::<u64>());
        }
        self.zobrist.insert((player_num, pt.to_owned()), randoms);
    }
}

