use std::io;
use std::process::{Command, Output};

/// This is to check if a dependencies exist in clients machine
/// This use execution of which command
pub fn check(dep: &str) -> bool {
    match execute(&format!("which {}", dep)) {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Executes a command
pub fn execute(cmd: &str) -> io::Result<Output> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no command provided",
        ));
    }

    let mut command = Command::new(parts[0]);
    if parts.len() > 1 {
        command.args(&parts[1..]);
    }

    command.output()
}
