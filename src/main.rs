pub mod config;

fn main() {
    config::read_config("bitcoin.conf");
}