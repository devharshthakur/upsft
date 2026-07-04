use crate::{
    config::Dependency,
    exec::{DepOutcome, DepResult},
};
use std::path::Path;
use std::process::ExitCode;

pub fn report_results(results: &[DepResult]) -> ExitCode {
    let mut failed = false;

    for r in results {
        match &r.outcome {
            DepOutcome::Success => {
                println!("[{}] Completed ({:.1}s)", r.name, r.elapsed);
            }
            DepOutcome::Failed(err) => {
                eprintln!("{err}");
                failed = true;
            }
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub fn list_deps(deps: &[Dependency]) -> ExitCode {
    println!("Managed dependencies ({}):", deps.len());
    for dep in deps {
        println!("  {} = \"{}\"", dep.name, dep.command);
    }

    ExitCode::SUCCESS
}

pub fn report_init(path: &Path) -> ExitCode {
    println!("Created config at: {}", path.display());
    ExitCode::SUCCESS
}
