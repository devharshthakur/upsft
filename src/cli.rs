use crate::config::Config;
use crate::util::fs::execute;
use clap::Parser;
use std::path::Path;
use std::process::ExitCode;

/// upsft — update all the things
#[derive(Parser, Debug)]
#[command(version, about, override_usage = "upsft [OPTIONS]")]
pub struct Cli {
    /// Path to custom config file
    #[arg(short, long = "config")]
    pub config_path: Option<String>,

    /// List all managed dependencies
    #[arg(long, conflicts_with = "init")]
    pub list: bool,

    /// Create a new config file
    #[arg(long, conflicts_with = "list")]
    pub init: bool,
}

impl Cli {
    /// Parse CLI arguments, load config, and dispatch to the appropriate command.
    pub fn run() -> ExitCode {
        let args = Cli::parse();
        let config_path = args.config_path.as_deref().map(Path::new);

        if args.init {
            return match Config::init_config(config_path) {
                Ok(path) => {
                    println!("Created config at: {}", path.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    ExitCode::FAILURE
                }
            };
        }

        if args.list {
            return match Config::load(config_path) {
                Ok(config) => {
                    Self::list_deps(config);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    ExitCode::FAILURE
                }
            };
        }
        // load the config and execut the update comands : the main job
        match Config::load(config_path) {
            Ok(config) => Self::execute_update_commands(config),
            Err(e) => {
                eprintln!("Error: {e}");
                ExitCode::FAILURE
            }
        }
    }

    /// Print all dependencies from config in a clean, sorted list.
    fn list_deps(config: Config) {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return;
        }

        let mut names: Vec<&String> = config.deps.iter().map(|dep| &dep.name).collect();
        names.sort();

        println!("Managed dependencies ({}):", names.len());
        for name in names {
            println!("- {name}");
        }
    }

    /// Execute the update command for each dependency in the config.
    fn execute_update_commands(config: Config) -> ExitCode {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return ExitCode::SUCCESS;
        }

        let mut deps: Vec<_> = config.deps.into_iter().collect();
        deps.sort();

        let mut failed = false;

        for dep in deps {
            let name = dep.name;
            let command = dep.update_command;
            println!("Updating {name}...");

            // execute update commands : print error msg with capture error via pattern match
            match execute(&command) {
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
