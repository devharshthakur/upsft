use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::deps::Dependency;
use crate::exec::{ExecError, ExecOutcome, Executor, OutputSink};

pub fn run_sequential(deps: Vec<Dependency>, exec: &impl Executor) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    let mut failed = false;
    let mut sink = PrintSink;

    for dep in deps {
        let name = dep.name.clone();
        println!("Updating {name}...");
        let start = Instant::now();

        match exec.run(dep, &mut sink) {
            Ok(ExecOutcome { success: true, .. }) => {
                let elapsed = start.elapsed().as_secs_f64();
                println!("[{name}] Completed ({elapsed:.1}s)");
            }
            Ok(ExecOutcome {
                success: false,
                exit_code: Some(code),
            }) => {
                let elapsed = start.elapsed().as_secs_f64();
                failed = true;
                eprintln!("[{name}] Failed: exit code {code} ({elapsed:.1}s)");
            }
            Ok(ExecOutcome {
                success: false,
                exit_code: None,
            }) => {
                let elapsed = start.elapsed().as_secs_f64();
                failed = true;
                eprintln!("[{name}] Failed: terminated by signal ({elapsed:.1}s)");
            }
            Err(e) => {
                let elapsed = start.elapsed().as_secs_f64();
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

enum DepMsg {
    Line {
        name: String,
        line: String,
    },
    Done {
        name: String,
        result: Result<ExecOutcome, ExecError>,
        elapsed_secs: f64,
    },
}

struct ChannelSink {
    tx: mpsc::Sender<DepMsg>,
}

impl OutputSink for ChannelSink {
    fn line(&mut self, name: &str, line: &str) {
        let _ = self.tx.send(DepMsg::Line {
            name: name.to_string(),
            line: line.to_string(),
        });
    }
}

pub fn run_parallel(deps: Vec<Dependency>, exec: &impl Executor) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    let (tx, rx) = mpsc::channel::<DepMsg>();

    for dep in &deps {
        println!("Updating {}...", dep.name);
    }

    let failed = thread::scope(move |s| {
        for dep in deps {
            let tx = tx.clone();
            let name = dep.name.clone();

            s.spawn(move || {
                let start = Instant::now();
                let mut sink = ChannelSink { tx: tx.clone() };

                let result = exec.run(dep, &mut sink);

                let elapsed_secs = start.elapsed().as_secs_f64();
                let _ = tx.send(DepMsg::Done {
                    name,
                    result,
                    elapsed_secs,
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
                    elapsed_secs,
                } => match result {
                    Ok(ExecOutcome { success: true, .. }) => {
                        println!("[{name}] Completed ({elapsed_secs:.1}s)");
                    }
                    Ok(ExecOutcome {
                        success: false,
                        exit_code: Some(code),
                    }) => {
                        failed = true;
                        eprintln!("[{name}] Failed: exit code {code} ({elapsed_secs:.1}s)");
                    }
                    Ok(ExecOutcome {
                        success: false,
                        exit_code: None,
                    }) => {
                        failed = true;
                        eprintln!("[{name}] Failed: terminated by signal ({elapsed_secs:.1}s)");
                    }
                    Err(e) => {
                        failed = true;
                        eprintln!("[{name}] Failed: {e} ({elapsed_secs:.1}s)");
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

struct PrintSink;

impl OutputSink for PrintSink {
    fn line(&mut self, name: &str, line: &str) {
        println!("[{name}] {line}");
    }
}
