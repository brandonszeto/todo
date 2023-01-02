use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{stdin, Read},
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "todo";
const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub token: String,
    pub color: bool,
    pub user_projects: HashMap<String, u32>,
    pub path: PathBuf,
}

impl UserConfig {
    pub fn new(token: &str) -> Result<UserConfig> {
        let user_projects: HashMap<String, u32> = HashMap::new();
        Ok(UserConfig {
            token: String::from(token),
            color: true,
            user_projects,
            path: get_path().unwrap(),
        })
    }

    pub fn load_config(path: &str) -> Result<UserConfig, String> {
        let mut json = String::new();

        fs::File::open(path)
            .or(Err("Could not find file"))?
            .read_to_string(&mut json)
            .or(Err("Could not read to string"))?;

        serde_json::from_str::<UserConfig>(&json).map_err(|_| String::from("Could not parse JSON"))
    }
}

// Requests and saves user input api token
pub fn get_api_token() -> Result<String> {
    let mut input_api = String::new();
    println!("Please enter your api token:");
    stdin().read_line(&mut input_api).unwrap();
    input_api = input_api.trim().to_string();

    // Validate api token
    match validate_api_token(&input_api) {
        Ok(_) => return Ok(input_api),
        Err(e) => return Err(e),
    }
}

pub fn validate_api_token(token: &str) -> Result<()> {
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

pub fn get_path() -> Result<PathBuf> {
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

            Ok(config_file_path.to_path_buf())
        }
        None => Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No $HOME directory found for config file"),
        ))),
    }
}
