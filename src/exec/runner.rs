use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::deps::Dependency;
use crate::error::ExecError;
use crate::exec::shell;

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

        let result = shell::run(&dep, |_, line| println!("[{name}] {line}"));

        let elapsed = start.elapsed().as_secs_f64();
        match result {
            Ok(status) if status.success() => {
                println!("[{name}] Completed ({elapsed:.1}s)");
            }
            Ok(status) => {
                failed = true;
                if let Some(code) = status.code() {
                    eprintln!("[{name}] Failed: exit code {code} ({elapsed:.1}s)");
                } else {
                    eprintln!("[{name}] Failed: terminated by signal ({elapsed:.1}s)");
                }
            }
            Err(e) => {
                failed = true;
                eprintln!("[{name}] Failed: {e} ({elapsed:.1}s)");
            }
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

    enum DepMsg {
        Line {
            name: String,
            line: String,
        },
        Done {
            name: String,
            result: Result<std::process::ExitStatus, ExecError>,
            elapsed: f64,
        },
    }

    let (tx, rx) = mpsc::channel::<DepMsg>();

    let failed = thread::scope(move |s| {
        for dep in deps {
            let tx = tx.clone();
            let name = dep.name.clone();

            s.spawn(move || {
                let start = Instant::now();
                let tx_line = tx.clone();
                let name_for_done = name.clone();

                let result = shell::run(&dep, move |_name, line| {
                    let _ = tx_line.send(DepMsg::Line {
                        name: name.clone(),
                        line: line.to_string(),
                    });
                });

                let elapsed = start.elapsed().as_secs_f64();
                let _ = tx.send(DepMsg::Done {
                    name: name_for_done,
                    result,
                    elapsed,
                });
            });
        }

        drop(tx);

        let mut failed = false;

        for msg in rx {
            match msg {
                DepMsg::Line { name, line } => {
                    println!("[{name}] {line}");
                }
                DepMsg::Done {
                    name,
                    result,
                    elapsed,
                } => match result {
                    Ok(status) if status.success() => {
                        println!("[{name}] Completed ({elapsed:.1}s)");
                    }
                    Ok(status) => {
                        failed = true;
                        if let Some(code) = status.code() {
                            eprintln!("[{name}] Failed: exit code {code} ({elapsed:.1}s)");
                        } else {
                            eprintln!("[{name}] Failed: terminated by signal ({elapsed:.1}s)");
                        }
                    }
                    Err(e) => {
                        failed = true;
                        eprintln!("[{name}] Failed: {e} ({elapsed:.1}s)");
                    }
                },
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
