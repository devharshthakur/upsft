use crate::config::Config;
use crate::execute;
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
    #[arg(short = 'l', long, conflicts_with = "init")]
    pub list: bool,

    /// Create a new config file
    #[arg(long, conflicts_with = "list")]
    pub init: bool,

    /// Run all dependencies update parallely
    #[arg(short = 'P', long)]
    pub parallel: bool,
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
            Ok(config) => Self::execute_update_commands(&args, config),
            Err(e) => {
                eprintln!("Error: {e}");
                ExitCode::FAILURE
            }
        }
    }

    /// Print all dependencies from config in a formatted table.
    fn list_deps(config: Config) {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return;
        }

        println!("Managed dependencies ({}):", config.deps.len());

        let mut table = tabled::Table::new(&config.deps);
        table.with(tabled::settings::Style::rounded());

        if config.deps.len() > 1 {
            let mut theme =
                tabled::settings::themes::Theme::from_style(tabled::settings::Style::rounded());
            for i in 2..=config.deps.len() {
                theme.insert_horizontal_line(
                    i,
                    tabled::grid::config::HorizontalLine::full('─', '┼', '├', '┤'),
                );
            }
            table.with(theme);
        }

        println!("{table}");
    }

    fn execute_update_commands(args: &Cli, config: Config) -> ExitCode {
        if config.deps.is_empty() {
            println!("No dependencies added yet");
            return ExitCode::SUCCESS;
        }

        if args.parallel {
            execute::execute_parallel(config)
        } else {
            execute::execute_sequential(config)
        }
    }
}
