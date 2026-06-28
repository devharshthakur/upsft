use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found: {0}")]
    NotFound(PathBuf),

    #[error("failed to read config at {}: {source}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config at {}: {source}", path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("config file is missing dependencies")]
    MissingDeps,

    #[error("[deps] at {path} must be a table, not a {actual}", path = path.display())]
    InvalidDepsType { path: PathBuf, actual: &'static str },

    #[error("dep name in {path} must not be empty", path = path.display())]
    EmptyDepName { path: PathBuf },

    #[error(
        "dep name '{name}' in {path} contains invalid characters; allowed: a-z, A-Z, 0-9, '_', '.', '-'",
        path = path.display()
    )]
    InvalidDepName { name: String, path: PathBuf },

    #[error(
        "update command for dep '{name}' in {path} must not be empty",
        path = path.display()
    )]
    EmptyUpdateCommand { name: String, path: PathBuf },

    #[error("config file already exists at {0}")]
    ConfigAlreadyExists(PathBuf),

    #[error("failed to create config directory: {source}")]
    CreateConfigDir {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write config file: {source}")]
    WriteConfig {
        #[source]
        source: std::io::Error,
    },

    #[error("home directory not set")]
    MissingHomeDir,

    #[error("value for key '{key}' at {path} must be a quoted string")]
    InvalidValue { path: PathBuf, key: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("no command provided")]
    EmptyCommand,

    #[error("failed to spawn the command: {source}")]
    Spawn {
        #[source]
        source: std::io::Error,
    },

    #[error("command I/O error: {source}")]
    Io {
        #[source]
        source: std::io::Error,
    },
}
