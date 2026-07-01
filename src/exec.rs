use std::process::{Command, ExitCode, ExitStatus, Stdio};
use std::time::Instant;

use crate::deps::Dependency;

pub fn run(deps: Vec<Dependency>) -> ExitCode {
    let mut failed = false;

    for dep in &deps {
        if execute_command(dep) {
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn execute_command(dep: &Dependency) -> bool {
    let command = dep.command.trim();
    if command.is_empty() {
        eprintln!("[{}] Failed: no command provided", dep.name);
        return true;
    }

    println!("Updating {}...", dep.name);
    let start = Instant::now();

    let status = match Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(mut child) => match child.wait() {
            Ok(s) => s,
            Err(_) => {
                eprintln!(
                    "[{}] Failed: process wait error ({:.1}s)",
                    dep.name,
                    start.elapsed().as_secs_f64(),
                );
                return true;
            }
        },
        Err(e) => {
            eprintln!(
                "[{}] Failed: could not spawn command — {e} ({:.1}s)",
                dep.name,
                start.elapsed().as_secs_f64(),
            );
            return true;
        }
    };

    let elapsed = start.elapsed().as_secs_f64();
    report_result(dep.name.as_str(), status, elapsed)
}

fn report_result(name: &str, status: ExitStatus, elapsed: f64) -> bool {
    if status.success() {
        println!("[{name}] Completed ({elapsed:.1}s)");
        false
    } else if let Some(code) = status.code() {
        eprintln!("[{name}] Failed: exit code {code} ({elapsed:.1}s)");
        true
    } else {
        eprintln!("[{name}] Failed: terminated by signal ({elapsed:.1}s)");
        true
    }
}
