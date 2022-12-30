use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::stdin;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    token: String,
    color: bool,
}

impl UserConfig {
    pub fn new() -> UserConfig {
        UserConfig {
            token: "".to_string(),
            color: true,
        }
    }

    // Loads api token from cache or config, requests if not found
    pub fn load_api_token() {}

    // Requests and saves user input api token
    pub fn get_api_token() -> Result<String> {
        let mut input_api = String::new();
        println!("Please enter your api token:");
        stdin().read_line(&mut input_api).unwrap();
        input_api = input_api.trim().to_string();

        // Make sure the inut api token is valid
        match UserConfig::validate_api_token(&input_api) {
            Ok(_) => return Ok(input_api),
            Err(e) => return Err(e),
        }

        // let p = UserConfig {
        //     token: input_api,
        //     color: true,
        // };

        // let serialized = serde_json::to_string(&p).unwrap();
        // // return serialized;
        // println!("serialized = {}", serialized);

        // let deserialized: UserConfig = serde_json::from_str(&serialized).unwrap();
        // println!("deserialized = {:?}", deserialized);
    }

    // TODO: Strengthen validation by test project get
    fn validate_api_token(token: &str) -> Result<()> {
        const EXPECTED_LEN: usize = 40;
        if token.len() != EXPECTED_LEN {
            Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid length: {} (must be {})", token.len(), EXPECTED_LEN,),
            )))
        } else {
            Ok(())
        }
    }
}
