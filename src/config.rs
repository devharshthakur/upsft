use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{deps::Dependency, error::ConfigError};

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
    pub fn load(config_path: Option<&Path>) -> Result<Config, ConfigError> {
        let path = if let Some(cp) = config_path {
            PathBuf::from(cp)
        } else {
            Self::default_path()?
        };

        if !path.exists() {
            return Err(ConfigError::NotFound(path));
        }

        let content = fs::read_to_string(&path).map_err(|source| ConfigError::Read {
            path: path.clone(),
            source,
        })?;

        let deps_table: toml::Table = content.parse().map_err(|source| ConfigError::Parse {
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
    pub fn init_config(config_path: Option<&Path>) -> Result<PathBuf, ConfigError> {
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
                .map_err(|source| ConfigError::ConfigDirCreate { source })?;
        }

        // Prevent overwriting existing config
        if config_path.exists() {
            return Err(ConfigError::ConfigAlreadyExists(config_path));
        }

        // Default config content — empty deps section
        let default_config = r#"[deps]"#;
        fs::write(&config_path, default_config)
            .map_err(|source| ConfigError::ConfigWrite { source })?;

        Ok(config_path)
    }

    /// Returns the default config path: `$HOME/.config/upsft/config.toml`
    fn default_path() -> Result<PathBuf, ConfigError> {
        let home = home::home_dir().ok_or(ConfigError::MissingHomeDir)?;
        Ok(home.join(".config/upsft/config.toml"))
    }
    /// Validates the config with required checks
    fn validate_config(table: toml::Table, config_path: PathBuf) -> Result<Config, ConfigError> {
        let deps_value = table.get("deps").ok_or(ConfigError::MissingDeps)?;
        let deps = if let Some(t) = deps_value.as_table() {
            t
        } else {
            return Err(ConfigError::InvalidDepsType {
                path: config_path.clone(),
                actual: deps_value.type_str(),
            });
        };

        let mut validated_deps: Vec<Dependency> = Vec::with_capacity(deps.len());

        // `toml::Table` preserves insertion order with `preserve_order` enabled.
        // Iterate directly so deps execute in the same order the user wrote them.
        for (key, value) in deps.iter() {
            // TOML permits `""` as a quoted key, so an empty dep name must be
            // rejected explicitly.
            if key.is_empty() {
                return Err(ConfigError::EmptyDepName {
                    path: config_path.clone(),
                });
            }

            if !key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
            {
                return Err(ConfigError::InvalidDepName {
                    name: key.clone(),
                    path: config_path.clone(),
                });
            }

            // validate value(update command): it should a shell command (string) not numbers or boolean
            let update_command = value.as_str().ok_or_else(|| ConfigError::InvalidValue {
                path: config_path.clone(),
                key: key.clone(),
            })?;

            // Catch empty commands at parse time so the user sees a clear error
            // here instead of a generic "no command provided" from the executor.
            if update_command.trim().is_empty() {
                return Err(ConfigError::EmptyUpdateCommand {
                    name: key.clone(),
                    path: config_path.clone(),
                });
            }

            let deps = Dependency::new(key.clone(), update_command.to_owned());
            validated_deps.push(deps);
        }

        Ok(Config {
            deps: validated_deps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Parse `content` as a TOML table and run it through the validator with a
    /// fixed test path so error variants can be matched without touching the FS.
    fn validate(content: &str) -> Result<Config, ConfigError> {
        let table: toml::Table = content
            .parse()
            .expect("test input must itself be valid TOML");
        Config::validate_config(table, PathBuf::from("/test/config.toml"))
    }

    #[test]
    fn valid_config_parses_in_insertion_order() {
        let cfg = validate(
            r#"[deps]
brew = "brew update"
rust = "rustup update"
"#,
        )
        .expect("valid config should parse");

        assert_eq!(cfg.deps.len(), 2);
        assert_eq!(cfg.deps[0].name, "brew");
        assert_eq!(cfg.deps[0].update_command, "brew update");
        assert_eq!(cfg.deps[1].name, "rust");
        assert_eq!(cfg.deps[1].update_command, "rustup update");
    }

    #[test]
    fn missing_deps_section_errors() {
        let err = validate(r#"other = "x""#).expect_err("should error");
        assert!(matches!(err, ConfigError::MissingDeps));
    }

    #[test]
    fn deps_as_array_errors() {
        let err = validate("deps = []").expect_err("should error");
        assert!(matches!(err, ConfigError::InvalidDepsType { .. }));
    }

    #[test]

    fn empty_dep_name_errors() {
        let err = validate(
            r#"[deps]
"" = "x"
"#,
        )
        .expect_err("should error");
        assert!(matches!(err, ConfigError::EmptyDepName { .. }));
    }

    #[test]
    fn invalid_dep_name_chars_errors() {
        // Quoted key with a space is valid TOML but fails our character whitelist.
        let err = validate(
            r#"[deps]
"brew tap" = "brew tap"
"#,
        )
        .expect_err("should error");
        assert!(matches!(err, ConfigError::InvalidDepName { .. }));
    }

    #[test]
    fn empty_update_command_errors() {
        let err = validate(
            r#"[deps]
brew = ""
"#,
        )
        .expect_err("should error");
        assert!(matches!(err, ConfigError::EmptyUpdateCommand { .. }));
    }

    #[test]
    fn whitespace_only_update_command_errors() {
        let err = validate(
            r#"[deps]
brew = "   "
"#,
        )
        .expect_err("should error");
        assert!(matches!(err, ConfigError::EmptyUpdateCommand { .. }));
    }
}
