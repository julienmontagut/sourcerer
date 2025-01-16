use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    token: String,
    save_file: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            token: String::new(),
            save_file: None,
        }
    }

    pub fn load(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.unwrap_or(crate::dirs::get_config_path()?);
        let config_str = std::fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
