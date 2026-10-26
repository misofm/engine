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
        pack: true,
        sinks,
        bound_banks,
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
/// chain renders runs exactly the effect banks the compile bound.
///
/// Test value: red when the counter and the skips disagree -- a run counted for a skipped chain,
/// or a builtin stage counted as an effect slot -- which the skip differential (gate 1) cannot see,
/// because a counter moves no output bit.
#[test]
fn bank_slot_runs_follow_the_skips() {
    let blocks = TAIL + 8;
    let schedules = schedules(blocks, 0x2132);
    let mut bound = bind(&session(first_sixteen), &schedules, false);
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
                u64::from(after.effect_slots) * (banks - skipped),
                "block {block}, units {}..={}: {skipped} of {banks} chains skipped",
                after.first_unit,
                after.last_unit
            );
            a_chain_skipped += u32::from(skipped > 0);
            total += runs;
        }
        every_chain_ran += u32::from(total == bound.bound_banks);
        previous = now;
    }
    assert!(
        every_chain_ran > 0 && a_chain_skipped > 0,
        "the schedule reaches both an all-running block ({every_chain_ran}) and a skipped chain \
         ({a_chain_skipped})"
    );
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
    let (first, last, lanes) = groups[0];
    let span = u64::from(last - first + 1) * u64::from(lanes);
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
/// group `graph::test_only_pack_groups` reports is `(first unit, last unit, lanes per unit)`, and
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
        for (index, &(first, last, _)) in groups.iter().enumerate() {
            for unit in first as usize..=last as usize {
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
