mod cli;
mod config;
mod errors;
mod exec;
mod output;

fn main() -> std::process::ExitCode {
    cli::run()
}
