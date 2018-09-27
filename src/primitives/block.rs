use uint::U256;

/** 
 * Nodes collect new transactions into a block, hash them into a hash tree,
 * and scan through nonce values to make the block's hash satisfy proof-of-work
 * requirements.  When they solve the proof-of-work, they broadcast the block
 * to everyone and the block is added to the block chain.  The first transaction
 * in the block is a special one that creates a new coin owned by the creator
 * of the block.
 */


#[derive(Clone, Debug)]
pub struct BlockHeader {
    version: i32,
    previous_hash: U256,
    merkle_root_hash: U256,
    time: u32,
    bits: u32,
    nonce: u32
}

impl BlockHeader {
    fn new() -> BlockHeader {
        BlockHeader {
            version: 0,
            previous_hash: U256::zero(),
            merkle_root_hash: U256::zero(),
            time: 0,
            bits: 0,
            nonce: 0
        }
    }

    fn is_null(&self) -> bool {
        self.bits == 0
    }
}


#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    transactions: Vec<TransactionRef>, // need to make transaction prims
    checked: bool // memory only
}

impl Block {
    fn new() -> Block {
        Block {
            header: BlockHeader::new(),
            transactions: Vec::new(),
            checked: false
        }
    }
}


/** 
 * Describes a place in the blockchain to another node such that if the
 * other node doesn't have the same branch, it can find a recent common trunk.
 * The further back it is, the further before the fork it may be.
 */

#[derive(Clone, Debug)]
pub struct BlockLocator {
    have: Vec<U256>
}
