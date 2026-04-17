use std::io;
use std::process::{Command, Output};

#[allow(dead_code)]
pub fn execute_command(args: &[String]) -> io::Result<Output> {
    if args.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no command provided",
        ));
    }

    let mut command = Command::new(&args[0]);
    if args.len() > 1 {
        command.args(&args[1..]);
    }

    command.output()
}
