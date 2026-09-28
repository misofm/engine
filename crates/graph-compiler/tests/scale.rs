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

/// The graph [`scale_session`] lowers to: seven stage nodes per track, the route and the output.
const NODES: u64 = 7 * TRACKS as u64 + 2;

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
/// where no bank attaches; the refused compile hands both prepared inputs back.
///
/// Issue #1045: this is the per-PR half, in the debug job with overflow checks on. The node cap
/// sits one below the graph this session lowers to, so the refusal is also a count of that graph
/// at 65,537 tracks: a compiled track ceiling adds a diagnostic other than
/// `graph.resource.limit`, and a narrowed track index that drops any track's nodes fits under the
/// cap and is no longer refused. The unconstrained compile, the bind and the render run nightly,
/// in release under a wall-clock bound, in [`compiles_and_binds_65_537_tracks_with_builtins`].
#[test]
fn compiles_65_537_tracks_or_rejects_only_a_configured_resource() {
    let session = scale_session();
    let builtins = prepare_session_builtins(&session, &[], builtin_caps())
        .expect("constrained scale builtins");

    let mut constrained = graph_caps();
    constrained.maximum_nodes = NODES - 1;
    let failure = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: constrained,
    })
    .err()
    .expect("a node cap one below the session's graph rejects");
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
    assert_eq!(
        failure.builtins.processor_count(),
        3 * TRACKS as usize,
        "every track's three builtin processors were prepared"
    );
}

/// Issue #962: the same session through the production entry, at the width this build renders at,
/// then bound and rendered for one block.
///
/// Until #964 the gate above compiled without builtins at `Backend::Scalar`, where no bank
/// attaches, and that is how a bank-membership scan quadratic in the track count stayed hidden on
/// every host's compile path (`PreparedGraphPlan::with_builtin_banks`, about 160 s in release
/// before the fix). This one attaches the builtin banks every host renders and then binds the
/// plan, because bind had quadratic scans of its own. The bank-membership scan and the bind scans
/// this session reaches are exercised by the track count alone, so a regression in them shows up
/// as this test's time, which the bound below turns into a failure. This session has no effects
/// and one route, so #962's effect-control, `Backend::Scalar` interval and route-fold metadata
/// fixes are not exercised here at scale (#967 adds sessions that reach them).
///
/// Issue #1045: this runs nightly, in release with overflow checks on (`nightly.yml`,
/// `release-budgets`), under a 60 s wall-clock bound on the whole path from the session compile to
/// the rendered block. The linear code takes about a third of that; #962's quadratic scans took
/// minutes. It is the only test that compiles without a cap and binds at this size, so a track
/// ceiling or a narrowed index in bank attachment or bind is caught here, a day late at most.
#[test]
#[ignore = "release-mode 65,537-track budget; runs nightly"]
fn compiles_and_binds_65_537_tracks_with_builtins() {
    use std::time::{Duration, Instant};

    let started = Instant::now();
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
    assert_eq!(artifact.report().estimate.logical_nodes, NODES);
    assert_eq!(artifact.report().estimate.edges, 6 * u64::from(TRACKS) + 2);
    assert_eq!(artifact.report().estimate.routes, 1);
    assert_eq!(artifact.report().estimate.effects, 0);
    assert_eq!(artifact.graph().sequential_schedule.len() as u64, NODES);
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
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("the bound plan renders");
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(60),
        "65,537-track session compile, builtins, graph compile, bind and one block took {elapsed:?}"
    );
}
