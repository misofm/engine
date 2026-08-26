//! Command-line entry point for the session authoring gate.

use std::process::ExitCode;

fn main() -> ExitCode {
    miso_engine_session_validator::run(std::env::args().skip(1))
}
