use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};
use thiserror::Error;
use toml::Table;

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
    #[error("Config File is missing dependencies")]
    MissingDeps,

    #[error("Key '{key}' at {path} should not be in double quotes")]
    InvalidKey { path: PathBuf, key: String },

    #[error("Value for key '{key}' at {path} must be a quoted string")]
    InvalidValue { path: PathBuf, key: String },
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub deps: HashMap<String, String>,
}

impl Config {
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

        let deps_table: Table = content.parse().map_err(|err| ConfigError::Parse {
            path: path.clone(),
            source: err,
        })?;

        

        Self::validate_config(deps_table, path)
    }

    /// Initialize a new config file at the provided path or the default location
    pub fn init_config(config_path: Option<&Path>) -> Result<PathBuf, String> {
        // Resolve path: use provided path or fall back to default
        let config_path = config_path
            .map(PathBuf::from)
            .unwrap_or_else(Self::default_path);

        // Ensure parent directory exists
        if let Some(config_dir) = config_path.parent().filter(|p| !p.as_os_str().is_empty())
            && !config_dir.exists()
        {
            fs::create_dir_all(config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        // Prevent overwriting existing config
        if config_path.exists() {
            return Err(format!(
                "Config file already exists at {}",
                config_path.display()
            ));
        }

        // Default config content — empty deps section
        let default_config = r#"[deps]"#;
        fs::write(&config_path, default_config)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(config_path)
    }

    /// It returns default path for the config file which is `~/.config/upsft/config.toml`
    fn default_path() -> PathBuf {
        PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/upsft/config.toml")
    }

    fn validate_config(table: Table, config_path: PathBuf) -> Result<Config, ConfigError> {
        // Empty file check
        let deps = table
            .get("deps")
            .and_then(|v| v.as_table())
            .ok_or(ConfigError::MissingDeps)?;

        let mut validated_deps: HashMap<String, String> = HashMap::new();

        // config file validations
        for (key, value) in deps.iter() {
            // validate value(update command): it should a shell command (string) not numbers or boolean
            let update_command = value.as_str().ok_or_else(|| ConfigError::InvalidValue {
                path: config_path.clone(),
                key: key.clone(),
            })?;

            validated_deps.insert(key.clone(), update_command.to_string());
        }

        let validated_config = Config {
            deps: validated_deps,
        };

        Ok(validated_config)
    }
}
