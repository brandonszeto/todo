// use curl::easy::{Easy, List};
// use std::io;
// use std::io::{stdout, Write};
use config::UserConfig;
mod config;
// use projects::*;
// mod projects;

fn main() {
    let string = String::from("example");
    let mut client_config = UserConfig::new();
    client_config.load_config().unwrap();
    // list_projects();
}
