use std::io;
use std::process::{Command, Output};

#[allow(dead_code)]
pub fn execute_command(cmd: &'static str) -> io::Result<Output> {
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
