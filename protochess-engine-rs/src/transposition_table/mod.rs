use crate::types::chess_move::Move;

const TABLE_SIZE:usize = 1_000_000;
#[derive(PartialEq, Eq, Debug)]
pub enum EntryFlag{
    ALPHA,
    EXACT,
    BETA,
    NULL,
}

pub struct Entry {
    pub flag: EntryFlag,
    pub value: isize,
    pub move_: Move,
    pub depth: u8,
}

pub struct TranspositionTable {
    data: Vec<Entry>
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let mut data = Vec::with_capacity(TABLE_SIZE);
        for _ in 0..TABLE_SIZE {
            data.push(Entry{
                flag: EntryFlag::NULL,
                value: 0,
                move_: Move::null(),
                depth: 0
            });
        }
        TranspositionTable {
            data
        }
    }

    pub fn insert(&mut self, zobrist_key:u64, entry: Entry){
        self.data[zobrist_key as usize % TABLE_SIZE] = entry;
    }

    pub fn retrieve(&mut self, zobrist_key:u64) -> Option<&Entry> {
        let entry = &self.data[zobrist_key as usize % TABLE_SIZE];
        if entry.flag != EntryFlag::NULL {
            Some(entry)
        }else{
            None
        }
    }
}