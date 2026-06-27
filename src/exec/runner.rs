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

pub fn run_parallel(deps: Vec<Dependency>, exec: &impl Executor) -> ExitCode {
    if deps.is_empty() {
        println!("No dependencies added yet");
        return ExitCode::SUCCESS;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[derive(Clone, Copy)]
    enum FakeResult {
        Success,
        Fail(i32),
        Terminated,
        SpawnError,
    }

    struct FakeExecutor {
        results: Mutex<HashMap<String, FakeResult>>,
        calls: Mutex<Vec<String>>,
    }

    impl FakeExecutor {
        fn new(deps: &[(&str, FakeResult)]) -> Self {
            let results = deps
                .iter()
                .map(|(name, res)| (name.to_string(), *res))
                .collect();
            Self {
                results: Mutex::new(results),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl Executor for FakeExecutor {
        fn run(
            &self,
            dep: Dependency,
            _out: &mut dyn OutputSink,
        ) -> Result<ExecOutcome, ExecError> {
            self.calls.lock().unwrap().push(dep.name.clone());
            let result = self
                .results
                .lock()
                .unwrap()
                .get(&dep.name)
                .copied()
                .unwrap_or(FakeResult::Success);
            match result {
                FakeResult::Success => Ok(ExecOutcome {
                    exit_code: Some(0),
                    success: true,
                }),
                FakeResult::Fail(code) => Ok(ExecOutcome {
                    exit_code: Some(code),
                    success: false,
                }),
                FakeResult::Terminated => Ok(ExecOutcome {
                    exit_code: None,
                    success: false,
                }),
                FakeResult::SpawnError => Err(ExecError::Spawn {
                    source: std::io::Error::new(std::io::ErrorKind::NotFound, "fake spawn error"),
                }),
            }
        }
    }

    fn dep(name: &str) -> Dependency {
        Dependency::new(name.to_string(), format!("echo {name}"))
    }

    fn is_success(code: ExitCode) -> bool {
        format!("{code:?}") == format!("{:?}", ExitCode::SUCCESS)
    }

    fn is_failure(code: ExitCode) -> bool {
        format!("{code:?}") == format!("{:?}", ExitCode::FAILURE)
    }

    #[test]
    fn seq_empty_deps_returns_success() {
        let exec = FakeExecutor::new(&[]);
        let code = run_sequential(Vec::new(), &exec);
        assert!(is_success(code));
        assert!(exec.calls().is_empty());
    }

    #[test]
    fn seq_all_success_returns_success() {
        let exec =
            FakeExecutor::new(&[("brew", FakeResult::Success), ("rust", FakeResult::Success)]);
        let code = run_sequential(vec![dep("brew"), dep("rust")], &exec);
        assert!(is_success(code));
        assert_eq!(exec.calls(), vec!["brew", "rust"]);
    }

    #[test]
    fn seq_one_failure_returns_failure_but_runs_all() {
        let exec = FakeExecutor::new(&[
            ("brew", FakeResult::Success),
            ("rust", FakeResult::Fail(1)),
            ("npm", FakeResult::Success),
        ]);
        let code = run_sequential(vec![dep("brew"), dep("rust"), dep("npm")], &exec);
        assert!(is_failure(code));
        assert_eq!(exec.calls(), vec!["brew", "rust", "npm"]);
    }

    #[test]
    fn seq_spawn_error_returns_failure() {
        let exec = FakeExecutor::new(&[("bad", FakeResult::SpawnError)]);
        let code = run_sequential(vec![dep("bad")], &exec);
        assert!(is_failure(code));
    }

    #[test]
    fn seq_terminated_returns_failure() {
        let exec = FakeExecutor::new(&[("killed", FakeResult::Terminated)]);
        let code = run_sequential(vec![dep("killed")], &exec);
        assert!(is_failure(code));
    }

    #[test]
    fn par_empty_deps_returns_success() {
        let exec = FakeExecutor::new(&[]);
        let code = run_parallel(Vec::new(), &exec);
        assert!(is_success(code));
        assert!(exec.calls().is_empty());
    }

    #[test]
    fn par_all_success_returns_success() {
        let exec =
            FakeExecutor::new(&[("brew", FakeResult::Success), ("rust", FakeResult::Success)]);
        let code = run_parallel(vec![dep("brew"), dep("rust")], &exec);
        assert!(is_success(code));
        let mut calls = exec.calls();
        calls.sort();
        assert_eq!(calls, vec!["brew", "rust"]);
    }

    #[test]
    fn par_one_failure_returns_failure_but_runs_all() {
        let exec = FakeExecutor::new(&[
            ("brew", FakeResult::Success),
            ("rust", FakeResult::Fail(42)),
            ("npm", FakeResult::Success),
        ]);
        let code = run_parallel(vec![dep("brew"), dep("rust"), dep("npm")], &exec);
        assert!(is_failure(code));
        let mut calls = exec.calls();
        calls.sort();
        assert_eq!(calls, vec!["brew", "npm", "rust"]);
    }

    #[test]
    fn par_spawn_error_returns_failure() {
        let exec = FakeExecutor::new(&[("bad", FakeResult::SpawnError)]);
        let code = run_parallel(vec![dep("bad")], &exec);
        assert!(is_failure(code));
    }

    #[test]
    fn par_preserves_full_dep_set() {
        let names: Vec<String> = (0..10).map(|i| format!("d{i}")).collect();
        let deps: Vec<Dependency> = names.iter().map(|n| dep(n)).collect();
        let exec = FakeExecutor::new(
            &names
                .iter()
                .map(|n| (n.as_str(), FakeResult::Success))
                .collect::<Vec<_>>(),
        );
        let code = run_parallel(deps, &exec);
        assert!(is_success(code));
        let mut calls = exec.calls();
        calls.sort();
        assert_eq!(calls, names);
    }
}
