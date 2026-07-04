use std::process::{Command, Stdio};
use std::time::Instant;

use crate::deps::Dependency;
use crate::errors::ExecError;

pub enum DepOutcome {
    Success,
    Failed(ExecError),
}

pub struct DepResult {
    pub name: String,
    pub elapsed: f64,
    pub outcome: DepOutcome,
}

pub fn run(deps: Vec<Dependency>) -> Vec<DepResult> {
    deps.iter().map(execute_command).collect()
}

fn execute_command(dep: &Dependency) -> DepResult {
    let start = Instant::now();

    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(dep.command.trim())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            return DepResult {
                name: dep.name.clone(),
                elapsed: start.elapsed().as_secs_f64(),
                outcome: DepOutcome::Failed(ExecError::Spawn {
                    source: e,
                    name: dep.name.clone(),
                    elapsed: start.elapsed().as_secs_f64(),
                }),
            };
        }
    };

    let status = match child.wait() {
        Ok(s) => s,
        Err(_) => {
            return DepResult {
                name: dep.name.clone(),
                elapsed: start.elapsed().as_secs_f64(),
                outcome: DepOutcome::Failed(ExecError::Wait {
                    name: dep.name.clone(),
                    elapsed: start.elapsed().as_secs_f64(),
                }),
            };
        }
    };

    let elapsed = start.elapsed().as_secs_f64();

    let outcome = if status.success() {
        DepOutcome::Success
    } else if let Some(code) = status.code() {
        DepOutcome::Failed(ExecError::NonZeroExit {
            code,
            name: dep.name.clone(),
            elapsed,
        })
    } else {
        DepOutcome::Failed(ExecError::Signal {
            name: dep.name.clone(),
            elapsed,
        })
    };

    DepResult {
        name: dep.name.clone(),
        elapsed,
        outcome,
    }
}
