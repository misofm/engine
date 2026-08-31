//! Native bootstrap host shell.
//!
//! Platform audio I/O is issue 023; this binary only attests the CPU and the floating-point
//! environment, then prints the target smoke values. The rules a real callback must honour are not restated here: they are the
//! `# Host callback contract (V1)` section of `host-core`, which is normative for every
//! embedding. A second transcription of them is exactly the defect #106 F1 removed from this tree.

use std::process::ExitCode;

fn main() -> ExitCode {
    // Master plan #83 D4: the engine is compiled for a pinned instruction set and dispatches
    // nothing at runtime, so a CPU without it would execute an illegal instruction rather than
    // degrade. Attest once, before anything else, and refuse to start on an error -- never a
    // silent scalar fallback.
    if let Err(attestation) = lane::attest_host() {
        eprintln!("host-native refusing to start: {attestation}");
        return ExitCode::FAILURE;
    }
    // Issue #146: this process can pin the canonical floating-point environment and hand the
    // caller's word back bit-exactly. It is the process-wide smoke check, not the load-bearing one
    // -- a control word belongs to a thread, so the render thread re-attests for itself when
    // `host_core::StartedRenderSession::start` runs there. Refusing here means the
    // build cannot pin at all, which no real audio callback would survive either.
    if let Err(rejection) = lane::attest_fp_environment() {
        eprintln!("host-native refusing to start: {rejection}");
        return ExitCode::FAILURE;
    }
    println!("{:?}", target_smoke::target_smoke());
    ExitCode::SUCCESS
}
