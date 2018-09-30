/**
 * The scripting language for authority of tx spending 
 * has been moved to "primitives" because it just seems 
 * to make more sense here. Might change in future.
 */


/*---- CONSTANTS ----*/

// Maximum number of bytes pushable to the stack
const MAX_SCRIPT_ELEMENT_SIZE: u16 = 520;

// Maximum number of non-push operations per script
const MAX_OPS_PER_SCRIPT: u8 = 201;

// Maximum number of public keys per multisig
const MAX_PUB_KEYS_PER_MULTISIG: u8 = 20;

// Maximum script length in bytes
const MAX_SCRIPT_SIZE: u16 = 10000;

// Maximum number of values on script interpreter stack
const MAX_STACK_SIZE: u16 = 1000;

// Threshold for lock_time: below this value it is interpreted as block number,
// otherwise as UNIX timestamp.
const LOCKTIME_THRESHOLD: u32 = 500000000; // Tue Nov  5 00:53:20 1985 UTC

// Maximum value that an opcode can be
const MAX_OPCODE: u8 = OpCode::OP_NOP10;


/*---- STRUCTS ----*/

enum OpCodes = {

    // push value
    OP_0 = 0x00,
    OP_PUSHDATA1 = 0x4c,
    OP_PUSHDATA2 = 0x4d,
    OP_PUSHDATA4 = 0x4e,
    OP_1NEGATE = 0x4f,
    OP_RESERVED = 0x50,
    OP_1 = 0x51,
    OP_2 = 0x52,
    OP_3 = 0x53,
    OP_4 = 0x54,
    OP_5 = 0x55,
    OP_6 = 0x56,
    OP_7 = 0x57,
    OP_8 = 0x58,
    OP_9 = 0x59,
    OP_10 = 0x5a,
    OP_11 = 0x5b,
    OP_12 = 0x5c,
    OP_13 = 0x5d,
    OP_14 = 0x5e,
    OP_15 = 0x5f,
    OP_16 = 0x60,

    // control
    OP_NOP = 0x61,
    OP_VER = 0x62,
    OP_IF = 0x63,
    OP_NOTIF = 0x64,
    OP_VERIF = 0x65,
    OP_VERNOTIF = 0x66,
    OP_ELSE = 0x67,
    OP_ENDIF = 0x68,
    OP_VERIFY = 0x69,
    OP_RETURN = 0x6a,
 
    // stack ops
    OP_TOALTSTACK = 0x6b,
    OP_FROMALTSTACK = 0x6c,
    OP_2DROP = 0x6d,
    OP_2DUP = 0x6e,
    OP_3DUP = 0x6f,
    OP_2OVER = 0x70,
    OP_2ROT = 0x71,
    OP_2SWAP = 0x72,
    OP_IFDUP = 0x73,
    OP_DEPTH = 0x74,
    OP_DROP = 0x75,
    OP_DUP = 0x76,
    OP_NIP = 0x77,
    OP_OVER = 0x78,
    OP_PICK = 0x79,
    OP_ROLL = 0x7a,
    OP_ROT = 0x7b,
    OP_SWAP = 0x7c,
    OP_TUCK = 0x7d,
 
    // splice ops
    OP_CAT = 0x7e,
    OP_SUBSTR = 0x7f,
    OP_LEFT = 0x80,
    OP_RIGHT = 0x81,
    OP_SIZE = 0x82,
 
    // bit logic
    OP_INVERT = 0x83,
    OP_AND = 0x84,
    OP_OR = 0x85,
    OP_XOR = 0x86,
    OP_EQUAL = 0x87,
    OP_EQUALVERIFY = 0x88,
    OP_RESERVED1 = 0x89,
    OP_RESERVED2 = 0x8a,
 
    // numeric
    OP_1ADD = 0x8b,
    OP_1SUB = 0x8c,
    OP_2MUL = 0x8d,
    OP_2DIV = 0x8e,
    OP_NEGATE = 0x8f,
    OP_ABS = 0x90,
    OP_NOT = 0x91,
    OP_0NOTEQUAL = 0x92,
 
    OP_ADD = 0x93,
    OP_SUB = 0x94,
    OP_MUL = 0x95,
    OP_DIV = 0x96,
    OP_MOD = 0x97,
    OP_LSHIFT = 0x98,
    OP_RSHIFT = 0x99,
 
    OP_BOOLAND = 0x9a,
    OP_BOOLOR = 0x9b,
    OP_NUMEQUAL = 0x9c,
    OP_NUMEQUALVERIFY = 0x9d,
    OP_NUMNOTEQUAL = 0x9e,
    OP_LESSTHAN = 0x9f,
    OP_GREATERTHAN = 0xa0,
    OP_LESSTHANOREQUAL = 0xa1,
    OP_GREATERTHANOREQUAL = 0xa2,
    OP_MIN = 0xa3,
    OP_MAX = 0xa4,
 
    OP_WITHIN = 0xa5,
 
    // crypto
    OP_SHA256 = 0xa8,
    OP_HASH160 = 0xa9,
    OP_HASH256 = 0xaa,
    OP_CODESEPARATOR = 0xab,
    OP_CHECKSIG = 0xac,
    OP_CHECKSIGVERIFY = 0xad,
    OP_CHECKMULTISIG = 0xae,
    OP_CHECKMULTISIGVERIFY = 0xaf,
 
    // expansion
    OP_NOP1 = 0xb0,
    OP_CHECKLOCKTIMEVERIFY = 0xb1,
    OP_NOP2 = OP_CHECKLOCKTIMEVERIFY,
    OP_CHECKSEQUENCEVERIFY = 0xb2,
    OP_NOP3 = OP_CHECKSEQUENCEVERIFY,
    OP_NOP4 = 0xb3,
    OP_NOP5 = 0xb4,
    OP_NOP6 = 0xb5,
    OP_NOP7 = 0xb6,
    OP_NOP8 = 0xb7,
    OP_NOP9 = 0xb8,
    OP_NOP10 = 0xb9,
 
    OP_INVALIDOPCODE = 0xff

};

pub struct Script {
    begin: Vec<u8>,
    end: u8
};

impl Script {
    pub fn new() -> Script {
        Script {
            begin: Vec::new(),
            end: 0
        }
    }

    /**
     * Check whether this is pay to script hash
     */

    pub fn is_p2sh(&self) -> bool {
        return self.begin.len() == 23 &&
               self.begin[0] == OpCodes::OP_HASH160 &&
               self.begin[1] == 0x14 &&
               self.begin[22] == OpCodes::OP_EQUAL;
    }

    /**
     * Check whether this is a witness program script.
     * 
     * A witness program is any valid Script that consists of a 1-byte push opcode
     * followed by a data push between 2 and 40 bytes.
     */

    pub fn is_witness_program(&self) -> bool {
        true
    }

    pub fn is_push_only(&self) -> bool {
        for entry in self.begin {
            if !entry.op_code {
                return false;
            }

            // Note that IsPushOnly() *does* consider OP_RESERVED to be a
            // push-type opcode, however execution of OP_RESERVED fails, so
            // it's not relevant to P2SH/BIP62 as the scriptSig would fail prior to
            // the P2SH special validation code being executed.
            if (entry.opcode > OpCode::OP_16) {
                return false;
            }
        }

        true
    }

    pub fn has_valid_ops(&self) -> bool {
        for entry in self.begin {

        }

        true
    }
}
