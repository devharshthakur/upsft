use crate::config::Config;
use crate::util::fs::execute;
use clap::{Parser, Subcommand};
use std::path::Path;

/// upsft — update all the things
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Path to custom config file (default: json/core.json)
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List all managed dependencies
    #[command(visible_alias("ls"))]
    List,
}

impl Cli {
    pub fn load() {
        let args = Cli::parse();
        let config_path = args.config_path.as_ref().map(Path::new);
        let config = Config::load(&config_path).unwrap();
        Self::execute_update_commands(config);
    }

    pub fn execute_update_commands(config: Config) {
        for dep in config.deps {
            let update_command = dep.1.as_str();
            let _ = execute(update_command);
        }
        std::process::exit(1)
    }
}
