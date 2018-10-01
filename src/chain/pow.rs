/**
 * Functions that deal with PoW
 */

use ramp::Int;
use utils::compact::set_compact;
use consensus::params::ConsensusParams;


/*---- FUNCTIONS ----*/


/**
 * Check whether a block hash satisfies the proof-of-work 
 * requirement specified by `bits`
 */

pub fn check_proof_of_work(hash: &Int, bits: &u32, params: &ConsensusParams) -> bool {
    let mut negative = false;
    let mut overflow = false;
    let target = set_compact(bits, &mut negative, &mut overflow);

    // Range check
    if 
       negative || overflow ||
       target == Int::zero() ||
       target > params.pow_limit
    {
        return false;
    }

    // Check PoW matches the claimed amount
    if *hash > target {
        return false;
    }

    true
}