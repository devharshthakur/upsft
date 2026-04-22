use crate::error::ConfigError;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

#[derive(serde::Deserialize)]
pub struct Config {
    pub deps: HashMap<String, String>,
}

impl Config {
    pub fn default_path() -> PathBuf {
        PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/upsft/config.toml")
    }

    /// Load dependencies from a config file
    pub fn load(config_path: Option<&Path>) -> Result<Config, ConfigError> {
        let path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(Self::default_path);

        if !path.exists() {
            return Err(ConfigError::NotFound(path));
        }

        let content = fs::read_to_string(&path).map_err(|source| ConfigError::Read {
            path: path.clone(),
            source,
        })?;

        toml::from_str::<Config>(&content).map_err(|source| ConfigError::Parse { path, source })
    }

    /// Initialize a new config file at the default location
    pub fn init_config() -> Result<PathBuf, String> {
        let config_dir = PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/upsft");

        // Create the config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let config_path = config_dir.join("config.toml");

        // Check if config already exists
        if config_path.exists() {
            return Err(format!(
                "Config file already exists at {}",
                config_path.display()
            ));
        }

        // Default config content ie. Empty file
        let default_config = r#"[deps]"#;

        fs::write(&config_path, default_config)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(config_path)
    }
}
