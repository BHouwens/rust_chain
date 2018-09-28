/**
 * Parameters that influence chain consensus.
 */

pub struct ConsensusParams {
    genesis_block_hash: String,
    subsidy_halving_interval: u8,
    bip16_exception: String,          // Block hash that is excepted from BIP16 enforcement
    bip34_height: u64,                // Block height and hash at which BIP34 becomes active
    bip34_hash: String,
    bip65_height: u64,                // Block height at which BIP65 becomes active
    bip66_height: u64,                // Block height at which BIP66 becomes active

    /**
     * Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
     * (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
     * Examples: 1916 for 95%, 1512 for testchains.
     */
    rule_change_activation_threshold: u32,
    miner_confirmation_window: u32,
    // BIP9Deployment vDeployments[MAX_VERSION_BITS_DEPLOYMENTS];

    // PoW Parameters
    // These params tend to be associated with mining difficulty
    pow_limit: String,
    pow_allow_min_difficulty_blocks: bool,
    pow_no_retargeting: bool,
    pow_target_spacing: i64,
    pow_target_timespan: i64,
    minimum_chain_work: String,
    default_assume_valid: String,
}

impl ConsensusParams {
    fn new() -> ConsensusParams {
        ConsensusParams {
            genesis_block_hash: String::from("")
            subsidy_halving_interval: 0,
            bip16_exception: String::from("")
            bip34_height: 0,      
            bip34_hash: String::from("")
            bip65_height: 0, 
            bip66_height: 0,
            rule_change_activation_threshold: 0,
            miner_confirmation_window: 0,
            // BIP9Deployment vDeployments[MAX_VERSION_BITS_DEPLOYMENTS];
            pow_limit: String::from("")
            pow_allow_min_difficulty_blocks: false,
            pow_no_retargeting: false,
            pow_target_spacing: 0,
            pow_target_timespan: 0,
            minimum_chain_work: String::from("")
            default_assume_valid: String::from("")
        }
    }

    fn difficulty_adjustment_interval(&self) -> i64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
