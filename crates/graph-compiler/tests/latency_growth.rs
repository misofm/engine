//! #1287 (slice 1 of the latency-growth chain): a floor on a track's `Input` node is a
//! source-claim compensation line (D1).
//!
//! * Gate 1: a warm successor compiled with the lemma's lead floors puts every carried node at
//!   exactly its predecessor arrival plus `P`, keeps every carried edge's compensation, gives every
//!   carried track a `P`-sample claim line and gives a restarted or added track none.
//! * Gate 2: a plan floored at its own arrivals plus `P` renders the unfloored plan's output
//!   delayed by exactly `P`, bit for bit, after `P` samples of `+0.0`, and its tail extent and PDC
//!   total grow by the line.
//!
//! Gate 3 (no change without floors) is the existing suites and the graph fixture corpus.

use std::collections::{BTreeMap, BTreeSet};

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::{LatencySamples, TailSamples};
use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphPreparedSourceSet,
    GraphPreparedSourceSetDriver, GraphRuntimeBindings, GraphRuntimeProcessor,
    GraphSourceInputClaim, GraphSourceSetResourceReport, SourceClaimLine, StableGraphId,
    TrackStage, floored_input_arrival,
};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use session::{
    CompileCaps, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, ParameterChannel,
    ParameterUnit, Route, RouteDestination, RouteSource, SendTap, SessionModel,
    SidechainDeclaration, StableId, Submix, compile_session, parse_session_json,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
/// The fixture's quantum, at 48 kHz.
const QUANTUM: usize = 128;

type Floors = BTreeMap<GraphNodeId, LatencySamples>;

fn stable(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

fn stage(strip: &str, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(strip).expect("graph ID"),
        stage,
    }
}

fn limiter() -> Effect {
    let param = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    Effect {
        id: stable("limiter"),
        identity: EffectIdentity::Native {
            effect_id: stable("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: vec![
            param(1, ParameterUnit::Db, -0.5),
            param(2, ParameterUnit::Milliseconds, 60.0),
            param(3, ParameterUnit::Milliseconds, 5.0),
        ],
        sidechain: SidechainDeclaration::None,
    }
}

/// The canonical fixture's track, cleared of inserts and automation, as `tracks`, each routed
/// from its `post_pan` tap to `destination` (a submix ID, or the output).
fn session(tracks: &[(&str, Option<&str>)], submixes: &[&str]) -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("canonical session");
    model.automation.clear();
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    let output = model.outputs[0].id.clone();
    model.tracks = tracks
        .iter()
        .map(|(id, _)| {
            let mut track = template.clone();
            track.id = stable(id);
            track.inserts.effects.clear();
            track
        })
        .collect();
    model.submixes = submixes
        .iter()
        .map(|id| Submix::unity(stable(id), &model.console))
        .collect();
    let destination = |target: Option<&str>| match target {
        Some(submix) => RouteDestination::SubmixInput {
            submix_id: stable(submix),
        },
        None => RouteDestination::OutputInput {
            output_id: output.clone(),
        },
    };
    let mut routes: Vec<Route> = tracks
        .iter()
        .map(|(id, target)| Route {
            id: stable(&format!("{id}-out")),
            source: RouteSource::Track {
                track_id: stable(id),
                tap: SendTap::PostPan,
            },
            destination: destination(*target),
            ..route.clone()
        })
        .collect();
    routes.extend(submixes.iter().map(|id| Route {
        id: stable(&format!("{id}-out")),
        source: RouteSource::Submix {
            submix_id: stable(id),
            tap: SendTap::PostPan,
        },
        destination: destination(None),
        ..route.clone()
    }));
    model.routes = routes;
    model
}

fn add_limiter(model: &mut SessionModel, track: &str) {
    let track = model
        .tracks
        .iter_mut()
        .find(|candidate| candidate.id.as_str() == track)
        .expect("named track");
    track.inserts.effects.push(limiter());
}

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

fn compile(
    model: &SessionModel,
    floors: &Floors,
    dispatch: Backend,
) -> PreparedGraphBuiltinsArtifact {
    let session = compile_session(
        model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session compiles");
    let builtins = prepare_session_builtins(
        &session,
        &[],
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
        },
    )
    .expect("builtins");
    let effects = prepare_native_session_effects(
        &session,
        launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("native effects");
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: floors.clone(),
        dispatch,
        plan_id: 1287,
        effects,
        builtins,
        caps: graph_caps(),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("graph compile: {:?}", failure.diagnostics))
}

fn arrival(artifact: &PreparedGraphBuiltinsArtifact, node: &GraphNodeId) -> u64 {
    floored_input_arrival(&artifact.report().input_arrivals, node)
}

/// Every edge's compensation in `artifact`, by edge ID.
fn compensation(artifact: &PreparedGraphBuiltinsArtifact) -> BTreeMap<graph::GraphEdgeId, u64> {
    artifact
        .graph()
        .inserted_delays
        .iter()
        .map(|delay| (delay.edge_id.clone(), delay.samples.0))
        .collect()
}

/// Compile `successor` with the lemma's floors over predecessor `predecessor` and check gate 1.
///
/// `restarted` is the strip whose nodes before the fader, and fader, are R; nodes absent from the
/// predecessor are N; every other node is C. `carried` and `uncarried` are the tracks expected to
/// have, and not to have, a `P`-sample claim line.
fn check_lead(
    predecessor: &SessionModel,
    successor: &SessionModel,
    restarted: Option<&str>,
    carried: &[&str],
    uncarried: &[&str],
) {
    const R_STAGES: [TrackStage; 6] = [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostDynamic,
        TrackStage::PostSimd2PreFader,
        TrackStage::PostFader,
    ];
    let dispatch = Backend::current();
    let a = compile(predecessor, &Floors::new(), dispatch);
    let natural = compile(successor, &Floors::new(), dispatch);
    let previous: BTreeSet<_> = a.graph().spec.nodes.iter().map(|node| &node.id).collect();
    let r: BTreeSet<_> = restarted
        .into_iter()
        .flat_map(|strip| R_STAGES.map(|stage_of| stage(strip, stage_of)))
        .collect();
    let c: BTreeSet<GraphNodeId> = natural
        .graph()
        .spec
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .filter(|node| previous.contains(node) && !r.contains(node))
        .collect();
    let delta = c
        .iter()
        .map(|node| arrival(&natural, node).saturating_sub(arrival(&a, node)))
        .max()
        .expect("a carried node");
    assert!(delta > 0, "the edit must grow a carried node's arrival");
    let p = delta.div_ceil(QUANTUM as u64) * QUANTUM as u64;
    let mut floors: Floors = c
        .iter()
        .map(|node| (node.clone(), LatencySamples(arrival(&a, node) + p)))
        .collect();
    floors.extend(
        r.iter()
            .map(|node| (node.clone(), LatencySamples(arrival(&a, node)))),
    );
    let w = compile(successor, &floors, dispatch);

    for node in &c {
        assert_eq!(arrival(&w, node), arrival(&a, node) + p, "C node {node:?}");
    }
    let (before, after) = (compensation(&a), compensation(&w));
    for edge in &w.graph().spec.edges {
        if c.contains(&edge.source.node) && c.contains(&edge.destination.node) {
            assert_eq!(
                after.get(&edge.id).copied().unwrap_or(0),
                before.get(&edge.id).copied().unwrap_or(0),
                "C->C edge {:?}",
                edge.id
            );
        }
    }
    let expected: Vec<_> = carried
        .iter()
        .map(|track| SourceClaimLine {
            node: stage(track, TrackStage::Input),
            samples: LatencySamples(p),
        })
        .collect();
    assert_eq!(w.graph().source_claim_lines(), expected.as_slice());
    for track in carried {
        let node = stage(track, TrackStage::Input);
        let delay = w
            .graph()
            .track_delays()
            .iter()
            .find(|delay| delay.node == node)
            .expect("a carried track's line is lowered");
        assert_eq!(
            [delay.left_samples, delay.right_samples].map(u64::from),
            [p, p]
        );
    }
    for track in uncarried {
        let node = stage(track, TrackStage::Input);
        assert_eq!(arrival(&w, &node), 0, "{track}'s Input stays at rest");
        assert!(
            w.graph()
                .track_delays()
                .iter()
                .all(|delay| delay.node != node),
            "{track}'s Input has no line"
        );
    }
}

/// Gate 1, both cases of the spec at 48 kHz and a 128-frame quantum.
///
/// Case 1: a limiter added to track `one` of two tracks routed to the output; `one` restarts.
/// Case 2: a muted limiter track `three` added inside a bus whose arrival grows while the
/// output's does not (track `two` already carries a limiter to the output).
#[test]
fn a_lead_floored_successor_aligns_carried_nodes_and_lines_only_carried_sources() {
    let a = session(&[("one", None), ("two", None)], &[]);
    let mut w = a.clone();
    add_limiter(&mut w, "one");
    check_lead(&a, &w, Some("one"), &["two"], &["one"]);

    let mut a = session(&[("one", Some("bus")), ("two", None)], &["bus"]);
    add_limiter(&mut a, "two");
    let mut w = session(
        &[("one", Some("bus")), ("three", Some("bus")), ("two", None)],
        &["bus"],
    );
    add_limiter(&mut w, "two");
    add_limiter(&mut w, "three");
    let three = w
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == "three")
        .expect("track three");
    three.fader.left_mute = true;
    three.fader.right_mute = true;
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main-out").expect("graph ID"),
    };
    let dispatch = Backend::current();
    let (before, natural) = (
        compile(&a, &Floors::new(), dispatch),
        compile(&w, &Floors::new(), dispatch),
    );
    assert_eq!(
        arrival(&natural, &output),
        arrival(&before, &output),
        "the output's arrival must not grow"
    );
    check_lead(&a, &w, None, &["one", "two"], &["three"]);
}

/// Plays a frame-encoded signal: a word depends on the claim, the channel and the absolute frame.
struct Encoded {
    claims: usize,
    first_sample: u64,
}

impl Encoded {
    fn word(claim: usize, channel: usize, frame: u64) -> f32 {
        let value = (frame * 13 + claim as u64 * 101 + channel as u64 * 37) % 509;
        value as f32 / 512.0 - 0.49
    }
}

impl GraphPreparedSourceSetDriver for Encoded {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn claim_count(&self) -> usize {
        self.claims
    }
    fn begin_block(&mut self, first_sample: u64, _frames: u32) -> Result<(), RenderError> {
        self.first_sample = first_sample;
        Ok(())
    }
    fn copy_track_input(
        &mut self,
        claim: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        for (channel, plane) in [left, right].into_iter().enumerate() {
            for (frame, word) in plane.iter_mut().enumerate() {
                *word = Self::word(claim, channel, self.first_sample + frame as u64);
            }
        }
        Ok(())
    }
}

struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

/// `blocks` quanta of `artifact`'s output, bound to [`Encoded`], as `[left, right]` planes.
fn render(artifact: PreparedGraphBuiltinsArtifact, blocks: usize) -> [Vec<f32>; 2] {
    let envelope = artifact.envelope();
    let claims: Vec<_> = artifact
        .graph()
        .required_bindings
        .iter()
        .filter(|node| {
            matches!(
                node,
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                }
            )
        })
        .map(|node| GraphSourceInputClaim { node: node.clone() })
        .collect();
    let claimed: BTreeSet<_> = claims.iter().map(|claim| claim.node.clone()).collect();
    let nodes = artifact
        .external_binding_nodes()
        .filter(|node| !claimed.contains(*node))
        .cloned()
        .map(|node| GraphNodeBinding::new(node, Box::new(Identity)))
        .collect();
    let source_set = GraphPreparedSourceSet::new(
        envelope,
        claims.clone(),
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: 0,
            overhead_bytes: 0,
            total_engine_owned_bytes: 0,
            largest_allocation_bytes: 0,
        },
        Box::new(Encoded {
            claims: claims.len(),
            first_sample: 0,
        }),
    );
    let mut bound = artifact
        .into_bound_with_source_set(
            GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            },
            source_set,
        )
        .unwrap_or_else(|failure| panic!("graph bind: {:?}", failure.code));
    let plan: &mut PreparedRenderPlan = &mut bound.plan;
    let mut planes = [Vec::new(), Vec::new()];
    let mut output = [0.0_f32; QUANTUM * 2];
    for block in 0..blocks {
        plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
            },
            RenderTime {
                absolute_sample: (block * QUANTUM) as u64,
            },
        )
        .expect("render");
        planes[0].extend_from_slice(&output[..QUANTUM]);
        planes[1].extend_from_slice(&output[QUANTUM..]);
    }
    planes
}

/// Gate 2 at 48 kHz on every vector backend: floors at the plan's own arrivals plus `P = 2q` on
/// every node render the unfloored output delayed by exactly `P`, bit for bit, after `P` samples
/// of `+0.0`; the tail extents and the PDC total grow by the line.
#[test]
fn a_plan_floored_by_p_renders_its_output_delayed_by_exactly_p() {
    const P: usize = 2 * QUANTUM;
    const BLOCKS: usize = 8;
    let model = session(&[("one", None), ("two", None)], &[]);
    for &dispatch in Backend::VECTOR {
        let a = compile(&model, &Floors::new(), dispatch);
        let floors: Floors = a
            .graph()
            .spec
            .nodes
            .iter()
            .map(|node| {
                (
                    node.id.clone(),
                    LatencySamples(arrival(&a, &node.id) + P as u64),
                )
            })
            .collect();
        let w = compile(&model, &floors, dispatch);
        let grown = |tail: TailSamples| match tail {
            TailSamples::Finite(samples) => TailSamples::Finite(samples + P as u64),
            TailSamples::Infinite => panic!("{dispatch:?}: the predecessor's tail is finite"),
        };
        assert_eq!(w.report().output_tail, grown(a.report().output_tail));
        assert_eq!(
            w.report().output_tail_every_peak,
            grown(a.report().output_tail_every_peak)
        );
        assert_eq!(
            w.graph().estimate.total_delay_samples,
            a.graph().estimate.total_delay_samples + 2 * P as u64,
            "{dispatch:?}: one line per track counts in the PDC total"
        );
        let (before, after) = (render(a, BLOCKS), render(w, BLOCKS));
        for channel in 0..2 {
            assert!(
                before[channel].iter().any(|word| *word != 0.0),
                "{dispatch:?}: the predecessor must play"
            );
            let lead: Vec<u32> = after[channel][..P]
                .iter()
                .map(|word| word.to_bits())
                .collect();
            assert_eq!(
                lead,
                vec![0; P],
                "{dispatch:?} channel {channel}: +0.0 lead"
            );
            let shifted: Vec<u32> = after[channel][P..]
                .iter()
                .map(|word| word.to_bits())
                .collect();
            let expected: Vec<u32> = before[channel][..BLOCKS * QUANTUM - P]
                .iter()
                .map(|word| word.to_bits())
                .collect();
            assert_eq!(
                shifted, expected,
                "{dispatch:?} channel {channel}: delayed by P"
            );
        }
    }
}
