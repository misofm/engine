//! The parametric EQ against the shared effect-contract conformance harness (issue #95, eval E6).
//!
//! A second, structurally different effect: zero declared latency, a much larger parameter table,
//! and a per-lane state payload that carries the runtime codec's two-word header. Together with
//! `miso-engine-compressor/tests/conformance.rs` (882 samples of lookahead, a linked detector,
//! a lookahead ring whose index advances on silence) it is what stops the harness from quietly
//! re-specialising to one effect's shape.
//!
//! Red mutation (run and observed RED): return `LatencySamples(1)` from the EQ's quality rows
//! while the kernel stays at zero delay, and `latency.impulse` fails.

use miso_engine_conformance::{ConformanceConfig, run_effect_conformance};
use miso_engine_parametric_eq::ParametricEqFactory;

#[test]
fn the_parametric_eq_passes_the_effect_contract_launch_gates() {
    let report = run_effect_conformance(
        &ParametricEqFactory,
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(
        report.launch_gates.failures.is_empty(),
        "launch gate failures: {:?}",
        report.launch_gates.failures
    );
    assert!(report.launch_gates.prepared_configurations > 0);
    assert!(report.passed());
}
