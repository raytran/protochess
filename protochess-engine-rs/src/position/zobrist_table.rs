use crate::PieceType;
use std::collections::HashMap;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use crate::position::piece::Piece;

//Holds the random numbers used in generating zobrist keys
pub struct ZobristTable {
    //zobrist[player_num][piecetype][index] -> zobrist key
    //king = 0
    //queen = 1
    //rook = 2
    //bishop = 3
    //knight = 4
    //pawn = 5
    zobrist: Vec<Vec<Vec<u64>>>,
    //Map of piecetype -> Vec of length 256, one random # for each index for each playernum
    custom_zobrist: HashMap<(u8, PieceType), Vec<u64>>,
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
        let mut zobrist = Vec::with_capacity(4);
        //TODO 2+ players(?)
        zobrist.push(Vec::new());
        zobrist.push(Vec::new());
        for i in 0..2 {
            for j in 0..6 {
                let mut randoms = Vec::with_capacity(256);
                for _ in 0..=255 {
                    randoms.push(rng.gen::<u64>());
                }
                zobrist[i].push(randoms)
            }
        }

        let table = ZobristTable{
            zobrist,
            custom_zobrist: Default::default(),
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

    pub fn get_zobrist_sq_from_pt(&self, pt: &PieceType, owner: u8, index: u8) -> u64 {
        match pt {
            PieceType::King => {
                self.zobrist[owner as usize][0][index as usize]
            }
            PieceType::Queen => {
                self.zobrist[owner as usize][1][index as usize]
            }
            PieceType::Rook => {
                self.zobrist[owner as usize][2][index as usize]
            }
            PieceType::Bishop => {
                self.zobrist[owner as usize][3][index as usize]
            }
            PieceType::Knight => {
                self.zobrist[owner as usize][4][index as usize]
            }
            PieceType::Pawn => {
                self.zobrist[owner as usize][5][index as usize]
            }
            PieceType::Custom(c) => {
                if !self.custom_zobrist.contains_key(&(owner, pt.to_owned())) {
                    return 0;
                    //self.register_piecetype(owner, pt);
                }
                self.custom_zobrist.get(&(owner, pt.to_owned())).unwrap()[index as usize]
            }
        }
    }

    pub fn get_zobrist_sq(&self, piece:&Piece, index:u8) -> u64 {
        self.get_zobrist_sq_from_pt(&piece.piece_type, piece.player_num, index)
    }

    pub fn get_ep_zobrist_file(&self, rank: u8) -> u64 {
        self.ep_zobrist[rank as usize]
    }

    //Registers a custom piece type
    pub fn register_piecetype(&mut self, player_num:u8, pt:&PieceType){
        let randoms = self.make_randoms();
        self.custom_zobrist.insert((player_num, pt.to_owned()), randoms);
    }

    fn make_randoms(&mut self) -> Vec<u64> {
        let mut randoms = Vec::with_capacity(256);
        for i in 0..=255 {
            randoms.push(self.rng.gen::<u64>());
        }
        randoms
    }
}

