use crate::types::chess_move::Move;
use crate::types::AttackDirection::EAST;

const TABLE_SIZE:usize = 15_000_000;
const ENTRIES_PER_CLUSTER:usize = 4;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryFlag{
    ALPHA,
    EXACT,
    BETA,
    NULL,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Entry {
    pub key: u64,
    pub flag: EntryFlag,
    pub value: isize,
    pub move_: Move,
    pub depth: u8,
}
impl Entry {
    pub fn null() -> Entry {
        Entry {
            key: 0,
            flag: EntryFlag::NULL,
            value: 0,
            move_: Move::null(),
            depth: 0
        }
    }
}

pub struct Cluster {
    entries: [Entry; ENTRIES_PER_CLUSTER]
}

pub struct TranspositionTable {
    data: Vec<Cluster>
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        let mut data = Vec::with_capacity(TABLE_SIZE);
        for _ in 0..TABLE_SIZE {
            data.push(Cluster{ entries: [Entry::null(); ENTRIES_PER_CLUSTER] })
        }
        TranspositionTable {
            data
        }
    }

    pub fn insert(&mut self, zobrist_key:u64, entry: Entry){
        let cluster:&mut Cluster = &mut self.data[zobrist_key as usize % TABLE_SIZE];
        //Replace the entry with the lowest depth (prefer greater depth entries)
        let mut lowest_depth = u8::MAX;
        let mut lowest_depth_index = 0;
        for i in 0..ENTRIES_PER_CLUSTER {
            if cluster.entries[i].depth <= lowest_depth {
                lowest_depth = cluster.entries[i].depth;
                lowest_depth_index = i;
            }
        }
        cluster.entries [lowest_depth_index] = entry;
    }

    pub fn retrieve(&mut self, zobrist_key:u64) -> Option<&Entry> {
        let cluster = &self.data[zobrist_key as usize % TABLE_SIZE];
        for entry in &cluster.entries {
            if entry.key == zobrist_key && entry.flag != EntryFlag::NULL {
                return Some(entry)
            }
        }
        None
    }
}