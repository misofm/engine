//! Stable process-boundary fingerprint for the canonical issue-006 graph fixture.

use core::fmt::Write as _;

use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::GraphCompileCaps;
use miso_engine_graph_compiler::{GraphCompileRequest, GraphCompiler};
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};
use sha2::{Digest, Sha256};

const SESSION: &str = include_str!("../../../../fixtures/session/v1/canonical.toml");

fn main() {
    let mut model = parse_session_toml(SESSION).unwrap_or_else(|diagnostics| {
        panic!("session parse diagnostics: {diagnostics:?}");
    });
    model.tracks[0].dynamic.effects.clear();
    model.automation.clear();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .unwrap_or_else(|diagnostics| panic!("session compile diagnostics: {diagnostics:?}"));
    let artifact = GraphCompiler::compile(GraphCompileRequest {
        plan_id: 0,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        caps: GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|failure| panic!("graph compile diagnostics: {:?}", failure.diagnostics));
    let report = artifact.report;
    println!(
        concat!(
            "{{\"schema\":1,\"fixture\":\"direct-route\",",
            "\"canonical_bytes\":{},\"graph_sha256\":\"{}\",",
            "\"dot_bytes\":{},\"dot_sha256\":\"{}\",",
            "\"nodes\":{},\"edges\":{},\"schedule_items\":{},",
            "\"levels\":{},\"route_timings\":{},\"buffer_assignments\":{}}}"
        ),
        report.canonical_debug_bytes.len(),
        report.sha256,
        report.dot.len(),
        sha256_hex(report.dot.as_bytes()),
        report.nodes.len(),
        report.edges.len(),
        report.sequential_schedule.len(),
        report.dependency_levels.len(),
        report.route_timings.len(),
        report.buffer_assignments.len(),
    );
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(&mut output, "{byte:02x}").expect("String write");
    }
    output
}
