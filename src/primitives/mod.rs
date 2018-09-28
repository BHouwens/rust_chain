pub mod block;
pub mod transaction;
pub mod script;

/*---- CONSTANTS ----*/

/* Setting nSequence to this value for every input in a transaction
 * disables nLockTime.
 */

pub const SEQUENCE_FINAL: u32 = 0xffffffff;

/* Below flags apply in the context of BIP 68
 * If this flag set, CTxIn::nSequence is NOT interpreted as a
 * relative lock-time. 
 */

pub const SEQUENCE_LOCKTIME_DISABLE_FLAG: u32 = (1 << 31);

/* If CTxIn::nSequence encodes a relative lock-time and this flag
 * is set, the relative lock-time has units of 512 seconds,
 * otherwise it specifies blocks with a granularity of 1.
 */

pub const SEQUENCE_LOCKTIME_TYPE_FLAG: u32 = (1 << 22);

/* If CTxIn::nSequence encodes a relative lock-time, this mask is
 * applied to extract that lock-time from the sequence field. 
 */

pub const SEQUENCE_LOCKTIME_MASK: u32 = 0x0000ffff;

/* In order to use the same number of bits to encode roughly the
 * same wall-clock duration, and because blocks are naturally
 * limited to occur every 600s on average, the minimum granularity
 * for time-based relative lock-time is fixed at 512 seconds.
 * Converting from CTxIn::nSequence to seconds is performed by
 * multiplying by 512 = 2^9, or equivalently shifting up by
 * 9 bits. 
 */

pub const SEQUENCE_LOCKTIME_GRANULARITY: u8 = 9;