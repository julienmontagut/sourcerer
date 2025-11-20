use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    #[serde(default)]
    pub organizations: Vec<String>,
    #[serde(default)]
    pub include_private: bool,
    #[serde(default = "default_sync_directory")]
    pub sync_directory: String,
    #[serde(default)]
    pub branch_patterns: Vec<String>,
}

fn default_sync_directory() -> String {
    if cfg!(target_os = "macos") {
        "~/Developer".to_string()
    } else if cfg!(target_os = "linux") || cfg!(target_os = "windows") {
        "~/Sources".to_string()
    } else {
        "~/sources".to_string()
    }
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn create_default_file(path: &Path) -> Result<()> {
        let default_config = Self::default();
        let content = format!(r#"# GitHub Sync Configuration

# Your GitHub username (will be auto-populated on first run if empty)
username = ""

# List of organizations to sync repositories from
organizations = [
    # "my-org",
    # "another-org"
]

# Whether to include private repositories
include_private = true

# Directory where repositories will be synced
sync_directory = "{}"

# Patterns of branches to sync (defaults to main and master)
branch_patterns = [
    "main",
    "master",
    # "develop",
    # "release/*"
]
"#, default_config.sync_directory);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: String::new(),
            organizations: Vec::new(),
            include_private: true,
            sync_directory: default_sync_directory(),
            branch_patterns: vec!["main".to_string(), "master".to_string()],
        }
    }
} 