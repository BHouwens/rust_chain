use uint::U256;
use primitives::SEQUENCE_FINAL;

/** 
 * An outpoint - a combination of a transaction hash and an index n into its vout 
 */

#[derive(Clone, Debug)]
pub struct OutPoint {
    pub hash: U256,
    pub n: i32
}

impl OutPoint {
    fn new(hash: U256, n: i32) -> OutPoint {
        OutPoint {
            hash: hash,
            n: n
        }
    }

    fn is_less_than(&self, comparison: &OutPoint) -> bool {
        if self.hash == comparison.hash {
            return self.n < comparison.n;
        }

        self.hash < comparison.hash
    }

    fn is_equal_to(&self, comparison: &OutPoint) -> bool {
        self.hash == comparison.hash && self.n == comparison.n
    }

    fn is_not_equal_to(&self, comparison: &OutPoint) -> bool {
        self.hash != comparison.hash || self.n != comparison.n
    }
}


/** 
 * An input of a transaction. It contains the location of the previous
 * transaction's output that it claims and a signature that matches the
 * output's public key.
 */

#[derive(Clone, Debug)]
pub struct TxIn {
    previous_out: OutPoint,
    sequence: u32
    // need scriptsig and scriptwitness
}

impl TxIn {
    fn new(out: OutPoint, sequence: u32) -> TxIn {
        TxIn {
            previous_out: out,
            sequence: sequence
        }
    }

    fn is_equal_to(&self, comparison: &TxIn) -> bool {
        
    }
}