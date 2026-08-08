use crate::errors::ConfigError;
use indexmap::IndexMap;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

const DEFAULT_CONFIG: &str = "[deps]\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub command: String,
}

#[derive(Debug)]
pub struct Config {
    pub deps: Vec<Dependency>,
}

/// Serde-facing shape of the TOML file. Deserialize first, then validate
/// into the domain [`Config`].
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    deps: Option<IndexMap<String, String>>,
}

impl Config {
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

        Self::from_toml_str(&content, &path)
    }

    pub fn create_config_file(config_path: Option<&Path>) -> Result<PathBuf, ConfigError> {
        let config_path = if let Some(cp) = config_path {
            PathBuf::from(cp)
        } else {
            Self::default_path()?
        };

        if let Some(config_dir) = config_path.parent().filter(|p| !p.as_os_str().is_empty())
            && !config_dir.exists()
        {
            fs::create_dir_all(config_dir)
                .map_err(|source| ConfigError::CreateConfigDir { source })?;
        }

        if config_path.exists() {
            return Err(ConfigError::ConfigAlreadyExists(config_path));
        }

        fs::write(&config_path, DEFAULT_CONFIG)
            .map_err(|source| ConfigError::WriteConfig { source })?;

        Ok(config_path)
    }

    /// Parse and validate a TOML string, reporting errors against
    /// `config_path`. Parsing is done by serde via [`RawConfig`]; any
    /// shape/type problem surfaces here as [`ConfigError::Parse`] with a
    /// line/column span from the toml crate.
    fn from_toml_str(content: &str, config_path: &Path) -> Result<Config, ConfigError> {
        let raw: RawConfig = toml::from_str(content).map_err(|source| ConfigError::Parse {
            path: config_path.to_path_buf(),
            source,
        })?;

        raw.into_config(config_path)
    }

    fn default_path() -> Result<PathBuf, ConfigError> {
        let home = home::home_dir().ok_or(ConfigError::MissingHomeDir)?;
        Ok(home.join(".config/upsft/config.toml"))
    }
}

impl RawConfig {
    /// Convert the raw serde-deserialized config into the validated domain
    /// [`Config`], applying the custom rules serde cannot express: deps key
    /// must exist, names must be non-empty and `[a-zA-Z0-9_.-]`, and update
    /// commands must be non-blank.
    fn into_config(self, config_path: &Path) -> Result<Config, ConfigError> {
        let Some(deps) = self.deps else {
            return Err(ConfigError::MissingDeps);
        };

        let mut validated_deps: Vec<Dependency> = Vec::with_capacity(deps.len());

        for (name, command) in deps {
            if name.is_empty() {
                return Err(ConfigError::EmptyDepName {
                    path: config_path.to_path_buf(),
                });
            }

            if !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
            {
                return Err(ConfigError::InvalidDepName {
                    name: name.clone(),
                    path: config_path.to_path_buf(),
                });
            }

            if command.trim().is_empty() {
                return Err(ConfigError::EmptyUpdateCommand {
                    name: name.clone(),
                    path: config_path.to_path_buf(),
                });
            }

            validated_deps.push(Dependency { name, command });
        }

        Ok(Config {
            deps: validated_deps,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(content: &str) -> Result<Config, ConfigError> {
        Config::from_toml_str(content, Path::new("config.toml"))
    }

    #[test]
    fn parses_deps_in_insertion_order() {
        let config = parse("[deps]\nzsh = \"echo zsh\"\nz = \"echo z\"\na = \"echo a\"\n").unwrap();

        let names: Vec<&str> = config.deps.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["zsh", "z", "a"]);

        let commands: Vec<&str> = config.deps.iter().map(|d| d.command.as_str()).collect();
        assert_eq!(commands, ["echo zsh", "echo z", "echo a"]);
    }

    #[test]
    fn missing_deps_is_an_error() {
        let err = parse("").unwrap_err();
        assert!(matches!(err, ConfigError::MissingDeps));
    }

    #[test]
    fn empty_deps_table_yields_no_deps() {
        let config = parse("[deps]\n").unwrap();
        assert!(config.deps.is_empty());
    }

    #[test]
    fn rejects_invalid_dep_name() {
        let err = parse("[deps]\n\"bad name\" = \"x\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::InvalidDepName { .. }));
    }

    #[test]
    fn rejects_empty_dep_name() {
        let err = parse("[deps]\n\"\" = \"x\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::EmptyDepName { .. }));
    }

    #[test]
    fn rejects_empty_update_command() {
        let err = parse("[deps]\na = \"\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::EmptyUpdateCommand { .. }));

        let err = parse("[deps]\na = \"   \"\n").unwrap_err();
        assert!(matches!(err, ConfigError::EmptyUpdateCommand { .. }));
    }

    #[test]
    fn rejects_unknown_top_level_key() {
        let err = parse("[deps]\na = \"x\"\n[dep]\nb = \"y\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::Parse { .. }));
        assert!(err.to_string().contains("unknown field"));
    }

    #[test]
    fn rejects_non_table_deps() {
        let err = parse("deps = \"nope\"\n").unwrap_err();
        assert!(matches!(err, ConfigError::Parse { .. }));
    }

    #[test]
    fn rejects_non_string_value() {
        let err = parse("[deps]\na = 42\n").unwrap_err();
        assert!(matches!(err, ConfigError::Parse { .. }));
    }
}
