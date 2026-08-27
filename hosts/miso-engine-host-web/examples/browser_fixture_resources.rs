//! Emit the browser qualification fixture's resource report, compiled natively.
//!
//! Issue #217. `hosts/miso-engine-host-web/tests/browser-v1/expected.json` pins the resource
//! report of the shipped `wasm32-unknown-unknown` worklet module. Those rows went stale twice
//! (#212, #216) with no gate red, because the only consumer of the pin is the browser-correctness
//! harness, which is not a sweep row.
//!
//! `scripts/check-browser-expected-resources.py` closes that. This example is its native leg: the
//! *same* fixture session, through the *same* facade, with the *same* configuration the direct
//! oracle writes into the module's `WebPrepareConfigV1` staging block -- differing only in the
//! target it is compiled for. The gate uses it as an independent witness for the rows that are
//! target-independent, and to prove that the rows that are *not* really are not.
//!
//! The configuration below must stay equal to `writeConfig`'s `LIMITS32`/`LIMITS64` in
//! `tests/browser-v1/direct-oracle.mjs`. `WebPrepareConfigV1::launch_defaults` is that vector
//! exactly, save for `source_ring_frames`: the oracle writes the quantum, the constructor derives
//! a launch default from the sample rate.

use miso_engine_host_web::{AudioWorkletEngineHost, RESULT_OK, WebPrepareConfigV1};

const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM_FRAMES: u32 = 128;

/// The fixture the browser harness compiles, verbatim.
const SESSION_TOML: &str = include_str!("../tests/browser-v1/session.toml");

fn main() {
    let config = WebPrepareConfigV1 {
        // The oracle writes `QUANTUM` here, not `default_source_ring_frames`.
        source_ring_frames: QUANTUM_FRAMES,
        ..WebPrepareConfigV1::launch_defaults(SAMPLE_RATE_HZ, QUANTUM_FRAMES)
    };
    let mut host = AudioWorkletEngineHost::new(config);
    assert_eq!(
        host.prepare(),
        RESULT_OK,
        "prepare the browser fixture host"
    );
    let staging = host
        .session_toml_mut()
        .expect("prepared session TOML staging");
    staging[..SESSION_TOML.len()].copy_from_slice(SESSION_TOML.as_bytes());
    assert_eq!(
        host.compile(SESSION_TOML.len()),
        RESULT_OK,
        "compile the browser fixture session: {:?}",
        host.diagnostic()
    );

    // Printed in the JSON shape and the key spelling `expected.json` uses, u64 rows as strings,
    // so the gate compares the two documents without a per-row translation table.
    let report = host.resources();
    let rows: [(&str, u64); 21] = [
        ("configBytes", report.config_bytes),
        ("statusBytes", report.status_bytes),
        ("sessionTomlBytes", report.session_toml_bytes),
        ("diagnosticBytes", report.diagnostic_bytes),
        ("sourceIdBytes", report.source_id_bytes),
        ("sourcePcmStagingBytes", report.source_pcm_staging_bytes),
        ("outputPcmBytes", report.output_pcm_bytes),
        ("bridgeMetadataBytes", report.bridge_metadata_bytes),
        ("bridgeRetainedBytes", report.bridge_retained_bytes),
        (
            "largestBridgeAllocationBytes",
            report.largest_bridge_allocation_bytes,
        ),
        ("sourceTotalBytes", report.source_total_bytes),
        ("sourceOverheadBytes", report.source_overhead_bytes),
        ("effectScalarStateBytes", report.effect_scalar_state_bytes),
        (
            "effectScalarScratchBytes",
            report.effect_scalar_scratch_bytes,
        ),
        ("builtinRetainedBytes", report.builtin_retained_bytes),
        (
            "graphSessionPlusPlanBytes",
            report.graph_session_plus_plan_bytes,
        ),
        (
            "graphIncrementalPlanBytes",
            report.graph_incremental_plan_bytes,
        ),
        ("graphMetadataBytes", report.graph_metadata_bytes),
        ("graphDelayBytes", report.graph_delay_bytes),
        (
            "largestNamedAllocationBytes",
            report.largest_named_allocation_bytes,
        ),
        (
            "observationRetainedBytes",
            report.observation_retained_bytes,
        ),
    ];

    let mut out = String::from("{\n");
    out.push_str(&format!("  \"sampleRateHz\": {},\n", report.sample_rate_hz));
    out.push_str(&format!(
        "  \"quantumFrames\": {},\n",
        report.quantum_frames
    ));
    out.push_str(&format!("  \"backend\": {}", report.backend));
    for (name, value) in rows {
        out.push_str(&format!(",\n  \"{name}\": \"{value}\""));
    }
    out.push_str("\n}\n");
    print!("{out}");
}
