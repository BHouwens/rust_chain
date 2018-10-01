/**
 * Structs and functions to deal largely with writing and reading 
 * block information to and from disk
 */

use ramp::Int;
use primitives::block::BlockHeader;


/*--- STRUCTS ---*/

#[derive(Clone, Debug)]
pub struct BlockFileInfo {
    blocks: u32,
    size: u32,
    undo_size: u8,
    height_first: u8,
    height_last: u8,
    time_first: u64,
    time_last: u64,
}

impl BlockFileInfo {
    pub fn new() -> BlockFileInfo {
        BlockFileInfo {
            blocks: 0,
            size: 0,
            undo_size: 0,
            height_first: 0,
            height_last: 0,
            time_first: 0,
            time_last: 0,
        }
    }

    pub fn add_block(&mut self, height_in: &u8, time_in: &u64) {
        if self.blocks == 0 || self.height_first > *height_in {
            self.height_first = *height_in;
        }

        if self.blocks == 0 || self.time_first > *time_in {
            self.time_first = *time_in;
        }

        if *height_in > self.height_last {
            self.height_last = *height_in;
        }

        if *time_in > self.time_last {
            self.time_last = *time_in;
        }

        // Increment blocks
        self.blocks += 1;
    }
}

/**
 * Block position in disk
 */

#[derive(Clone, Debug)]
pub struct DiskBlockPosition {
    pub position: u8,
    pub file: i16
}

impl DiskBlockPosition {
    pub fn new() -> DiskBlockPosition {
        DiskBlockPosition {
            position: 0,
            file: -1
        }
    }

    pub fn is_null(&self) -> bool {
        self.file == -1
    }
}

/**
 * The block chain is a tree shaped structure starting with the genesis block at the root, 
 * with each block potentially having multiple candidates to be the next block. 
 * 
 * A BlockIndex may have multiple indices ("previous") pointing to it, but at most one of 
 * them can be part of the currently active branch.
 * 
 * In this implementation I've replaced version, nonce, bits etc from the Bitcoin version 
 * with a BlockHeader struct. The only thing the BlockHeader has which this struct does not need
 * is a previous_hash (32 bytes) (*BYRON*) -- Consider reworking.
 */

pub enum BlockStatus {
    ValidityUnknown = 0,
    ValidHeader = 1,
    ValidTree = 2,
    ValidTransactions = 3,
    ValidChain = 4,
    ValidScripts = 5,
    HaveData = 8, 
    HaveUndo = 16, 
    
    FailedValidity = 32, 
    FailedChild = 64, 
    OptWitness = 128
}

#[derive(Clone, Debug)]
pub struct BlockIndex {
    pub block_hash: Int,
    previous: Option<Box<BlockIndex>>,
    skip: Option<Box<BlockIndex>>,
    height: u8,

    pub disk_position: DiskBlockPosition,

    undo_position: u8,
    chain_work: Int,
    tx: u8,
    chain_tx: u64,
    status: u32,
    sequence_id: i32,
    pub time_max: u32,

    pub version: u32,
    pub merkle_root_hash: Int,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32
}

impl BlockIndex {
    pub fn new() -> BlockIndex {
        BlockIndex {
            block_hash: Int::zero(),
            previous: None,
            skip: None,
            height: 0,
            
            disk_position: DiskBlockPosition::new(),

            undo_position: 0,
            chain_work: Int::zero(),
            tx: 0,
            chain_tx: 0,
            status: 0,
            sequence_id: 0,
            time_max: 0,
            
            version: 0,
            merkle_root_hash: Int::zero(),
            time: 0,
            bits: 0,
            nonce: 0
        }
    }

    // Build from an existing block header
    pub fn new_from_header(header: &BlockHeader) -> BlockIndex {
        let mut block_index = BlockIndex::new();

        block_index.version = header.version;
        block_index.merkle_root_hash = header.merkle_root_hash.clone();
        block_index.time = header.time;
        block_index.bits = header.bits;
        block_index.nonce = header.nonce;

        block_index
    }

    // Get undo position
    pub fn get_undo_position(&self) -> DiskBlockPosition {
        let mut undo_pos = DiskBlockPosition::new();

        if self.status as i32 & BlockStatus::HaveUndo as i32 > 0 {
            undo_pos.file = self.disk_position.file;
            undo_pos.position = self.undo_position;
        }

        undo_pos
    }

    // Get internal "block header"
    pub fn get_block_header(&self) -> BlockHeader {
        let mut block_header = BlockHeader::new();

        block_header.version = self.version;
        block_header.merkle_root_hash = self.merkle_root_hash.clone();
        block_header.nonce = self.nonce;
        block_header.time = self.time;
        block_header.bits = self.bits;

        if !self.previous.is_none() {
            let previous_block = self.previous.clone().unwrap();
            block_header.previous_hash = previous_block.block_hash;
        }

        block_header
    }

    // Check validity of blocks up to a passed point (default 3)
    pub fn is_valid_to(&self, up_to: BlockStatus) -> bool {
        if (
            (self.status as i32 & BlockStatus::FailedValidity as i32 > 0) ||
            (self.status as i32 & BlockStatus::FailedChild as i32 > 0)
        ) {
            return false;
        }

        

        false
    }
}