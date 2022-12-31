// use curl::easy::{Easy, List};
// use std::io;
// use std::io::{stdout, Write};
use config::UserConfig;
mod config;

fn main() {
    let mut client_config = UserConfig::new();
    client_config.load_config().unwrap();
    client_config.get_config_path().unwrap();
}
