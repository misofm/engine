//! Issue #1100 (console strip P1b): a session bypass is bounded, charged and allocation-free.
//!
//! P1 (#1087) lowered a session `bypass` to a prepared `bypass = false` plus a channel-less lane on
//! a latency-preserving shunt, so every bypassed instance now has a live-control owner: a per-node
//! `graph` `LiveControlEffect`, or a `rack::LiveControlEffectBankStage` for its bank slot. This
//! file gates the three things that owner must not cost:
//!
//! * **Gate 1, bounded.** A channel-less lane holds no staging window, so a live-control-free
//!   bypass binds at any automation capacity, per node and banked.
//! * **Gate 2, charged.** The graph estimate charges every byte a bypass or live controls make
//!   bind retain -- windows, shunts and the owners that hold them -- measured here by the
//!   workspace's audited allocator, not taken from the accounting under test.
//! * **Gate 4, allocation-free.** A plan with mixed session bypass, banked and per node, renders
//!   without a single allocator call after warm-up, at `Simd8`, `Simd4` and `Scalar`.
//!
//! The session is the standing console strip (`eq -> comp` in SIMD rack 1, the true-peak limiter in
//! SIMD rack 2) from the intended 64-track fixture, cut to the tracks each case needs.

use core::num::NonZeroUsize;

use bench_support::alloc::{
    self as bench_alloc, Mode, assert_installed, current_thread_counters,
    current_thread_delta_since,
};
use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, attach_effect_live_controls, launch_native_effect_registry,
    prepare_native_session_effects,
};
use effect_contract::BankWidth;
use engine::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, RackId, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{CompileCaps, SessionModel, compile_session, parse_session_json};

const INTENDED: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");

/// The strip's three slots, as `(rack, effect id)`.
const SLOTS: [(RackId, &str); 3] = [
    (RackId::Simd1, "eq"),
    (RackId::Simd1, "comp"),
    (RackId::Simd2, "limiter"),
];

fn host_lanes() -> usize {
    BankWidth::for_backend(Backend::current())
        .expect("the delivery host has a bank width; the evidence is vacuous otherwise")
        .lanes() as usize
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

fn effect_caps(spans: u32) -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 30,
        maximum_scratch_bytes: 1 << 28,
        maximum_automation_spans_per_block: spans,
    }
}

/// The first `tracks` tracks of the standing console, each routed to the main output, with slot
/// `s` of track `t` bypassed when bit `t` of `masks[s]` is set.
fn console(tracks: usize, masks: [u64; 3]) -> SessionModel {
    let mut model = parse_session_json(INTENDED).expect("console fixture");
    model.tracks.truncate(tracks);
    model.routes.truncate(tracks);
    // Every strip slot is a console slot (decision 12): `pre_insert` lowers to SIMD rack 1 and
    // `post_insert` to SIMD rack 2, and a track's bypass for a slot is its console entry's.
    for (index, track) in model.tracks.iter_mut().enumerate() {
        for (slot, (_, id)) in SLOTS.iter().enumerate() {
            let entry = track
                .console
                .iter_mut()
                .find(|entry| entry.slot.as_str() == *id)
                .expect("strip slot");
            entry.bypass = masks[slot] >> index & 1 == 1;
        }
    }
    model
}

/// A per-track tone: a slow sine and a cosine around `-20 dBFS` to `0 dBFS`. Allocation-free.
struct TrackSource {
    gain: f32,
}
impl GraphRuntimeProcessor for TrackSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let t = (block.first_sample + index as u64) as f32;
            *left = self.gain * math::sinf(t * 0.013);
            *right = self.gain * math::cosf(t * 0.021);
        }
        Ok(())
    }
}

struct Identity;
impl GraphRuntimeProcessor for Identity {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

/// Where each effect of one compiled plan renders.
#[derive(Debug, Default)]
struct Shape {
    /// Bound effect banks.
    banks: usize,
    /// Bound effect banks that hold at least one session-bypassed lane.
    banks_with_a_bypassed_lane: usize,
    /// Session-bypassed instances that render per node.
    bypassed_per_node: usize,
}

/// One prepared, compiled and bound plan.
struct Bound {
    plan: PreparedRenderPlan,
    /// The whole-plan graph estimate the plan was admitted against.
    estimate: u64,
    /// Bytes the preparation left live: everything the bound plan retains, every transient the
    /// preparation freed already subtracted (`bench_support::alloc::Counters::released_bytes`).
    live: u64,
    shape: Shape,
}

/// Prepare, compile and bind `model` at `dispatch`, with live controls on every effect when
/// `live_controls` is set, measuring what the preparation leaves live.
///
/// The measured window starts at effect preparation, so it frees nothing it did not allocate, and
/// ends with the bound plan and nothing else alive: the live controls' producers, which a host
/// keeps beside the plan and charges separately, are dropped inside it.
fn bind(model: &SessionModel, dispatch: Backend, spans: u32, live_controls: bool) -> Bound {
    let session = compile_session(model, compile_caps()).expect("compiled session");
    let registry = launch_native_effect_registry().expect("launch registry");
    let bypassed: Vec<(String, String)> = model
        .tracks
        .iter()
        .flat_map(|track| {
            // A lowered console effect's id is its slot.
            let console = track
                .console
                .iter()
                .filter(|entry| entry.bypass)
                .map(|entry| entry.slot.as_str());
            let inserts = track
                .inserts
                .effects
                .iter()
                .filter(|effect| effect.bypass)
                .map(|effect| effect.id.as_str());
            console
                .chain(inserts)
                .map(|effect| (track.id.as_str().to_owned(), effect.to_owned()))
        })
        .collect();
    assert_installed();
    let mark = current_thread_counters();
    let mut effects =
        prepare_native_session_effects(&session, &registry, effect_caps(spans)).expect("effects");
    let producers = live_controls.then(|| {
        attach_effect_live_controls(&mut effects, NonZeroUsize::new(8).expect("depth"))
            .expect("live controls")
    });
    let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 1100,
        effects,
        builtins,
        caps: graph_caps(),
        dispatch,
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    let estimate = artifact.graph_resource_estimate().incremental_plan_bytes;
    let is_bypassed = |track: &str, effect: &str| {
        bypassed
            .iter()
            .any(|(t, e)| t.as_str() == track && e.as_str() == effect)
    };
    let mut shape = Shape::default();
    let mut banked_bypasses = 0;
    for members in artifact.graph().effect_bank_members() {
        shape.banks += 1;
        let bypassed_lanes = members
            .iter()
            .filter(|member| is_bypassed(member.track_id.as_str(), member.effect_id.as_str()))
            .count();
        banked_bypasses += bypassed_lanes;
        if bypassed_lanes != 0 {
            shape.banks_with_a_bypassed_lane += 1;
        }
    }
    shape.bypassed_per_node = bypassed.len() - banked_bypasses;
    let envelope = artifact.envelope();
    let nodes = artifact
        .external_binding_nodes()
        .map(|node| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    track_id,
                    stage: TrackStage::Input,
                } => {
                    let index: u32 = track_id.as_str()[2..].parse().expect("chNN");
                    Box::new(TrackSource {
                        gain: 0.1 + index as f32 * 0.07,
                    })
                }
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    let plan = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
        .plan;
    drop(producers);
    let window = current_thread_delta_since(mark);
    let live = window
        .requested_bytes
        .checked_sub(window.released_bytes)
        .expect("the window frees nothing it did not allocate");
    Bound {
        plan,
        estimate,
        live,
        shape,
    }
}

fn render_blocks(plan: &mut PreparedRenderPlan, pcm: &mut [f32], first: u64, blocks: u64) {
    let frames = pcm.len() / 2;
    for block in first..first + blocks {
        plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(pcm, 2, frames, frames).expect("output"),
            },
            RenderTime {
                absolute_sample: block * frames as u64,
            },
        )
        .expect("render");
    }
}

/// Gate 1: a live-control-free session bypass binds and renders at `u32::MAX` automation spans, per
/// node and banked.
///
/// A channel-less lane stages nothing, so neither owner holds a window: before #1100 the per-node
/// `LiveControlEffect` sized one from the automation capacity whatever the lane, and bind aborted
/// on `memory allocation of 171798691800 bytes failed` (`u32::MAX` spans of 40 bytes). Its estimate
/// charges no window either, so it stays far below the 172 GB the window would have cost.
///
/// A host with that much memory does not abort, so the test also asserts, from the audited
/// allocator, that the bound plan retains less than 1 GiB.
///
/// Red mutations: size `LiveControlEffect::new`'s window by the automation capacity whatever the
/// lane (the per-node leg); size `LiveControlEffectBankStage::new`'s windows likewise (the banked
/// leg, `baa03f09`'s twin). Each aborts bind or retains 172 GB.
#[test]
fn a_live_control_free_bypass_binds_at_any_automation_capacity() {
    let lanes = host_lanes();
    for (label, tracks, eq_mask) in [
        ("one per-node bypassed EQ", 1, 1_u64),
        ("one bypassed lane of an EQ bank", lanes, 1 << (lanes - 1)),
    ] {
        let mut bound = bind(
            &console(tracks, [eq_mask, 0, 0]),
            Backend::current(),
            u32::MAX,
            false,
        );
        if tracks == 1 {
            assert_eq!(bound.shape.banks, 0, "{label}: one track banks nothing");
            assert_eq!(bound.shape.bypassed_per_node, 1, "{label}: per node");
        } else {
            assert_eq!(
                bound.shape.banks_with_a_bypassed_lane, 1,
                "{label}: the bypassed lane stays in its bank"
            );
        }
        assert!(
            bound.live < 1 << 30,
            "{label}: the plan retains {} bytes, so a channel-less lane holds a window",
            bound.live
        );
        assert!(
            bound.estimate < 1 << 30,
            "{label}: the estimate charges no window for a channel-less lane ({} bytes)",
            bound.estimate
        );
        let mut pcm = vec![0.0_f32; 2 * 128];
        render_blocks(&mut bound.plan, &mut pcm, 0, 4);
        assert!(
            pcm.iter().any(|sample| *sample != 0.0),
            "{label}: the plan renders"
        );
    }
}

/// Gate 2: the graph estimate charges at least the bytes a session bypass, or live controls,
/// make the bound plan retain.
///
/// Each case is a pair of plans that differ only in the bypass or the live controls, and compares
/// the two differences: the estimate's (`incremental_plan_bytes`) against the bytes the audited
/// allocator saw the preparation leave live. The allocator is the independent witness, so an
/// owner the estimate forgets, or one that outgrows its charge, is red here without a byte literal.
///
/// * One per-node bypassed EQ: its `LiveControlEffect` box, its lane and its shunt's dry blocks. P1
///   retained 6,186 bytes here against a 72-byte charge (#1087 verdict, M1).
/// * One bypassed lane of a limiter bank: the live-control stage's growth over the plain stage and
///   the shunt over the whole AoSoA block, including the limiter's 486-sample line on every lane.
/// * Live controls, per node and banked: the same owners plus their staging windows of 128 spans.
///
/// Red mutations: drop the shunt from `effect_control_resource` (every case); drop the window
/// (the live cases); drop the per-node owner's box (the per-node cases).
#[test]
fn a_bypass_or_live_controls_are_charged_at_least_what_they_retain() {
    const SPANS: u32 = 128;
    let lanes = host_lanes();
    let cases: [(&str, usize, [u64; 3], bool); 4] = [
        ("one per-node bypassed EQ", 1, [1, 0, 0], false),
        (
            "one bypassed lane of a limiter bank",
            lanes,
            [0, 0, 1],
            false,
        ),
        ("per-node live controls", 1, [0, 0, 0], true),
        ("banked live controls", lanes, [0, 0, 0], true),
    ];
    for (label, tracks, masks, live_controls_attached) in cases {
        let plain = bind(
            &console(tracks, [0, 0, 0]),
            Backend::current(),
            SPANS,
            false,
        );
        let owned = bind(
            &console(tracks, masks),
            Backend::current(),
            SPANS,
            live_controls_attached,
        );
        if tracks == 1 {
            assert_eq!(owned.shape.banks, 0, "{label}: one track banks nothing");
        } else {
            assert_eq!(owned.shape.banks, 3, "{label}: one bank per slot");
            assert_eq!(plain.shape.banks, 3, "{label}: one bank per slot");
        }
        if !live_controls_attached {
            let bypassed = owned.shape.bypassed_per_node + owned.shape.banks_with_a_bypassed_lane;
            assert_eq!(bypassed, 1, "{label}: exactly one bypassed owner");
        }
        let retained = owned
            .live
            .checked_sub(plain.live)
            .unwrap_or_else(|| panic!("{label}: the owner retains nothing"));
        let charged = owned
            .estimate
            .checked_sub(plain.estimate)
            .unwrap_or_else(|| panic!("{label}: the owner lowers the estimate"));
        println!("{label}: retained {retained} B, charged {charged} B");
        assert!(
            retained > 0,
            "{label}: the owner must retain something, or this proves nothing"
        );
        assert!(
            charged >= retained,
            "{label}: the estimate charges {charged} bytes for an owner that retains {retained}"
        );
    }
}

/// Restores the audited allocator's process-wide mode when a test leaves, even by panic.
struct RestoreMode(Mode);
impl Drop for RestoreMode {
    fn drop(&mut self) {
        bench_alloc::set_mode(self.0);
    }
}

/// Gate 4: mixed session bypass renders without a single allocator call after warm-up, banked
/// and per node, at `Simd8`, `Simd4` and `Scalar`.
///
/// Thirteen tracks: at eight lanes one bank per slot plus a five-track per-node remainder, at four
/// lanes the limiter's three banks plus a per-node remainder (and every EQ and compressor per node
/// on hosts that bind no four-lane bank for them), and at `Scalar` every instance per node. Each
/// slot bypasses a different mix of tracks, so every leg that binds a bank has bypassed lanes in
/// it, and every leg has bypassed instances per node. The counters are this thread's, so another test's
/// allocations cannot reach them; the audited allocator counts in `Count` mode, so an allocation
/// inside the render scope is reported here rather than aborting the process.
///
/// Red mutation: `black_box(Vec::<u8>::with_capacity(1))` in `BypassShunt::capture` -> every leg
/// counts allocations.
#[test]
fn a_mixed_session_bypass_renders_without_allocating() {
    const TRACKS: usize = 13;
    const WARM_UP: u64 = 16;
    const BLOCKS: u64 = 256;
    assert_installed();
    let _restore = RestoreMode(bench_alloc::mode());
    bench_alloc::set_mode(Mode::Count);
    let model = console(
        TRACKS,
        [0b1_0010_1001_0110, 0b0_1101_0010_0101, 0b1_0100_1010_1001],
    );
    for dispatch in [Backend::Simd8, Backend::Simd4, Backend::Scalar] {
        let mut bound = bind(&model, dispatch, 32, false);
        let shape = &bound.shape;
        assert!(
            shape.bypassed_per_node > 0,
            "{dispatch:?}: a bypassed instance renders per node ({shape:?})"
        );
        // A build binds no bank wider than its own backend (decision D4), so on a four-lane host
        // the `Simd8` leg is all per node, like `Scalar`.
        match BankWidth::for_backend(dispatch) {
            Some(width) if width.lanes() as usize <= host_lanes() => assert!(
                shape.banks_with_a_bypassed_lane > 0,
                "{dispatch:?}: a bypassed lane renders in a bank ({shape:?})"
            ),
            _ => assert_eq!(shape.banks, 0, "{dispatch:?}: nothing binds"),
        }
        let mut pcm = vec![0.0_f32; 2 * 128];
        render_blocks(&mut bound.plan, &mut pcm, 0, WARM_UP);
        let mark = current_thread_counters();
        render_blocks(&mut bound.plan, &mut pcm, WARM_UP, BLOCKS);
        let counted = current_thread_delta_since(mark);
        assert_eq!(
            (counted.allocations, counted.deallocations),
            (0, 0),
            "{dispatch:?}: mixed session bypass must render without allocating or freeing"
        );
        assert!(
            pcm.iter().any(|sample| *sample != 0.0),
            "{dispatch:?}: the plan renders"
        );
    }
}
