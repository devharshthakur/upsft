mod cli;
mod config;
mod deps;
mod error;
mod exec;

fn main() -> std::process::ExitCode {
    cli::run()
}
