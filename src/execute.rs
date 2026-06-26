use std::io::{BufRead, BufReader};
use std::process::{ExitCode, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crate::cmd::execute;
use crate::config::Config;

/// Execute the update command sequentially for each dependency in the config.
pub fn execute_sequential(config: Config) -> ExitCode {
    if config.deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
    }

    let mut failed = false;

    for dep in config.deps {
        let name = dep.name;
        let command = dep.update_command;
        println!("Updating {name}...");

        // execute update commands : print error msg with capture error via pattern match
        match execute(&command) {
            Ok(status) if status.success() => {}
            // execution failed case print approprate error messages
            Ok(status) => {
                failed = true;
                match status.code() {
                    Some(code) => {
                        eprintln!("Error: update failed for {name} with exit code {code}")
                    }
                    None => eprintln!(
                        "Error: update failed for {name} because the process was terminated"
                    ),
                }
            }
            Err(error) => {
                failed = true;
                eprintln!("Error: failed to run update for {name}: {error}");
            }
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Execute update command in parallel.
pub fn execute_parallel(config: Config) -> ExitCode {
    // Worker threads send either a line of output or a completion signal.
    enum DepMsg {
        /// A single line of output from a dependency's process.
        Line { name: String, line: String },

        Done {
            name: String,
            result: Result<ExitStatus, String>,
            elapsed_secs: f64,
        },
    }

    let (tx, rx) = mpsc::channel::<DepMsg>();
    let mut handles = Vec::new();

    for dep in config.deps {
        let tx = tx.clone();

        // Move ownership into the thread.
        let name = dep.name;
        let command = dep.update_command;

        // Print before spawning so feedback is immediate.
        println!("Updating {name}...");

        let handle = thread::spawn(move || {
            let start = Instant::now();

            // Subshell redirects stderr from all commands, not just the last after shell operators.
            let merged_cmd = format!("({}) 2>&1", command);

            let mut child = match std::process::Command::new("sh")
                .arg("-c")
                .arg(&merged_cmd)
                .stdout(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    // Spawn failed at OS level (e.g. sh not found — extremely unlikely).
                    let _ = tx.send(DepMsg::Done {
                        name,
                        result: Err(e.to_string()),
                        elapsed_secs: start.elapsed().as_secs_f64(),
                    });
                    return;
                }
            };

            // Take the piped stdout handle and read it line by line.
            let stdout = child.stdout.take().expect("stdout handle available");
            let reader = BufReader::new(stdout);

            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => {
                        // If receiver is gone (main loop finished), stop sending.
                        if tx
                            .send(DepMsg::Line {
                                name: name.clone(),
                                line,
                            })
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(_) => {
                        // Non-UTF-8 bytes in output — skip that line and continue.
                        continue;
                    }
                }
            }

            // Wait for process to finish and read exit status.
            let elapsed_secs = start.elapsed().as_secs_f64();
            let result = match child.wait() {
                Ok(status) => Ok(status),
                Err(e) => Err(e.to_string()),
            };

            let _ = tx.send(DepMsg::Done {
                name,
                result,
                elapsed_secs,
            });
        });

        handles.push(handle);
    }

    // Drop our sender so the channel closes when all worker senders drop.
    drop(tx);

    // ── Receive and display messages until all workers finish ──
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
                Ok(status) if status.success() => {
                    println!("[{name}] Completed ({elapsed_secs:.1}s)");
                }
                Ok(status) => {
                    failed = true;
                    match status.code() {
                        Some(code) => {
                            eprintln!("[{name}] Failed: exit code {code} ({elapsed_secs:.1}s)")
                        }
                        None => {
                            eprintln!("[{name}] Failed: terminated by signal ({elapsed_secs:.1}s)")
                        }
                    }
                }
                Err(e) => {
                    failed = true;
                    eprintln!("[{name}] Failed: {e} ({elapsed_secs:.1}s)");
                }
            },
        }
    }

    // Ensure all threads are joined (they should be done since channel drained).
    for handle in handles {
        if handle.join().is_err() {
            eprintln!("Internal error: a worker thread panicked");
            failed = true;
        }
    }

    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
