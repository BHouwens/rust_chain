use uint::U256;
use primitives::SEQUENCE_FINAL;
use utils::amount::is_valid_amount;

/** 
 * An outpoint - a combination of a transaction hash and an index n into its vout 
 */

#[derive(Clone, Debug, Eq, PartialEq)]
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
}


/** 
 * An input of a transaction. It contains the location of the previous
 * transaction's output that it claims and a signature that matches the
 * output's public key.
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxIn {
    pub previous_out: Option<OutPoint>,
    pub sequence: u32,
    pub script_signature: Option<String>
}

impl TxIn {
    pub fn new() -> TxIn {
        TxIn {
            previous_out: None,
            sequence: 0,
            script_signature: None
        }
    }
}


/** 
 * An output of a transaction. It contains the public key that the next input
 * must be able to sign with to claim it.
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxOut {
    pub value: i64, // amount in satoshis (original bitcoin)
    pub public_key: Option<String>
}

impl TxOut {
    pub fn new() -> TxOut {
        TxOut {
            value: 0,
            public_key: None
        }
    }
}


/** 
 * The basic transaction that is broadcasted on the network and contained in
 * blocks. A transaction can contain multiple inputs and outputs.
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub version: i32,
    pub lock_time: u32
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            version: 0,
            lock_time: 0
        }
    }

    /**
     * Return sum of txouts
     */

    fn get_output_value(&self) -> u64 {
        let mut total_value: i64 = 0;

        for txout in self.outputs {
            signed_value = txout.value.parse::<i64>().unwrap();

            if !is_valid_amount(&signed_value) {
                panic!("TxOut value {value} out of range", value = signed_value);
            }

            total_value += signed_value;

            if !is_valid_amount(&total_value) {
                panic!("Total TxOut value of {value} out of range", value = total_value);
            }
        }

        total_value.parse::<u64>().unwrap()
    }

    /**
     * Get the total transaction size in bytes, including witness data.
     * "Total Size" defined in BIP141 and BIP144.
     * @return Total transaction size in bytes
     */

    fn get_total_size(&self) {
        // get this from serialization size
    }

    /**
     * Returns whether current transaction is a coinbase tx
     */

    fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].previous_out != None
    }

    // include check for witness
}

