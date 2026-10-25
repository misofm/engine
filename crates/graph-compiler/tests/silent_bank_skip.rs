//! Issue #1107: a bank chain whose every lane has exact `+0.0` input and is at rest skips the
//! block whole, and a skipped block renders exactly the bits a processed one does.
//!
//! The gates, on compiled plans at the widest width this build runs (`Backend::VECTOR`'s last:
//! eight lanes where `avx2` is enabled, four on the arm64 legs):
//!
//! * gate 1, the class A differential: one session compiled twice, one plan rendering with the
//!   skip allowed and its twin with it forbidden (`graph::test_only_disable_bank_skip`), block by
//!   block over the same inputs. Every block's host output and every track's post-matrix words
//!   must agree by `to_bits`, NaNs folded;
//! * gate 2: after warm-up, a skipping block allocates and frees nothing on the render thread.
//!
//! The session is the 64-track mono desk cut to [`TRACKS`] tracks, three banks at eight lanes:
//!
//! * `ch00..ch07`: mono sources, so the bank collapses once armed; the first bank of the master's
//!   route fold, whose lane 0 stores. It skips, collapsed.
//! * `ch08..ch15`: stereo sources, with a stored fader ride on `ch11` and the desk's own
//!   compressor settings. A lane with stored automation never reports rest (#1781), so this bank
//!   never skips: it plays while the others skip.
//! * `ch16..ch19`: stereo sources, a padded bank of four lanes. It skips dual.
//!
//! The console is the desk's EQ, compressor and true-peak limiter. Left out, because in this
//! session they stop every bank from skipping rather than test the skip: the gate and the
//! transient shaper at their defaults do not reach exact rest within a test's horizon (the gate
//! takes about 50,000 samples, the transient shaper did not within 200,000); a soft clip does not
//! collapse, in the console, and as an insert it declines the route fold. A compressor lane that
//! has reduced gain did not report rest again within 300 blocks, so the skipping banks run the
//! compressor with a 0 dB threshold, which this material never reaches.
//!
//! The route matrices make a silent fold's master carry both signed zeros: every left row is
//! negative, and gate 1 renders two layouts of negative right rows (see its doc; gate 2 uses the
//! first four tracks). Each track's input
//! follows its own schedule of signal runs (levels down to `1e-30`), `-0.0` blocks and exact
//! `+0.0` runs, with windows where the first bank, every bank, or the padded bank is silent long
//! enough to reach rest.
//!
//! Live writes (a mid-ramp fader or trim edit, a bypass edge) are not attached here: this level
//! compiles without live controls. The #1781 rest tests in the stage crates cover them.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins_compiler::{
    BuiltinCompileCaps, prepare_session_builtins, session_structural_symmetry,
};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime, ResponseSnapshotError,
    ResponseSnapshotOwnerInfo, ResponseSnapshotRequest, ResponseSnapshotSection,
    ResponseSnapshotSink,
};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    StableGraphId, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, CompileCaps, ControlAddress,
    Lane, Owner, ParameterUnit, RouteSource, Section, SessionModel, StableId, compile_session,
    parse_session_json,
};

const MONO: &str = include_str!("../../../fixtures/session/v1/console-sixty-four-track-mono.json");

/// Two full eight-lane banks and a padded one of four (five four-lane banks on the arm64 legs).
const TRACKS: usize = 20;

fn sid(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

/// The widest width this build runs: Simd8 where `avx2` is enabled, Simd4 otherwise.
fn width() -> Backend {
    *Backend::VECTOR.last().expect("a vector width")
}

// ---- The session --------------------------------------------------------------------------------

/// Every track from `ch08` on reads two source channels; the first bank is mono-mapped.
fn stereo(track: usize) -> bool {
    track >= 8
}

/// Which tracks' routes carry a negative right row (every left row is negative).
type RightNegative = fn(usize) -> bool;

/// The first four tracks only: the shape gate 2 renders.
fn first_four(index: usize) -> bool {
    index < 4
}

/// Layout A: tracks `0..16`, so a skipped padded bank adds `+0.0` onto a right master that the
/// store bank and the second bank leave at `-0.0` (the `+= +0.0` pass of the silent fold).
fn first_sixteen(index: usize) -> bool {
    index < 16
}

/// Layout B: every track except `4..8`, so the store bank's first lane routes to `-0.0` and a
/// later lane of it to `+0.0` (the store sign judged over every lane, not lane 0).
fn all_but_four_to_eight(index: usize) -> bool {
    !(4..8).contains(&index)
}

fn session(right_negative: RightNegative) -> SessionModel {
    let mut model = parse_session_json(MONO).expect("the mono desk parses");
    model.tracks.truncate(TRACKS);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| {
        matches!(&route.source, RouteSource::Track { track_id, .. } if kept.contains(track_id))
    });
    model.automation.clear();
    for (index, track) in model.tracks.iter_mut().enumerate() {
        assert_eq!(track.console[1].slot.as_str(), "comp");
        if !(8..16).contains(&index) {
            // Parameter 1 is the threshold.
            for param in &mut track.console[1].params {
                if param.parameter_id == 1 {
                    param.value = 0.0;
                }
            }
        }
        // A higher high-pass corner shortens the input section's decay to rest.
        track.builtins.left.hpf_hz = 100.0;
        track.builtins.right.hpf_hz = 100.0;
        if stereo(index) {
            track.left_source_channel = 0;
            track.right_source_channel = 1;
        }
    }
    for route in &mut model.routes {
        let RouteSource::Track { track_id, .. } = &route.source else {
            continue;
        };
        let index: usize = track_id.as_str()[2..].parse().expect("chNN");
        route.channel_matrix.ll = -1.0;
        route.channel_matrix.lr = -0.25;
        if right_negative(index) {
            route.channel_matrix.rl = -0.25;
            route.channel_matrix.rr = -1.0;
        }
    }
    model.automation.push(Automation {
        id: sid("ride11"),
        target: ControlAddress {
            owner: Owner::Strip(sid("ch11")),
            section: Section::Builtins,
            control: 5,
            lane: Lane::Both,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 100 * 128,
            end_sample: 300 * 128,
            start_value: 0.0,
            end_value: -9.0,
            unit: ParameterUnit::Db,
        }],
        offset: AutomationOffset {
            left_db: 0.0,
            right_db: 0.0,
        },
    });
    model
}

// ---- The inputs ---------------------------------------------------------------------------------

/// xorshift64*: deterministic and the same on every target.
struct Rng(u64);
impl Rng {
    fn below(&mut self, bound: u64) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d) % bound
    }
}

/// One block of one track's input.
#[derive(Clone, Copy, Debug)]
enum Block {
    /// A tone and a little hash noise at this level.
    Signal(f32),
    /// Every word `-0.0`.
    NegativeZero,
    /// Every word exact `+0.0`.
    Silence,
    /// Every word exact `+0.0` but the right plane's last, an impulse: the one non-zero word a
    /// skip's input test must find.
    RightTail,
}

/// The windows forced silent, as `(blocks, tracks)`: the first bank alone while the others play,
/// every bank at once, then the padded bank alone. Each is long enough for a bank to reach rest
/// (about 70 blocks here) and then skip.
const WINDOWS: [(core::ops::Range<usize>, core::ops::Range<usize>); 3] = [
    (60..190, 0..8),
    (230..360, 0..TRACKS),
    (400..530, 16..TRACKS),
];

/// Every track's per-block schedule: random signal runs (a sixth of them at `1e-30`, another
/// sixth at `1e-6`), a `-0.0` block after a quarter of them and `+0.0` runs of up to 120 blocks;
/// then [`WINDOWS`], and one [`Block::RightTail`] in the last of them.
fn schedules(blocks: usize, seed: u64) -> Vec<Arc<[Block]>> {
    let mut rng = Rng(seed);
    (0..TRACKS)
        .map(|track| {
            let mut schedule = Vec::with_capacity(blocks);
            while schedule.len() < blocks {
                let level = match rng.below(6) {
                    0 => 1e-30,
                    1 => 1e-6,
                    _ => 0.02 + rng.below(1000) as f32 * 0.0008,
                };
                for _ in 0..2 + rng.below(10) {
                    schedule.push(Block::Signal(level));
                }
                if rng.below(4) == 0 {
                    schedule.push(Block::NegativeZero);
                }
                for _ in 0..rng.below(120) {
                    schedule.push(Block::Silence);
                }
            }
            schedule.truncate(blocks);
            for (block, kind) in schedule.iter_mut().enumerate() {
                if WINDOWS
                    .iter()
                    .any(|(blocks, tracks)| blocks.contains(&block) && tracks.contains(&track))
                {
                    *kind = Block::Silence;
                }
            }
            // Inside the padded bank's window, once it rests: one late right-plane word on a
            // stereo track.
            if track == 18 {
                schedule[500] = Block::RightTail;
            }
            schedule.into()
        })
        .collect()
}

struct TrackSource {
    schedule: Arc<[Block]>,
    stereo: bool,
    seed: u32,
}

impl GraphRuntimeProcessor for TrackSource {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.schedule)
    }

    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let frames = block.left.len() as u64;
        let kind = self.schedule[(block.first_sample / frames) as usize];
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let t = block.first_sample + index as u64;
            let (l, r) = match kind {
                Block::Silence => (0.0, 0.0),
                Block::RightTail => (
                    0.0,
                    if index + 1 == frames as usize {
                        0.5
                    } else {
                        0.0
                    },
                ),
                Block::NegativeZero => (-0.0, -0.0),
                Block::Signal(level) => {
                    let hash = (t as u32 ^ self.seed).wrapping_mul(0x9e37_79b9) >> 9;
                    let noise = hash as f32 / (1 << 23) as f32 - 0.5;
                    let l = level * (math::sinf(t as f32 * 0.031 + self.seed as f32) + 0.2 * noise);
                    let r = level * math::cosf(t as f32 * 0.017);
                    (l, if self.stereo { r } else { l })
                }
            };
            *left = l;
            *right = r;
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

struct Capture(Arc<Mutex<Vec<u32>>>);
impl GraphRuntimeObserver for Capture {
    fn retained_bytes(&self) -> u64 {
        effect_contract::retained::arc(&self.0)
            + self
                .0
                .lock()
                .map_or(0, |v| effect_contract::retained::vec(&v))
    }

    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        let mut sink = self.0.lock().expect("capture");
        sink.extend(block.left.iter().map(|sample| sample.to_bits()));
        sink.extend(block.right.iter().map(|sample| sample.to_bits()));
        Ok(())
    }
}

// ---- Compile, bind, render ----------------------------------------------------------------------

struct Bound {
    plan: PreparedRenderPlan,
    frames: usize,
    latency: u64,
    /// Each track's post-matrix words, when observed.
    sinks: Vec<Arc<Mutex<Vec<u32>>>>,
}

/// Compile `model` at [`width`] the way every host does, bind every track input to its schedule,
/// observe every track's post-matrix stage when `observed`, and arm the mono collapse as
/// `host-core`'s prepare does.
fn bind(model: &SessionModel, schedules: &[Arc<[Block]>], observed: bool) -> Bound {
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
    .unwrap_or_else(|error| panic!("session: {:?}", error.diagnostics()));
    let effects = prepare_native_session_effects(
        &session,
        launch_native_effect_registry().expect("launch registry"),
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 28,
            maximum_automation_spans_per_block: 32,
        },
    )
    .unwrap_or_else(|error| panic!("effects: {}", error.0[0].code));
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
    .expect("the session's builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        input_floors: Default::default(),
        plan_id: 1107,
        effects,
        builtins,
        caps: GraphCompileCaps {
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
        },
        dispatch: width(),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .unwrap_or_else(|failure| panic!("graph: {:?}", failure.diagnostics));
    let latency = artifact.report().output_latency.0;
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
                    let index: usize = track_id.as_str()[2..].parse().expect("chNN");
                    Box::new(TrackSource {
                        schedule: Arc::clone(&schedules[index]),
                        stereo: stereo(index),
                        seed: 17 + index as u32 * 7919,
                    })
                }
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node.clone(), processor)
        })
        .collect();
    let mut sinks = Vec::new();
    let mut observers = Vec::new();
    if observed {
        for track in &model.tracks {
            // Preallocated: the audited allocator aborts on a render-thread allocation, and the
            // capture runs inside render.
            let sink = Arc::new(Mutex::new(Vec::with_capacity(
                schedules[0].len() * frames * 2,
            )));
            observers.push(GraphNodeObserverBinding::new(
                GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track.id.as_str()).expect("track id"),
                    stage: TrackStage::PostMatrix,
                },
                observers.len() as u64 + 1,
                Box::new(Capture(Arc::clone(&sink))),
            ));
            sinks.push(sink);
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
    let mono_source: std::collections::BTreeSet<Box<str>> = session_structural_symmetry(&session)
        .into_iter()
        .filter(|(_, witness)| witness.eligible())
        .map(|(track, _)| track)
        .collect();
    plan.arm_mono_collapse(&|track: &str| mono_source.contains(track));
    Bound {
        plan,
        frames,
        latency,
        sinks,
    }
}

impl Bound {
    /// Render one block with the skip allowed or forbidden; the host planes start at a sentinel,
    /// so an unwritten word shows.
    fn render(&mut self, block: u64, skip: bool, pcm: &mut [f32]) {
        graph::test_only_disable_bank_skip(!skip);
        pcm.fill(7.0);
        let frames = self.frames;
        let rendered = self.plan.render(
            RenderIo {
                output: PlanarBufferMut::try_new(pcm, 2, frames, frames).expect("output planes"),
            },
            RenderTime {
                absolute_sample: block * frames as u64,
            },
        );
        graph::test_only_disable_bank_skip(false);
        rendered.expect("render");
    }

    fn skips(&self) -> u64 {
        graph::test_only_bank_silent_skips(&self.plan)
    }
}

/// Word equality with every NaN folded to one value.
fn fold_nan(word: u32) -> u32 {
    if f32::from_bits(word).is_nan() {
        0x7fc0_0000
    } else {
        word
    }
}

// ---- Gate 1 -------------------------------------------------------------------------------------

/// Gate 1. Test value: red when a skipped block leaves any output or state bit other than a
/// processed block would -- a clock not advanced, a resident scratch not zeroed, a silent fold's
/// signed zero wrong, an input test that misses a non-zero word, or a skip taken while a lane is
/// not at rest -- on a compiled plan, which no rack-level test reaches.
///
/// It renders two route-sign layouts (left rows negative on every track in both): with right rows
/// negative on tracks `0..16` ([`first_sixteen`]), a skipped bank's `+0.0` add lands on a `-0.0`
/// right master; with right rows negative on every track but `4..8`
/// ([`all_but_four_to_eight`]), the store bank mixes a `-0.0` first lane with `+0.0` later lanes.
/// A `-0.0` master block is asserted on the left plane only, the one plane where every
/// contributor can route to `-0.0` in both layouts.
#[test]
fn a_skipped_bank_renders_the_bits_of_a_processed_one() {
    for (name, layout) in [
        (
            "right rows negative on 0..16",
            first_sixteen as RightNegative,
        ),
        ("right rows negative except on 4..8", all_but_four_to_eight),
    ] {
        eprintln!("layout: {name}");
        gate_one(layout);
    }
}

fn gate_one(layout: RightNegative) {
    let blocks = 560;
    let schedules = schedules(blocks, 0x1107);
    let model = session(layout);
    let mut skip = bind(&model, &schedules, true);
    let mut twin = bind(&model, &schedules, true);
    let frames = skip.frames;
    let mut skip_pcm = vec![0.0_f32; frames * 2];
    let mut twin_pcm = vec![0.0_f32; frames * 2];
    // Blocks in which some chain skipped while the host output was audible: other banks played.
    let mut skipped_while_audible = 0;
    let mut audible = 0;
    let mut silent_master = [0_usize; 2];
    let mut skipped = vec![0_u64; blocks];
    for block in 0..blocks as u64 {
        let before = skip.skips();
        skip.render(block, true, &mut skip_pcm);
        twin.render(block, false, &mut twin_pcm);
        for (index, (a, b)) in skip_pcm.iter().zip(&twin_pcm).enumerate() {
            assert_eq!(
                fold_nan(a.to_bits()),
                fold_nan(b.to_bits()),
                "block {block}: host word {index} differs ({a:e} skipping, {b:e} processed)"
            );
        }
        skipped[block as usize] = skip.skips() - before;
        let loud = skip_pcm.iter().any(|word| *word != 0.0);
        audible += usize::from(loud);
        if loud && skip.skips() > before {
            skipped_while_audible += 1;
        }
        for (plane, count) in silent_master.iter_mut().enumerate() {
            let words = &skip_pcm[plane * frames..(plane + 1) * frames];
            if words
                .iter()
                .all(|word| word.to_bits() == (-0.0_f32).to_bits())
            {
                *count += 1;
            }
        }
    }
    assert_eq!(twin.skips(), 0, "the twin never skips");
    let skips = skip.skips();
    eprintln!(
        "skips {skips}, blocks skipped while audible {skipped_while_audible}, audible blocks \
         {audible}, -0.0 master blocks {silent_master:?}, latency {}, collapses {:?}, folds {}",
        skip.latency,
        skip.plan.bank_collapse_counters(),
        skip.plan.bank_route_folds()
    );
    for (window, _) in &WINDOWS {
        assert!(
            skipped[window.clone()].iter().sum::<u64>() > 0,
            "a bank skipped in window {window:?}"
        );
    }
    assert!(skips > 0, "the skipping plan skipped");
    assert!(
        skipped_while_audible > 0,
        "some bank skipped while another played"
    );
    assert!(audible > blocks / 4, "the output is audible");
    assert!(
        silent_master[0] > 0,
        "a silent fold left the master at -0.0"
    );
    assert!(
        skip.plan.bank_collapse_counters()[0] > 0,
        "a mono bank collapsed"
    );
    for (track, (a, b)) in skip.sinks.iter().zip(&twin.sinks).enumerate() {
        let a = a.lock().expect("capture");
        let b = b.lock().expect("capture");
        assert_eq!(a.len(), b.len(), "ch{track:02}: observed lengths");
        assert!(
            a.iter().any(|word| f32::from_bits(*word) != 0.0),
            "ch{track:02} is audible"
        );
        for (index, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            assert_eq!(
                fold_nan(*x),
                fold_nan(*y),
                "ch{track:02}: observed word {index} differs"
            );
        }
    }
}

// ---- Gate 2 -------------------------------------------------------------------------------------

/// Gate 2. Test value: red if a skipped block -- the input test, the rest query, the scratch
/// zeroing, the clock advance or the silent fold -- allocates or frees on the render thread; and
/// (#1782a) if a forced lane move, or a block of a displaced group -- the deferred epilogues, the
/// home-tile re-materialisation and the group fold, skipping or not -- does.
#[test]
fn a_skipping_block_allocates_nothing() {
    let blocks = 400;
    // Signal for the first 40 blocks, then exact silence on every track.
    let schedules: Vec<Arc<[Block]>> = (0..TRACKS)
        .map(|_| {
            (0..blocks)
                .map(|block| {
                    if block < 40 {
                        Block::Signal(0.1)
                    } else {
                        Block::Silence
                    }
                })
                .collect::<Vec<_>>()
                .into()
        })
        .collect();
    let mut bound = bind(&session(first_four), &schedules, false);
    let mut placement = Placement::of(&bound.plan);
    let mut pcm = vec![0.0_f32; bound.frames * 2];
    assert_installed();
    let mut measured = 0;
    for block in 0..blocks as u64 {
        // #1782a: moves while the tracks play, and again once every bank skips.
        if block == 10 || block == 100 {
            let mark = current_thread_counters();
            for (x, y) in [(0, 5), (9, 17), (12, 16)] {
                placement.swap(&mut bound.plan, x, y);
            }
            let delta = current_thread_delta_since(mark);
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: a lane move allocated or freed"
            );
        }
        let before = bound.skips();
        let mark = current_thread_counters();
        bound.render(block, true, &mut pcm);
        let delta = current_thread_delta_since(mark);
        if block >= 10 || (block >= 2 && bound.skips() > before) {
            measured += usize::from(bound.skips() > before);
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: a skipping or displaced block allocated or freed"
            );
        }
    }
    assert!(measured > 0, "some block skipped");
}

// ---- Gate 3 (issue #1782a) ----------------------------------------------------------------------

/// Where each track renders in a plan whose one pack group holds tracks `from..TRACKS`, as
/// `(unit, lane)`: home `(track - from) / W, (track - from) % W` from the group's first unit
/// until a forced move trades two. A track before `from` is in no group and never moves.
struct Placement {
    at: Vec<(usize, usize)>,
}

impl Placement {
    fn of(plan: &PreparedRenderPlan) -> Self {
        Self::from(plan, 0)
    }

    fn from(plan: &PreparedRenderPlan, from: usize) -> Self {
        let groups = graph::test_only_pack_groups(plan);
        let &[(first, last, width)] = groups.as_slice() else {
            panic!("one pack group: {groups:?}");
        };
        let (first, width) = (first as usize, width as usize);
        assert!(
            (last as usize - first + 1) * width >= TRACKS - from,
            "the group holds tracks {from}..{TRACKS}: {groups:?}"
        );
        Self {
            at: (0..TRACKS)
                .map(|track| {
                    let index = track.saturating_sub(from);
                    (first + index / width, index % width)
                })
                .collect(),
        }
    }

    /// Trade the lanes tracks `x` and `y` render at.
    fn swap(&mut self, plan: &mut PreparedRenderPlan, x: usize, y: usize) {
        assert!(
            graph::test_only_swap_bank_lanes(plan, self.at[x], self.at[y]),
            "ch{x:02} at {:?} and ch{y:02} at {:?} trade lanes",
            self.at[x],
            self.at[y]
        );
        self.at.swap(x, y);
    }
}

/// The forced permutation, as `(block, x, y)`: tracks `x` and `y` trade lanes before `block`
/// renders. Every move is legal at both widths (each track pair shares a collapse arming, and the
/// automated `ch11` stays in its own bank): the store lane `ch00` leaves lane 0 and comes back,
/// strips cross between the stereo banks and the padded one, moves land inside a window where every
/// bank skips (230..360) and inside the padded bank's (400..530), and the successor is moved after
/// it took the carry. The two moves at 440 are the regression row of the carry's way home: they
/// leave `ch11` (automated) on a home lane whose own strip sits outside the fader bank that has
/// automation slots, so the direct trade back refuses at both widths and the walk must send
/// `ch11` home instead (red before that fallback: the carry was dropped).
const MOVES: [(u64, usize, usize); 13] = [
    (0, 0, 5),
    (0, 9, 17),
    (0, 12, 16),
    (0, 8, 15),
    (120, 0, 3),
    (120, 10, 11),
    (250, 17, 19),
    (250, 9, 17),
    (410, 5, 0),
    (440, 8, 9),
    (440, 8, 10),
    (480, 1, 6),
    (480, 13, 18),
];

/// The swap block: the successor takes the carry from the predecessor, which is displaced.
const SWAP: u64 = 450;

/// Blocks before which every track's response snapshot is read from both plans.
const SNAPSHOTS: [u64; 4] = [100, 300, SWAP, 500];

/// Every owner's identity, bypass and section words, in graph order.
struct SnapshotWords(Vec<u32>);
impl ResponseSnapshotSink for SnapshotWords {
    fn copy_owner(
        &mut self,
        owner: ResponseSnapshotOwnerInfo<'_>,
        left: &[ResponseSnapshotSection],
        right: &[ResponseSnapshotSection],
    ) -> Result<(), ResponseSnapshotError> {
        self.0.extend(owner.track_id.bytes().map(u32::from));
        self.0.extend([
            u32::from(owner.rack),
            owner.slot,
            owner.kind,
            u32::from(owner.bypassed),
        ]);
        for section in left.iter().chain(right) {
            self.0.extend([
                section.id,
                section.kind,
                u32::from(section.enabled),
                u32::from(section.word_count),
            ]);
            self.0.extend(section.words);
        }
        Ok(())
    }
}

fn snapshot(plan: &mut PreparedRenderPlan, track: usize) -> Vec<u32> {
    let id = format!("ch{track:02}");
    let mut sink = SnapshotWords(Vec::new());
    plan.copy_response_snapshot(ResponseSnapshotRequest {
        track_id: &id,
        sink: &mut sink,
    })
    .unwrap_or_else(|error| panic!("{id}: snapshot {error:?}"));
    sink.0
}

/// A predecessor and the successor built to displace it, with the carry host-core installs: every
/// strip input section, fader and matrix lane, and console effect lane, joined by strip ID.
struct Pair {
    before: Bound,
    after: Bound,
}

fn carried_pair(model: &SessionModel, schedules: &[Arc<[Block]>]) -> Pair {
    let mut before = bind(model, schedules, true);
    let mut after = bind(model, schedules, true);
    graph::install_carry_program(
        &mut after.plan,
        graph::GraphCarryProgram {
            predecessor: graph::plan_identity(&mut before.plan).expect("a graph plan"),
            sources: Box::default(),
            timeline: false,
        },
    )
    .expect("carry program");
    let inputs: BTreeMap<_, _> = graph::builtin_input_lanes(&mut before.plan)
        .expect("a graph plan")
        .into_iter()
        .collect();
    let input_moves: Vec<_> = graph::builtin_input_lanes(&mut after.plan)
        .expect("a graph plan")
        .into_iter()
        .map(|(strip, successor)| graph::GraphLaneMove {
            successor,
            predecessor: inputs[&strip],
        })
        .collect();
    assert_eq!(input_moves.len(), TRACKS, "every input section carries");
    graph::install_builtin_input_carry(&mut after.plan, input_moves).expect("input carry");
    let strips: BTreeMap<_, _> = graph::builtin_strip_lanes(&mut before.plan)
        .expect("a graph plan")
        .into_iter()
        .map(|(strip, lane)| ((strip, lane.owner), lane.location))
        .collect();
    let strip_moves: Vec<_> = graph::builtin_strip_lanes(&mut after.plan)
        .expect("a graph plan")
        .into_iter()
        .map(|(strip, lane)| graph::GraphStripMove {
            owner: lane.owner,
            successor: lane.location,
            predecessor: strips[&(strip, lane.owner)],
            continued_cells: 0,
        })
        .collect();
    assert_eq!(
        strip_moves.len(),
        2 * TRACKS,
        "every fader and matrix carries"
    );
    graph::install_builtin_strip_carry(&mut after.plan, strip_moves).expect("strip carry");
    let effects: BTreeMap<_, _> = graph::bank_effect_lanes(&mut before.plan)
        .expect("a graph plan")
        .into_iter()
        .map(|(strip, lane)| ((strip, lane.effect_id), lane.location))
        .collect();
    let effect_moves: Vec<_> = graph::bank_effect_lanes(&mut after.plan)
        .expect("a graph plan")
        .into_iter()
        .map(|(strip, lane)| graph::GraphEffectMove {
            successor: lane.location,
            predecessor: effects[&(strip, lane.effect_id)],
        })
        .collect();
    assert_eq!(effect_moves.len(), effects.len(), "every effect carries");
    assert!(
        effect_moves.len() >= 3 * TRACKS,
        "every console effect carries"
    );
    graph::install_effect_carry(&mut after.plan, effect_moves, 0).expect("effect carry");
    Pair { before, after }
}

/// Gate 3 (#1782a): a plan whose strips trade bank lanes renders exactly the unmoved plan.
///
/// One session, two carried pairs: one pair renders under [`MOVES`], the other never moves. Before
/// [`SWAP`] the predecessors render; at it each successor takes its predecessor's state, the moved
/// predecessor displaced; then the successors render, the moved one moved again. Every block's host
/// output, every track's post-matrix words and every track's response snapshot at [`SNAPSHOTS`]
/// must agree by `to_bits`, NaNs folded. The tracks' input filters differ, so a snapshot read of
/// another strip's lane shows.
///
/// Test value: red when a move leaves a lane-keyed reader behind -- a member op (gather source,
/// scatter target, observer), a route fold entry or its store bit, a source-claim bit, a
/// response-snapshot binding, the carry's predecessor side -- or when a displaced group's master
/// fold changes its association order or its signed zeros, which no other test reaches.
#[test]
fn a_moved_strip_renders_the_bits_of_an_unmoved_one() {
    let mut model = session(first_sixteen);
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.builtins.left.hpf_hz = 100.0 + index as f32;
        track.builtins.right.hpf_hz = 100.0 + index as f32;
    }
    eprintln!("layout: one group, the master's first contributor in it");
    gate_three(&model, 0);
    // Tracks 0..8 carry an EQ insert, so their bank is no peer of the next: the group is the
    // other banks, and the master's first contributor (ch00, which stores) is an earlier unit
    // outside it. A displaced group's fold then adds onto a live master, which shows any early
    // epilogue write the store would otherwise overwrite. (Four tracks are not enough: a padded
    // insert bank declines the route fold outright.)
    let eq = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    for track in &mut model.tracks[..8] {
        let mut insert = eq.clone();
        insert.id = sid("insert0-eq");
        track.inserts.effects.push(insert);
    }
    eprintln!("layout: the master's first contributor before the group");
    gate_three(&model, 8);
}

/// Gate 3 over `model`, whose one pack group holds tracks `from..TRACKS`; the moves of tracks
/// before `from` are left out.
fn gate_three(model: &SessionModel, from: usize) {
    let blocks = 560;
    let schedules = schedules(blocks, 0x1782);
    let mut moved = carried_pair(model, &schedules);
    let mut home = carried_pair(model, &schedules);
    assert_eq!(
        moved.before.plan.bank_route_folds(),
        TRACKS as u64,
        "every track folds into the master"
    );
    let mut placement = Placement::from(&moved.before.plan, from);
    let frames = moved.before.frames;
    let mut moved_pcm = vec![0.0_f32; frames * 2];
    let mut home_pcm = vec![0.0_f32; frames * 2];
    let mut displaced_skips = 0;
    for block in 0..blocks as u64 {
        if block == SWAP {
            for pair in [&mut moved, &mut home] {
                assert_eq!(
                    pair.after
                        .plan
                        .adopt_predecessor_plan(&mut pair.before.plan),
                    engine::realtime::CarryOutcome::Carried,
                    "the successor takes the carry"
                );
            }
            placement = Placement::from(&moved.after.plan, from);
        }
        let (moved_plan, home_plan) = if block < SWAP {
            (&mut moved.before, &mut home.before)
        } else {
            (&mut moved.after, &mut home.after)
        };
        for &(at, x, y) in &MOVES {
            if at == block && x >= from && y >= from {
                placement.swap(&mut moved_plan.plan, x, y);
            }
        }
        if SNAPSHOTS.contains(&block) {
            for track in 0..TRACKS {
                assert_eq!(
                    snapshot(&mut moved_plan.plan, track),
                    snapshot(&mut home_plan.plan, track),
                    "block {block}: ch{track:02}'s response snapshot"
                );
            }
        }
        let before = moved_plan.skips();
        moved_plan.render(block, true, &mut moved_pcm);
        home_plan.render(block, true, &mut home_pcm);
        displaced_skips += usize::from(moved_plan.skips() > before);
        for (index, (a, b)) in moved_pcm.iter().zip(&home_pcm).enumerate() {
            assert_eq!(
                fold_nan(a.to_bits()),
                fold_nan(b.to_bits()),
                "block {block}: host word {index} differs ({a:e} moved, {b:e} unmoved)"
            );
        }
    }
    assert!(displaced_skips > 0, "a displaced group skipped a bank");
    for (side, (moved, home)) in [(&moved.before, &home.before), (&moved.after, &home.after)]
        .into_iter()
        .enumerate()
    {
        for (track, (a, b)) in moved.sinks.iter().zip(&home.sinks).enumerate() {
            let a = a.lock().expect("capture");
            let b = b.lock().expect("capture");
            assert_eq!(
                a.len(),
                b.len(),
                "plan {side}, ch{track:02}: observed lengths"
            );
            // The predecessor's 450 blocks reach every track's signal.
            assert!(
                side == 1 || a.iter().any(|word| f32::from_bits(*word) != 0.0),
                "ch{track:02} is audible"
            );
            for (index, (x, y)) in a.iter().zip(b.iter()).enumerate() {
                assert_eq!(
                    fold_nan(*x),
                    fold_nan(*y),
                    "plan {side}, ch{track:02}: observed word {index} differs"
                );
            }
        }
    }
}
