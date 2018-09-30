/**
 * Generation and handling of signing and encryption keys, 
 * for use in transactions and blocks.
 * 
 * Bitcoin uses ECDSA seckp256 for its keypair generation. 
 */

use sodiumoxide::crypto::sign;
