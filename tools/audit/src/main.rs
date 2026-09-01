#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Consolidated audit and fixture subjects.

use std::ffi::OsString;
use std::process::Command;

mod builtins;
mod builtins_fixture_check;
mod builtins_graph;
mod capi;
mod compressor;
mod delay;
mod fixture_builtins;
mod fixture_builtins_listening;
mod fp_env;
mod gate_expander;
mod graph;
mod parametric_eq;
mod protocol;
mod realtime;
mod record;
mod source;
mod source_duration;
mod source_fixture;
mod unfused_fma;
mod vectorization;

const INTERNAL_SUBJECT: &str = "ENGINE_V2_INTERNAL_AUDIT_SUBJECT";
const SUBJECTS: &[&str] = &[
    "builtins",
    "builtins-fixture",
    "builtins-graph",
    "capi",
    "compressor",
    "delay",
    "fixture-builtins",
    "fixture-builtins-listening",
    "fixture-source",
    "fp-env",
    "gate-expander",
    "graph",
    "parametric-eq",
    "protocol",
    "realtime",
    "source",
    "source-duration",
    "unfused-fma",
    "vectorization",
];

fn run_subject(subject: &str) {
    match subject {
        "builtins" => builtins::main(),
        "builtins-fixture" => builtins_fixture_check::main(),
        "builtins-graph" => builtins_graph::main(),
        "capi" => capi::main(),
        "compressor" => compressor::main(),
        "delay" => delay::main(),
        "fixture-builtins" => fixture_builtins::main(),
        "fixture-builtins-listening" => fixture_builtins_listening::main(),
        "fixture-source" => source_fixture::main(),
        "fp-env" => fp_env::main(),
        "gate-expander" => gate_expander::main(),
        "graph" => graph::main(),
        "parametric-eq" => parametric_eq::main(),
        "protocol" => protocol::main(),
        "realtime" => realtime::main(),
        "source" => source::main(),
        "source-duration" => source_duration::main(),
        "unfused-fma" => unfused_fma::main(),
        "vectorization" => vectorization::main(),
        _ => unreachable!("dispatcher validates internal subjects"),
    }
}

fn usage() -> ! {
    eprintln!("usage: audit <{}> [subject arguments]", SUBJECTS.join("|"));
    std::process::exit(2);
}

fn launch(mut command: Command) -> ! {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let error = command.exec();
        eprintln!("failed to launch audit subject: {error}");
        std::process::exit(1);
    }
    #[cfg(not(unix))]
    {
        let status = command.status().unwrap_or_else(|error| {
            eprintln!("failed to launch audit subject: {error}");
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
    let subject_args: Vec<OsString> = args.collect();
    let mut command = Command::new(std::env::current_exe().expect("current executable path"));
    command.args(subject_args).env(INTERNAL_SUBJECT, subject);
    launch(command);
}
