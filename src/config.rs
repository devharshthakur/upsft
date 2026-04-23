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

    /// Initialize a new config file at the provided path or the default location
    pub fn init_config(config_path: Option<&Path>) -> Result<PathBuf, String> {
        let config_path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(Self::default_path);

        // Check if config_path exist if not create it
        if let Some(config_dir) = config_path
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
        // handles empty dir edge case
            && !config_dir.exists()
        {
            fs::create_dir_all(config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

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
