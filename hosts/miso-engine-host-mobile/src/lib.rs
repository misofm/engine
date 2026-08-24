//! Pure-Rust mobile host shell.
//!
//! Platform linking and audio callbacks are intentionally deferred to issue 023. The rules such a
//! callback must honour -- which call runs on which thread, that a `RenderError` is sticky and
//! frees nothing, and who owns the worker pool and thread priority -- are the
//! `# Host callback contract (V1)` section of `miso-engine-host-core`, normative for every
//! embedding and deliberately not restated here.

use miso_engine_lane::{HostAttestation, attest_host};
use miso_engine_target_smoke::TargetSmoke;

/// Attest the CPU, then return portable bootstrap values for a mobile embedding host.
///
/// The attestation is master plan #83 D4: the engine is built for a pinned instruction set with
/// no runtime dispatch, so a host whose CPU cannot execute it must refuse to start rather than
/// degrade silently. On AArch64 — the only mobile target — NEON is baseline and this always
/// succeeds; the call is here so that the mobile shell cannot become the one entry point that
/// forgot to make the check when it grows a real audio callback.
///
/// # Errors
///
/// Returns [`HostAttestation`] naming the first pinned CPU feature this host lacks.
pub fn mobile_target_smoke() -> Result<TargetSmoke, HostAttestation> {
    attest_host()?;
    Ok(miso_engine_target_smoke::target_smoke())
}
