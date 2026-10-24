//! Issue #210 phase 2: what the compiler does, and pointedly does not do, with a track delay.
//!
//! Three of the phase's five evals live here, because all three are statements about the compiled
//! plan rather than about rendered audio:
//!
//! * **P2-2, the off gate.** A session that declares `delay_samples = 0` -- which is every session
//!   in the tree, and will be almost every session a host ever writes -- must compile to the
//!   current zero-delay semantic plan: the structural row below proves that track delay itself
//!   adds no node or ring.
//! * **P2-3, PDC non-interaction.** A delayed track's *output* shifts. Its *PDC report* does not.
//! * **P2-4, accounting.** The rings are charged to `delay_bytes`, exactly `sum * 4` per lane, and
//!   a hostile session that asks for more than the caps allow is rejected rather than allocated.
//!
//! Eval P2-1 (shift exactness) and the mono-collapse interaction are rendered facts and live in
//! `host-core`'s `track_delay.rs`.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use graph::{GraphCompileCaps, PreparedGraphPlan};
use graph_compiler::{
    Backend, GraphBuiltinsCompileRequest, GraphCompiler, PreparedGraphBuiltinsArtifact,
};
use session::{CompileCaps, CompiledSession, SessionModel, compile_session, parse_session_json};

/// Nine tracks, a real parametric EQ on each, routes into the session output, and a non-zero
/// compiled `output_latency` -- so the PDC rows this file pins are rows that actually carry
/// something. A fixture whose PDC report were empty in both arms would make P2-3 vacuous.
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

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

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

/// The fixture with the one track's two lanes delayed by `left` and `right` samples.
fn model_with_delay(left: u32, right: u32) -> SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture parses");
    // One delayed track among nine. The other eight are the control: whatever PDC does to them
    // must be what it does to them when track zero is undelayed.
    model.tracks[0].builtins.left.delay_samples = left;
    model.tracks[0].builtins.right.delay_samples = right;
    model
}

/// The same nine tracks, with a **latency-carrying** limiter added to track zero's inserts and to
/// nothing else.
///
/// This is what makes P2-3 mean something. The fixture as checked in compiles to
/// `output_latency = 0` with no inserted delays at all -- every path through it is the same length
/// -- so asserting "the compensation sets did not move" on it would be asserting that an empty set
/// stayed empty. One latent effect on one of nine tracks routed to a common output is the smallest
/// session in which PDC has real work to do, and the delayed track is the one carrying it.
fn latent_model_with_delay(left: u32, right: u32) -> SessionModel {
    use session::{
        Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, ParameterChannel,
        ParameterUnit, SidechainDeclaration, StableId,
    };
    let mut model = model_with_delay(left, right);
    model.tracks[0].inserts.effects.push(Effect {
        id: StableId::parse("limiter").expect("effect id"),
        identity: EffectIdentity::Native {
            effect_id: StableId::parse("miso.true-peak-limiter").expect("native id"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: vec![
            EffectParam {
                parameter_id: 1,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Db,
                value: -0.5,
            },
            EffectParam {
                parameter_id: 2,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Milliseconds,
                value: 60.0,
            },
            EffectParam {
                parameter_id: 3,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Milliseconds,
                value: 5.0,
            },
        ],
        sidechain: SidechainDeclaration::None,
    });
    model
}

fn latent(left: u32, right: u32) -> Compiled {
    compile(
        compile_session(&latent_model_with_delay(left, right), compile_caps())
            .expect("session compiles"),
        graph_caps(),
    )
    .expect("latent session compiles")
}

fn session_with_delay(left: u32, right: u32) -> CompiledSession {
    compile_session(&model_with_delay(left, right), compile_caps()).expect("session compiles")
}

struct Compiled {
    artifact: PreparedGraphBuiltinsArtifact,
    output_latency: u64,
    delay_bytes: u64,
    largest_allocation_bytes: u64,
}

impl Compiled {
    fn plan(&self) -> &PreparedGraphPlan {
        self.artifact.graph()
    }
}

/// The production compile: the session's own builtins, prepared with no meters and no live
/// controls, attached through `compile_with_builtins` at the width this build renders at (#964).
fn compile(session: CompiledSession, caps: GraphCompileCaps) -> Result<Compiled, String> {
    let registry = launch_native_effect_registry().expect("launch registry");
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
    .expect("the fixture's builtins prepare");
    let effects = prepare_native_session_effects(
        &session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the fixture's native effects prepare");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        dispatch: Backend::current(),
        plan_id: 1,
        effects,
        builtins,
        caps,
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .map_err(|failure| {
        failure
            .diagnostics
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.code.to_string())
            .collect::<Vec<_>>()
            .join(",")
    })?;
    // The whole-plan estimate, with the builtin banks attached: the one the caps are checked
    // against, and the one a host publishes.
    let estimate = artifact.graph_resource_estimate().clone();
    Ok(Compiled {
        output_latency: artifact.report().output_latency.0,
        delay_bytes: estimate.delay_bytes,
        largest_allocation_bytes: estimate.largest_allocation_bytes,
        artifact,
    })
}

fn compiled(left: u32, right: u32) -> Compiled {
    compile(session_with_delay(left, right), graph_caps()).expect("delayed session compiles")
}

// ---------------------------------------------------------------------------------------------
// P2-2: the off gate
// ---------------------------------------------------------------------------------------------

/// A zero-delay session lowers no delay node at all.
///
/// This is the *structural* half of the off gate and the one that matters most: `node_kind` reaches
/// its `TrackDelay` arm only for a node present in this list, so an empty list is a proof about the
/// lowered program rather than a claim about it.
///
/// Red mutation: emit a `PreparedTrackDelay` unconditionally in `compile.rs` (drop the
/// `filter`) -> this fails.
#[test]
fn a_zero_delay_session_lowers_no_delay_node() {
    let zero = compiled(0, 0);
    assert!(
        zero.plan().track_delays().is_empty(),
        "a session declaring no delay must carry no delay entry"
    );
    assert_eq!(zero.delay_bytes, 0, "no ring, no bytes");
}

// ---------------------------------------------------------------------------------------------
// P2-3: PDC non-interaction
// ---------------------------------------------------------------------------------------------

/// Track delay is not latency, so nothing PDC owns moves when a track declares one.
///
/// The four things PDC owns are asserted individually, so a failure says *which* of them moved.
/// `inserted_delays` and `route_timings` are the compensation sets the design names;
/// `output_latency` is what a host reports to its own clock; and the per-node latencies are the
/// input PDC computes all three from.
///
/// Red mutation: add `delay_samples` into the `GraphNode.latency` of the input node -> the
/// latencies row fails first, then `output_latency`, then the compensation sets on any fixture with
/// a second path.
#[test]
fn a_track_delay_moves_no_pdc_row() {
    let zero = latent(0, 0);
    // The precondition: PDC has real work on this session, so the equalities below are equalities
    // between non-empty sets.
    assert!(
        zero.output_latency > 0,
        "the latent fixture must declare latency"
    );
    assert!(
        !zero.plan().inserted_delays.is_empty(),
        "the latent fixture must make PDC insert at least one compensation delay"
    );
    for (left, right) in [(1_u32, 1_u32), (128, 128), (4_800, 4_800), (48_000, 0)] {
        let delayed = latent(left, right);
        assert_eq!(
            delayed.plan().inserted_delays,
            zero.plan().inserted_delays,
            "PDC inserted no delay for {left}/{right}"
        );
        assert_eq!(
            delayed.plan().route_timings,
            zero.plan().route_timings,
            "PDC retimed no route for {left}/{right}"
        );
        assert_eq!(
            delayed.output_latency, zero.output_latency,
            "the plan's declared output latency moved for {left}/{right}"
        );
        let latencies: Vec<_> = delayed
            .plan()
            .spec
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        let baseline: Vec<_> = zero
            .plan()
            .spec
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        assert_eq!(
            latencies, baseline,
            "a track delay declared node latency for {left}/{right}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// P2-4: accounting
// ---------------------------------------------------------------------------------------------

/// The samples stay out of PDC's counts even though the bytes join PDC's row.
#[test]
fn the_pdc_counts_are_untouched() {
    let zero = compiled(0, 0);
    let delayed = compiled(48_000, 48_000);
    let (a, b) = (&zero.plan().estimate, &delayed.plan().estimate);
    assert_eq!(a.total_delay_samples, b.total_delay_samples);
    assert_eq!(a.materialized_nodes, b.materialized_nodes);
    assert_eq!(a.edges, b.edges);
    assert_eq!(a.schedule_items, b.schedule_items);
    assert_eq!(a.dependency_levels, b.dependency_levels);
    // ...while the bytes did move, so the row above is not vacuous.
    assert_ne!(a.delay_bytes, b.delay_bytes);
}

/// One ring is one named allocation, so it participates in `largest_allocation_bytes`.
#[test]
fn a_ring_is_a_named_allocation() {
    let zero = compiled(0, 0);
    let delayed = compiled(48_000, 1);
    assert!(
        delayed.largest_allocation_bytes >= 48_000 * 4,
        "the largest single allocation must cover the largest ring"
    );
    assert!(delayed.largest_allocation_bytes > zero.largest_allocation_bytes);
}

/// A hostile session is rejected at the cap rather than allocated.
///
/// The schema's own `0..=48000` is the first line -- `validate.rs` refuses anything above it -- so
/// this covers the second: a session inside the schema domain whose *aggregate* rings exceed the
/// caps the host set.
#[test]
fn an_oversized_delay_is_rejected_by_the_caps() {
    let mut caps = graph_caps();
    // Room for the plan's audio and metadata, but not for two 48,000-sample rings on top.
    caps.maximum_plan_bytes = compiled(0, 0).plan().estimate.incremental_plan_bytes + 1_024;
    assert!(
        compile(session_with_delay(0, 0), caps).is_ok(),
        "the cap must admit the undelayed session, or the rejection below proves nothing"
    );
    let Err(rejection) = compile(session_with_delay(48_000, 48_000), caps) else {
        panic!("a session whose rings exceed the plan cap must be refused");
    };
    assert!(
        rejection.contains("graph.resource.limit"),
        "expected a resource-limit refusal, got {rejection}"
    );
}
