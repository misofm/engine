//! Native bootstrap host shell.
//!
//! Platform audio I/O is issue 023; this binary only attests the CPU and prints the target smoke
//! values. The rules a real callback must honour are not restated here: they are the
//! `# Host callback contract (V1)` section of `miso-engine-host-core`, which is normative for every
//! embedding. A second transcription of them is exactly the defect #106 F1 removed from this tree.

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
