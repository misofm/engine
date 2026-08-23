//! The compressor against the shared effect-contract conformance harness (issue #95, eval E6).
//!
//! Until #95 the harness could only run against its own reference mock: it built the prepare
//! request from a hard-coded `parameter 0, Left/Right, 1.0` pair, and two of its probes asserted
//! the *deleted* per-value sanitisation contract (`report.sanitized_main_samples == 3`). Both are
//! fixed at the harness — the request is built from `default_initial_values(descriptor)`, and the
//! sanitisation probes assert the D7 property (`x == x && |x| < 1e30` on the produced block)
//! instead of a per-sample count. A contract whose only conforming implementation is its own mock
//! is not evidence, so this test exists to make the harness answer for a real effect.
//!
//! What it covers, per declared quality x link mode x enabled/bypass: exact prepared metadata,
//! metadata immutability across `process`, the D7 output-block bounds, impulse latency equal to
//! the declared `latency`, deterministic all-or-none snapshot/restore, lane isolation, and
//! rejection of a malformed automation span.
//!
//! Red mutation (run and observed RED): make the compressor's bypass path emit its dry impulse one
//! sample early, and `latency.impulse` fails for every bypassed configuration.

use miso_engine_compressor::CompressorFactory;
use miso_engine_conformance::{ConformanceConfig, run_effect_conformance};

#[test]
fn the_compressor_passes_the_effect_contract_launch_gates() {
    let report = run_effect_conformance(
        &CompressorFactory,
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
    assert!(
        report.launch_gates.prepared_configurations > 0,
        "the harness must actually prepare something"
    );
    assert!(report.passed());
}
