use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{deps::Dependency, error::ConfigError};

#[derive(Debug)]
pub struct Config {
    pub deps: Vec<Dependency>,
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

        let deps_table: toml::Table = content.parse().map_err(|source| ConfigError::Parse {
            path: path.clone(),
            source,
        })?;

        Self::validate_config(deps_table, path)
    }

    pub fn init_config(config_path: Option<&Path>) -> Result<PathBuf, ConfigError> {
        let config_path = if let Some(cp) = config_path {
            PathBuf::from(cp)
        } else {
            Self::default_path()?
        };

        if let Some(config_dir) = config_path.parent().filter(|p| !p.as_os_str().is_empty())
            && !config_dir.exists()
        {
            fs::create_dir_all(config_dir)
                .map_err(|source| ConfigError::ConfigDirCreate { source })?;
        }

        if config_path.exists() {
            return Err(ConfigError::ConfigAlreadyExists(config_path));
        }

        let default_config = r#"[deps]"#;
        fs::write(&config_path, default_config)
            .map_err(|source| ConfigError::ConfigWrite { source })?;

        Ok(config_path)
    }

    fn default_path() -> Result<PathBuf, ConfigError> {
        let home = home::home_dir().ok_or(ConfigError::MissingHomeDir)?;
        Ok(home.join(".config/upsft/config.toml"))
    }
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

        for (key, value) in deps.iter() {
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

            let update_command = value.as_str().ok_or_else(|| ConfigError::InvalidValue {
                path: config_path.clone(),
                key: key.clone(),
            })?;

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
