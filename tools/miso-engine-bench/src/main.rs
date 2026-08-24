//! Consolidated benchmark subjects.

#[cfg(not(target_arch = "wasm32"))]
use std::ffi::OsString;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
mod bootstrap;
#[cfg(not(target_arch = "wasm32"))]
mod builtins;
#[cfg(not(target_arch = "wasm32"))]
mod conformance;
#[cfg(not(target_arch = "wasm32"))]
mod effect_contract;
#[cfg(not(target_arch = "wasm32"))]
mod effect_interchange;
#[cfg(not(target_arch = "wasm32"))]
mod graph;
mod protocol;
#[cfg(not(target_arch = "wasm32"))]
mod rack;
#[cfg(not(target_arch = "wasm32"))]
mod scheduler;
#[cfg(not(target_arch = "wasm32"))]
mod session;

#[cfg(not(target_arch = "wasm32"))]
const INTERNAL_SUBJECT: &str = "ENGINE_V2_INTERNAL_BENCH_SUBJECT";
#[cfg(not(target_arch = "wasm32"))]
const SUBJECTS: &[&str] = &[
    "bootstrap",
    "builtins",
    "conformance",
    "effect-contract",
    "effect-interchange",
    "graph",
    "protocol",
    "rack",
    "scheduler",
    "session",
];

#[cfg(not(target_arch = "wasm32"))]
fn run_subject(subject: &str) {
    match subject {
        "bootstrap" => bootstrap::main(),
        "builtins" => builtins::main(),
        "conformance" => conformance::main(),
        "effect-contract" => effect_contract::main(),
        "effect-interchange" => effect_interchange::main(),
        "graph" => graph::main(),
        "protocol" => protocol::main(),
        "rack" => rack::main(),
        "scheduler" => scheduler::main(),
        "session" => session::main(),
        _ => unreachable!("dispatcher validates internal subjects"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn usage() -> ! {
    eprintln!(
        "usage: miso_engine_bench <{}> [subject arguments]",
        SUBJECTS.join("|")
    );
    std::process::exit(2);
}

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {
    protocol::main();
}

#[cfg(not(target_arch = "wasm32"))]
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
    let subject_args: Vec<OsString> = args.collect();
    let mut command = Command::new(std::env::current_exe().expect("current executable path"));
    command.args(subject_args).env(INTERNAL_SUBJECT, subject);
    launch(command);
}
