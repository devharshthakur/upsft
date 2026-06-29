use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;

use crate::deps::Dependency;
use crate::error::ExecError;

pub fn run(dep: &Dependency) -> Result<ExitStatus, ExecError> {
    let command = dep.command.trim();
    if command.is_empty() {
        return Err(ExecError::EmptyCommand);
    }

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| ExecError::Spawn { source })?;

    let name = &dep.name;

    let stdout = child.stdout.take().ok_or_else(|| ExecError::Io {
        source: std::io::Error::other("stdout pipe was not captured"),
    })?;
    let stderr = child.stderr.take().ok_or_else(|| ExecError::Io {
        source: std::io::Error::other("stderr pipe was not captured"),
    })?;

    let (tx, rx) = std::sync::mpsc::channel::<String>();

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
            println!("[{name}] {line}");
        }
    });

    child.wait().map_err(|source| ExecError::Io { source })
}
