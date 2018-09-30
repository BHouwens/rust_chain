extern crate uint;
extern crate sha3;
extern crate blake2;
extern crate tiny_keccak;

pub mod config;
pub mod net;
pub mod primitives;
pub mod utils;
pub mod chain;
pub mod consensus;
pub mod cryptography;

fn main() {
    config::read_config("chain.conf");
}