mod cli;
mod config;
mod deps;
mod util;

fn main() -> std::process::ExitCode {
    cli::Cli::run()
}
