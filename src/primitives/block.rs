use utils::PSZ_TIMESTAMP;
use primitives::transaction::Transaction;
use std::hash::{ Hash, Hasher };

/** 
 * Nodes collect new transactions into a block, hash them into a hash tree,
 * and scan through nonce values to make the block's hash satisfy proof-of-work
 * requirements.  When they solve the proof-of-work, they broadcast the block
 * to everyone and the block is added to the block chain.  The first transaction
 * in the block is a special one that creates a new coin owned by the creator
 * of the block.
 * 
 * Hashes are represented as Vec<u8> until a better solution comes along
 */


/*---- STRUCTS ----*/

#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub version: i32,
    pub previous_hash: Vec<u8>,
    pub merkle_root_hash: Vec<u8>,
    pub time: u32,
    pub bits: i32,
    pub nonce: u32
}

impl BlockHeader {
    fn new() -> BlockHeader {
        BlockHeader {
            version: 0,
            previous_hash: Vec::with_capacity(33),
            merkle_root_hash: Vec::with_capacity(33),
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
    pub transactions: Vec<Transaction>,
    checked: bool // memory only
}

impl Block {
    pub fn new() -> Block {
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
    have: Vec<u8>
}


/*---- FUNCTIONS ----*/

/**
 * Builds the raw shell of a genesis block
 */

fn create_raw_genesis_block(
    psz_timestamp: &'static str, // this will change
    // genesis_output_script: ??? sort out script struct
    time: &u32,
    nonce: &u32,
    bits: &u32,
    version: &u32,
    genesis_reward: &i64
) -> Block 
{
    // let psz_timestamp = String::from("The Times 03/Jan/2009 Chancellor on brink of second bailout for banks");
    let mut genesis = Block::new();
    // let mut gen_transaction = Transaction::new();
    // let mut tx_in = TxIn::new();
    // let mut tx_out = TxOut::new();

    // // Handle genesis transaction
    // tx_in.script_signature = Some( ToHex(psz_timestamp + psz_timestamp.length()) );
    // tx_out.value = genesis_reward;
    // tx_out.public_key = Some(genesis_output);

    // gen_transaction.inputs.push(tx_in);
    // gen_transaction.outputs.push(tx_out);

    // // Handle block header
    // genesis.header.version = version;
    // genesis.header.bits = bits;
    // genesis.header.nonce = nonce;
    // genesis.header.time = time;

    // // Add genesis transaction
    // genesis.transactions.push(gen_transaction);

    // // Other stuff accepts defaults, so just return the block
    genesis
}


/**
 * Creates a final genesis block for inclusion in the chain
 */

pub fn create_genesis_block(time: u32, nonce: u32, bits: u32, version: u32, genesis_reward: i64) -> Block {
    // Using straight constant in this case, but will need to incorporate some kind of scripting situation
    create_raw_genesis_block(&PSZ_TIMESTAMP, &time, &nonce, &bits, &version, &genesis_reward)
}