extern crate uint;

pub mod config;
pub mod net;
pub mod primitives;
pub mod utils;

fn main() {
    config::read_config("chain.conf");
}