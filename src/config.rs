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
    pub fn load(config_path: &Option<&Path>) -> Config {
        let default_path =
            PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/upsft/config.toml");

        let path = match config_path {
            Some(p) => PathBuf::from(p),
            None => default_path,
        };
        // If config file does not exist
        if !path.exists() {
            eprintln!("Config file not found");
            std::process::exit(1);
        }

        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str::<Config>(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to parse config: {}", e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Failed to read config: {}", e);
                std::process::exit(1);
            }
        }
    }
}
