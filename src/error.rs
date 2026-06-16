use std::path::PathBuf;
use thiserror::Error;

/// Unified error type for all upsft operations.
#[derive(Debug, Error)]
pub enum UpsftError {
    /// Config file not found.
    #[error("config file not found: {0}")]
    ConfigNotFound(PathBuf),

    /// Failed to read config file.
    #[error("failed to read config at {path}: {source}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse TOML config.
    #[error("failed to parse config at {path}: {source}")]
    ConfigParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    /// Config file is missing the `[deps]` section.
    #[error("config file is missing [deps] section")]
    MissingDeps,

    /// A dep value is not a valid string.
    #[error("value for key '{key}' at {path} must be a quoted string")]
    InvalidValue { path: PathBuf, key: String },

    /// Config file already exists at the init target path.
    #[error("config file already exists at {0}")]
    ConfigAlreadyExists(PathBuf),

    /// Failed to create parent directory for config.
    #[error("failed to create config directory: {source}")]
    ConfigDirCreate {
        #[source]
        source: std::io::Error,
    },

    /// Failed to write config file during init.
    #[error("failed to write config file: {source}")]
    ConfigWrite {
        #[source]
        source: std::io::Error,
    },

    /// Command execution failed at the OS level.
    #[error("failed to execute command: {source}")]
    CommandExec {
        #[source]
        source: std::io::Error,
    },

    /// Empty command string provided.
    #[error("no command provided")]
    EmptyCommand,

    /// $HOME environment variable is not set.
    #[error("HOME directory not set")]
    MissingHomeDir,
}
