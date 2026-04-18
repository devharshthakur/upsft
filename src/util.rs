use std::io;
use std::process::{Command, Output};

pub fn check(dep: &str) -> bool {
    execute(&format!("which {}", dep))
        .map(|output| output.status.success())
        .unwrap_or(false)
}

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
