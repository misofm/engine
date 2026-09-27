//! Generated graph scale gates above the former 65,536 boundary.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::EffectPreparedSession;
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use graph::{GraphCompileCaps, GraphNodeBinding, GraphRuntimeBindings};
use graph_compiler::Backend;
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, CompiledSession, RouteSource, SendTap, StableId, compile_session,
    parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// One past the former 65,536 boundary.
const TRACKS: u32 = 65_537;

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

/// The fixture's first track, stripped of its effects, [`TRACKS`] times, with one route out of
/// track zero.
fn scale_session() -> CompiledSession {
    let mut model = parse_session_json(SESSION).expect("fixture");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.tracks.clear();
    model.tracks.reserve(TRACKS as usize);
    for index in 0..TRACKS {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
    compile_session(
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
    .expect("session scale gate")
}

fn builtin_caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

/// The session compiles at 65,537 tracks, and the only thing that can refuse it is a cap the
/// host configured.
///
/// Issue #964: through `compile_with_builtins` at `Backend::current()`, the production entry at
/// the width this build renders at. It compiled without builtins at `Backend::Scalar` until then,
/// where no bank attaches; both compiles now carry the builtin banks, and the refused one hands
/// both prepared inputs back.
#[test]
fn compiles_65_537_tracks_or_rejects_only_a_configured_resource() {
    let session = scale_session();
    let dispatch = Backend::current();

    let mut constrained = graph_caps();
    constrained.maximum_nodes = 1;
    let failure = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id: 1,
        effects: EffectPreparedSession {
            session: session.clone(),
            entries: Vec::new(),
        },
        builtins: prepare_session_builtins(&session, &[], builtin_caps())
            .expect("constrained scale builtins"),
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
    assert_eq!(
        failure.builtins.tails().count(),
        TRACKS as usize,
        "the refused compile hands every track's builtins back"
    );

    let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("scale builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id: 2,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: graph_caps(),
    })
    .unwrap_or_else(|failure| panic!("scale diagnostics: {:?}", failure.diagnostics));
    assert_eq!(artifact.report().estimate.logical_nodes, 458_761);
    assert_eq!(artifact.report().estimate.edges, 393_224);
    assert_eq!(artifact.report().estimate.routes, 1);
    assert_eq!(artifact.report().estimate.effects, 0);
    assert_eq!(artifact.graph().sequential_schedule.len(), 458_761);
}

/// Issue #962: the same session through the production entry, at the width this build renders at,
/// then bound and rendered for one block.
///
/// Until #964 the gate above compiled without builtins at `Backend::Scalar`, where no bank
/// attaches, and that is how a bank-membership scan quadratic in the track count stayed hidden on
/// every host's compile path (`PreparedGraphPlan::with_builtin_banks`, about 160 s in release
/// before the fix). This one attaches the builtin banks every host renders and then binds the
/// plan, because bind had quadratic scans of its own. Both are exercised by the track count alone,
/// so a regression shows up as this test's time rather than as an assertion.
///
/// There is no wall-clock bound, because a test-harness clock is not one CI can hold reliably: the
/// runners' speed varies and this binary's tests run in parallel. The backstop is the debug job's
/// timeout, which the quadratic code cannot finish inside. The measured times are recorded in the
/// #962 brief's evidence.
#[test]
fn compiles_and_binds_65_537_tracks_with_builtins() {
    let session = scale_session();
    let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("scale builtins");
    let dispatch = Backend::current();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id: 3,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: graph_caps(),
    })
    .unwrap_or_else(|failure| panic!("with-builtins scale diagnostics: {:?}", failure.diagnostics));
    assert_eq!(artifact.graph().sequential_schedule.len(), 458_761);
    // Every track's three bankable builtin stages -- post-input, fader, matrix -- are bank members
    // at a SIMD width; at scalar width no bank attaches at all.
    let members = if dispatch.width() > 1 {
        3 * TRACKS as usize
    } else {
        0
    };
    assert_eq!(artifact.graph().builtin_bank_members().count(), members);

    let envelope = artifact.envelope();
    let nodes: Vec<_> = artifact
        .external_binding_nodes()
        .map(|node| GraphNodeBinding::identity(node.clone()))
        .collect();
    assert_eq!(
        nodes.len(),
        TRACKS as usize + 1,
        "every track input and the session output"
    );
    let mut bound = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("with-builtins scale bind: {}", failure.code));
    let frames = envelope.quantum.0 as usize;
    let mut pcm = vec![0.0_f32; frames * 2];
    bound
        .plan
        .render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("the bound plan renders");
}
