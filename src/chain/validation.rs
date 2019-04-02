use ramp::Int;
use std::sync::{ Arc, Mutex };
use std::collections::HashMap;
use chain::chain_block_info::BlockIndex;


/*---- STRUCTS ----*/


/// ChainState stores and provides an API to update our local knowledge of the
/// current best chain and header tree.
///
/// It generally provides access to the current block tree, as well as functions
/// to provide new data, which it will appropriately validate and incorporate in
/// its state as necessary.
///
/// Eventually, the API here is targeted at being exposed externally as a
/// consumable libconsensus library, so any functions added must only call
/// other class member functions, pure functions in other parts of the consensus
/// library, callbacks via the validation interface, or read/write-to-disk
/// functions (eventually this will also be via callbacks).
pub struct ChainState {
    block_index_candidates: HashMap<BlockIndex, u8>, // u8?

    // Every received block is assigned a unique and increasing identifier, so we
    // know which one to give priority in case of a fork.
    block_sequence_id_arc: Arc<u64>,

    // Blocks loaded from disk are assigned id 0, so start the counter at 1.
    block_sequence_id: u64,

    // Decreasing counter (used by subsequent preciousblock calls).
    block_sequence_reverse_id: i64,

    // chainwork for the last block that preciousblock has been applied to.
    last_precious_block_chain_work: Int,
 
    // In order to efficiently track invalidity of headers, we keep the set of
    // blocks which we tried to connect and found to be invalid here (ie which
    // were set to BLOCK_FAILED_VALID since the last restart). We can then
    // walk this set and check if a new header is a descendant of something in
    // this set, preventing us from having to walk mapBlockIndex when we try
    // to connect a bad block and fail.
    //
    // While this is more complicated than marking everything which descends
    // from an invalid block as invalid at the time we discover it to be
    // invalid, doing so would require walking all of mapBlockIndex to find all
    // descendants. Since this case should be very rare, keeping track of all
    // BLOCK_FAILED_VALID blocks in a set should be just fine and work just as
    // well.
    //
    // Because we already walk mapBlockIndex in height-order at startup, we go
    // ahead and mark descendants of invalid blocks as FAILED_CHILD at that time,
    // instead of putting things in this set.

    failed_blocks: Vec<BlockIndex>,

    // the ChainState CriticalSection
    // A lock that must be held when modifying this ChainState - held in ActivateBestChain()
    chain_state_arc: Mutex<u8>
}