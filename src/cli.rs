use clap::{Parser, Subcommand};

/// upsft — update all the things
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    /// Update a specific package only (defaults to all if omitted)
    pub package: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List all managed dependencies
    #[command(visible_alias("ls"))]
    List,
}
