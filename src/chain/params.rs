use ramp::Int;
use utils::amount::COINS;
use net::address::AddressSet;
use std::collections::HashMap;
use consensus::params::ConsensusParams;
use primitives::block::{ Block, create_genesis_block };

/// Base58 encoding representation
enum Base58Type {
    pubkey_address,
    script_address,
    secret_key,
    ext_public_key,
    ext_secret_key,
    max_base58_types
}


/*----- STRUCTS -----*/

/// Holds various statistics on transactions within a chain. Used to estimate
/// verification progress during chain sync.
///
/// See also: ChainParams::TxData, GuessVerificationProgress. (original Bitcoin)
pub struct ChainTxData {
    time: i64,      // UNIX timestamp of last known number of transactions
    tx_count: i64,  // total number of transactions between genesis and that timestamp
    tx_rate: f64    // estimated number of transactions per second after that timestamp (type might change)
}


/// ChainParams defines various tweakable parameters of a given instance of the
/// Bitcoin system. 
/// 
/// There are three: 
///      - the main network on which people trade goods and services
///      - the public test network which gets reset from time to time
///      - a regression test mode which is intended for private networks only. It has minimal difficulty to ensure that blocks can be found instantly.
pub struct ChainParams {
    consensus: ConsensusParams,
    default_port: u32,
    prune_after_height: Option<u64>,
    seeds: Vec<AddressSet>,
    base58_prefixes: Vec<u8>,
    bech32_hrp: String,
    network_id: String,
    genesis_block: Block,
    default_consistency_checks: bool,
    require_standard: bool,
    mine_blocks_on_demand: bool,
    fallback_fee_enabled: bool,
    chain_tx_data: ChainTxData,
    fixed_seeds: Vec<AddressSet>,
    checkpoint_data: HashMap<u64, Int>,
    message_start: Vec<u8> // possibly String?
}


/*----- IMPLEMENTATIONS -----*/

impl ChainParams {
    fn new(network_type: &str) -> ChainParams {
        match network_type {
            "main" => get_main_params(),
            "test" => get_main_params(),        // get_test_params()
            "regression" => get_main_params(),  // get_regression_params()
            _ => get_main_params()
        }
    }
}


/*----- FUNCTIONS ------*/


/// Main network params
fn get_main_params() -> ChainParams {
    let mut consensus = ConsensusParams::new();

    // Handle consensus
    consensus.subsidy_halving_interval = 21000;
    consensus.bip16_exception = Int::from_str_radix("00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22", 16).unwrap();
    consensus.bip34_height = 227931;
    consensus.bip34_hash = Int::from_str_radix("000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8", 16).unwrap();
    consensus.bip65_height = 388381;
    consensus.bip66_height = 363725;

    consensus.pow_limit = Int::from_str_radix("00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();
    consensus.pow_target_timespan = 14 * 24 * 60 * 60;  // two weeks
    consensus.pow_target_spacing = 10 * 60;             // 10 minutes
    consensus.rule_change_activation_threshold = 1916;  // 95% of 2016
    consensus.miner_confirmation_window = consensus.difficulty_adjustment_interval(); // should be 2016
    // consensus.vDeployments[Consensus::DEPLOYMENT_TESTDUMMY].bit = 28;
    // consensus.vDeployments[Consensus::DEPLOYMENT_TESTDUMMY].nStartTime = 1199145601; // January 1, 2008
    // consensus.vDeployments[Consensus::DEPLOYMENT_TESTDUMMY].nTimeout = 1230767999; // December 31, 2008

    // Deployment of BIP68, BIP112, and BIP113.
    // consensus.vDeployments[Consensus::DEPLOYMENT_CSV].bit = 0;
    // consensus.vDeployments[Consensus::DEPLOYMENT_CSV].nStartTime = 1462060800; // May 1st, 2016
    // consensus.vDeployments[Consensus::DEPLOYMENT_CSV].nTimeout = 1493596800; // May 1st, 2017

    // Deployment of SegWit (BIP141, BIP143, and BIP147)
    // consensus.vDeployments[Consensus::DEPLOYMENT_SEGWIT].bit = 1;
    // consensus.vDeployments[Consensus::DEPLOYMENT_SEGWIT].nStartTime = 1479168000; // November 15th, 2016.
    // consensus.vDeployments[Consensus::DEPLOYMENT_SEGWIT].nTimeout = 1510704000; // November 15th, 2017.

    // The best chain should have at least this much work.
    consensus.minimum_chain_work = Int::from_str_radix("0000000000000000000000000000000000000000028822fef1c230963535a90d", 16).unwrap();

    // By default assume that the signatures in ancestors of this block are valid.
    consensus.default_assume_valid = Int::from_str_radix("0000000000000000002e63058c023a9a1de233554f28c7b21380b6c9003f36a8", 16).unwrap();

    // The message start string is designed to be unlikely to occur in normal data.
    // The characters are rarely used upper ASCII, not valid as UTF-8, and produce
    // a large 32-bit integer with any alignment.
    let message = vec![0xf9, 0xbe, 0xb4, 0xd9];

    // Handle genesis
    let genesis = create_genesis_block(1231006505, 2083236893, 0x1d00ffff, 1, 50 * COINS);
    // let genesis_hash = 
    // consensus.genesis_block_hash = genesis_hash;

    // Checkpoint data
    let mut checkpoint_data = HashMap::new();
    checkpoint_data.insert(11111, Int::from_str_radix("0000000069e244f73d78e8fd29ba2fd2ed618bd6fa2ee92559f542fdb26e7c1d", 16).unwrap());
    checkpoint_data.insert(33333, Int::from_str_radix("000000002dd5588a74784eaa7ab0507a18ad16a236e7b1ce69f00d7ddfb5d0a6", 16).unwrap());
    checkpoint_data.insert(74000, Int::from_str_radix("0000000000573993a3c9e41ce34471c079dcf5f52a0e824a81e7f953b8661a20", 16).unwrap());
    checkpoint_data.insert(105000, Int::from_str_radix("00000000000291ce28027faea320c8d2b054b2e0fe44a773f3eefb151d6bdc97", 16).unwrap());

    // TX data
    let chain_tx_data = ChainTxData {
        time: 1532884444,
        tx_count: 331282217,
        tx_rate: 2.4
    };


    // HANDLE SEEDS HERE (BELOW ARE BITCOIN EXAMPLES)

    // vSeeds.emplace_back("testnet-seed.bitcoin.jonasschnelli.ch");
    // vSeeds.emplace_back("seed.tbtc.petertodd.org");
    // vSeeds.emplace_back("seed.testnet.bitcoin.sprovoost.nl");
    // vSeeds.emplace_back("testnet-seed.bluematt.me"); // Just a static list of stable node(s), only supports x9


    ChainParams {
        consensus: consensus,
        network_id: String::from("main"),
        default_port: 8333,
        prune_after_height: Some(100000),
        genesis_block: genesis,
        bech32_hrp: String::from("bc"),
        fixed_seeds: Vec::new(),
        default_consistency_checks: false,
        require_standard: true,
        mine_blocks_on_demand: false,
        fallback_fee_enabled: false,
        checkpoint_data: checkpoint_data,
        chain_tx_data: chain_tx_data,
        message_start: message,
        seeds: Vec::new(),                  // change this when real seeds come along
        base58_prefixes: Vec::new()         // change this guy too
    }
}