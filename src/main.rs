mod cli;
mod config;
mod deps;
mod errors;
mod exec;
mod output;

fn main() -> std::process::ExitCode {
    cli::run()
}
