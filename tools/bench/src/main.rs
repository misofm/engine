#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Consolidated benchmark subjects.
//!
//! A native tool: #1075 retired the `protocol` subject, the last one with a wasm32 build (owner
//! ruling R9, only real host paths are benchmarked).

use std::process::Command;

mod console;
mod effect_contract;
mod floor;

const INTERNAL_SUBJECT: &str = "ENGINE_V1_INTERNAL_BENCH_SUBJECT";
const SUBJECTS: &[&str] = &["console", "effect-contract"];

fn run_subject(subject: &str) {
    match subject {
        "console" => console::main(),
        "effect-contract" => effect_contract::main(),
        _ => unreachable!("dispatcher validates internal subjects"),
    }
}

fn usage() -> ! {
    eprintln!("usage: bench <{}> [subject arguments]", SUBJECTS.join("|"));
    std::process::exit(2);
}

fn launch(mut command: Command) -> ! {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let error = command.exec();
        eprintln!("failed to launch benchmark subject: {error}");
        std::process::exit(1);
    }
    #[cfg(not(unix))]
    {
        let status = command.status().unwrap_or_else(|error| {
            eprintln!("failed to launch benchmark subject: {error}");
            std::process::exit(1);
        });
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn main() {
    if let Ok(subject) = std::env::var(INTERNAL_SUBJECT) {
        run_subject(&subject);
        return;
    }

    let mut args = std::env::args_os().skip(1);
    let subject = args
        .next()
        .and_then(|value| value.into_string().ok())
        .unwrap_or_else(|| usage());
    if !SUBJECTS.contains(&subject.as_str()) {
        usage();
    }
    let mut command = Command::new(std::env::current_exe().expect("current executable path"));
    command.args(args).env(INTERNAL_SUBJECT, subject);
    launch(command);
}
