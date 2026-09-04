//! Emit the browser qualification fixture's resource report, compiled natively.
//!
//! Issue #217. `hosts/host-web/tests/browser-v1/expected.json` pins the resource
//! report of the shipped `wasm32-unknown-unknown` worklet module. Those rows went stale twice
//! (#212, #216) with no gate red, because the only consumer of the pin is the browser-correctness
//! harness, which is not a sweep row.
//!
//! `scripts/check-browser-expected-resources.py` closes that. This example is its native leg: the
//! *same* fixture session, through the *same* facade, with the *same* options the direct
//! oracle writes into the module's `WebBootOptions` block -- differing only in the
//! target it is compiled for. The gate uses it as an independent witness for the rows that are
//! target-independent, and to prove that the rows that are *not* really are not.
//!
//! The options below stay equal to the direct oracle: physical rate/quantum plus a one-quantum
//! ring override that makes backpressure deterministic.

use host_web::{AudioWorkletEngineHost, WebBootOptions};

const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM_FRAMES: u32 = 128;

/// The fixture the browser harness compiles, verbatim.
const SESSION_DOCUMENT: &str = include_str!("../tests/browser-v1/session.json");

fn main() {
    let options = WebBootOptions {
        require_sample_rate_hz: SAMPLE_RATE_HZ,
        require_quantum_frames: QUANTUM_FRAMES,
        source_ring_frames: QUANTUM_FRAMES,
        ..WebBootOptions::explicit_defaults()
    };
    let host = AudioWorkletEngineHost::boot(SESSION_DOCUMENT.as_bytes(), options).unwrap_or_else(
        |failure| panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic())),
    );

    // Printed in the JSON shape and the key spelling `expected.json` uses, u64 rows as strings,
    // so the gate compares the two documents without a per-row translation table.
    let report = host.resources();
    let rows: [(&str, u64); 21] = [
        ("optionsBytes", report.options_bytes),
        ("statusBytes", report.status_bytes),
        ("sessionDocumentBytes", report.session_document_bytes),
        ("diagnosticBytes", report.diagnostic_bytes),
        ("idStagingBytes", report.id_staging_bytes),
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
