use crate::config::Config;
use crate::util::fs::execute;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::ExitCode;

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
    /// Create a new config file at the default location
    Init,
}

impl Cli {
    /// Parse CLI arguments, load config, and dispatch to the appropriate command.
    pub fn run() -> ExitCode {
        let args = Cli::parse();
        let config_path = args.config_path.as_deref().map(Path::new);

        match &args.command {
            Some(Command::Init) => match Config::init_config() {
                Ok(path) => {
                    println!("Created config: {}", path.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    ExitCode::FAILURE
                }
            },
            Some(Command::List) => match Config::load(config_path) {
                Ok(config) => {
                    Self::list_deps(config);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    ExitCode::FAILURE
                }
            },
            None => match Config::load(config_path) {
                Ok(config) => Self::execute_update_commands(config),
                Err(e) => {
                    eprintln!("Error: {e}");
                    ExitCode::FAILURE
                }
            },
        }
    }

    /// Print all dependencies from config in a clean, sorted list.
    fn list_deps(config: Config) {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return;
        }

        let mut names: Vec<_> = config.deps.keys().collect();
        names.sort_unstable();

        println!("Managed dependencies ({}):", names.len());
        for name in names {
            println!("- {name}");
        }
    }

    /// Execute the update command for each dependency in the config.
    pub fn execute_update_commands(config: Config) -> ExitCode {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return ExitCode::SUCCESS;
        }

        let mut deps: Vec<_> = config.deps.into_iter().collect();
        deps.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut failed = false;

        for (name, update_command) in deps {
            println!("Updating {name}...");

            // execute update commands : print error msg with capture error via pattern match
            match execute(&update_command) {
                Ok(status) if status.success() => {}
                // execution failed case print approprate error messages
                Ok(status) => {
                    failed = true;
                    match status.code() {
                        Some(code) => {
                            eprintln!("Error: update failed for {name} with exit code {code}")
                        }
                        None => eprintln!(
                            "Error: update failed for {name} because the process was terminated"
                        ),
                    }
                }
                Err(error) => {
                    failed = true;
                    eprintln!("Error: failed to run update for {name}: {error}");
                }
            }
        }

        if failed {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}
