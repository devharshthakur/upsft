use crate::config::Config;
use crate::util::fs::execute;
use clap::{Parser, Subcommand};
use std::path::Path;

/// upsft — update all the things
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Path to custom config file
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
    /// Parse CLI arguments, load config, and dispatch to the appropriate command.
    /// Exits with code 1 on any error during config loading or command execution.
    pub fn load() {
        let args = Cli::parse();
        let config_path = args.config_path.as_ref().map(Path::new);

        // Load config (errors handled internally by Config::load)
        let config = Config::load(&config_path);

        // Dispatch to command handler
        match &args.command {
            Some(Command::List) => Self::list_deps(config),
            None => Self::execute_update_commands(config),
        }
    }

    /// Print all dependencies from config in `name: command` format.
    /// Prints "No dependencies added yet" if the deps map is empty.
    fn list_deps(config: Config) {
        // config file is empty
        if config.deps.is_empty() {
            println!("No dependencies added yet");
        } else {
            for (name, cmd) in &config.deps {
                println!("{}: {}", name, cmd);
            }
        }
    }

    /// Execute the update command for each dependency in the config.
    pub fn execute_update_commands(config: Config) {
        for dep in config.deps {
            let update_command = dep.1.as_str();
            let _ = execute(update_command);
        }
        std::process::exit(1)
    }
}
