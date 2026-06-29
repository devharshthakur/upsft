pub mod shell;

use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::deps::Dependency;
use crate::error::ExecError;

pub enum RunMode {
    Sequential,
    Parallel,
}

pub fn run(deps: Vec<Dependency>, mode: RunMode) -> ExitCode {
    match mode {
        RunMode::Sequential => run_sequential(deps),
        RunMode::Parallel => run_parallel(deps),
    }
}

fn run_sequential(deps: Vec<Dependency>) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    let mut failed = false;

    for dep in deps {
        let name = &dep.name;
        println!("Updating {name}...");
        let start = Instant::now();

        let result = shell::run(&dep);

        let elapsed = start.elapsed().as_secs_f64();
        if report_result(name, result, elapsed) {
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run_parallel(deps: Vec<Dependency>) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    for dep in &deps {
        println!("Updating {}...", dep.name);
    }

    struct DepResult {
        name: String,
        result: Result<std::process::ExitStatus, ExecError>,
        elapsed: f64,
    }

    let (tx, rx) = mpsc::channel::<DepResult>();

    let failed = thread::scope(move |s| {
        for dep in deps {
            let tx = tx.clone();
            s.spawn(move || {
                let start = Instant::now();
                let result = shell::run(&dep);
                let elapsed = start.elapsed().as_secs_f64();
                let _ = tx.send(DepResult {
                    name: dep.name,
                    result,
                    elapsed,
                });
            });
        }

        drop(tx);

        let mut failed = false;

        for res in rx {
            if report_result(&res.name, res.result, res.elapsed) {
                failed = true;
            }
        }

        failed
    });

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn report_result(
    name: &str,
    result: Result<std::process::ExitStatus, ExecError>,
    elapsed: f64,
) -> bool {
    match result {
        Ok(status) if status.success() => {
            println!("[{name}] Completed ({elapsed:.1}s)");
            false
        }
        Ok(status) => {
            if let Some(code) = status.code() {
                eprintln!("[{name}] Failed: exit code {code} ({elapsed:.1}s)");
            } else {
                eprintln!("[{name}] Failed: terminated by signal ({elapsed:.1}s)");
            }
            true
        }
        Err(e) => {
            eprintln!("[{name}] Failed: {e} ({elapsed:.1}s)");
            true
        }
    }
}
