//! Generated graph scale gate above the former 65,536 boundary.

use miso_engine_core::TargetCapabilities;
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::GraphCompileCaps;
use miso_engine_graph_compiler::KernelDispatch;
use miso_engine_graph_compiler::{GraphCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

fn graph_caps() -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: u64::MAX,
        maximum_edges: u64::MAX,
        maximum_schedule_items: u64::MAX,
        maximum_dependency_levels: u64::MAX,
        maximum_audio_buffer_samples: u64::MAX,
        maximum_delay_samples_per_edge: u64::MAX,
        maximum_total_delay_samples: u64::MAX,
        maximum_graph_bytes: u64::MAX,
        maximum_plan_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_finite_tail_samples: u64::MAX,
    }
}

#[test]
fn compiles_65_537_tracks_or_rejects_only_a_configured_resource() {
    let mut model = parse_session_toml(SESSION).expect("fixture");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.limits.memory_bytes = u64::MAX;
    model.tracks.clear();
    model.tracks.reserve(65_537);
    for index in 0..65_537_u32 {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
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
    .expect("session scale gate");

    let mut constrained = graph_caps();
    constrained.maximum_nodes = 1;
    let failure = GraphCompiler::compile(GraphCompileRequest {
        dispatch: KernelDispatch::select(TargetCapabilities::from_detected(
            false, false, false, false,
        )),
        plan_id: 1,
        effects: EffectPreparedSession {
            session: session.clone(),
            entries: Vec::new(),
        },
        caps: constrained,
    })
    .err()
    .expect("configured cap rejects");
    assert!(
        failure
            .diagnostics
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.code == "graph.resource.limit")
    );

    let artifact = GraphCompiler::compile(GraphCompileRequest {
        dispatch: KernelDispatch::select(TargetCapabilities::from_detected(
            false, false, false, false,
        )),
        plan_id: 2,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        caps: graph_caps(),
    })
    .unwrap_or_else(|failure| panic!("scale diagnostics: {:?}", failure.diagnostics));
    assert_eq!(artifact.report.estimate.logical_nodes, 458_761);
    assert_eq!(artifact.report.estimate.edges, 393_224);
    assert_eq!(artifact.report.estimate.routes, 1);
    assert_eq!(artifact.report.estimate.effects, 0);
    assert_eq!(artifact.report.sequential_schedule.len(), 458_761);
}
