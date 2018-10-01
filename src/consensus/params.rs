use ramp::Int;

/**
 * Parameters that influence chain consensus.
 */

pub struct ConsensusParams {
    pub genesis_block_hash: Vec<u8>,
    pub subsidy_halving_interval: u8,
    pub bip16_exception: Int,   // Block hash that is excepted from BIP16 enforcement
    pub bip34_height: u64,                // Block height and hash at which BIP34 becomes active
    pub bip34_hash: Int,
    pub bip65_height: u64,                // Block height at which BIP65 becomes active
    pub bip66_height: u64,                // Block height at which BIP66 becomes active

    /**
     * Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
     * (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
     * Examples: 1916 for 95%, 1512 for testchains.
     */
    pub rule_change_activation_threshold: u32,
    pub miner_confirmation_window: i64,
    // BIP9Deployment vDeployments[MAX_VERSION_BITS_DEPLOYMENTS];

    // PoW Parameters
    // These params tend to be associated with mining difficulty
    pub pow_limit: Int,
    pub pow_allow_min_difficulty_blocks: bool,
    pub pow_no_retargeting: bool,
    pub pow_target_spacing: i64,
    pub pow_target_timespan: i64,
    pub minimum_chain_work: Int,
    pub default_assume_valid: Int,
}

impl ConsensusParams {
    pub fn new() -> ConsensusParams {
        ConsensusParams {
            genesis_block_hash: Vec::new(),
            subsidy_halving_interval: 0,
            bip16_exception: Int::zero(),
            bip34_height: 0,      
            bip34_hash: Int::zero(),
            bip65_height: 0, 
            bip66_height: 0,
            rule_change_activation_threshold: 0,
            miner_confirmation_window: 0,
            // BIP9Deployment vDeployments[MAX_VERSION_BITS_DEPLOYMENTS];
            pow_limit: Int::zero(),
            pow_allow_min_difficulty_blocks: false,
            pow_no_retargeting: false,
            pow_target_spacing: 0,
            pow_target_timespan: 0,
            minimum_chain_work: Int::zero(),
            default_assume_valid: Int::zero()
        }
    }

    pub fn difficulty_adjustment_interval(&self) -> i64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
