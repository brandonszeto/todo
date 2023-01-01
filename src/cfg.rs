use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs,
    // io::{stdin, Write},
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "todo";
const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub token: String,
    pub color: bool,
    pub config_file_path: PathBuf,
}

impl UserConfig {
    pub fn new(token: &str) -> Result<UserConfig> {
        Ok(UserConfig {
            token: String::from(token),
            color: true,
            config_file_path: get_path()?,
        })
    }

    //     pub fn load_config(config_file_path: PathBuf) -> Result<()> {
    //         let mut json = config_file_path.to_str();
    //         serde_json::from_str::<UserConfig>(&json).map_err(|_| String::from("Could not parse JSON");
    //     }
    pub fn set_path(self, config_file_path: PathBuf) -> UserConfig {
        UserConfig {
            config_file_path: config_file_path,
            ..self
        }
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

            let _ = &app_config_dir.join(CONFIG_FILE);

            Ok(app_config_dir)
        }
        None => Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No $HOME directory found for config file"),
        ))),
    }
}
