//! Pure-Rust mobile host shell.
//!
//! Platform linking and audio callbacks are intentionally deferred to issue 023.

use miso_engine_target_smoke::TargetSmoke;

/// Return portable bootstrap values for a mobile embedding host.
#[must_use]
pub fn mobile_target_smoke() -> TargetSmoke {
    miso_engine_target_smoke::target_smoke()
}
