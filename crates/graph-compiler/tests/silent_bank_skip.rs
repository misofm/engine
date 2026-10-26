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

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use builtins::BuiltinLaneSelector;
use builtins_compiler::{
    BuiltinCompileCaps, TestOnlyWriteGate, TrackControlProducer, TrackControlRequest,
    TrackInputRecord, prepare_session_builtins, prepare_session_builtins_with_live_controls,
    session_structural_symmetry,
};
use effect_compiler::{
    EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
};
use engine::realtime::{
    PlanRevisionGate, PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderTime,
    ResponseSnapshotError, ResponseSnapshotOwnerInfo, ResponseSnapshotRequest,
    ResponseSnapshotSection, ResponseSnapshotSink,
};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
    StableGraphId, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, CompileCaps, ControlAddress,
    EffectParam, Lane, Owner, ParameterChannel, ParameterUnit, RouteSource, Section, SendTap,
    SessionModel, StableId, compile_session, parse_session_json,
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

/// The block from which every track plays (issue #1981): in a schedule of more blocks than this,
/// the all-active tail after which a packing plan must be back at its home lanes.
const TAIL: usize = 560;

/// Every track's per-block schedule: random signal runs (a sixth of them at `1e-30`, another
/// sixth at `1e-6`), a `-0.0` block after a quarter of them and `+0.0` runs of up to 120 blocks;
/// then [`WINDOWS`], one [`Block::RightTail`] in the last of them, and from [`TAIL`] on signal.
fn schedules(blocks: usize, seed: u64) -> Vec<Arc<[Block]>> {
    let mut rng = Rng(seed);
    (0..TRACKS)
        .map(|track| {
            // The random part is the same for every `blocks` from [`TAIL`] on.
            let random = blocks.min(TAIL);
            let mut schedule = Vec::with_capacity(blocks);
            while schedule.len() < random {
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
            schedule.truncate(random);
            // Gate 1's all-active tail (issue #1981): every track plays, so every strip runs.
            schedule.resize(blocks, Block::Signal(0.05));
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
    /// Whether the plan packs its lanes each block (issue #1981; on, as every shipped plan).
    pack: bool,
    /// Each track's post-matrix words, when observed.
    sinks: Vec<Arc<Mutex<Vec<u32>>>>,
    /// The effect banks the compile bound: one bank-slot run each on a block every chain renders
    /// (issue #2132).
    bound_banks: u64,
    /// Each track's live control channel, in track order, when [`Setup::live`].
    controls: Vec<TrackControlProducer>,
    /// The plan's revision gate, which publishes a live write, when [`Setup::live`].
    gate: Option<PlanRevisionGate>,
}

/// Compile `model` at [`width`] the way every host does, bind every track input to its schedule,
/// observe every track's post-matrix stage when `observed`, and arm the mono collapse as
/// `host-core`'s prepare does.
fn bind(model: &SessionModel, schedules: &[Arc<[Block]>], observed: bool) -> Bound {
    bind_with(model, schedules, observed, Setup::DESK)
}

/// How [`bind_with`] prepares a plan beyond [`bind`]'s defaults.
#[derive(Clone, Copy)]
struct Setup {
    /// Which tracks' sources write two different planes; every other track's source writes its
    /// left plane on both.
    stereo: fn(usize) -> bool,
    /// Whether every track gets a live input-lane control channel (trim, polarity, filters), as a
    /// `Concurrent` host prepares it (issue #2142's collapse rows write a live trim through it).
    live: bool,
}

impl Setup {
    /// The desk of [`session`]: [`stereo`] sources, no live channel.
    const DESK: Self = Self {
        stereo,
        live: false,
    };
}

/// [`bind`] under `setup`.
fn bind_with(
    model: &SessionModel,
    schedules: &[Arc<[Block]>],
    observed: bool,
    setup: Setup,
) -> Bound {
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
    let caps = BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    };
    let builtins = if setup.live {
        let requests: Vec<TrackControlRequest> = model
            .tracks
            .iter()
            .map(|track| TrackControlRequest {
                track_id: track.id.as_str().to_owned(),
                superseded: builtins_compiler::LiveValuesSuperseded::default(),
                input_lane: true,
            })
            .collect();
        prepare_session_builtins_with_live_controls(
            &session,
            &controls::effective(
                session.normalized_model(),
                &controls::MonitorOverlay::default(),
            ),
            &[],
            &requests,
            caps,
        )
    } else {
        prepare_session_builtins(&session, &[], caps)
    }
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
    let bound_banks = artifact.report().rack_cohorts.bound_slots.len() as u64;
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
                        stereo: (setup.stereo)(index),
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
    let bound = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers,
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
    let controls = bound.track_controls;
    let mut plan = bound.plan;
    let gate = if setup.live {
        plan.take_revision_gate()
    } else {
        None
    };
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
        pack: true,
        sinks,
        bound_banks,
        controls,
        gate,
    }
}

impl Bound {
    /// Render one block with the skip allowed or forbidden; the host planes start at a sentinel,
    /// so an unwritten word shows.
    fn render(&mut self, block: u64, skip: bool, pcm: &mut [f32]) {
        graph::test_only_disable_bank_skip(!skip);
        graph::test_only_disable_lane_packing(!self.pack);
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
        graph::test_only_disable_lane_packing(false);
        rendered.expect("render");
    }

    fn skips(&self) -> u64 {
        graph::test_only_bank_silent_skips(&self.plan)
    }

    fn lane_moves(&self) -> u64 {
        graph::test_only_bank_lane_moves(&self.plan)
    }

    fn displaced(&self) -> u64 {
        graph::test_only_displaced_strips(&self.plan)
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

// ---- Issue #2132 gate 2: bank-slot runs ---------------------------------------------------------

/// Issue #2132 gate 2: a pack group's bank-slot runs per block are its banks times its effect slots
/// when every chain renders, and none for a chain the block skipped; over the plan, a block every
/// chain renders runs exactly the effect banks the compile bound. With a console slot bypassed on
/// every track (issue #2129), that slot's banks skip their wet path and run nothing.
///
/// Test value: red when the counter and the skips disagree -- a run counted for a skipped chain,
/// a builtin stage counted as an effect slot, or a bypass-skipped slot counted as a run -- which
/// the skip differential (gate 1) cannot see, because a counter moves no output bit.
#[test]
fn bank_slot_runs_follow_the_skips() {
    // Issue #2129: with the compressor slot bypassed on every track, every bank of that slot
    // skips its wet path on every block its chain renders, and runs nothing.
    let mut bypassed = session(first_sixteen);
    for track in &mut bypassed.tracks {
        track.console[1].bypass = true;
    }
    for (model, skipped_slots) in [(session(first_sixteen), 0), (bypassed, 1)] {
        let blocks = TAIL + 8;
        let schedules = schedules(blocks, 0x2132);
        let mut bound = bind(&model, &schedules, false);
        let mut pcm = vec![0.0_f32; bound.frames * 2];
        let mut previous = graph::test_only_bank_slot_runs(&mut bound.plan);
        assert!(!previous.is_empty(), "the console has bank units");
        let per_block: u64 = previous
            .iter()
            .map(|group| {
                u64::from(group.effect_slots) * u64::from(group.last_unit - group.first_unit + 1)
            })
            .sum();
        assert_eq!(
            per_block, bound.bound_banks,
            "every bound effect bank is one slot of one unit, and no builtin stage is counted"
        );
        let units: u64 = previous
            .iter()
            .map(|group| u64::from(group.last_unit - group.first_unit + 1))
            .sum();
        let [mut every_chain_ran, mut a_chain_skipped] = [0_u32; 2];
        for block in 0..blocks as u64 {
            bound.render(block, true, &mut pcm);
            let now = graph::test_only_bank_slot_runs(&mut bound.plan);
            assert_eq!(now.len(), previous.len());
            let mut total = 0;
            for (before, after) in previous.iter().zip(&now) {
                let banks = u64::from(after.last_unit - after.first_unit + 1);
                let skipped = after.skips - before.skips;
                let runs = after.runs - before.runs;
                assert_eq!(
                    runs,
                    (u64::from(after.effect_slots) - skipped_slots) * (banks - skipped),
                    "block {block}, units {}..={}: {skipped} of {banks} chains skipped, \
                     {skipped_slots} slot(s) bypassed",
                    after.first_unit,
                    after.last_unit
                );
                a_chain_skipped += u32::from(skipped > 0);
                total += runs;
            }
            every_chain_ran += u32::from(total == bound.bound_banks - skipped_slots * units);
            previous = now;
        }
        assert!(
            every_chain_ran > 0 && a_chain_skipped > 0,
            "the schedule reaches both an all-running block ({every_chain_ran}) and a skipped \
             chain ({a_chain_skipped})"
        );
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
        gate_one(layout, Twin::Processed);
    }
}

/// Gate 1 of issue #1981: one session compiled twice, one plan packing its lanes each block and
/// its twin never packing (`graph::test_only_disable_lane_packing`), the skip allowed on both,
/// over gate 1's inputs and both route-sign layouts, then an all-active tail ([`TAIL`]). Every
/// block's host output and every track's post-matrix words must agree by `to_bits`, NaNs folded;
/// the packing plan must have moved lanes, skipped more banks than its twin, and be back at its
/// home lanes after the tail. The moves meet both refusals: the mono bank is collapse-armed and
/// the others are not, and `ch11`'s stored ride keeps it running and its fader bank the only one
/// with automation slots.
///
/// The generator also seeks (the render clock jumps [`SEEK_JUMP`] blocks at [`SEEK`], strips
/// displaced), and gives `ch09` and `ch17` a linked compressor detector (#1369), which their moves
/// must carry (#1371).
///
/// Test value: red when a pack moves a strip's or the master's bits, makes a pack that skips no
/// more than the unpacked twin, never packs, or leaves a group displaced (paying the displaced
/// fold) after activity returns, which no other test reaches: the #1782a gate moves lanes only by
/// hand.
#[test]
fn a_packing_plan_renders_the_bits_of_an_unpacked_one() {
    for (name, layout) in [
        (
            "right rows negative on 0..16",
            first_sixteen as RightNegative,
        ),
        ("right rows negative except on 4..8", all_but_four_to_eight),
    ] {
        eprintln!("layout: {name}");
        gate_one(layout, Twin::Unpacked);
    }
}

/// What gate 1's twin plan leaves out. Both plans pack their lanes in [`Twin::Processed`], so the
/// skip differential also runs over displaced groups.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Twin {
    /// The whole-bank skip (issue #1107): every block is processed.
    Processed,
    /// The per-block lane packing (issue #1981).
    Unpacked,
}

/// The packing gate's discontinuity: from this block on, the render clock runs [`SEEK_JUMP`] blocks
/// ahead, while strips are displaced (asserted).
const SEEK: u64 = 545;
const SEEK_JUMP: u64 = 10;

fn gate_one(layout: RightNegative, kind: Twin) {
    let blocks = TAIL + 40;
    let jump = if kind == Twin::Unpacked { SEEK_JUMP } else { 0 };
    let schedules = schedules(blocks + jump as usize, 0x1107);
    let mut model = session(layout);
    if kind == Twin::Unpacked {
        resting_second_bank(&mut model);
        // Two strips whose compressor links its detectors (#1369): a move must carry the lane's
        // link (#1371) with the rest of its state.
        for track in [9, 17] {
            model.tracks[track].console[1].link_mode = session::ConsoleLinkMode::Maximum;
        }
    }
    let mut skip = bind(&model, &schedules, true);
    let mut twin = bind(&model, &schedules, true);
    twin.pack = kind != Twin::Unpacked;
    let frames = skip.frames;
    let mut skip_pcm = vec![0.0_f32; frames * 2];
    let mut twin_pcm = vec![0.0_f32; frames * 2];
    // Blocks in which some chain skipped while the host output was audible: other banks played.
    let mut skipped_while_audible = 0;
    let mut audible = 0;
    let mut silent_master = [0_usize; 2];
    let mut skipped = vec![0_u64; blocks];
    for block in 0..blocks as u64 {
        if jump != 0 && block == SEEK {
            assert!(skip.displaced() > 0, "strips are displaced at the seek");
        }
        let time = if block >= SEEK { block + jump } else { block };
        let before = skip.skips();
        skip.render(time, true, &mut skip_pcm);
        twin.render(time, kind == Twin::Unpacked, &mut twin_pcm);
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
    let skips = skip.skips();
    eprintln!(
        "skips {skips} (twin {}), blocks skipped while audible {skipped_while_audible}, audible \
         blocks {audible}, -0.0 master blocks {silent_master:?}, latency {}, collapses {:?}, \
         folds {}, lane moves {} (twin {}), displaced at the end {}",
        twin.skips(),
        skip.latency,
        skip.plan.bank_collapse_counters(),
        skip.plan.bank_route_folds(),
        skip.lane_moves(),
        twin.lane_moves(),
        skip.displaced(),
    );
    match kind {
        Twin::Processed => assert_eq!(twin.skips(), 0, "the twin never skips"),
        Twin::Unpacked => {
            assert_eq!(twin.lane_moves(), 0, "the twin never packs");
            assert!(skip.lane_moves() > 0, "the packing plan moved lanes");
            assert!(
                skips > twin.skips(),
                "packing let more banks skip ({skips} against {})",
                twin.skips()
            );
            assert_eq!(skip.displaced(), 0, "every strip is home after the tail");
        }
    }
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
/// zeroing, the clock advance or the silent fold -- allocates or frees on the render thread;
/// (#1782a) if a forced lane move, or a block of a displaced group -- the deferred epilogues, the
/// home-tile re-materialisation and the group fold, skipping or not -- does; and (#1981) if the
/// per-block packer -- its facts, a pack or homing move -- or a carry from a displaced predecessor
/// and the successor's first block do.
#[test]
fn a_skipping_block_allocates_nothing() {
    let blocks = 400;
    // Signal for the first 40 blocks, then exact silence on every track but `ch17`, which plays
    // again in 200..280: the packer moves it into the second bank, which `ch11`'s ride keeps
    // rendering, and the predecessor is still displaced at the swap.
    let schedules: Vec<Arc<[Block]>> = (0..TRACKS)
        .map(|track| {
            (0..blocks)
                .map(|block| {
                    if block < 40 || (track == 17 && (200..280).contains(&block)) {
                        Block::Signal(0.1)
                    } else {
                        Block::Silence
                    }
                })
                .collect::<Vec<_>>()
                .into()
        })
        .collect();
    let mut model = session(first_four);
    resting_second_bank(&mut model);
    let Pair {
        mut before,
        mut after,
    } = carried_pair(&model, &schedules);
    // Forced moves only until block 100's, so `Placement` stays exact; the packer from then on.
    before.pack = false;
    let mut placement = Placement::of(&before.plan);
    let mut pcm = vec![0.0_f32; before.frames * 2];
    assert_installed();
    let mut measured = 0;
    let swap = 300;
    for block in 0..blocks as u64 {
        // #1782a: moves while the tracks play, and again once every bank skips.
        if block == 10 || block == 100 {
            let mark = current_thread_counters();
            for (x, y) in [(0, 5), (9, 17), (12, 16)] {
                placement.swap(&mut before.plan, x, y);
            }
            let delta = current_thread_delta_since(mark);
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: a lane move allocated or freed"
            );
        }
        before.pack = block > 100;
        let mark = current_thread_counters();
        if block == swap {
            assert!(
                before.displaced() > 0,
                "the predecessor is displaced at the swap"
            );
            assert_eq!(
                after.plan.adopt_predecessor_plan(&mut before.plan),
                engine::realtime::CarryOutcome::Carried,
                "the successor takes the carry"
            );
        }
        let bound = if block < swap {
            &mut before
        } else {
            &mut after
        };
        let skips = bound.skips();
        bound.render(block, true, &mut pcm);
        let delta = current_thread_delta_since(mark);
        if block >= 10 || (block >= 2 && bound.skips() > skips) {
            measured += usize::from(bound.skips() > skips);
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: a skipping, displaced, packing or carried block allocated or freed"
            );
        }
    }
    assert!(measured > 0, "some block skipped");
    assert!(before.lane_moves() > 0, "the packer moved lanes");
}

/// Which tracks play a signal.
type Playing = fn(usize) -> bool;

/// Issue #1981: a pack shape the chains refuse, whose facts do not change, is attempted once and
/// not again. `ch00` keeps the collapse-armed mono bank running with packable lanes beside it, and
/// stereo strips run in higher banks; the only lower bank that runs is the mono one, and the
/// chains refuse a move between differently armed banks. No ride: the second bank rests and does
/// not run. Two shapes: `ch17` alone, whose refusals stay below the attempt budget, and enough
/// running stereo strips that one step's refusals reach the group's span (`ch16..ch20` on eight
/// lanes, 4 x 7 attempts against 24; `ch12..ch20` on four, 8 x 3 against 20).
///
/// Test value: red when the packer retries a refused move every block although nothing it decides
/// on changed (the `PackGroup::dirty` bit), a standing render cost no bit shows, including when the
/// refusals use up the step's attempt budget.
#[test]
fn a_refused_pack_is_not_retried() {
    let blocks = 200;
    let silent: Vec<Arc<[Block]>> = (0..TRACKS)
        .map(|_| vec![Block::Silence; blocks].into())
        .collect();
    let probe = bind(&session(first_sixteen), &silent, false);
    let groups = graph::test_only_pack_groups(&probe.plan);
    assert_eq!(groups.len(), 1, "one pack group: {groups:?}");
    let lanes = groups[0].width;
    let span = groups[0].chains.len() as u64 * u64::from(lanes);
    drop(probe);
    let wide: Playing = if lanes == 8 {
        |track| track == 0 || track >= 16
    } else {
        |track| track == 0 || track >= 12
    };
    let shapes: [(&str, Playing, bool); 2] = [
        ("ch17", |track| track == 0 || track == 17, false),
        ("the span", wide, true),
    ];
    for (name, playing, reaches_span) in shapes {
        let schedules: Vec<Arc<[Block]>> = (0..TRACKS)
            .map(|track| {
                let kind = if playing(track) {
                    Block::Signal(0.1)
                } else {
                    Block::Silence
                };
                vec![kind; blocks].into()
            })
            .collect();
        let mut model = session(first_sixteen);
        model.automation.clear();
        resting_second_bank(&mut model);
        let mut bound = bind(&model, &schedules, false);
        let mut pcm = vec![0.0_f32; bound.frames * 2];
        let (mut settled, mut peak) = (0, 0);
        for block in 0..blocks as u64 {
            if block == 60 {
                settled = graph::test_only_bank_pack_attempts(&bound.plan);
            }
            let before = graph::test_only_bank_pack_attempts(&bound.plan);
            bound.render(block, true, &mut pcm);
            peak = peak.max(graph::test_only_bank_pack_attempts(&bound.plan) - before);
        }
        let attempts = graph::test_only_bank_pack_attempts(&bound.plan);
        eprintln!(
            "{name}: attempts by block 60 {settled}, by the end {attempts}, peak step {peak}"
        );
        assert!(settled > 0, "{name}: the packer tried the refused move");
        assert_eq!(bound.lane_moves(), 0, "{name}: every move was refused");
        if reaches_span {
            assert_eq!(peak, span, "{name}: a step's refusals reach the span");
        } else {
            assert!(peak < span, "{name}: the refusals stay below the span");
        }
        assert_eq!(
            attempts, settled,
            "{name}: no attempt after the shape settled: the same facts make the same refusal"
        );
    }
}

/// Give the second bank (`ch08..ch16`) the 0 dB compressor threshold the other banks have, so its
/// strips rest once silent and can take a running strip from a higher bank (issue #1981);
/// `ch11`'s ride still keeps that bank rendering every block.
fn resting_second_bank(model: &mut SessionModel) {
    for track in &mut model.tracks[8..16] {
        for param in &mut track.console[1].params {
            if param.parameter_id == 1 {
                param.value = 0.0;
            }
        }
    }
}

// ---- Gate 3 (issue #1782a) ----------------------------------------------------------------------

/// Where each grouped track renders, as `(unit, lane)`, read from the plan itself: each pack
/// group `graph::test_only_pack_groups` reports lists its lane chains (one unit each here), and
/// each of its units' `PlanUnitEligibility::lane_tracks` names the track bind put on each lane
/// (its home). A track in no group is `None` and never moves.
///
/// What the plans group, for the record (the test asserts none of it): a group is a run of
/// adjacent peer banks, and every console bank of [`session`] has one slot program (input
/// builtins, EQ, compressor, limiter, fader, matrix), gathers directly and folds into the master;
/// the mono collapse arming is joined after bind and is no group term. So all of the session's
/// banks are one group at both widths: `(20, 22, 8)` at eight lanes and `(20, 24, 4)` at four.
/// With the EQ insert on tracks 0..8 their banks hold one slot more and are no peers of the
/// others: at eight lanes they are one unit, which is no group (a group has two or more), so
/// the group is `(21, 22, 8)`; at four lanes they are two units and a group of their own,
/// `[(20, 21, 4), (22, 24, 4)]`.
struct Placement {
    at: Vec<Option<(usize, usize)>>,
    /// Each track's group, by its index in the report.
    group: Vec<Option<usize>>,
}

impl Placement {
    fn of(plan: &PreparedRenderPlan) -> Self {
        let groups = graph::test_only_pack_groups(plan);
        assert!(!groups.is_empty(), "the plan has a pack group");
        let rows = plan.unit_eligibility();
        let mut at = vec![None; TRACKS];
        let mut group = vec![None; TRACKS];
        for (index, pack) in groups.iter().enumerate() {
            assert!(
                pack.chains.iter().all(|chain| chain.len() == 1),
                "the session splits no strip"
            );
            for unit in pack.chains.iter().map(|chain| chain[0] as usize) {
                let row = rows
                    .iter()
                    .find(|row| row.unit as usize == unit)
                    .expect("a grouped unit's row");
                assert!(row.banked, "a grouped unit is a bank");
                for (lane, track) in row.lane_tracks.iter().enumerate() {
                    let track: usize = track[2..].parse().expect("chNN");
                    assert!(at[track].is_none(), "ch{track:02} is in one group");
                    at[track] = Some((unit, lane));
                    group[track] = Some(index);
                }
            }
        }
        Self { at, group }
    }

    /// Whether tracks `x` and `y` are in one group, so the plan lets them trade lanes.
    fn together(&self, x: usize, y: usize) -> bool {
        self.group[x].is_some() && self.group[x] == self.group[y]
    }

    /// Whether track `track`'s group holds `ch00`, the master's first contributor, which stores:
    /// a group without it folds onto a live master.
    fn with_store(&self, track: usize) -> bool {
        self.group[track].is_some() && self.group[track] == self.group[0]
    }

    /// Trade the lanes tracks `x` and `y` render at.
    fn swap(&mut self, plan: &mut PreparedRenderPlan, x: usize, y: usize) {
        assert!(
            self.together(x, y),
            "ch{x:02} and ch{y:02} are in one group"
        );
        let (Some(a), Some(b)) = (self.at[x], self.at[y]) else {
            unreachable!("grouped tracks are placed");
        };
        assert!(
            graph::test_only_swap_bank_lanes(plan, a, b),
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
    carried_pair_with(model, schedules, Setup::DESK)
}

/// [`carried_pair`] under `setup`, for `model`'s tracks.
fn carried_pair_with(model: &SessionModel, schedules: &[Arc<[Block]>], setup: Setup) -> Pair {
    let tracks = model.tracks.len();
    let all_strips = tracks + model.submixes.len();
    let mut before = bind_with(model, schedules, true, setup);
    let mut after = bind_with(model, schedules, true, setup);
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
    assert_eq!(input_moves.len(), all_strips, "every input section carries");
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
        2 * all_strips,
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
        effect_moves.len() >= 3 * tracks,
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
    gate_three(&model, false);
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
    gate_three(&model, true);
}

/// Gate 3 over `model`; a move of two tracks the plan does not group together is left out
/// ([`Placement`]: only the insert layout's mono tracks at eight lanes). At every width it must
/// still make the regression row's two moves, a move of the store lane `ch00` (`inserts`
/// false), and a move in a group that folds onto a live master (`inserts` true); else it fails.
fn gate_three(model: &SessionModel, inserts: bool) {
    let blocks = 560;
    let schedules = schedules(blocks, 0x1782);
    let mut moved = carried_pair(model, &schedules);
    let mut home = carried_pair(model, &schedules);
    // The moves here are the forced ones alone: the per-block packer (issue #1981) is off, so the
    // unmoved pair stays at the identity and `Placement` stays exact.
    for bound in [
        &mut moved.before,
        &mut moved.after,
        &mut home.before,
        &mut home.after,
    ] {
        bound.pack = false;
    }
    assert_eq!(
        moved.before.plan.bank_route_folds(),
        TRACKS as u64,
        "every track folds into the master"
    );
    let mut placement = Placement::of(&moved.before.plan);
    let (mut regression, mut store_lane, mut live_master) = (0, 0, 0);
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
            placement = Placement::of(&moved.after.plan);
        }
        let (moved_plan, home_plan) = if block < SWAP {
            (&mut moved.before, &mut home.before)
        } else {
            (&mut moved.after, &mut home.after)
        };
        for &(at, x, y) in &MOVES {
            if at == block && placement.together(x, y) {
                regression += usize::from(at == 440);
                store_lane += usize::from(x == 0 || y == 0);
                live_master += usize::from(!placement.with_store(x));
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
    assert_eq!(regression, 2, "the regression row's two moves were made");
    if inserts {
        assert!(
            live_master > 0,
            "a move in a group that folds onto a live master"
        );
    } else {
        assert!(store_lane > 0, "the store lane moved");
    }
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

// ---- Issue #2142: a split strip packs as one strip -------------------------------------------

const C1: &str = include_str!("../../../fixtures/session/v1/console-c1-16.json");

/// The C1 console at 16 tracks (issue #2132's fixture) in its `AllActive` state: every console
/// slot of every strip and the master's limiter un-bypassed. Its two muted post-fader sends per
/// track split every track strip at the fader: part one is the fused input-to-fader chain, part two
/// the matrix, which gathers part one's lanes through its producer row.
fn c1_all_active() -> SessionModel {
    let mut model = parse_session_json(C1).expect("the C1 fixture parses");
    for track in &mut model.tracks {
        for entry in &mut track.console {
            entry.bypass = false;
        }
    }
    for submix in &mut model.submixes {
        for entry in &mut submix.console {
            entry.bypass = false;
        }
        for effect in &mut submix.inserts.effects {
            if effect.id.as_str() == "limiter" {
                effect.bypass = false;
            }
        }
    }
    model
}

/// [`c1_all_active`] with every track mono-mapped, so every track bank is collapse-armed, and the
/// sends tapped at the insert send instead: part one is the input section and the pre-insert slots,
/// part two the post-insert slot, the fader and the matrix, whose collapse prefix is not empty. So
/// part two's collapse reads its producer row: whether part one rendered identical planes.
fn c1_collapse_armed() -> SessionModel {
    let mut model = c1_all_active();
    for track in &mut model.tracks {
        track.right_source_channel = track.left_source_channel;
    }
    for route in &mut model.routes {
        if let RouteSource::Track { tap, .. } = &mut route.source
            && *tap == SendTap::PostFader
        {
            *tap = SendTap::InsertSend;
        }
    }
    model
}

/// A block by which C1's silent strips are packed and their chains skip: a fresh gate lane at its
/// defaults reports rest only after about 348 blocks of silence (its release from the open state;
/// the chain holds the lane until then), then [`PACK_IDLE_BLOCKS`](graph) = 16 more blocks, then
/// one pack step. Measured at eight lanes: the track group first runs its ideal at block 363.
const C1_PACKED: usize = 400;

/// #2132's `Sparse` input: every odd track exact `+0.0`, every even track playing, for `blocks`
/// blocks (one schedule per [`TRACKS`] slot; C1 reads the first 16).
fn c1_sparse(blocks: usize) -> Vec<Arc<[Block]>> {
    c1_input(blocks, |track| track % 2 == 1)
}

/// Every track for which `silent` holds exact `+0.0`, every other track playing, for `blocks`
/// blocks.
fn c1_input(blocks: usize, silent: fn(usize) -> bool) -> Vec<Arc<[Block]>> {
    (0..TRACKS)
        .map(|track| {
            let kind = if silent(track) {
                Block::Silence
            } else {
                Block::Signal(0.05)
            };
            vec![kind; blocks].into()
        })
        .collect()
}

/// The pack group that holds the track banks, if any, and every bank unit that renders a track.
fn track_group(plan: &PreparedRenderPlan) -> (Option<graph::TestOnlyPackGroup>, BTreeSet<u32>) {
    let track_units: BTreeSet<u32> = plan
        .unit_eligibility()
        .iter()
        .filter(|row| row.banked && row.lane_tracks.iter().any(|track| track.starts_with("ch")))
        .map(|row| row.unit)
        .collect();
    let group = graph::test_only_pack_groups(plan)
        .into_iter()
        .find(|group| {
            group
                .chains
                .iter()
                .flatten()
                .any(|unit| track_units.contains(unit))
        });
    (group, track_units)
}

/// Issue #2142 gate 1: on C1 at 16 tracks, every track's two units are in one pack group, two
/// units per lane chain and `ceil(16 / W)` chains; and on the `Sparse` state the track group's
/// bank-slot runs fall from every chain's (`slots x ceil(16 / W)` per block) to the per-key-class
/// ideal `slots x ceil(8 / W)` (one bank per slot at eight lanes, two at four) once the silent strips
/// rest, are packable ([`PACK_IDLE_BLOCKS`](graph) = 16 blocks later) and one pack step has run
/// ([`C1_PACKED`]).
///
/// Test value: red when a send keeps a console strip out of packing -- a producer row or a resident
/// pairing treated as a reason to leave both units ungrouped (#1981's `pack_member_eligible`), so
/// no C1 track bank packs -- which no other test reaches: #1981's rows use sessions without sends.
#[test]
fn a_split_strip_packs_as_one_strip() {
    let model = c1_all_active();
    let tracks = model.tracks.len();
    let blocks = C1_PACKED + 16;
    let mut bound = bind_with(
        &model,
        &c1_sparse(blocks),
        false,
        Setup {
            stereo: |_| true,
            live: false,
        },
    );
    let (group, track_units) = track_group(&bound.plan);
    let group = group.expect("the track banks are in a pack group");
    let lanes = group.width as usize;
    assert_eq!(
        group.chains.len(),
        tracks.div_ceil(lanes),
        "ceil(16 / W) lane chains: {group:?}"
    );
    assert!(
        group.chains.iter().all(|chain| chain.len() == 2),
        "two units per chain, the strip split at the fader: {group:?}"
    );
    let grouped: BTreeSet<u32> = group.chains.iter().flatten().copied().collect();
    assert_eq!(grouped, track_units, "every track unit is in the group");
    let first = *grouped.first().expect("a grouped unit");
    let runs = |plan: &mut PreparedRenderPlan| {
        let row = graph::test_only_bank_slot_runs(plan)
            .into_iter()
            .find(|row| row.first_unit == first)
            .expect("the group's bank-slot row");
        (row.runs, u64::from(row.effect_slots))
    };
    let mut pcm = vec![0.0_f32; bound.frames * 2];
    let mut per_block = Vec::with_capacity(blocks);
    let mut slots = 0;
    for block in 0..blocks as u64 {
        let (before, _) = runs(&mut bound.plan);
        bound.render(block, true, &mut pcm);
        let (after, effect_slots) = runs(&mut bound.plan);
        slots = effect_slots;
        per_block.push(after - before);
    }
    let every_chain = slots * tracks.div_ceil(lanes) as u64;
    let ideal = slots * (tracks / 2).div_ceil(lanes) as u64;
    assert_eq!(
        per_block[1], every_chain,
        "before the silent strips are packable every chain renders: {per_block:?}"
    );
    assert!(
        per_block[C1_PACKED..].iter().all(|&runs| runs == ideal),
        "packed, the group runs the per-key-class ideal {ideal} per block: {per_block:?}"
    );
    assert!(bound.lane_moves() > 0, "strips moved");
}

/// `ch10`: a playing strip of the second chain at both widths, which the packer moves into a lower
/// chain (with `ch08`, `ch12` and `ch14`) once the odd strips are packable.
const SPLIT_MOVED: &str = "ch10";

/// The block before which [`SPLIT_MOVED`]'s right-channel trim moves live: after the packer moved
/// it, so the chain that renders it then is not its home chain.
const SPLIT_TRIM: u64 = C1_PACKED as u64;

/// Write [`SPLIT_MOVED`]'s right-channel trim to -6 dB through its live input lane: from then on its
/// two channels differ, while every other strip's agree.
fn trim_moved_strip(bound: &mut Bound) {
    let Bound { controls, gate, .. } = bound;
    let input = controls
        .iter_mut()
        .find(|channel| &*channel.track_id == SPLIT_MOVED)
        .and_then(|channel| channel.input.as_mut())
        .expect("the moved strip's input lane");
    gate.as_mut()
        .expect("the plan's gate")
        .write_published(|stamp| {
            input.write(
                stamp,
                TrackInputRecord::TrimDb {
                    lanes: BuiltinLaneSelector::Right,
                    db: -6.0,
                    smoothing_samples: 64,
                },
            );
        });
}

/// Render `model` twice over `input` ([`c1_input`]), packing and not packing, with every track
/// collapse-armed and [`SPLIT_MOVED`]'s right trim moved live before [`SPLIT_TRIM`]; every block's
/// host words and every track's post-matrix words must agree by `to_bits`, NaNs folded. `moves`:
/// whether the packer must have moved strips by then, if the caller knows. Returns the packing
/// plan, rendered, for the caller's own checks.
fn split_strip_differential(
    model: &SessionModel,
    silent: fn(usize) -> bool,
    moves: Option<bool>,
) -> Bound {
    let blocks = C1_PACKED + 32;
    let schedules = c1_input(blocks, silent);
    let setup = Setup {
        stereo: |_| false,
        live: true,
    };
    let mut packed = bind_with(model, &schedules, true, setup);
    let mut unpacked = bind_with(model, &schedules, true, setup);
    unpacked.pack = false;
    let frames = packed.frames;
    let mut packed_pcm = vec![0.0_f32; frames * 2];
    let mut unpacked_pcm = vec![0.0_f32; frames * 2];
    for block in 0..blocks as u64 {
        if block == SPLIT_TRIM {
            if let Some(moves) = moves {
                assert_eq!(
                    packed.displaced() > 0,
                    moves,
                    "whether the packer moved strips before the trim"
                );
            }
            trim_moved_strip(&mut packed);
            trim_moved_strip(&mut unpacked);
        }
        packed.render(block, true, &mut packed_pcm);
        unpacked.render(block, true, &mut unpacked_pcm);
        for (index, (a, b)) in packed_pcm.iter().zip(&unpacked_pcm).enumerate() {
            assert_eq!(
                fold_nan(a.to_bits()),
                fold_nan(b.to_bits()),
                "block {block}: host word {index} differs ({a:e} packed, {b:e} unpacked)"
            );
        }
    }
    let moved = model
        .tracks
        .iter()
        .position(|track| track.id.as_str() == SPLIT_MOVED)
        .expect("the moved strip");
    for (track, (a, b)) in packed.sinks.iter().zip(&unpacked.sinks).enumerate() {
        let a = a.lock().expect("capture");
        let b = b.lock().expect("capture");
        assert_eq!(a.len(), b.len(), "ch{track:02}: observed lengths");
        for (index, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            assert_eq!(
                fold_nan(*x),
                fold_nan(*y),
                "ch{track:02}: post-matrix word {index} differs"
            );
        }
        if track == moved && moves == Some(true) {
            let block = 2 * frames;
            let last = &a[a.len() - block..];
            assert_ne!(
                last[..frames],
                last[frames..],
                "the moved strip's two channels differ after its trim"
            );
        }
    }
    for bound in [&packed, &unpacked] {
        assert!(
            bound.plan.bank_collapse_counters()[0] > 0,
            "the plan collapses: the armed rows are live"
        );
    }
    packed
}

/// Issue #2142 gate 2: a packed plan renders the bits of its unpacked twin when the moved strip is
/// split, collapse-armed, and its two channels come to differ after the move. The strip split at
/// the insert send has a part two whose collapse prefix is not empty, so part two collapses on a
/// block whose producer row says part one rendered identical planes: a row that asks the wrong
/// unit would collapse the moved strip's differing channels.
///
/// Test value: red when a pack moves a split strip's bits or collapses channels that differ: a
/// move that trades part one only (part two's row then asks a chain of silent strips, which skips,
/// and collapses the moved strip's differing planes) or part two only (it asks the unmoved strip's
/// first chain, which collapses). Trading every unit needs no row translation, which is the rule.
#[test]
fn a_packed_split_strip_renders_the_bits_of_an_unpacked_one() {
    let model = c1_collapse_armed();
    let mut probe = bind_with(
        &model,
        &c1_sparse(1),
        false,
        Setup {
            stereo: |_| false,
            live: true,
        },
    );
    let (group, _) = track_group(&probe.plan);
    let group = group.expect("the track banks are in a pack group");
    let rows = probe.plan.unit_eligibility();
    for chain in &group.chains {
        assert_eq!(
            chain.len(),
            2,
            "the strip splits at the insert send: {group:?}"
        );
        let later = rows
            .iter()
            .find(|row| row.unit == chain[1])
            .expect("part two's row");
        assert!(
            later.armed_on_input,
            "part two collapses on its producer row's proof"
        );
    }
    // Both rows hold effect slots here (part two has the drive): each row's units count that row's
    // slots, so the rows' counts sum to the banks the compile bound (issue #2132's relation).
    let counted: u64 = graph::test_only_bank_slot_runs(&mut probe.plan)
        .iter()
        .map(|row| u64::from(row.effect_slots) * u64::from(row.last_unit - row.first_unit + 1))
        .sum();
    assert_eq!(
        counted, probe.bound_banks,
        "every bank-slot row counts its own row's slots once per unit"
    );
    drop(probe);
    let packed = split_strip_differential(&model, |track| track % 2 == 1, Some(true));
    assert!(packed.lane_moves() > 0, "strips moved");
}

/// Issue #2142 gate 3: a per-node insert ends a lane chain on both sides. Every track of the
/// collapse-armed C1 carries a per-node delay insert between its pre-insert and post-insert slots:
/// its pre bank packs, and each delay op's input term (#1899), which a later bank's collapse
/// reads, must ask the unit that renders its strip **now**; the bank after the delays, whose row
/// names the ops, stays in no group. The moved strip's trim makes its pre bank's planes differ
/// after the move, and the packed plan must still render its unpacked twin's bits.
///
/// Test value: red when a per-node insert loses or misreads its collapse proof under packing: an
/// op row read at the strip's home unit asks a chain of silent strips that skips, keeps the delay's
/// agreement, and the bank after it collapses the moved strip's differing planes.
#[test]
fn a_per_node_insert_reads_the_moved_strip() {
    let mut model = c1_collapse_armed();
    let delay = model
        .submixes
        .iter()
        .find(|submix| submix.id.as_str() == "aux.a")
        .and_then(|submix| submix.inserts.effects.first())
        .cloned()
        .map(|mut delay| {
            // A 1 ms line, so the trimmed words reach the post-matrix tap within the run.
            delay.params.push(EffectParam {
                parameter_id: 1,
                channel: ParameterChannel::Both,
                unit: ParameterUnit::Milliseconds,
                value: 1.0,
            });
            delay
        })
        .expect("aux A's delay insert");
    for track in &mut model.tracks {
        track.inserts.effects.push(delay.clone());
    }
    let probe = bind_with(
        &model,
        &c1_sparse(1),
        false,
        Setup {
            stereo: |_| false,
            live: true,
        },
    );
    let (group, track_units) = track_group(&probe.plan);
    let group = group.expect("the pre banks are in a pack group");
    assert!(
        group.chains.iter().all(|chain| chain.len() == 1),
        "the delay ends each pre bank's chain: {group:?}"
    );
    let grouped: BTreeSet<u32> = group.chains.iter().flatten().copied().collect();
    let pack_groups = graph::test_only_pack_groups(&probe.plan);
    for unit in track_units.difference(&grouped) {
        assert!(
            pack_groups.iter().all(|group| !group
                .chains
                .iter()
                .flatten()
                .any(|other| other == unit)),
            "unit {unit}, after the delay ops, is in no group"
        );
    }
    drop(probe);
    let packed = split_strip_differential(&model, |track| track % 2 == 1, Some(true));
    assert!(packed.lane_moves() > 0, "strips moved");
}

/// Issue #2142 gate 1a, the layout term: C1, collapse-armed, with its sends tapped after the input
/// section. Part one is then the input section alone, and the second chain's part one writes its
/// outputs into the buffers the first chain's part one gathered its inputs from: the colouring
/// recycles them, since the first chain has read them by then (at eight lanes; the four-lane plan's
/// colouring is its own). A move would have a strip written by an earlier unit overwrite an input a
/// later unit still gathers, so such chains form no group, and the packing plan renders its
/// unpacked twin's bits whatever the width.
///
/// Test value: red when a group forms over a buffer that holds two values in its span (the
/// "inputs are never written" term of `group_layout_holds` dropped): a moved strip's part one
/// overwrites a bank mate's input and the packed plan's bits move, which none of the other rows
/// reaches, because the colouring recycles no group buffer there.
#[test]
fn a_recycled_buffer_keeps_its_strips_apart() {
    let mut model = c1_collapse_armed();
    for route in &mut model.routes {
        if let RouteSource::Track { tap, .. } = &mut route.source
            && *tap == SendTap::InsertSend
        {
            *tap = SendTap::PostInput;
        }
    }
    // The first chain's odd strips silent, every other strip playing: at eight lanes the second
    // chain's strips move into the first chain's silent lanes, and a strip moved into the first
    // chain writes a buffer a displaced strip still gathers from.
    split_strip_differential(&model, |track| track < 8 && track % 2 == 1, None);
}

/// Issue #2142 gate 4: on C1 at 16 tracks, a move of a split strip (both units of both lane chains)
/// allocates and frees nothing on the render thread, made by hand or by the packer, and neither
/// does the carry out of a displaced split-strip group (`restore_home_lanes` walks the chains) or
/// any block between.
///
/// Test value: red when a chain swap, the walk home over lane chains or a displaced split group's
/// render allocates, which the eight-track desk's allocation gate (#1981) cannot reach: its strips
/// are not split.
#[test]
fn a_split_strip_move_allocates_nothing() {
    let model = c1_all_active();
    let blocks = 48;
    let setup = Setup {
        stereo: |_| true,
        live: false,
    };
    let Pair {
        mut before,
        mut after,
    } = carried_pair_with(&model, &c1_sparse(blocks), setup);
    let (group, _) = track_group(&before.plan);
    let group = group.expect("the track banks are in a pack group");
    assert!(group.chains.iter().all(|chain| chain.len() == 2));
    // Two strips of the second chain trade with two of the first, named by their part-one units.
    let (low, high) = (group.chains[0][0] as usize, group.chains[1][0] as usize);
    let moves = [((low, 1), (high, 0)), ((low, 3), (high, 2))];
    let mut pcm = vec![0.0_f32; before.frames * 2];
    assert_installed();
    let swap = 24;
    before.pack = false;
    for block in 0..blocks as u64 {
        let bound = if block < swap {
            &mut before
        } else {
            &mut after
        };
        let mark = current_thread_counters();
        if block == 10 || block == swap + 4 {
            for (a, b) in moves {
                assert!(
                    graph::test_only_swap_bank_lanes(&mut bound.plan, a, b),
                    "block {block}: {a:?} and {b:?} trade"
                );
            }
        }
        if block == swap {
            assert!(
                before.displaced() > 0,
                "the predecessor is displaced at the swap"
            );
            assert_eq!(
                after.plan.adopt_predecessor_plan(&mut before.plan),
                engine::realtime::CarryOutcome::Carried,
                "the successor takes the carry"
            );
        }
        let bound = if block < swap {
            &mut before
        } else {
            &mut after
        };
        bound.render(block, true, &mut pcm);
        let delta = current_thread_delta_since(mark);
        if block >= 10 {
            assert_eq!(
                (delta.allocations, delta.reallocations, delta.deallocations),
                (0, 0, 0),
                "block {block}: a split strip's move, carry or displaced block allocated or freed"
            );
        }
    }
    assert!(
        after.lane_moves() > 0,
        "the successor's packer moved the displaced strips home"
    );
    assert_eq!(after.displaced(), 0, "the successor is home again");
}

/// Issue #2142: the packer's fact path writes nothing for a first-row unit whose lanes are all
/// unquiet and whose idle counts are all zero (the load state's every block). A strip that plays
/// again after it was packed must still reset its count, in two cases on C1 at 16 tracks:
///
/// * `Sparse` packs by [`C1_PACKED`]; then every strip plays, every strip is running, and the
///   packer takes the group home within a few blocks;
/// * only `ch01` is silent, so the packer trades it with a playing strip of the second chain, a
///   unit whose every strip played until then; `ch01` plays from the very block of that move. The
///   unit that now renders it must count its idle down, and the group goes home.
///
/// Test value: red when the write-nothing path hides a count that must fall to zero -- taken on
/// every unquiet block, or (the second case) taken by a unit a move just filled with a strip whose
/// count is not zero -- so the strip that plays again stays packable, the displaced strips never
/// trade home, and the group pays the displaced fold and a lost skip with no bit to show it.
#[test]
fn a_strip_that_plays_again_is_not_packable() {
    let model = c1_all_active();
    let setup = Setup {
        stereo: |_| true,
        live: false,
    };
    let schedules = |blocks: usize, silent: &dyn Fn(usize, usize) -> bool| -> Vec<Arc<[Block]>> {
        (0..TRACKS)
            .map(|track| {
                (0..blocks)
                    .map(|block| {
                        if silent(track, block) {
                            Block::Silence
                        } else {
                            Block::Signal(0.05)
                        }
                    })
                    .collect::<Vec<_>>()
                    .into()
            })
            .collect()
    };
    // Case 1: every strip plays again.
    let blocks = C1_PACKED + 24;
    let mut bound = bind_with(
        &model,
        &schedules(blocks, &|track, block| track % 2 == 1 && block < C1_PACKED),
        false,
        setup,
    );
    let mut pcm = vec![0.0_f32; bound.frames * 2];
    for block in 0..blocks as u64 {
        if block == C1_PACKED as u64 {
            assert!(
                bound.displaced() > 0,
                "the group is packed before the strips play"
            );
        }
        bound.render(block, true, &mut pcm);
    }
    assert_eq!(
        bound.displaced(),
        0,
        "every strip plays again, so the packer took the group home"
    );
    // Case 2: find the block whose top moves `ch01`, then play it from that block on.
    let blocks = C1_PACKED + 24;
    let silent_alone = |track: usize, _: usize| track == 1;
    let mut probe = bind_with(&model, &schedules(blocks, &silent_alone), false, setup);
    let mut moved = None;
    for block in 0..blocks as u64 {
        probe.render(block, true, &mut pcm);
        if probe.lane_moves() > 0 {
            moved = Some(block as usize);
            break;
        }
    }
    let moved = moved.expect("the packer moves ch01");
    let mut bound = bind_with(
        &model,
        &schedules(blocks, &|track, block| track == 1 && block < moved),
        false,
        setup,
    );
    for block in 0..blocks as u64 {
        bound.render(block, true, &mut pcm);
        if block == moved as u64 {
            assert!(
                bound.displaced() > 0,
                "ch01 moved at the top of block {moved}"
            );
        }
    }
    assert_eq!(
        bound.displaced(),
        0,
        "ch01 plays from the block it moved, so the packer took it home"
    );
}
