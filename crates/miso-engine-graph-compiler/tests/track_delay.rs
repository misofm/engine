//! Issue #210 phase 2: what the compiler does, and pointedly does not do, with a track delay.
//!
//! Three of the phase's five evals live here, because all three are statements about the compiled
//! plan rather than about rendered audio:
//!
//! * **P2-2, the off gate.** A session that declares `delay_samples = 0` -- which is every session
//!   in the tree, and will be almost every session a host ever writes -- must compile to the plan
//!   it compiled to before this feature existed. Not an equivalent plan: the same plan, with the
//!   same canonical text and therefore the same SHA-256.
//! * **P2-3, PDC non-interaction.** A delayed track's *output* shifts. Its *PDC report* does not.
//! * **P2-4, accounting.** The rings are charged to `delay_bytes`, exactly `sum * 4` per lane, and
//!   a hostile session that asks for more than the caps allow is rejected rather than allocated.
//!
//! Eval P2-1 (shift exactness) and the mono-collapse interaction are rendered facts and live in
//! `miso-engine-host-core`'s `track_delay.rs`.

use miso_engine_effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use miso_engine_graph::{GraphCompileCaps, PreparedGraphPlan};
use miso_engine_graph_compiler::{Backend, GraphCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, CompiledSession, SessionToml, compile_session, parse_session_toml,
};

/// Nine tracks, a real parametric EQ on each, routes into the session output, and a non-zero
/// compiled `output_latency` -- so the PDC rows this file pins are rows that actually carry
/// something. A fixture whose PDC report were empty in both arms would make P2-3 vacuous.
const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

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
fn model_with_delay(left: u32, right: u32) -> SessionToml {
    let mut model = parse_session_toml(SESSION).expect("fixture parses");
    // One delayed track among nine. The other eight are the control: whatever PDC does to them
    // must be what it does to them when track zero is undelayed.
    model.tracks[0].builtins.left.delay_samples = left;
    model.tracks[0].builtins.right.delay_samples = right;
    model
}

/// The same nine tracks, with a **latency-carrying** limiter added to track zero's `simd2` rack and
/// to nothing else.
///
/// This is what makes P2-3 mean something. The fixture as checked in compiles to
/// `output_latency = 0` with no inserted delays at all -- every path through it is the same length
/// -- so asserting "the compensation sets did not move" on it would be asserting that an empty set
/// stayed empty. One latent effect on one of nine tracks routed to a common output is the smallest
/// session in which PDC has real work to do, and the delayed track is the one carrying it.
fn latent_model_with_delay(left: u32, right: u32) -> SessionToml {
    use miso_engine_session::{
        Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, ParameterChannel,
        ParameterUnit, SidechainDeclaration, StableId,
    };
    let mut model = model_with_delay(left, right);
    model.tracks[0].simd2.effects.push(Effect {
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
    plan: PreparedGraphPlan,
    sha256: String,
    output_latency: u64,
    delay_bytes: u64,
    largest_allocation_bytes: u64,
}

fn compile(session: CompiledSession, caps: GraphCompileCaps) -> Result<Compiled, String> {
    let registry = launch_native_effect_registry().expect("launch registry");
    let effects = prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 24,
            maximum_scratch_bytes: 1 << 24,
            maximum_automation_spans_per_block: 128,
        },
    )
    .expect("the fixture's native effects prepare");
    let artifact = GraphCompiler::compile(GraphCompileRequest {
        dispatch: Backend::Scalar,
        plan_id: 1,
        effects,
        caps,
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
    let sha256 = GraphCompiler::sha256(&artifact.graph, &artifact.report);
    let estimate = artifact.graph.estimate.clone();
    Ok(Compiled {
        output_latency: artifact.report.output_latency.0,
        sha256,
        delay_bytes: estimate.delay_bytes,
        largest_allocation_bytes: estimate.largest_allocation_bytes,
        plan: artifact.graph,
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
/// `filter`) -> this fails, and so does every digest row below.
#[test]
fn a_zero_delay_session_lowers_no_delay_node() {
    let zero = compiled(0, 0);
    assert!(
        zero.plan.track_delays().is_empty(),
        "a session declaring no delay must carry no delay entry"
    );
    assert_eq!(zero.delay_bytes, 0, "no ring, no bytes");
}

/// The zero-delay plan is byte-for-byte the plan this fixture compiled to before the feature.
///
/// The constant is not a snapshot of this tree: it was measured by running this exact digest on
/// `origin/main` at 17682b4, against that tree's `canonical.toml` -- the one without the
/// `delay_samples` key. That the two agree is the whole class-A claim, and it holds for a
/// non-obvious reason worth stating: the session's *text* grew, but the graph's canonical text is
/// derived from the compiled plan and its estimate, and the estimate's session term is
/// `requested_runtime_bytes` (a function of counts and limits) rather than the canonical byte
/// count. So a schema key that no plan reads moves no plan byte.
///
/// Red mutation: none of this row's own. Measured, not assumed: emitting a *zero-length* entry for
/// every track (dropping `compile.rs`'s `filter`) leaves this digest **unchanged**, because a
/// zero-sample ring adds nothing to `delay_bytes` and the canonical text never names the entry. The
/// digest catches a delay that reaches the estimate; only `a_zero_delay_session_lowers_no_delay_node`
/// catches one that reaches the *program*. That is why both rows exist, and neither is redundant.
#[test]
fn the_zero_delay_plan_digest_is_the_pre_feature_digest() {
    assert_eq!(
        compiled(0, 0).sha256,
        ZERO_DELAY_CANONICAL_SHA256,
        "a zero-delay session must compile to the ruled canonical plan"
    );
}

/// Originally measured on `origin/main` (17682b4), whose `canonical.toml` has no `delay_samples`
/// key at all. Re-pinned by issue #241 because source content identity is now part of the compiled
/// graph's semantic text; the zero-delay structure and all render-bearing identities remain fixed.
const ZERO_DELAY_CANONICAL_SHA256: &str =
    "213617ba7e5774e831785e725f8cb70bdd0f043cba9ae071e139888935acf4b0";

/// ...and a delayed one is a genuinely different plan, so the digest above is not inert.
#[test]
fn a_delayed_session_is_a_different_plan() {
    assert_ne!(
        compiled(480, 480).sha256,
        ZERO_DELAY_CANONICAL_SHA256,
        "a delay that changes the estimate must change the plan digest"
    );
}

// ---------------------------------------------------------------------------------------------
// P2-3: PDC non-interaction
// ---------------------------------------------------------------------------------------------

/// Track delay is not latency, so nothing PDC owns moves when a track declares one.
///
/// The four things PDC owns are asserted individually rather than through the digest, because the
/// digest is a conjunction: it would go red for the right reason and give no evidence about *which*
/// of them moved. `inserted_delays` and `route_timings` are the compensation sets the design names;
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
        !zero.plan.inserted_delays.is_empty(),
        "the latent fixture must make PDC insert at least one compensation delay"
    );
    for (left, right) in [(1_u32, 1_u32), (128, 128), (4_800, 4_800), (48_000, 0)] {
        let delayed = latent(left, right);
        assert_eq!(
            delayed.plan.inserted_delays, zero.plan.inserted_delays,
            "PDC inserted no delay for {left}/{right}"
        );
        assert_eq!(
            delayed.plan.route_timings, zero.plan.route_timings,
            "PDC retimed no route for {left}/{right}"
        );
        assert_eq!(
            delayed.output_latency, zero.output_latency,
            "the plan's declared output latency moved for {left}/{right}"
        );
        let latencies: Vec<_> = delayed
            .plan
            .spec
            .nodes
            .iter()
            .map(|node| (node.id.clone(), node.latency))
            .collect();
        let baseline: Vec<_> = zero
            .plan
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

/// `delay_bytes` grows by exactly four bytes per declared sample per lane, and by nothing else.
///
/// Four and not eight: PDC's `* 8` is one `CompensationDelay` of `n` samples holding *two* rings of
/// `n`, because PDC's skew is per edge and therefore per pair. A track delay is declared per lane,
/// so the two rings are sized independently and each lane is charged for its own.
///
/// Red mutations, both measured: folding the term into `timing.total_delay` before the `* 8`
/// instead of adding it beside trips this row (the per-lane sum stops being recoverable from the
/// per-pair one); declaring `delay_samples` as the input node's `GraphNode.latency` trips this row,
/// `a_track_delay_moves_no_pdc_row` and `the_pdc_counts_are_untouched` together, which is the
/// three-way signature of the one mistake this feature most needs to be unable to make.
#[test]
fn the_estimate_charges_each_lane_its_own_ring() {
    let base = compiled(0, 0).delay_bytes;
    for (left, right) in [
        (1_u32, 0_u32),
        (0, 1),
        (480, 480),
        (48_000, 12),
        (7, 48_000),
    ] {
        let expected = base + u64::from(left) * 4 + u64::from(right) * 4;
        assert_eq!(
            compiled(left, right).delay_bytes,
            expected,
            "delay_bytes for {left}/{right}"
        );
    }
}

/// The samples stay out of PDC's counts even though the bytes join PDC's row.
#[test]
fn the_pdc_counts_are_untouched() {
    let zero = compiled(0, 0);
    let delayed = compiled(48_000, 48_000);
    let (a, b) = (&zero.plan.estimate, &delayed.plan.estimate);
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
    caps.maximum_plan_bytes = compiled(0, 0).plan.estimate.incremental_plan_bytes + 1_024;
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
