use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{stdin, Write},
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

    // Gets path for todo config file
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

    // Requests and saves user input api token
    pub fn get_api_token() -> Result<String> {
        let mut input_api = String::new();
        println!("Please enter your api token:");
        stdin().read_line(&mut input_api).unwrap();
        input_api = input_api.trim().to_string();

        // Validate api token
        match UserConfig::validate_api_token(&input_api) {
            Ok(_) => return Ok(input_api),
            Err(e) => return Err(e),
        }
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

    // Loads preexisting config file
    pub fn load_config(&mut self) -> Result<()> {
        let paths = self.get_config_path()?;
        if paths.config_file_path.exists() {
            let config_string = fs::read_to_string(&paths.config_file_path)?;
            let config_json: UserConfig = serde_json::from_str(&config_string)?;

            self.token = config_json.token;
            self.color = config_json.color;

            Ok(())
        } else {
            // Create and save config file to new path
            println!(
                "Config will be saved to {}",
                paths.config_file_path.display()
            );

            let token = UserConfig::get_api_token()?;

            let color = true;

            let config_json = UserConfig { token, color };

            let content_json = serde_json::to_string_pretty(&config_json)?;

            let mut new_config = fs::File::create(&paths.config_file_path)?;
            write!(new_config, "{}", content_json)?;

            self.token = config_json.token;
            self.color = config_json.color;

            Ok(())
        }
    }
}