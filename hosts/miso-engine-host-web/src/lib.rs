//! Browser-Wasm host shell.
//!
//! AudioWorklet integration is intentionally deferred to issue 024.

use miso_engine_target_smoke::TargetSmoke;

/// Return portable bootstrap values for a Web embedding host.
#[must_use]
pub fn web_target_smoke() -> TargetSmoke {
    miso_engine_target_smoke::target_smoke()
}
