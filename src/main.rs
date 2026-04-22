mod cli;
mod config;
mod deps;
mod error;
mod util;

fn main() -> std::process::ExitCode {
    cli::Cli::run()
}
