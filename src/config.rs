use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::deps::Dependency;
use crate::error::UpsftError;

/// Parsed configuration from a `config.toml` file.
#[derive(Debug)]
pub struct Config {
    /// Managed dependencies listed in the config.
    pub deps: Vec<Dependency>,
}

impl Config {
    /// Load dependencies from a config file.
    ///
    /// If `config_path` is `None`, the default path `$HOME/.config/upsft/config.toml`
    /// is used.
    ///
    /// # Errors
    ///
    /// Returns an error if the config file is not found, cannot be read, cannot be
    /// parsed as TOML, is missing the `[deps]` section, or contains invalid values.
    pub fn load(config_path: Option<&Path>) -> Result<Config, UpsftError> {
        let path = if let Some(cp) = config_path {
            PathBuf::from(cp)
        } else {
            Self::default_path()?
        };

        if !path.exists() {
            return Err(UpsftError::ConfigNotFound(path));
        }

        let content = fs::read_to_string(&path).map_err(|source| UpsftError::ConfigRead {
            path: path.clone(),
            source,
        })?;

        let deps_table: toml::Table =
            content.parse().map_err(|source| UpsftError::ConfigParse {
                path: path.clone(),
                source,
            })?;

        Self::validate_config(deps_table, path)
    }

    /// Initialize a new config file at the provided path or the default location.
    ///
    /// Creates the parent directory if it does not exist. Refuses to overwrite
    /// an existing config file.
    ///
    /// ## Errors
    ///
    /// Returns an error if the parent directory cannot be created, a config file
    /// already exists at the target path, or the default config template cannot
    /// be written.
    pub fn init_config(config_path: Option<&Path>) -> Result<PathBuf, UpsftError> {
        let config_path = if let Some(cp) = config_path {
            PathBuf::from(cp)
        } else {
            Self::default_path()?
        };

        // Ensure parent directory exists
        if let Some(config_dir) = config_path.parent().filter(|p| !p.as_os_str().is_empty())
            && !config_dir.exists()
        {
            fs::create_dir_all(config_dir)
                .map_err(|source| UpsftError::ConfigDirCreate { source })?;
        }

        // Prevent overwriting existing config
        if config_path.exists() {
            return Err(UpsftError::ConfigAlreadyExists(config_path));
        }

        // Default config content — empty deps section
        let default_config = r#"[deps]"#;
        fs::write(&config_path, default_config)
            .map_err(|source| UpsftError::ConfigWrite { source })?;

        Ok(config_path)
    }

    /// Returns the default config path: `$HOME/.config/upsft/config.toml`
    fn default_path() -> Result<PathBuf, UpsftError> {
        let home = home::home_dir().ok_or(UpsftError::MissingHomeDir)?;
        Ok(home.join(".config/upsft/config.toml"))
    }

    /// Validates the config with required checks.
    fn validate_config(table: toml::Table, config_path: PathBuf) -> Result<Config, UpsftError> {
        let deps = table
            .get("deps")
            .and_then(|v| v.as_table())
            .ok_or(UpsftError::MissingDeps)?;

        let mut validated_deps: Vec<Dependency> = Vec::with_capacity(deps.len());

        // `toml::Table` preserves insertion order with `preserve_order` enabled.
        // Iterate directly so deps execute in the same order the user wrote them.
        for (key, value) in deps.iter() {
            let update_command = value.as_str().ok_or_else(|| UpsftError::InvalidValue {
                path: config_path.clone(),
                key: key.clone(),
            })?;

            validated_deps.push(Dependency::new(key.clone(), update_command.to_owned()));
        }

        Ok(Config {
            deps: validated_deps,
        })
    }
}
