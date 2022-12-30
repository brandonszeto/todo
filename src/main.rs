// use curl::easy::{Easy, List};
// use std::io;
// use std::io::{stdout, Write};
use config::UserConfig;
mod config;

fn main() {
    UserConfig::get_api_token();
}
