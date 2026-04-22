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
    /// Load dependencies from a config file
    pub fn load(config_path: &Option<&Path>) -> Result<Config, ConfigError> {
        let default_path =
            PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/upsft/config.toml");
        let path = match config_path {
            Some(p) => PathBuf::from(p),
            None => {
                // return the default path for config on mac : "~/.config.upsft.config.toml"
                default_path
            }
        };

        let content = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
