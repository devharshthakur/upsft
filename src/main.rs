mod cli;
mod deps;
mod util;

use std::collections::HashMap;

use crate::{deps::Dependency, util::fs::execute};
use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    let dependencies = Dependency::new().unwrap();
    // case: upsft list (alias ls)
    if let Some(cli::Command::List) = args.command {
        for (label, dep) in &dependencies {
            println!("{label:>12}  {}", dep.hint);
        }
        return;
    }

    match args.package {
        Some(pkg_name) => {
            // Update specific package
            if let Some((_, pkg)) = dependencies.iter().find(|(key, _)| *key == &pkg_name) {
                if let Err(e) = execute(&pkg.update_command) {
                    eprintln!("Failed to update {}: {e}", pkg_name);
                }
            } else {
                eprintln!("Package '{pkg_name}' not found");
            }
        }
        None => {
            // No args → update all
            update_all(&dependencies);
        }
    }
}

fn update_all(dependencies: &HashMap<String, Dependency>) {
    println!("Updating all packages…");
    for (_, pkg) in dependencies {
        if let Err(e) = execute(&pkg.update_command) {
            eprintln!("Failed to update {}: {e}", pkg.update_command);
        }
    }
}
