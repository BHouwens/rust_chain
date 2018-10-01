extern crate num_bigint as bigint;
extern crate ramp;
extern crate sha3;
extern crate blake2;
extern crate tiny_keccak;
extern crate digest;
extern crate sodiumoxide;

pub mod config;
pub mod net;
pub mod primitives;
pub mod utils;
pub mod chain;
pub mod consensus;
pub mod cryptography;

use ramp::int::Int;

fn main() {
    let value = Int::from_str_radix("00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22", 16).unwrap();
    let bytes_original = String::from("00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22").bytes().len();

    println!("bytes: {}", bytes_original);

    let hex_value = value.to_str_radix(16, false);
    let bytes = hex_value.bytes().len();

    println!("bytes second: {}", bytes);

    println!("{}", value);
    println!("{}", hex_value);
}