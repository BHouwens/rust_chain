/** 
 * SATOSHI NAKAMOTO (BITCOIN TEAM):
 * 
 * No amount larger than this (in satoshi) is valid. (i.e. total number of satoshis)
 *
 * Note that this constant is *not* the total money supply, which in Bitcoin
 * currently happens to be less than 21,000,000 BTC for various reasons, but
 * rather a sanity check. As this sanity check is used by consensus-critical
 * validation code, the exact value of the MAX_MONEY constant is consensus
 * critical; in unusual circumstances like a(nother) overflow bug that allowed
 * for the creation of coins out of thin air modification could lead to a fork.
 */

/**
 * BYRON HOUWENS:
 * 
 * If the intent is to divide the number of whole coins based on a separate 
 * consensus then this would be the place to do it. The values below (COINS = 100000000
 * and MAX_MONEY = 21000000 * COINS) are directly from the Bitcoin protocol.
 */

pub const COINS: u64 = 100000000; // whole coins
const MAX_MONEY: u64 = 21000000 * COINS;

pub fn is_valid_amount(value: &u64) -> bool {
    value >= &0 && value <= &MAX_MONEY
}