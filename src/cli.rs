use clap::Parser;
use std::path::Path;
use std::process::ExitCode;

use crate::config::Config;
use crate::deps::Dependency;
use crate::exec::runner::{self, RunMode};

#[derive(Parser, Debug)]
#[command(version, about, override_usage = "upsft [OPTIONS]")]
pub struct Args {
    #[arg(short, long = "config")]
    pub config_path: Option<String>,

    #[arg(short = 'l', long, conflicts_with = "init")]
    pub list: bool,

    #[arg(long, conflicts_with = "list")]
    pub init: bool,

    #[arg(short = 'P', long)]
    pub parallel: bool,
}

pub fn run() -> ExitCode {
    let args = Args::parse();

    if args.init {
        return init_config(args.config_path.as_deref().map(Path::new));
    }

    let config = match Config::load(args.config_path.as_deref().map(Path::new)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };

    if args.list {
        return list_deps(&config.deps);
    }

    run_updates(config.deps, args.parallel)
}

fn init_config(config_path: Option<&Path>) -> ExitCode {
    match Config::init(config_path) {
        Ok(path) => {
            println!("Created config at: {}", path.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn list_deps(deps: &[Dependency]) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    println!("Managed dependencies ({}):", deps.len());
    for dep in deps {
        println!("  {} = \"{}\"", dep.name, dep.command);
    }

    ExitCode::SUCCESS
}

fn run_updates(deps: Vec<Dependency>, parallel: bool) -> ExitCode {
    let mode = if parallel {
        RunMode::Parallel
    } else {
        RunMode::Sequential
    };
    runner::run(deps, mode)
}
