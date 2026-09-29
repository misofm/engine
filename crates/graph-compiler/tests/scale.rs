//! Generated graph scale gates above the former 65,536 boundary.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::EffectPreparedSession;
use effect_contract::{LatencySamples, TailSamples};
use engine::realtime::{PlanarBufferMut, RenderEnvelope, RenderError, RenderIo, RenderTime};
use graph::{
    DependencyLevel, GraphBindingBlock, GraphCompileCaps, GraphEdge, GraphEdgeId, GraphNode,
    GraphNodeBinding, GraphNodeId, GraphPortId, GraphPortKind, GraphResourceEstimate,
    GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, RouteTransform, StableGraphId, TrackStage,
};
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
/// at 65,537 tracks: a compiled track ceiling adds a second diagnostic, whatever its code, and a
/// narrowed track index that drops any track's nodes fits under the cap and is no longer refused.
/// Bind and render at this size are [`a_hand_built_65_537_input_plan_binds_and_renders_every_track`]'s;
/// the unconstrained compile runs nightly, in release under a wall-clock bound, in
/// [`compiles_and_binds_65_537_tracks_with_builtins`].
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
    let diagnostics: Vec<_> = failure
        .diagnostics
        .diagnostics()
        .iter()
        .map(|diagnostic| (diagnostic.code, diagnostic.path.as_str()))
        .collect();
    assert_eq!(
        diagnostics,
        [("graph.resource.limit", "$.graph_compile_caps")],
        "the node cap is the only refusal"
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

/// A track input that writes a constant block: every word of it, as a node the graph feeds nothing
/// must (`GraphRuntimeProcessor::process`).
struct Constant;

impl GraphRuntimeProcessor for Constant {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(1.0);
        block.right.fill(2.0);
        Ok(())
    }
}

/// Issue #1045: bind and render above 65,536 track inputs, per PR.
///
/// The session compile above refuses before anything binds, and the full compile and bind run
/// nightly, so this plan is built by hand, through the public [`PreparedGraphPlan::new`]:
/// [`TRACKS`] track inputs, each routed through its own unity route to one output. Every input
/// writes 1.0 left and 2.0 right, so the output is the exact count of the tracks that reached it
/// (every partial sum is an integer below 2^24). A ceiling in bind or render refuses, and a
/// narrowed track or unit index drops tracks from the sum. It builds, binds and renders in a few
/// seconds in debug, beside the compile above.
#[test]
fn a_hand_built_65_537_input_plan_binds_and_renders_every_track() {
    const FRAMES: u32 = 16;
    let envelope = RenderEnvelope {
        sample_rate: engine::SampleRateHz(48_000),
        quantum: engine::QuantumFrames(FRAMES),
        output_channels: core::num::NonZeroUsize::new(2).expect("stereo"),
    };
    let id = |text: String| StableGraphId::parse(&text).expect("generated ID");
    let mut inputs: Vec<_> = (0..TRACKS)
        .map(|index| GraphNodeId::TrackStage {
            track_id: id(format!("track-{index}")),
            stage: TrackStage::Input,
        })
        .collect();
    let route_ids: Vec<_> = (0..TRACKS)
        .map(|index| id(format!("route-{index}")))
        .collect();
    let mut routes: Vec<_> = route_ids
        .iter()
        .map(|route_id| GraphNodeId::Route {
            route_id: route_id.clone(),
        })
        .collect();
    let output = GraphNodeId::Output {
        output_id: id("main".to_owned()),
    };
    let port = |node: &GraphNodeId, kind| GraphPortId {
        node: node.clone(),
        kind,
        effect_port: None,
    };
    let mut edges = Vec::with_capacity(2 * TRACKS as usize);
    for ((input, route), route_id) in inputs.iter().zip(&routes).zip(&route_ids) {
        edges.push(GraphEdge {
            id: GraphEdgeId::RouteSource {
                route_id: route_id.clone(),
            },
            source: port(input, GraphPortKind::MainOutput),
            destination: port(route, GraphPortKind::MainInput),
            path: "$.scale".to_owned(),
        });
        edges.push(GraphEdge {
            id: GraphEdgeId::RouteDestination {
                route_id: route_id.clone(),
            },
            source: port(route, GraphPortKind::MainOutput),
            destination: port(&output, GraphPortKind::MainInput),
            path: "$.scale".to_owned(),
        });
    }
    edges.sort_by(|left, right| left.id.cmp(&right.id));
    inputs.sort();
    routes.sort();
    let levels = [inputs.clone(), routes.clone(), vec![output.clone()]];
    let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
    let mut nodes: Vec<_> = schedule
        .iter()
        .map(|node| GraphNode {
            id: node.clone(),
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        })
        .collect();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    let mut required = inputs.clone();
    required.push(output.clone());
    let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 1045,
        spec: GraphSpec {
            nodes,
            ports: Vec::new(),
            edges,
        },
        sequential_schedule: schedule,
        dependency_levels: levels
            .into_iter()
            .enumerate()
            .map(|(level, nodes)| DependencyLevel {
                level: level as u64,
                nodes,
            })
            .collect(),
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: GraphResourceEstimate {
            logical_nodes: 0,
            materialized_nodes: 0,
            edges: 0,
            schedule_items: 0,
            dependency_levels: 0,
            reductions: 0,
            routes: 0,
            effects: 0,
            audio_buffer_samples: 0,
            total_delay_samples: 0,
            delay_bytes: 0,
            graph_metadata_bytes: 0,
            declared_effect_bytes: 0,
            effect_bank_count: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        },
        envelope,
        required_bindings: required,
        routes: routes
            .iter()
            .map(|node| PreparedRoute {
                node: node.clone(),
                transform: RouteTransform {
                    gain: 1.0,
                    ll: 1.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
            })
            .collect(),
        track_delays: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        effect_observations: Vec::new(),
        banks: Vec::new(),
        builtin_banks: Vec::new(),
        observers: Vec::new(),
    });
    let mut bindings: Vec<_> = inputs
        .into_iter()
        .map(|node| GraphNodeBinding::new(node, Box::new(Constant)))
        .collect();
    bindings.push(GraphNodeBinding::identity(output));
    assert_eq!(bindings.len(), TRACKS as usize + 1);
    let mut bound = plan
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("65,537-input bind: {}", failure.code));
    let frames = FRAMES as usize;
    let mut pcm = vec![f32::NAN; frames * 2];
    bound
        .render(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime { absolute_sample: 0 },
        )
        .expect("the 65,537-input plan renders");
    let tracks = TRACKS as f32;
    assert!(
        pcm[..frames].iter().all(|word| *word == tracks)
            && pcm[frames..].iter().all(|word| *word == 2.0 * tracks),
        "every track reaches the output: {:?}",
        &pcm[..2]
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
