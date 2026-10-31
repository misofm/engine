//! Generated graph scale gates above the former 65,536 boundary.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, EffectPreparedSession, attach_effect_live_controls,
    launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::{BankWidth, LatencySamples};
use engine::realtime::{
    LiveValuesSuperseded, PlanarBufferMut, RenderEnvelope, RenderError, RenderIo, RenderTime,
};
use graph::{
    DependencyLevel, GraphBindingBlock, GraphBuiltinBankResourceEstimate, GraphCompileCaps,
    GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding, GraphNodeId, GraphPortId, GraphPortKind,
    GraphPreparedBuiltinBank, GraphPreparedBuiltinBankProcessor, GraphResourceEstimate,
    GraphRuntimeBindings, GraphRuntimeProcessor, GraphSpec, PreparedGraphPlan,
    PreparedGraphPlanParts, PreparedRoute, RouteGate, RouteTransform, StableGraphId, TrackStage,
};
use graph_compiler::Backend;
use graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, CompiledSession, RouteSource, SendTap, StableId, compile_session,
    parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// The standing console, for its `eq` console slot (issue #967).
const CONSOLE: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");

/// One past the former 65,536 boundary.
const TRACKS: u32 = 65_537;

/// The graph [`scale_session`] lowers [`PLAIN`] to: seven stage nodes per track, the route and
/// the output.
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

/// What [`scale_session`] adds to its effect-less, one-route strip (issue #967).
#[derive(Clone, Copy)]
struct Shape {
    /// One route per track to the output, instead of one route out of track zero.
    routes: bool,
    /// The standing console's `eq` as a console slot on every track.
    console: bool,
}

/// The shape the per-PR gates compile: no effects and one route.
const PLAIN: Shape = Shape {
    routes: false,
    console: false,
};

/// The fixture's first track, stripped of its effects, [`TRACKS`] times, with one route out of
/// track zero, or the larger [`Shape`] `shape` asks for.
fn scale_session(shape: Shape) -> CompiledSession {
    let mut model = parse_session_json(SESSION).expect("fixture");
    let mut template = model.tracks[0].clone();
    template.console.clear();
    template.inserts.effects.clear();
    if shape.console {
        let console = parse_session_json(CONSOLE).expect("console fixture");
        model.console.pre_insert = console
            .console
            .pre_insert
            .iter()
            .filter(|declared| declared.slot.as_str() == "eq")
            .cloned()
            .collect();
        template.console = console.tracks[0]
            .console
            .iter()
            .filter(|entry| entry.slot.as_str() == "eq")
            .cloned()
            .collect();
    }
    model.automation.clear();
    model.tracks.clear();
    model.tracks.reserve(TRACKS as usize);
    for index in 0..TRACKS {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated ID");
        model.tracks.push(track);
    }
    let source = |index: u32| RouteSource::Track {
        track_id: StableId::parse(&format!("track-{index}")).expect("route track"),
        tap: SendTap::PostPan,
    };
    if shape.routes {
        let template = model.routes[0].clone();
        model.routes = (0..TRACKS)
            .map(|index| {
                let mut route = template.clone();
                route.id = StableId::parse(&format!("route-{index}")).expect("generated ID");
                route.source = source(index);
                route
            })
            .collect();
    } else {
        model.routes[0].source = source(0);
    }
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
/// It also plans the compiler's builtin banks for the same builtins, the step the refused compile
/// never reaches, so a ceiling in that plan or its resource accounting refuses here. Bank
/// attachment, bind and render at this size are the hand-built tests' below; the unconstrained
/// compile runs nightly, in release under a wall-clock bound, in
/// [`compiles_and_binds_65_537_tracks_with_builtins`].
#[test]
fn compiles_65_537_tracks_or_rejects_only_a_configured_resource() {
    let session = scale_session(PLAIN);
    let builtins = prepare_session_builtins(&session, &[], builtin_caps())
        .expect("constrained scale builtins");

    // The compiler's builtin bank plan for these builtins, which the refused compile never
    // reaches: every track's three bankable stages planned into full-width banks at the width
    // this build renders at (issue #1045, Sol's attempt-3 check).
    let backend = Backend::current();
    let lanes = u64::from(
        BankWidth::for_backend(backend)
            .expect("native builds render in banks")
            .lanes(),
    );
    let levels: Vec<DependencyLevel> = [
        TrackStage::PostInputBuiltins,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
    .into_iter()
    .zip(1..)
    .map(|(stage, level)| DependencyLevel {
        level,
        nodes: track_stages(stage),
    })
    .collect();
    let bank_resource = builtins
        .graph_builtin_bank_resource(
            backend,
            &levels,
            &builtins_compiler::SessionPoolClasses::from_session(&session),
        )
        .expect("the builtin bank plan at 65,537 tracks");
    assert_eq!(
        bank_resource.bank_count,
        3 * u64::from(TRACKS).div_ceil(lanes),
        "every track's three bankable stages are planned into banks"
    );

    let mut constrained = graph_caps();
    constrained.maximum_nodes = NODES - 1;
    let failure = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        dispatch: Backend::current(),
        plan_id: 1,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: constrained,
        gain_liveness: graph_compiler::GainLiveness::FIXED,
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
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(1.0);
        block.right.fill(2.0);
        Ok(())
    }
}

/// A builtin bank that leaves its lanes as they are.
struct IdentityBank;

impl GraphPreparedBuiltinBankProcessor for IdentityBank {
    fn retained_bytes(&self) -> u64 {
        0
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn process(
        &mut self,
        _left: &mut [f32],
        _right: &mut [f32],
        _frames: u32,
        _first_sample: u64,
    ) -> Result<(), RenderError> {
        Ok(())
    }
}

/// The hand-built plans' block length.
const HAND_BUILT_FRAMES: u32 = 16;

fn hand_built_envelope() -> RenderEnvelope {
    RenderEnvelope {
        sample_rate: engine::SampleRateHz(48_000),
        quantum: engine::QuantumFrames(HAND_BUILT_FRAMES),
        output_channels: core::num::NonZeroUsize::new(2).expect("stereo"),
    }
}

fn stable(text: String) -> StableGraphId {
    StableGraphId::parse(&text).expect("generated ID")
}

/// Every track's `stage` node, sorted as a dependency level must be.
fn track_stages(stage: TrackStage) -> Vec<GraphNodeId> {
    let mut nodes: Vec<_> = (0..TRACKS)
        .map(|index| GraphNodeId::TrackStage {
            track_id: stable(format!("track-{index}")),
            stage,
        })
        .collect();
    nodes.sort();
    nodes
}

fn edge(id: GraphEdgeId, source: &GraphNodeId, destination: &GraphNodeId) -> GraphEdge {
    let port = |node: &GraphNodeId, kind| GraphPortId {
        node: node.clone(),
        kind,
        effect_port: None,
    };
    GraphEdge {
        id,
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path: "$.scale".to_owned(),
    }
}

/// A plan through the public [`PreparedGraphPlan::new`], one dependency level per entry of
/// `levels` (each already sorted), with no effects, delays or observers.
fn hand_built_plan(
    levels: Vec<Vec<GraphNodeId>>,
    mut edges: Vec<GraphEdge>,
    required_bindings: Vec<GraphNodeId>,
    routes: Vec<PreparedRoute>,
) -> PreparedGraphPlan {
    edges.sort_by(|left, right| left.id.cmp(&right.id));
    let schedule: Vec<_> = levels.iter().flatten().cloned().collect();
    let mut nodes: Vec<_> = schedule
        .iter()
        .map(|node| GraphNode {
            id: node.clone(),
            latency: LatencySamples(0),
            bound: effect_contract::NodeTailBound::ZERO,
        })
        .collect();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    PreparedGraphPlan::new(PreparedGraphPlanParts {
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
            effect_bank_processor_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        },
        envelope: hand_built_envelope(),
        required_bindings,
        routes,
        track_delays: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        effect_observations: Vec::new(),
        banks: Vec::new(),
        builtin_banks: Vec::new(),
        observers: Vec::new(),
    })
}

/// Binds every track input to [`Constant`] and the output to identity, renders one block, and
/// asserts the output is the exact count of the tracks that reached it: every input writes 1.0
/// left and 2.0 right, and every partial sum is an integer below 2^24, so no summation order can
/// round it.
fn bind_and_render_every_track(plan: PreparedGraphPlan, inputs: Vec<GraphNodeId>) {
    let envelope = hand_built_envelope();
    let mut bindings: Vec<_> = inputs
        .into_iter()
        .map(|node| GraphNodeBinding::new(node, Box::new(Constant)))
        .collect();
    bindings.push(GraphNodeBinding::identity(GraphNodeId::Output {
        output_id: stable("main".to_owned()),
    }));
    assert_eq!(bindings.len(), TRACKS as usize + 1);
    let mut bound = plan
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("65,537-input bind: {}", failure.code));
    let frames = HAND_BUILT_FRAMES as usize;
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

/// Issue #1045: bind and render above 65,536 track inputs, per PR.
///
/// The session compile above refuses before anything binds, and the full compile and bind run
/// nightly, so this plan is built by hand: [`TRACKS`] track inputs, each routed through its own
/// unity route to one output. A ceiling in bind or render refuses, and a narrowed track or unit
/// index drops tracks from [`bind_and_render_every_track`]'s count. It runs beside the compile
/// above, in a few seconds of debug CPU.
#[test]
fn a_hand_built_65_537_input_plan_binds_and_renders_every_track() {
    let inputs = track_stages(TrackStage::Input);
    let output = GraphNodeId::Output {
        output_id: stable("main".to_owned()),
    };
    let mut routes = Vec::with_capacity(TRACKS as usize);
    let mut edges = Vec::with_capacity(2 * TRACKS as usize);
    for input in &inputs {
        let GraphNodeId::TrackStage { track_id, .. } = input else {
            unreachable!("track stages")
        };
        let route_id = stable(format!("route-{}", track_id.as_str()));
        let route = GraphNodeId::Route {
            route_id: route_id.clone(),
        };
        edges.push(edge(
            GraphEdgeId::RouteSource {
                route_id: route_id.clone(),
            },
            input,
            &route,
        ));
        edges.push(edge(
            GraphEdgeId::RouteDestination { route_id },
            &route,
            &output,
        ));
        routes.push(route);
    }
    routes.sort();
    let mut required = inputs.clone();
    required.push(output.clone());
    let transforms = routes
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
            gate: RouteGate::OPEN,
        })
        .collect();
    let plan = hand_built_plan(
        vec![inputs.clone(), routes, vec![output]],
        edges,
        required,
        transforms,
    );
    bind_and_render_every_track(plan, inputs);
}

/// [`TRACKS`] track inputs, each feeding its post-input builtin stage, attached as builtin banks
/// through the public [`PreparedGraphPlan::with_builtin_banks`] in lane order of the sorted tracks.
/// Every stage sums straight into the output, or, with `routes`, through a unity route of its own
/// (issue #967). Returns the plan and the track inputs to bind.
fn hand_built_bank_plan(routes: bool) -> (PreparedGraphPlan, Vec<GraphNodeId>) {
    let backend = Backend::current();
    let width = BankWidth::for_backend(backend).expect("native builds render in banks");
    let inputs = track_stages(TrackStage::Input);
    let members = track_stages(TrackStage::PostInputBuiltins);
    let output = GraphNodeId::Output {
        output_id: stable("main".to_owned()),
    };
    let mut edges = Vec::with_capacity(3 * TRACKS as usize);
    let mut route_nodes = Vec::new();
    for (input, member) in inputs.iter().zip(&members) {
        let GraphNodeId::TrackStage { track_id, .. } = member else {
            unreachable!("track stages")
        };
        edges.push(edge(
            GraphEdgeId::TrackMain {
                target: member.clone(),
            },
            input,
            member,
        ));
        let route_id = stable(format!("route-{}", track_id.as_str()));
        if routes {
            let route = GraphNodeId::Route {
                route_id: route_id.clone(),
            };
            edges.push(edge(
                GraphEdgeId::RouteSource {
                    route_id: route_id.clone(),
                },
                member,
                &route,
            ));
            edges.push(edge(
                GraphEdgeId::RouteDestination { route_id },
                &route,
                &output,
            ));
            route_nodes.push(route);
        } else {
            edges.push(edge(GraphEdgeId::RouteSource { route_id }, member, &output));
        }
    }
    route_nodes.sort();
    let mut required = inputs.clone();
    required.extend(members.iter().cloned());
    required.push(output.clone());
    let transforms = route_nodes
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
            gate: RouteGate::OPEN,
        })
        .collect();
    let banks: Vec<_> = members
        .chunks(width.lanes() as usize)
        .map(|lanes| GraphPreparedBuiltinBank {
            backend,
            members: lanes.to_vec().into_boxed_slice(),
            processor: Box::new(IdentityBank),
            scratch: rack::AoSoaScratch::new(width, HAND_BUILT_FRAMES).expect("bank scratch"),
        })
        .collect();
    let bank_count = banks.len() as u64;
    let mut levels = vec![inputs.clone(), members];
    if routes {
        levels.push(route_nodes);
    }
    levels.push(vec![output]);
    let plan = hand_built_plan(levels, edges, required, transforms)
        .with_builtin_banks(
            banks,
            GraphBuiltinBankResourceEstimate {
                bank_count,
                ..GraphBuiltinBankResourceEstimate::default()
            },
        )
        .unwrap_or_else(|error| panic!("65,537-track bank attachment: {error:?}"));
    assert_eq!(plan.builtin_bank_members().count(), TRACKS as usize);
    let expected_routes = if routes { TRACKS as usize } else { 0 };
    assert_eq!(plan.routes().len(), expected_routes);
    (plan, inputs)
}

/// Issue #1045: builtin bank attachment above 65,536 tracks, per PR, at the bank width this build
/// renders at.
///
/// [`hand_built_bank_plan`] without routes binds and renders through
/// [`bind_and_render_every_track`]. A ceiling in bank attachment refuses, and one in the runtime's
/// bank gather or scatter drops lanes from the count. The route-per-track shape binds in the
/// nightly [`compiles_and_binds_65_537_tracks_with_builtins`] instead, under a time bound.
#[test]
fn a_hand_built_65_537_track_plan_attaches_builtin_banks_binds_and_renders() {
    let (plan, inputs) = hand_built_bank_plan(false);
    bind_and_render_every_track(plan, inputs);
}

/// Issue #1045: builtin bank lowering above 65,536 tracks, per PR, at the bank width this build
/// renders at (Sol's attempt-4 check).
///
/// `compile_with_builtins` hands its graph to the public
/// [`builtins_compiler::PreparedBuiltinsSession::into_graph_artifact_with_banks`], which builds
/// every input, fader and matrix bank and attaches them. The constrained compile above refuses
/// before that, and the hand-built tests attach banks of their own, so this lowers the scale
/// session's prepared builtins onto a hand-built strip plan (input, post-input, fader, matrix,
/// output per track) and counts what it banked. A ceiling in lowering refuses or panics, and a
/// silent fallback that banks nothing, or fewer tracks, changes the counts. It runs beside the
/// compile above; bind and render at this size are the other hand-built tests'.
#[test]
fn lowers_65_537_tracks_of_builtin_banks() {
    let session = scale_session(PLAIN);
    let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("scale builtins");
    let classes = builtins_compiler::SessionPoolClasses::from_session(&session);
    let backend = Backend::current();
    let lanes = BankWidth::for_backend(backend)
        .expect("native builds render in banks")
        .lanes() as usize;
    let inputs = track_stages(TrackStage::Input);
    let strip: Vec<Vec<GraphNodeId>> = [
        TrackStage::PostInputBuiltins,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ]
    .into_iter()
    .map(track_stages)
    .collect();
    let output = GraphNodeId::Output {
        output_id: stable("main".to_owned()),
    };
    let mut edges = Vec::with_capacity(4 * TRACKS as usize);
    for (index, input) in inputs.iter().enumerate() {
        let mut previous = input;
        for level in &strip {
            edges.push(edge(
                GraphEdgeId::TrackMain {
                    target: level[index].clone(),
                },
                previous,
                &level[index],
            ));
            previous = &level[index];
        }
        let GraphNodeId::TrackStage { track_id, .. } = previous else {
            unreachable!("track stages")
        };
        edges.push(edge(
            GraphEdgeId::RouteSource {
                route_id: stable(format!("route-{}", track_id.as_str())),
            },
            previous,
            &output,
        ));
    }
    let mut levels = vec![inputs];
    levels.extend(strip);
    levels.push(vec![output]);
    let mut required: Vec<GraphNodeId> = levels.iter().flatten().cloned().collect();
    required.sort();
    let dependency_levels: Vec<DependencyLevel> = levels
        .iter()
        .cloned()
        .zip(0..)
        .map(|(nodes, level)| DependencyLevel { level, nodes })
        .collect();
    let plan = hand_built_plan(levels, edges, required, Vec::new());
    let artifact =
        builtins.into_graph_artifact_with_banks(plan, (), backend, &dependency_levels, &classes);
    assert_eq!(
        artifact.prepared_builtin_bank_count(),
        3 * (TRACKS as usize).div_ceil(lanes),
        "every track's input, fader and matrix stages are lowered into banks"
    );
    assert_eq!(
        artifact.graph().builtin_bank_members().count(),
        3 * TRACKS as usize,
        "every bankable stage of every track is a bank member"
    );
}

/// Issue #962: a session through the production entry, at the width this build renders at, then
/// bound and rendered for one block.
///
/// Until #964 the gate above compiled without builtins at `Backend::Scalar`, where no bank
/// attaches, and that is how a bank-membership scan quadratic in the track count stayed hidden on
/// every host's compile path (`PreparedGraphPlan::with_builtin_banks`, about 160 s in release
/// before the fix). This one attaches the builtin banks every host renders and then binds the
/// plan, because bind had quadratic scans of its own. A regression in any scan this session
/// reaches shows up as this test's time, which the bound below turns into a failure.
///
/// Issue #967: the session is the per-PR gates' strip with one route per track and the standing
/// console's `eq` as a console slot carrying a live control lane on every track. The routes put
/// every track's post-pan route into the bind's route-fold metadata, and the controls put every
/// track's lane through #962's `effect_control_resource`. The per-PR gates' effect-less, one-route
/// session reaches neither at scale. It does not exercise `scatter_redirects`' in-between check:
/// it binds in the same time with or without #967's linearisation (the spec's evidence). So the
/// test then binds [`hand_built_bank_plan`] with one route per track, the shape that reaches that
/// check, under a 5 s bound of its own, which the old quadratic walk exceeded. #962's other two
/// fixes (the `Backend::Scalar` interval scan and the fused-split crossing scan) guarded code that
/// #1767 and owner ruling R8 removed.
///
/// Issue #1045: this runs nightly, in release with overflow checks on (`nightly.yml`,
/// `release-budgets`), under a 60 s wall-clock bound on the whole path from the session compile to
/// the rendered block; #962's quadratic scans took longer than that. It is the only test that
/// compiles without a cap and binds at this size, so a track ceiling or a narrowed index in bank
/// attachment or bind is caught here, a day late at most.
#[test]
#[ignore = "release-mode 65,537-track budget; runs nightly"]
fn compiles_and_binds_65_537_tracks_with_builtins() {
    use std::time::{Duration, Instant};

    let started = Instant::now();
    let session = scale_session(Shape {
        routes: true,
        console: true,
    });
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("scale effects");
    let superseded = LiveValuesSuperseded::new();
    let producers =
        attach_effect_live_controls(&mut effects, &superseded, 0).expect("scale live controls");
    assert_eq!(producers.len(), TRACKS as usize, "one control per track");
    let builtins =
        prepare_session_builtins(&effects.session, &[], builtin_caps()).expect("scale builtins");
    let dispatch = Backend::current();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        dispatch,
        plan_id: 3,
        effects,
        builtins,
        caps: graph_caps(),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("with-builtins scale diagnostics: {:?}", failure.diagnostics));
    let estimate = &artifact.report().estimate;
    // Per track: the seven stage nodes, the `eq` slot and the route, in one chain of nine edges,
    // the last into the output.
    let nodes = 9 * u64::from(TRACKS) + 1;
    assert_eq!(estimate.logical_nodes, nodes);
    assert_eq!(estimate.edges, 9 * u64::from(TRACKS));
    assert_eq!(estimate.routes, u64::from(TRACKS));
    assert_eq!(estimate.effects, u64::from(TRACKS));
    assert_eq!(artifact.graph().sequential_schedule.len() as u64, nodes);
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
        "65,537-track session compile, effects, builtins, graph compile, bind and one block took \
         {elapsed:?}"
    );

    // Issue #967: the shape that reaches `scatter_redirects`' in-between check, which the
    // compiled session above does not. Every lane of a builtin bank has a route as its sole
    // reader, scheduled a track count of ops after the bank's first op, so the old per-lane
    // walk was quadratic: about 16.7 s for this bind and render in release before the fix and
    // about 1.1 s after (the spec's evidence).
    let (plan, inputs) = hand_built_bank_plan(true);
    let started = Instant::now();
    bind_and_render_every_track(plan, inputs);
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(5),
        "65,537-track builtin banks with one route per track took {elapsed:?} to bind and render"
    );
}
