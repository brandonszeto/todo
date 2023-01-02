// use std::io;
// use std::io::{stdout, Write};
use config::UserConfig;
mod config;

fn main() {
    let mut client_config = UserConfig::new("token");
    client_config.load_config("String").unwrap();
}
