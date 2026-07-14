use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::config::Config;
use crate::exec;
use crate::output;

#[derive(Parser, Debug)]
#[command(version, about, override_usage = "upsft [OPTIONS]")]
pub struct Args {
    #[arg(short, long = "config")]
    pub config_path: Option<PathBuf>,

    #[arg(short = 'l', long, conflicts_with = "init")]
    pub list: bool,

    #[arg(long, conflicts_with = "list")]
    pub init: bool,
}

pub fn run() -> ExitCode {
    let args = Args::parse();

    if args.init {
        return init_config(args.config_path.as_deref());
    }

    let config = match Config::load(args.config_path.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };

    if config.deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    if args.list {
        return output::list_deps(&config.deps);
    }

    let results = exec::run(config.deps);
    output::report_results(&results)
}

fn init_config(config_path: Option<&Path>) -> ExitCode {
    match Config::create_config_file(config_path) {
        Ok(path) => output::report_init(&path),
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
