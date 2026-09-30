//! Issue #1087 (console strip P1), gates 1 and 2 through the real compiler and the real graph.
//!
//! A session's `bypass` used to become the effect's prepared bypass, which is part of its
//! `EffectProgramKey`, so a bypassed track left its cohort. It now lowers to a prepared
//! `bypass = false` plus a bypassed lane on the rack's latency-preserving shunt, so a cohort binds
//! one bank per slot whatever mix of its tracks is bypassed.
//!
//! "Today's render" is reconstructed here exactly: every lowered entry is prepared again with the
//! session's bypass as its prepared flag and without a lane, which is what
//! `prepare_native_session_effects` produced before #1087. The planner then splits the bypassed
//! tracks out again and renders them as it always did. Both plans must render the same bits, at
//! every track's rack boundaries and at the session output, and the lowered plan must also render
//! them at `Backend::Scalar`, where every lane takes the graph's per-node shunt.
//!
//! The session is the standing console strip -- `eq -> comp` in SIMD rack 1 and the true-peak
//! limiter (`rate/100 + 6` samples of latency, `link_mode: maximum`) in SIMD rack 2 -- on one
//! cohort of the host's bank width, so on x86-64 this is the `Simd8` leg and under
//! `scripts/run-aarch64-tests.sh` the `Simd4` one.

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::{
    EffectCompileCaps, EffectPreparedSession, PREPARED_BYPASS_EFFECTS,
    launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::{BankWidth, EffectControlLane};
use engine::realtime::{PlanarBufferMut, RenderError, RenderIo, RenderTime};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    RackId, StableGraphId, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, EffectIdentity, SessionModel, StableId, compile_session, parse_session_json,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

const INTENDED: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track-intended.json");

/// Past the limiter's four-block latency line, with room for its release.
const BLOCKS: u64 = 12;

/// The strip's three slots, as `(rack, effect id)`.
const SLOTS: [(RackId, &str); 3] = [
    (RackId::Simd1, "eq"),
    (RackId::Simd1, "comp"),
    (RackId::Simd2, "limiter"),
];

fn lanes() -> usize {
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

/// The first `tracks` tracks of the standing console, each routed to the main output, with slot
/// `s` of track `t` bypassed when bit `t` of `masks[s]` is set.
fn console(tracks: usize, masks: [u64; 3]) -> SessionModel {
    let mut model = parse_session_json(INTENDED).expect("console fixture");
    model.tracks.truncate(tracks);
    model.routes.truncate(tracks);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        for (slot, (rack, id)) in SLOTS.iter().enumerate() {
            let effects = match rack {
                RackId::Simd1 => &mut track.simd1.effects,
                RackId::Dynamic => &mut track.dynamic.effects,
                RackId::Simd2 => &mut track.simd2.effects,
            };
            let effect = effects
                .iter_mut()
                .find(|effect| effect.id.as_str() == *id)
                .expect("strip slot");
            effect.bypass = masks[slot] >> index & 1 == 1;
        }
    }
    model
}

/// Rebuilds `prepare_native_session_effects`' pre-#1087 output: every lowered entry prepared again
/// with its session bypass as the prepared flag, and without the lane that carried it.
fn todays_lowering(effects: &mut EffectPreparedSession) {
    for entry in &mut effects.entries {
        if !entry.initial_bypass || entry.metadata.bypass {
            continue;
        }
        let lane = entry.control.take().expect("a lowered bypass rides a lane");
        assert!(!lane.has_channel() && lane.bypassed());
        entry.bank_preparation.bypass = true;
        entry.processor = entry
            .factory
            .prepare(entry.bank_preparation.request())
            .expect("the same request, prepared bypassed");
        entry.metadata = entry.processor.metadata();
    }
}

/// What a sample source feeds a track.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Feed {
    /// A per-track impulse, a slow sine and a cosine, around `-20 dBFS` to `0 dBFS`.
    Music,
    /// The same, with NaN, both infinities and samples at and beyond `1e30` planted in it.
    NonFinite,
    /// The same, with runs of the smallest negative subnormal, `-2^-149`, in place of every other
    /// 32 samples: a stage that scales by less than a half turns each one into `-0.0`.
    NegativeZero,
    /// A constant-magnitude `6e29` on both channels, its sign flipping every eight samples: legal
    /// (below the input stage's `1e30` bound), and past the level at which the multiband's
    /// block-boundary check trips at its defaults (#1087 M2). A constant `6e29` does not trip it:
    /// its crossover passes DC without the overshoot a step makes.
    Hot,
}

struct TrackSource {
    gain: f32,
    feed: Feed,
}
impl GraphRuntimeProcessor for TrackSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let sample = block.first_sample + index as u64;
            let t = sample as f32;
            let impulse = if sample.is_multiple_of(1_000) {
                1.0
            } else {
                0.0
            };
            *left = self.gain * (math::sinf(t * 0.013) + impulse);
            *right = self.gain * math::cosf(t * 0.021);
            if self.feed == Feed::NegativeZero && (sample / 32) % 2 == 1 {
                *left = -f32::from_bits(1);
                *right = -f32::from_bits(1);
            }
            if self.feed == Feed::Hot {
                let hot = if (sample / 8).is_multiple_of(2) {
                    6.0e29
                } else {
                    -6.0e29
                };
                *left = hot;
                *right = hot;
            }
            if self.feed == Feed::NonFinite {
                match sample % 97 {
                    3 => *left = f32::NAN,
                    11 => *right = f32::INFINITY,
                    19 => *left = f32::NEG_INFINITY,
                    29 => *right = 1.0e30,
                    41 => *left = -3.0e38,
                    _ => {}
                }
            }
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

/// Records every observed word, per node.
struct Capture(Arc<Mutex<Vec<u32>>>);
impl GraphRuntimeObserver for Capture {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        let mut sink = self.0.lock().expect("capture");
        sink.extend(block.left.iter().map(|sample| sample.to_bits()));
        sink.extend(block.right.iter().map(|sample| sample.to_bits()));
        Ok(())
    }
}

/// One compiled and rendered plan.
struct Rendered {
    /// Bound effect banks, each as its members' `(track, effect)` pairs.
    banks: Vec<Vec<(String, String)>>,
    /// Each track's words after SIMD rack 1 (`eq -> comp`) and after SIMD rack 2 (the limiter),
    /// per `(track, stage)`. Observers attach to track stages only.
    nodes: BTreeMap<(String, String), Vec<u32>>,
    /// The session output's words.
    output: Vec<u32>,
}

/// Compile `model` at `dispatch`, optionally with today's lowering, and render it.
fn render(model: &SessionModel, dispatch: Backend, today: bool, feed: Feed) -> Rendered {
    let lowering = if today {
        Lowering::Today
    } else {
        Lowering::Session
    };
    render_with(model, dispatch, lowering, &|_| feed)
}

/// Which preparation a render uses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Lowering {
    /// `prepare_native_session_effects` as it is.
    Session,
    /// [`todays_lowering`]: every session bypass a prepared bypass, as before #1087.
    Today,
    /// [`p1_lowering`]: every session bypass lowered to a shunt, as #1087 left it.
    P1,
}

/// Rebuilds #1087's lowering for the effects that keep a prepared bypass since #1100
/// (`effect_compiler::PREPARED_BYPASS_EFFECTS`): prepared enabled, with a bypassed channel-less
/// lane, so a bypassed instance shares its enabled neighbours' bank again.
fn p1_lowering(effects: &mut EffectPreparedSession) {
    for entry in &mut effects.entries {
        let id = entry.factory.descriptor().id.as_str();
        if !entry.initial_bypass || !PREPARED_BYPASS_EFFECTS.contains(&id) {
            continue;
        }
        assert!(entry.metadata.bypass && entry.control.is_none());
        entry.bank_preparation.bypass = false;
        entry.processor = entry
            .factory
            .prepare(entry.bank_preparation.request())
            .expect("the same request, prepared enabled");
        entry.metadata = entry.processor.metadata();
        entry.control = Some(Box::new(EffectControlLane::without_channel(true)));
    }
}

/// Compile `model` at `dispatch` with `lowering`, and render it with `feed(track)` on each track.
fn render_with(
    model: &SessionModel,
    dispatch: Backend,
    lowering: Lowering,
    feed: &dyn Fn(u32) -> Feed,
) -> Rendered {
    let session = compile_session(model, compile_caps()).expect("compiled session");
    let registry = launch_native_effect_registry().expect("launch registry");
    let mut effects = prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    )
    .expect("prepared effects");
    match lowering {
        Lowering::Session => {}
        Lowering::Today => todays_lowering(&mut effects),
        Lowering::P1 => p1_lowering(&mut effects),
    }
    let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id: 1087,
        effects,
        builtins,
        caps: graph_caps(),
        dispatch,
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    let banks = artifact
        .graph()
        .effect_bank_members()
        .map(|members| {
            members
                .iter()
                .map(|member| {
                    (
                        member.track_id.as_str().to_owned(),
                        member.effect_id.as_str().to_owned(),
                    )
                })
                .collect()
        })
        .collect();

    let envelope = artifact.envelope();
    let frames = envelope.quantum.0 as usize;
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
                        feed: feed(index),
                    })
                }
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    let mut sinks = BTreeMap::new();
    let mut observers = Vec::new();
    for track in &model.tracks {
        for stage in [TrackStage::PostSimd1, TrackStage::PostSimd2PreFader] {
            let sink = Arc::new(Mutex::new(Vec::new()));
            observers.push(GraphNodeObserverBinding::new(
                GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track.id.as_str()).expect("track id"),
                    stage,
                },
                observers.len() as u64 + 1,
                Box::new(Capture(Arc::clone(&sink))),
            ));
            sinks.insert((track.id.as_str().to_owned(), format!("{stage:?}")), sink);
        }
    }
    let mut plan = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers,
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
        .plan;
    let mut output = Vec::new();
    let mut pcm = vec![0.0_f32; frames * 2];
    for block in 0..BLOCKS {
        pcm.fill(0.0);
        plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output"),
            },
            RenderTime {
                absolute_sample: block * frames as u64,
            },
        )
        .expect("render");
        output.extend(pcm.iter().map(|sample| sample.to_bits()));
    }
    drop(plan);
    let nodes = sinks
        .into_iter()
        .map(|(key, sink)| {
            let words = Arc::try_unwrap(sink)
                .expect("the plan is gone")
                .into_inner()
                .expect("capture");
            (key, words)
        })
        .collect();
    Rendered {
        banks,
        nodes,
        output,
    }
}

/// Word equality with every NaN folded to one value (decision 10).
fn folded(words: &[u32]) -> Vec<u32> {
    words
        .iter()
        .map(|word| {
            if f32::from_bits(*word).is_nan() {
                0x7fc0_0000
            } else {
                *word
            }
        })
        .collect()
}

fn assert_same_bits(what: &str, actual: &Rendered, expected: &Rendered) {
    assert_eq!(
        actual.nodes.keys().collect::<Vec<_>>(),
        expected.nodes.keys().collect::<Vec<_>>()
    );
    for (key, words) in &actual.nodes {
        let expected_words = &expected.nodes[key];
        assert_eq!(words.len(), expected_words.len(), "{what} {key:?}: length");
        if let Some(index) = folded(words)
            .iter()
            .zip(folded(expected_words))
            .position(|(a, e)| *a != e)
        {
            panic!(
                "{what} {key:?}: word {index} is {:#010x}, today's render has {:#010x}",
                words[index], expected_words[index]
            );
        }
    }
    assert_eq!(
        folded(&actual.output),
        folded(&expected.output),
        "{what}: the session output"
    );
}

/// Masks for one cohort of `lanes` tracks: none, all, each edge lane alone, alternating, and a
/// run of deterministic random ones. The three slots take different masks from one draw.
fn masks(lanes: usize, random: usize) -> Vec<[u64; 3]> {
    let full = (1_u64 << lanes) - 1;
    let mut masks = vec![
        [0, 0, 0],
        [full, full, full],
        [1, 1 << (lanes - 1), 1],
        [0x5555 & full, 0xaaaa & full, 0x3333 & full],
        [full, 0, full ^ 1],
    ];
    let mut state = 0x1087_u64;
    for _ in 0..random {
        let mut draw = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state & full
        };
        masks.push([draw(), draw(), draw()]);
    }
    masks
}

/// Gate 1: one cohort of the host's width, with any mix of bypassed and enabled lanes on each of
/// its three slots, binds one bank per slot holding every track.
///
/// Before #1087 a bypassed track's program key differed from its neighbours', so every mask but
/// "none" and "all" split the cohort and left each slot's group short of full, and nothing bound.
///
/// Red mutation: prepare with `bypass: effect.bypass` in `prepare_native_session_effects` again ->
/// every mixed mask binds no bank.
#[test]
fn a_mixed_bypass_cohort_binds_one_bank_per_slot() {
    let lanes = lanes();
    for masks in masks(lanes, 24) {
        let session = compile_session(&console(lanes, masks), compile_caps()).expect("session");
        let registry = launch_native_effect_registry().expect("launch registry");
        let effects = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 30,
                maximum_scratch_bytes: 1 << 28,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("prepared effects");
        let builtins = prepare_session_builtins(&session, &[], builtin_caps()).expect("builtins");
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            plan_id: 1087,
            effects,
            builtins,
            caps: graph_caps(),
            dispatch: Backend::current(),
        })
        .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
        let banks: Vec<Vec<String>> = artifact
            .graph()
            .effect_bank_members()
            .map(|members| {
                members
                    .iter()
                    .map(|member| {
                        format!("{}/{}", member.track_id.as_str(), member.effect_id.as_str())
                    })
                    .collect()
            })
            .collect();
        assert_eq!(
            banks.len(),
            SLOTS.len(),
            "masks {masks:x?}: one bank per slot"
        );
        for (bank, (_, effect)) in banks.iter().zip(SLOTS) {
            let expected: Vec<String> = (0..lanes)
                .map(|track| format!("ch{track:02}/{effect}"))
                .collect();
            assert_eq!(
                bank, &expected,
                "masks {masks:x?}: the {effect} bank holds every track"
            );
        }
        let report = &artifact.report().rack_cohorts;
        assert!(
            report.plan.groups.iter().all(|group| group.is_full()),
            "masks {masks:x?}: one full group"
        );
    }
}

/// Gate 2 through the graph: a session with bypassed tracks renders today's bits, per slot and at
/// the output, at the host's width and at `Backend::Scalar`.
///
/// Today's plan splits the bypassed tracks out of the cohort and runs them with the effect's own
/// prepared bypass; the lowered plan keeps one bank per slot and restores each bypassed lane's
/// latency-matched dry signal through the shunt, and at `Backend::Scalar` runs every lane per node
/// through the graph's per-node shunt.
///
/// Red mutation: the shunt built only for a live channel (`has_channel()` without
/// `|| bypassed()`) -> every bypassed lane of a bank renders wet. (A source never carries `-0.0`
/// past the input section's filters, so the `fma(0, wet, dry)` restore is `bypass_shunt_identity`'s
/// to catch.)
#[test]
fn a_session_bypass_renders_todays_bits() {
    let lanes = lanes();
    for masks in masks(lanes, 3) {
        let model = console(lanes, masks);
        let lowered = render(&model, Backend::current(), false, Feed::Music);
        let today = render(&model, Backend::current(), true, Feed::Music);
        let scalar = render(&model, Backend::Scalar, false, Feed::Music);
        assert_eq!(
            lowered.banks.len(),
            SLOTS.len(),
            "masks {masks:x?}: every slot banked"
        );
        assert!(scalar.banks.is_empty(), "the scalar oracle binds nothing");
        assert_same_bits(&format!("masks {masks:x?} lowered"), &lowered, &today);
        assert_same_bits(&format!("masks {masks:x?} scalar"), &scalar, &today);
        assert!(
            lowered
                .output
                .iter()
                .any(|word| f32::from_bits(*word) != 0.0),
            "masks {masks:x?}: the render is audible"
        );
    }
}

/// No source sample outside the range an effect accepts reaches one: NaN, both infinities and
/// samples at or beyond `1e30` are sanitised to `+0.0` by each track's input stage, so a lowered
/// plan with bypassed lanes renders today's bits on them too.
///
/// This is what bounds `bypass_shunt_identity`'s input domain. A shunt copies a non-finite dry
/// block where an effect's prepared bypass would zero it, so the day an effect can receive one,
/// this test is where it shows.
///
/// Red mutation: the input stage's bound raised to infinity (`lane::kernels::builtins::
/// NONFINITE_LIMIT`) -> the planted `1e30` and `-3e38` reach lane 0's bypassed compressor, whose
/// prepared bypass zeroes the block its shunt copies.
#[test]
fn non_finite_sources_render_todays_bits() {
    let lanes = lanes();
    let full = (1_u64 << lanes) - 1;
    // Lane 0 bypasses all three slots and lane 2 the first two, so a planted sample would pass
    // the EQ's dry path into a bypassed compressor, whose prepared bypass zeroes what a shunt
    // copies; lane 3 bypasses nothing.
    let model = console(lanes, [0x5555 & full, 0x7777 & full, 0x3333 & full]);
    let lowered = render(&model, Backend::current(), false, Feed::NonFinite);
    let today = render(&model, Backend::current(), true, Feed::NonFinite);
    assert_same_bits("non-finite sources", &lowered, &today);
}

/// Every track's input section at identity: no trim, no filters, no polarity inversion, so a
/// source sample reaches the first effect unchanged.
fn identity_inputs(model: &mut SessionModel) {
    for track in &mut model.tracks {
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.polarity_invert = false;
            lane.trim_db = 0.0;
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
    }
}

/// Issue #1100 (P1's verdict, L3): `-0.0` from an enabled upstream stage reaches a bypassed slot,
/// and the slot hands it on unchanged, banked and per node.
///
/// A source never carries `-0.0` past the input section, but an effect can make one: here SIMD
/// rack 1 runs `comp -> eq` with the compressor enabled on every track at -12 dB makeup, so each
/// of the source's `-2^-149` samples leaves it as `-0.0` and reaches the EQ slot, which is bypassed
/// on alternate tracks. The EQ's wet path is still ringing from the music before each run, so an
/// arithmetic restore that computes `0 * wet + dry` turns every `-0.0` whose wet sample is not
/// negative into `+0.0`; the shunt copies it. Today's prepared bypass, the lowered bank and the
/// scalar per-node shunt must render the same words, and the bypassed tracks must actually carry
/// `-0.0` out of the slot.
///
/// Red mutations: the rack's restore as `fma(0, wet, dry)` -> the lowered leg; `BypassShunt::apply`
/// as the same blend -> the scalar leg.
#[test]
fn a_negative_zero_from_an_enabled_stage_passes_a_bypassed_slot_unchanged() {
    let lanes = lanes();
    let full = (1_u64 << lanes) - 1;
    let eq_mask = 0x5555 & full;
    let mut model = console(lanes, [eq_mask, 0, 0]);
    identity_inputs(&mut model);
    for track in &mut model.tracks {
        track.simd1.effects.swap(0, 1);
        let comp = &mut track.simd1.effects[0];
        assert_eq!(comp.id.as_str(), "comp", "SIMD rack 1 is comp -> eq");
        comp.params
            .iter_mut()
            .find(|param| param.parameter_id == 6)
            .expect("the compressor's makeup")
            .value = -12.0;
    }
    let lowered = render(&model, Backend::current(), false, Feed::NegativeZero);
    let today = render(&model, Backend::current(), true, Feed::NegativeZero);
    let scalar = render(&model, Backend::Scalar, false, Feed::NegativeZero);
    assert_eq!(
        lowered.banks.len(),
        SLOTS.len(),
        "every slot banked, the EQ with its bypassed lanes"
    );
    assert!(scalar.banks.is_empty(), "the scalar oracle binds nothing");
    assert_same_bits("-0.0 lowered", &lowered, &today);
    assert_same_bits("-0.0 scalar", &scalar, &today);
    let negative_zeros: usize = (0..lanes)
        .filter(|track| eq_mask >> track & 1 == 1)
        .map(|track| {
            lowered.nodes[&(format!("ch{track:02}"), "PostSimd1".to_owned())]
                .iter()
                .filter(|word| **word == (-0.0_f32).to_bits())
                .count()
        })
        .sum();
    assert!(
        negative_zeros > 0,
        "no -0.0 left a bypassed EQ slot, so the case proves nothing"
    );
}

/// `tracks` tracks of the multiband compressor alone, at its defaults, in SIMD rack 1, with
/// identity input sections and track `t`'s instance bypassed when bit `t` of `mask` is set.
fn multiband_console(tracks: usize, mask: u64) -> SessionModel {
    let mut model = console(tracks, [0, 0, 0]);
    identity_inputs(&mut model);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.simd1.effects.truncate(1);
        track.simd2.effects.clear();
        let effect = &mut track.simd1.effects[0];
        effect.id = StableId::parse("multiband").expect("effect id");
        effect.identity = EffectIdentity::Native {
            effect_id: StableId::parse("miso.multiband-compressor").expect("multiband id"),
        };
        effect.params.clear();
        effect.bypass = mask >> index & 1 == 1;
    }
    model
}

/// Issue #1100, gate 3: a session bypass on the multiband stays a prepared bypass, so a mixed
/// bypass cohort declines a bank exactly as it did before #1087, and renders the bits #1087's
/// lowering renders at music levels.
///
/// The multiband's bank recovery (D7) is still whole-bank, so it is not lowered until the slice
/// that makes it console-eligible gives it per-lane recovery (`PREPARED_BYPASS_EFFECTS`). A
/// bypassed instance is prepared `bypass = true` with no lane, and its program key differs from an
/// enabled one's: no bound bank mixes the two, and at eight lanes, where neither half of this
/// cohort fills a bank, nothing binds. #1087's lowering, rebuilt here, banks the whole cohort; at
/// music levels, where no recovery fires, both render the same words, and so does `Scalar`.
///
/// Red mutations: drop the multiband from `PREPARED_BYPASS_EFFECTS` -> the bypassed instances are
/// prepared enabled and the cohort banks mixed.
#[test]
fn a_mixed_bypass_multiband_cohort_keeps_its_prepared_bypass() {
    const TRACKS: usize = 8;
    const MASK: u64 = 0b0110_1001;
    let model = multiband_console(TRACKS, MASK);
    let session = compile_session(&model, compile_caps()).expect("session");
    let registry = launch_native_effect_registry().expect("launch registry");
    let effects = prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    )
    .expect("prepared effects");
    for entry in &effects.entries {
        let index: u32 = entry.track_id[2..].parse().expect("chNN");
        let bypassed = MASK >> index & 1 == 1;
        assert_eq!(entry.initial_bypass, bypassed);
        assert_eq!(
            entry.metadata.bypass, bypassed,
            "{}: the session bypass is the prepared one",
            entry.track_id
        );
        assert!(
            entry.control.is_none(),
            "{}: no lane carries a multiband bypass",
            entry.track_id
        );
    }
    let kept = render_with(&model, Backend::current(), Lowering::Session, &|_| {
        Feed::Music
    });
    for bank in &kept.banks {
        let bypassed = bank
            .iter()
            .filter(|(track, _)| MASK >> track[2..].parse::<u32>().expect("chNN") & 1 == 1)
            .count();
        assert!(
            bypassed == 0 || bypassed == bank.len(),
            "a bound multiband bank mixes bypassed and enabled lanes: {bank:?}"
        );
    }
    if lanes() == TRACKS {
        assert!(
            kept.banks.is_empty(),
            "neither half of the cohort fills an eight-lane bank"
        );
    }
    let lowered = render_with(&model, Backend::current(), Lowering::P1, &|_| Feed::Music);
    assert_eq!(
        lowered.banks.len(),
        TRACKS / lanes(),
        "#1087's lowering banks the whole mixed cohort"
    );
    let scalar = render_with(&model, Backend::Scalar, Lowering::Session, &|_| Feed::Music);
    assert_same_bits("multiband against #1087's lowering", &kept, &lowered);
    assert_same_bits("multiband at Scalar", &scalar, &kept);
    assert!(
        kept.output.iter().any(|word| f32::from_bits(*word) != 0.0),
        "the render is audible"
    );
}

/// Issue #1100, gate 3 (#1087 verdict, M2): one bypassed multiband lane fed `6e29` leaves its
/// enabled neighbours' bits unchanged.
///
/// `6e29` is a legal sample: the input stage passes anything finite below `1e30`. At the
/// multiband's defaults it trips the block-boundary check (D7) of any bank it runs in, and that
/// check zeroes the whole bank. The positive control is #1087's lowering, rebuilt here: there the
/// hot bypassed lane shares a bank with enabled ones and silences them. With the prepared bypass
/// kept, the bypassed instance never shares a bank with an enabled one, and every enabled track
/// renders the same words whatever the bypassed track is fed.
///
/// Red mutation: drop the multiband from `PREPARED_BYPASS_EFFECTS` -> the enabled neighbours of
/// the hot lane are zeroed.
#[test]
fn a_hot_bypassed_multiband_lane_leaves_its_neighbours_bits_unchanged() {
    const TRACKS: usize = 8;
    const HOT: u32 = 3;
    let model = multiband_console(TRACKS, 1 << HOT);
    let neighbours = |rendered: &Rendered| -> BTreeMap<(String, String), Vec<u32>> {
        rendered
            .nodes
            .iter()
            .filter(|((track, _), _)| track != &format!("ch{HOT:02}"))
            .map(|(key, words)| (key.clone(), words.clone()))
            .collect()
    };
    for lowering in [Lowering::Session, Lowering::P1] {
        let quiet = render_with(&model, Backend::current(), lowering, &|_| Feed::Music);
        let hot = render_with(&model, Backend::current(), lowering, &|track| {
            if track == HOT { Feed::Hot } else { Feed::Music }
        });
        let moved = neighbours(&quiet)
            .iter()
            .zip(neighbours(&hot).iter())
            .filter(|((key, quiet), (_, hot))| {
                assert_eq!(quiet.len(), hot.len(), "{key:?}: length");
                folded(quiet) != folded(hot)
            })
            .count();
        match lowering {
            Lowering::Session => assert_eq!(
                moved, 0,
                "a hot bypassed multiband lane moved {moved} of its enabled neighbours' taps"
            ),
            _ => assert!(
                moved > 0,
                "the positive control: under #1087's lowering the hot lane must reach its \
                 bank-mates, or this proves nothing"
            ),
        }
    }
}
