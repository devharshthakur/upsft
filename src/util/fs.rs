use std::io;
use std::process::{Command, ExitStatus};

/// Executes a command via the user's shell (supports &&, pipes, etc.)
pub fn execute(cmd: &str) -> io::Result<ExitStatus> {
    if cmd.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no command provided",
        ));
    }

    Command::new("sh").arg("-c").arg(cmd).status()
}
