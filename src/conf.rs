use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(serialize_with = "path_to_string", deserialize_with = "string_to_path")]
    workspaces: Vec<PathBuf>,
    token: String,
    save_file: Option<String>,
    user: User,
}

fn path_to_string<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

fn string_to_path<D>(path: &str, deserializer: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer,
{
    Ok(PathBuf::from(path))
}

impl Config {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
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
