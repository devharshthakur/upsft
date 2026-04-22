use std::path::PathBuf;
use thiserror::Error;

/// Custom error type for config loading
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Config file not found
    #[error("Config not found at: {0}")]
    NotFound(PathBuf),

    /// Failed to read config file
    #[error("Failed to read config: {0}")]
    Read(#[from] std::io::Error),

    /// Failed to parse TOML
    #[error("Failed to parse TOML: {0}")]
    Parse(#[from] toml::de::Error),
}
