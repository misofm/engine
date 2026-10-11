//! The `realtime_policy` binary: checks the workspace in the current directory, or the one named.

use std::process::ExitCode;

use realtime_policy::{Policy, run};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let Ok(cwd) = std::env::current_dir() else {
        eprintln!("realtime policy failure: the working directory is unavailable");
        return ExitCode::from(1);
    };
    ExitCode::from(run(
        &args,
        &cwd,
        &Policy::workspace(),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
    ))
}
