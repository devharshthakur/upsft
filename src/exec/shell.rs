use std::io::{BufRead, BufReader};
use std::{process::Stdio, sync::mpsc, thread};

use crate::{
    deps,
    error::ExecError,
    exec::{ExecOutcome, Executor},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct ShellExecutor;

impl ShellExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Executor for ShellExecutor {
    fn run(
        &self,
        dep: deps::Dependency,
        out: &mut dyn super::OutputSink,
    ) -> Result<ExecOutcome, ExecError> {
        let command = dep.update_command.trim();
        if command.is_empty() {
            return Err(ExecError::EmptyCommand);
        }

        let mut child = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|source| ExecError::Spawn { source })?;

        let name = &dep.name;

        let stderr = child.stderr.take().ok_or_else(|| ExecError::Io {
            source: std::io::Error::other("stderr pipe was not captured"),
        })?;

        let stdout = child.stdout.take().ok_or_else(|| ExecError::Io {
            source: std::io::Error::other("stdout pipe was not captured"),
        })?;

        let (tx, rx) = mpsc::channel::<String>();

        thread::scope(|scope| {
            let tx_stdout = tx.clone();
            let tx_stderr = tx;

            scope.spawn(move || {
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if tx_stdout.send(line).is_err() {
                        break;
                    }
                }
            });
            scope.spawn(move || {
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    if tx_stderr.send(line).is_err() {
                        break;
                    }
                }
            });

            while let Ok(line) = rx.recv() {
                out.line(name, &line);
            }
        });

        let status = child.wait().map_err(|source| ExecError::Io { source })?;
        Ok(ExecOutcome::from_status(status))
    }
}
