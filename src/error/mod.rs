use std::path::PathBuf;
use thiserror::Error;

/// Custom error type for config loading
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Config file not found
    #[error("Config file not found: {0}")]
    NotFound(PathBuf),

    /// Failed to read config file
    #[error("Failed to read config at {}: {source}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse TOML
    #[error("Failed to parse config at {}: {source}", path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
}
