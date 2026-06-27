pub mod runner;
pub mod shell;

use crate::{deps::Dependency, error::ExecError};

pub trait OutputHandler {
    fn line(&mut self, name: &str, line: &str);
}

pub trait Executor: Send + Sync {
    fn run(&self, dep: Dependency, out: &mut dyn OutputHandler) -> Result<ExecOutcome, ExecError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecOutcome {
    pub exit_code: Option<i32>,
    pub success: bool,
}

impl ExecOutcome {
    fn from_status(status: std::process::ExitStatus) -> Self {
        Self {
            exit_code: status.code(),
            success: status.success(),
        }
    }
}
