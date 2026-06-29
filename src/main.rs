mod cli;
mod config;
mod deps;
mod errors;
mod exec;

fn main() -> std::process::ExitCode {
    cli::run()
}
