//! Gate G5 runner: compare the frozen corpus's digests native versus wasm.
//!
//! ```text
//! miso_engine_wasm_gates --native
//! miso_engine_wasm_gates <guest.wasm> --expect-backend scalar|simd4|simd8
//! miso_engine_wasm_gates --print-pins
//! ```
//!
//! Each run prints one JSON evidence line and exits non-zero on the first mismatch.
//! `scripts/run-wasm-gates.sh` builds both guest artifacts and runs all three legs.

use std::path::PathBuf;
use std::process::ExitCode;

use miso_engine_wasm_gates::{
    ExpectedBackend, WASMTIME_LICENCE, WASMTIME_VERSION, native_report, native_timing_report,
    print_lane_pins, wasm_report, wasm_timing_report,
};

/// Usage text, printed on an argument error.
const USAGE: &str = "usage:\n  \
     miso_engine_wasm_gates --native\n  \
     miso_engine_wasm_gates <guest.wasm> --expect-backend scalar|simd4|simd8\n  \
     miso_engine_wasm_gates --native-timing scalar|simd4|simd8\n  \
     miso_engine_wasm_gates <guest.wasm> --wasm-timing scalar|simd4|simd8 \
--expect-backend scalar|simd4|simd8\n  \
     miso_engine_wasm_gates --print-pins";

/// Parses a width name into the corpus width index the timing arm drives.
fn width_index(name: &str) -> Option<usize> {
    match name {
        "scalar" => Some(0),
        "simd4" => Some(1),
        "simd8" => Some(2),
        _ => None,
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("--native") if arguments.len() == 1 => run_native(),
        // Issue #163 phase 0b. Descriptive only, and a separate record family: a `wasm-simd128`
        // line is never comparable with a native console record.
        Some("--native-timing") if arguments.len() == 2 => match width_index(&arguments[1]) {
            Some(width) => {
                println!("{}", native_timing_report(width).json());
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("unknown width '{}'\n{USAGE}", arguments[1]);
                ExitCode::from(2)
            }
        },
        Some("--print-pins") if arguments.len() == 1 => {
            print!("{}", print_lane_pins());
            ExitCode::SUCCESS
        }
        Some("--version") if arguments.len() == 1 => {
            println!("wasmtime {WASMTIME_VERSION} ({WASMTIME_LICENCE})");
            ExitCode::SUCCESS
        }
        Some(path)
            if arguments.len() == 5
                && arguments[1] == "--wasm-timing"
                && arguments[3] == "--expect-backend" =>
        {
            match (
                width_index(&arguments[2]),
                ExpectedBackend::parse(&arguments[4]),
            ) {
                (Some(width), Ok(expected)) => {
                    match wasm_timing_report(&PathBuf::from(path), expected, width) {
                        Ok(report) => {
                            println!("{}", report.json());
                            ExitCode::SUCCESS
                        }
                        Err(error) => {
                            eprintln!("wasm timing failure: {error:?}");
                            ExitCode::FAILURE
                        }
                    }
                }
                _ => {
                    eprintln!("{USAGE}");
                    ExitCode::from(2)
                }
            }
        }
        Some(path) if arguments.len() == 3 && arguments[1] == "--expect-backend" => {
            match ExpectedBackend::parse(&arguments[2]) {
                Ok(expected) => run_wasm(PathBuf::from(path), expected),
                Err(unknown) => {
                    eprintln!("unknown backend '{unknown}'\n{USAGE}");
                    ExitCode::from(2)
                }
            }
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// The native leg: the corpus run in this process at every width against the pins.
fn run_native() -> ExitCode {
    let report = native_report();
    println!("{}", report.json());
    if report.mismatches.is_empty() {
        ExitCode::SUCCESS
    } else {
        for mismatch in &report.mismatches {
            eprintln!("native mismatch: {mismatch}");
        }
        ExitCode::FAILURE
    }
}

/// The wasm leg: the same corpus executed under wasmtime against the same pins.
fn run_wasm(path: PathBuf, expected: ExpectedBackend) -> ExitCode {
    match wasm_report(&path, expected) {
        Ok(report) => {
            println!("{}", report.json());
            if report.mismatches.is_empty() {
                ExitCode::SUCCESS
            } else {
                for mismatch in &report.mismatches {
                    eprintln!("wasm mismatch: {mismatch}");
                }
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            eprintln!("wasm gate failure: {error:?}");
            ExitCode::FAILURE
        }
    }
}
