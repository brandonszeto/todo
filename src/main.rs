// use curl::easy::{Easy, List};
// use std::io;
// use std::io::{stdout, Write};

mod args;

use args::TodoArgs;
use clap::Parser;

// Write the contents of rust-lang.org to stdout
fn main() {
    // println!("Please enter your api token:");
    // let mut input_api = String::new();
    // io::stdin()
    //     .read_line(&mut input_api)
    //     .expect("failed to read API");

    // let trimmed = input_api.trim();

    // let mut easy = Easy::new();
    // easy.url("https://api.todoist.com/rest/v2/projects")
    //     .unwrap();

    // let mut list = List::new();

    // let api_token = String::from("Authorization: Bearer ").to_owned();

    // let passed_api = api_token + trimmed;

    // list.append(&passed_api).unwrap();

    // easy.http_headers(list).unwrap();

    // easy.write_function(|data| {
    //     stdout().write_all(data).unwrap();
    //     Ok(data.len())
    // })
    // .unwrap();

    // // Printing json file to stdout
    // easy.perform().unwrap();

    // // Printing response code
    // println!("{}", easy.response_code().unwrap());

    let args: TodoArgs = TodoArgs::parse();
}
