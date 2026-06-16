use std::process::{Command, ExitStatus};

use crate::error::UpsftError;

/// Execute a shell command string. Supports pipes, redirects, `&&`, etc.
///
/// # Errors
///
/// Returns [`UpsftError::EmptyCommand`] if the command string is empty or whitespace-only.
/// Returns [`UpsftError::CommandExec`] if the shell process cannot be spawned.
pub fn execute(cmd: &str) -> Result<ExitStatus, UpsftError> {
    if cmd.trim().is_empty() {
        return Err(UpsftError::EmptyCommand);
    }

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .map_err(|source| UpsftError::CommandExec { source })
}
