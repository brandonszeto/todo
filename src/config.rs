use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::stdin,
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "todo";
const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub token: String,
    pub color: bool,
}

pub struct ConfigPath {
    pub config_file_path: PathBuf,
}

impl UserConfig {
    pub fn new() -> UserConfig {
        UserConfig {
            token: "".to_string(),
            color: true,
        }
    }

    pub fn get_config_path(&self) -> Result<ConfigPath> {
        match dirs::home_dir() {
            Some(home) => {
                let home_path = Path::new(&home);
                let config_dir = home_path.join(CONFIG_DIR);
                let app_config_dir = config_dir.join(APP_CONFIG_DIR);
                if !config_dir.exists() {
                    fs::create_dir(&config_dir)?;
                }

                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(CONFIG_FILE);

                let final_path = ConfigPath {
                    config_file_path: config_file_path.to_path_buf(),
                };
                Ok(final_path)
            }
            None => Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No $HOME directory found for config file"),
            ))),
        }
    }

    // Loads api token from cache or config, requests if not found
    pub fn load_config() {}

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
