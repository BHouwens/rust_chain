/**
 * Handling of "coins" for the chain. Important to note 
 * that coins as such do not exist in blockchains, only 
 * the ability to spend the value of a coin
 */

use ramp::Int;
use primitives::transaction::{ TxOut, OutPoint };
use cryptography::hash::hash_message;
use std::collections::HashMap;


/*---- CONSTANTS ----*/

const SALT_1: [u8; 4] = [0,1,2,3];
const SALT_2: [u8; 4] = [4,5,6,7];


/*---- TRAITS ----*/

/// Abstract view of the open txout dataset
///  
/// Equivalent to Bitcoin's CCoinsView
trait CoinSet {

    /// Retrieve the Coin (unspent transaction output) for a given outpoint.
    /// Returns true only when an unspent coin was found, which is returned in coin.
    /// When false is returned, coin's value is unspecified.
    fn get_coin(outpoint: &OutPoint, coin: &Coin) -> bool;

    /// Just check whether a given outpoint is unspent.
    fn have_coin(outpoint: &OutPoint) -> bool;

    /// Retrieve the block hash whose state this CoinSet currently represents
    fn get_best_block() -> Int;

    /// Retrieve the range of blocks that may have been only partially written.
    /// If the database is in a consistent state, the result is the empty vector.
    /// Otherwise, a two-element vector is returned consisting of the new and
    /// the old block hash, in that order.
    fn get_head_blocks() -> Vec<Int>;

    // BATCH WRITE
    
}


/*---- STRUCTS ----*/


/// A UTXO entry.
///
/// Serialized format:
/// - VARINT((coinbase ? 1 : 0) | (height << 1))
/// - the non-spent CTxOut (via CTxOutCompressor)
pub struct Coin {
    pub out: TxOut,         // unspent tx output
    pub is_coinbase: bool,  // whether containing transaction was coinbase
    pub height: u64         // at which height this containing tx was included in the active block chain
}

/// Equivalent of Bitcoin's CCoinsViewCache
pub struct CoinSetCache {
    block_hash: Int,
    coins_cache: HashMap<String, Coin>
}


/*---- IMPLEMENTATIONS ----*/

impl Coin {
    fn new() -> Coin {
        Coin {
            out: TxOut::new(),
            is_coinbase: false,
            height: 0
        }
    }

    fn new_from_data(tx_out: TxOut, is_coinbase: bool, height: u64) -> Coin {
        Coin {
            out: tx_out,
            is_coinbase: is_coinbase,
            height: height
        }
    }

    fn is_spent(&self) -> bool {
        self.out.value.is_none()
    }
}

impl CoinSetCache {
    pub fn new() -> CoinSetCache {
        CoinSetCache {
            block_hash: Int::zero(),
            coins_cache: HashMap::new()
        }
    }

    // pub fn fetch_coin(&mut self, outpoint: &OutPoint) -> (String, Coin) {
    //     // Not sure what salts will be at this point
    //     let key_to_find = salt_and_hash_outpoint(&SALT_1, &SALT_2, outpoint);
    //     let potential_coin = self.coins_cache.get(&key_to_find).unwrap();

    //     // if potential_coin.is_none() {

    //     // }
        
    //     (key_to_find, potential_coin)
    // }
}


/*---- FUNCTIONS ----*/

pub fn salt_and_hash_outpoint(first_salt: &[u8], second_salt: &[u8], outpoint: &OutPoint) -> String {
    let mut message = Vec::<u8>::new();
    let outpoint_clone = outpoint.clone();

    message.extend(first_salt);
    message.extend(second_salt);
    message.extend(outpoint_clone.hash);
    message.push(outpoint_clone.n as u8);

    let result = match String::from_utf8(hash_message(&message)) {
        Ok(r) => r,
        Err(e) => panic!("Output of hash and salt outpoint is invalid: {}", e)
    };

    result
}