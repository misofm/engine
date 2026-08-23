//! Native bootstrap host shell.

use std::process::ExitCode;

fn main() -> ExitCode {
    // Master plan #83 D4: the engine is compiled for a pinned instruction set and dispatches
    // nothing at runtime, so a CPU without it would execute an illegal instruction rather than
    // degrade. Attest once, before anything else, and refuse to start on an error -- never a
    // silent scalar fallback.
    if let Err(attestation) = miso_engine_lane::attest_host() {
        eprintln!("miso-engine-host-native refusing to start: {attestation}");
        return ExitCode::FAILURE;
    }
    println!("{:?}", miso_engine_target_smoke::target_smoke());
    ExitCode::SUCCESS
}
