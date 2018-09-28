extern crate uint;

pub mod config;
pub mod net;
pub mod primitives;
pub mod utils;
pub mod chain;
pub mod consensus;

fn main() {
    config::read_config("chain.conf");
}