//! Stored mute automation on the strip (#1651 T3a): a mute cell renders its steps on the fader
//! stage at their exact samples over the session mute length, through the one event driver, on
//! every bank width and on the scalar oracle (R17-4); a seek that leaves the stage mute unchanged
//! lets a ramp toward that state finish (root ruling R18-1); and a mute value the strip's live cell
//! carries is the lane's `terms`, which render ORs with the curve (OQ1 = C). On a lane with fader
//! and mute automation, each mute event changes where the stage holds the gain, inside the block
//! (T3b): fader events inside the hold are only remembered, and the move the hold owes lands at
//! the unmute ramp's end `E` (#1647 D1.6).
//!
//! Every test prepares on a chosen backend, a `test-support` entry of host-core.
#![cfg(feature = "test-support")]

use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{HostPrepareCaps, HostShapePolicy, PreparedHost, SourceSubmission};
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, ControlSmoothing, Owner,
    ParameterChannel, ParameterUnit, Section, SessionModel, StableId, canonical_session_json,
    parse_session_json,
};

const NINE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const NINE_SOURCE: &[u8] = b"fixture-source";
const RATE: u32 = 48_000;
/// The mute row (builtin parameter 6).
const MUTE: u32 = 6;
/// The fader row (builtin parameter 5).
const FADER_DB: u32 = 5;
/// The session mute length of every test here: `mute_ms` 10 at 48 kHz.
const M: u32 = 480;

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn caps(model: &SessionModel) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 32 * model.quantum_frames,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

/// The first `tracks` tracks of the nine-track fixture at quantum `quantum`: no console, no
/// insert, no input filter, an identity matrix, each routed to the output, `control_smoothing`
/// explicit (fader 5 ms, mute 10 ms). On a DC source of `1.0` a track's output is its fader gain.
fn plain(tracks: usize, quantum: u32) -> SessionModel {
    let model = plain_with(tracks, quantum, 10.0);
    assert_eq!(controls::LiveRamps::for_session(&model).mute_samples, M);
    model
}

/// [`plain`] with the session mute length `mute_ms`.
fn plain_with(tracks: usize, quantum: u32, mute_ms: f32) -> SessionModel {
    let mut model = parse_session_json(NINE).expect("fixture parses");
    model.automation.clear();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.truncate(tracks);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| match &route.source {
        session::RouteSource::Track { track_id, .. } => kept.contains(track_id),
        _ => false,
    });
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
        track.matrix_or_pan = session::MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples: 0,
        };
    }
    model.sources[0].frames = 1 << 20;
    model.quantum_frames = quantum;
    model.control_smoothing = Some(ControlSmoothing {
        mute_ms,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    model
}

/// A mute entry on `lane` of track `track` with steps at each `(sample, value)` of `steps`, from
/// `from` (`0` unmuted, `1` muted).
fn mute_entry(
    model: &SessionModel,
    track: usize,
    lane: ParameterChannel,
    from: f32,
    steps: &[(u64, f32)],
) -> Automation {
    let mut at = (0_u64, from);
    let mut segments: Vec<AutomationSegment> = steps
        .iter()
        .map(|&(sample, value)| {
            let segment = AutomationSegment {
                shape: AutomationShape::Step,
                start_sample: at.0,
                end_sample: sample,
                start_value: at.1,
                end_value: value,
                unit: ParameterUnit::Linear,
            };
            at = (sample, value);
            segment
        })
        .collect();
    if segments.is_empty() {
        segments.push(AutomationSegment {
            shape: AutomationShape::Step,
            start_sample: 0,
            end_sample: 1,
            start_value: from,
            end_value: from,
            unit: ParameterUnit::Linear,
        });
    }
    Automation {
        id: id(&format!("mute-{track}")),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[track].id.clone()),
            section: Section::Builtins,
            control: MUTE,
            lane,
        },
        segments,
        offset: session::AutomationOffset::default(),
    }
}

/// A plan of `model` rendered block by block on a DC source of `1.0`.
struct Rig {
    prepared: PreparedHost,
    handles: Option<host_core::HostLiveControlHandles>,
    gate: Option<engine::realtime::PlanRevisionGate>,
    quantum: usize,
    generation: u64,
    frame: u64,
    rendered: u64,
    out: [Vec<f32>; 2],
    /// The render calls' allocations, reallocations and frees.
    allocations: (u64, u64, u64),
}

/// How a [`Rig`] is prepared: a backend, and live controls or none.
#[derive(Clone, Copy, Debug)]
struct Form {
    backend: graph_compiler::Backend,
    live: bool,
}

/// Every form of the target: Simd8 (on an AVX2 target only), Simd4 and the scalar oracle, each
/// with no live control and with live controls.
fn forms() -> Vec<Form> {
    let backends = [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
        graph_compiler::Backend::Scalar,
    ];
    let mut forms = Vec::new();
    for backend in backends {
        for live in [false, true] {
            forms.push(Form { backend, live });
        }
    }
    forms
}

impl Rig {
    fn new(model: &SessionModel, form: Form) -> Self {
        let document = canonical_session_json(model).expect("canonical");
        let caps = caps(model);
        let compiled = host_core::compile_host_session(&document, &caps)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let request = host_core::HostLiveControlRequest::default();
        let (mut prepared, handles) = if !form.live {
            (
                host_core::test_only_prepare_host_runtime_on(&compiled, &caps, form.backend)
                    .unwrap_or_else(|failure| {
                        panic!("{}", String::from_utf8_lossy(failure.as_bytes()))
                    }),
                None,
            )
        } else {
            let (prepared, handles) =
                host_core::test_only_prepare_host_runtime_with_live_controls_on(
                    &compiled,
                    &caps,
                    &request,
                    form.backend,
                )
                .unwrap_or_else(|failure| {
                    panic!("{}", String::from_utf8_lossy(failure.as_bytes()))
                });
            (prepared, Some(handles))
        };
        let gate = if handles.is_some() {
            prepared.plan.take_revision_gate()
        } else {
            None
        };
        Self {
            prepared,
            handles,
            gate,
            quantum: model.quantum_frames as usize,
            generation: 1,
            frame: 0,
            rendered: 0,
            out: [Vec::new(), Vec::new()],
            allocations: (0, 0, 0),
        }
    }

    /// A plain session seek to timeline sample `timeline` (#1641), with its runway submitted at
    /// once (#2205), so it lands at the next block; `block` keeps the runway queued ahead.
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.prepared
            .sources
            .seek_session(self.generation, timeline, None)
            .expect("the seek is admitted");
        self.frame = timeline;
        let quantum = self.quantum as u32;
        for _ in 0..host_core::source_seek_runway_blocks(RATE, quantum, 32 * quantum) {
            self.submit_one();
        }
    }

    /// Submit the source's next block of DC `1.0`.
    fn submit_one(&mut self) {
        let quantum = self.quantum;
        let plane = vec![1.0_f32; quantum];
        self.prepared
            .sources
            .submit(
                NINE_SOURCE,
                SourceSubmission {
                    generation: self.generation,
                    start_frame: self.frame,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        self.frame += quantum as u64;
    }

    /// Write a live `Mute` record on track 0's both lanes over `window`, applied at the next
    /// block entry: under OQ1 = C the shared commit's record of the effective mute, `terms`.
    fn live_mute(&mut self, muted: bool, window: u32) {
        use builtins_compiler::TestOnlyWriteGate;
        let handles = self.handles.as_mut().expect("live controls");
        self.gate
            .as_mut()
            .expect("the plan's gate")
            .write_published(|stamp| {
                handles.strip_controls[0].fader.write(
                    stamp,
                    builtins_compiler::TrackFaderRecord::Mute {
                        lanes: builtins::BuiltinLaneSelector::Both,
                        muted,
                        smoothing_samples: window,
                    },
                );
            });
    }

    /// Write a live offsets record `S = offsets_sum` (ramp `window`) on both channels of track 0,
    /// applied at the next block entry: the shared commit's record of a VCA fader move.
    fn live_offsets(&mut self, offsets_sum: f64, window: u32) {
        use builtins_compiler::TestOnlyWriteGate;
        let handles = self.handles.as_mut().expect("live controls");
        self.gate
            .as_mut()
            .expect("the plan's gate")
            .write_published(|stamp| {
                for channel in 0..2 {
                    handles.strip_controls[0].fader.write_offsets(
                        stamp,
                        channel,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum,
                            smoothing_samples: window,
                            own_db: None,
                        },
                    );
                }
            });
    }

    /// Render one block; returns its first render sample.
    fn block(&mut self) -> u64 {
        let quantum = self.quantum;
        self.submit_one();
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let first = self.rendered;
        let mark = bench_support::alloc::current_thread_counters();
        self.prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: first,
                },
            )
            .expect("render");
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        self.allocations.0 += delta.allocations;
        self.allocations.1 += delta.reallocations;
        self.allocations.2 += delta.deallocations;
        self.out[0].extend_from_slice(&samples[..quantum]);
        self.out[1].extend_from_slice(&samples[quantum..]);
        self.rendered += quantum as u64;
        first
    }

    /// Render until at least `frames` samples are out; the output cut to `frames`.
    fn run(mut self, frames: u64) -> ([Vec<f32>; 2], Self) {
        while self.rendered < frames {
            self.block();
        }
        let out = self.out.clone().map(|mut plane| {
            plane.truncate(frames as usize);
            plane
        });
        (out, self)
    }
}

/// The stage driven directly: a width-one ramped fader at `0` dB, unmuted, given each
/// `(sample, operation)` of `operations` at its sample (#1638), over `frames` samples of DC `1.0`.
fn reference(operations: &[(u64, builtins::FaderOperation)], frames: u64) -> Vec<f32> {
    let mut stage = builtins::FaderMuteRampBuiltins::new(builtins::BuiltinParameters::default())
        .expect("default parameters");
    let mut out = Vec::with_capacity(frames as usize);
    let mut next = operations.iter().peekable();
    let mut at = 0_u64;
    while at < frames {
        while let Some(&&(sample, operation)) = next.peek()
            && sample == at
        {
            stage
                .apply_fader_operation(builtins::BuiltinLaneSelector::Both, operation)
                .expect("operation");
            next.next();
        }
        let end = next
            .peek()
            .map_or(frames, |&&(sample, _)| sample.min(frames));
        let length = (end - at) as usize;
        let mut left = vec![1.0_f32; length];
        let mut right = vec![1.0_f32; length];
        stage.process(builtins::DualMonoBlock::new(&mut left, &mut right, at).expect("block"));
        out.extend_from_slice(&left);
        at = end;
    }
    out
}

const fn set_mute(muted: bool, samples: u32) -> builtins::FaderOperation {
    builtins::FaderOperation::SetMute { muted, samples }
}

/// The first sample index at which `a` and `b` differ by bits.
fn first_difference(a: &[f32], b: &[f32]) -> Option<usize> {
    a.iter()
        .zip(b)
        .position(|(a, b)| a.to_bits() != b.to_bits())
}

/// The mute events the driver pushed since the last take, as `(channel, sample, muted, ramp)`.
fn mute_events() -> Vec<(usize, u64, bool, u32)> {
    let (events, overflow) = builtins_compiler::test_only_take_fader_events();
    assert!(!overflow);
    events
        .iter()
        .filter(|event| event.mute)
        .map(|event| (event.channel, event.sample, event.value != 0.0, event.ramp))
        .collect()
}

/// #1651 gate 1 (widths, T3a): a mute entry on `both` of track 0 with steps 0, 1, 0 at samples
/// 1,000 and 2,500 (not grid samples). On every form (Simd8 on AVX2, Simd4 and the scalar oracle;
/// with no live control and with live controls) and at quanta 128, 100 and 37, the driver pushes
/// `SetMute` at exactly those samples over the session mute length `M`, and the output is bit for
/// bit the stage driven directly with those two operations: each ramp starts at its step sample,
/// the lane is the exact `+0.0` of a static mute from the end of the mute ramp, and back at the
/// fader from the end of the unmute ramp. Render allocates nothing.
///
/// Test value: red if a mute step snaps to the grid or a block boundary, ramps over another row's
/// length (the fader's 240), fires twice or not at all, depends on the quantum or the form, or if
/// the scalar oracle skips the mute cells.
#[test]
fn a_mute_step_mutes_the_lane_at_its_exact_sample_over_the_mute_length() {
    const FRAMES: u64 = 4_096;
    let expected = reference(
        &[(1_000, set_mute(true, M)), (2_500, set_mute(false, M))],
        FRAMES,
    );
    assert_eq!(expected[999].to_bits(), 1.0_f32.to_bits());
    assert_eq!(
        expected[1_000 + M as usize - 1].to_bits(),
        0.0_f32.to_bits()
    );
    assert_eq!(
        expected[2_500 + M as usize - 1].to_bits(),
        1.0_f32.to_bits()
    );
    for quantum in [128, 100, 37] {
        let mut model = plain(1, quantum);
        model.automation = vec![mute_entry(
            &model,
            0,
            ParameterChannel::Both,
            0.0,
            &[(1_000, 1.0), (2_500, 0.0)],
        )];
        for form in forms() {
            let _ = builtins_compiler::test_only_take_fader_events();
            let (out, rig) = Rig::new(&model, form).run(FRAMES);
            let label = format!("q={quantum} {form:?}");
            assert_eq!(
                mute_events(),
                [
                    (0, 1_000, true, M),
                    (1, 1_000, true, M),
                    (0, 2_500, false, M),
                    (1, 2_500, false, M)
                ],
                "{label}"
            );
            for plane in &out {
                assert_eq!(first_difference(plane, &expected), None, "{label}");
            }
            drop(rig);
        }
    }
    // Render is allocation-free after a warm-up, with a mute event in the measured block.
    let mut model = plain(1, 128);
    model.automation = vec![mute_entry(
        &model,
        0,
        ParameterChannel::Both,
        0.0,
        &[(1_000, 1.0), (2_500, 0.0)],
    )];
    bench_support::alloc::assert_installed();
    let mut rig = Rig::new(&model, forms()[0]);
    // The first block warms the process statics.
    rig.block();
    rig.allocations = (0, 0, 0);
    while rig.rendered < FRAMES {
        rig.block();
    }
    assert_eq!(rig.allocations, (0, 0, 0), "mute events allocate nothing");
}

/// #1651 D1, a lane whose mute starts muted: a curve that starts at `1` prepares the stage muted
/// (the hold rule: the curve before its first segment is its first `start_value`), so the lane is
/// the exact `+0.0` from sample 0 and its first block emits nothing; the unmute step at 777 ramps
/// over `M`. A curve on the left lane only leaves the right lane at the fader.
///
/// Test value: red if preparation bakes the static mute instead of `curve || terms`, if a fresh
/// lane's first block lands a `Set` it does not need, or if a `left` entry reaches the right lane.
#[test]
fn a_curve_that_starts_muted_prepares_the_lane_muted() {
    const FRAMES: u64 = 2_048;
    let muted_start = {
        let mut stage =
            builtins::FaderMuteRampBuiltins::new(builtins::BuiltinParameters::default())
                .expect("default parameters");
        stage.set_mute(builtins::BuiltinLaneSelector::Both, true, 0);
        let mut left = vec![1.0_f32; 777];
        let mut right = vec![1.0_f32; 777];
        stage.process(builtins::DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        assert!(left.iter().all(|sample| sample.to_bits() == 0));
        stage
            .apply_fader_operation(builtins::BuiltinLaneSelector::Both, set_mute(false, M))
            .expect("unmute");
        let mut tail_l = vec![1.0_f32; (FRAMES - 777) as usize];
        let mut tail_r = tail_l.clone();
        stage.process(builtins::DualMonoBlock::new(&mut tail_l, &mut tail_r, 777).expect("tail"));
        left.extend_from_slice(&tail_l);
        left
    };
    let mut model = plain(1, 128);
    model.automation = vec![mute_entry(
        &model,
        0,
        ParameterChannel::Left,
        1.0,
        &[(777, 0.0)],
    )];
    for form in forms() {
        let _ = builtins_compiler::test_only_take_fader_events();
        let (out, _) = Rig::new(&model, form).run(FRAMES);
        assert_eq!(mute_events(), [(0, 777, false, M)], "{form:?}");
        assert_eq!(first_difference(&out[0], &muted_start), None, "{form:?}");
        assert!(
            out[1]
                .iter()
                .all(|sample| sample.to_bits() == 1.0_f32.to_bits()),
            "{form:?}: the right lane is not automated"
        );
    }
}

/// Root ruling R18-1 (#1647 constraint 5, the batch 13b rule): a session seek during an unmute
/// ramp to a timeline point where the curve is also unmuted lands a `Set` that changes nothing, so
/// the ramp finishes: the output is bit for bit the twin with no seek. A seek into the curve's
/// muted span instead sets the lane muted exactly (`n = 0`) from the seek's render sample.
///
/// Test value: red if a seek `Set` that repeats the stage's mute restarts, snaps or cuts the unmute
/// ramp in flight (R18-1), or if a seek into a muted span ramps or lands late.
#[test]
fn a_seek_that_leaves_the_mute_unchanged_lets_the_unmute_ramp_finish() {
    const FRAMES: u64 = 4_096;
    // Muted on [1,000, 2,000) of the timeline; the unmute ramp runs [2,000, 2,000 + M).
    let mut model = plain(1, 128);
    model.automation = vec![mute_entry(
        &model,
        0,
        ParameterChannel::Both,
        0.0,
        &[(1_000, 1.0), (2_000, 0.0)],
    )];
    // The seek is pushed before the block at render sample 2,176, inside the unmute ramp.
    let seek_at = 17 * 128;
    assert!(seek_at > 2_000 && seek_at < 2_000 + u64::from(M));
    for form in forms() {
        let (twin, _) = Rig::new(&model, form).run(FRAMES);
        let mut rig = Rig::new(&model, form);
        while rig.rendered < seek_at {
            rig.block();
        }
        let _ = builtins_compiler::test_only_take_fader_events();
        rig.seek(3_000);
        let (out, _) = rig.run(FRAMES);
        assert_eq!(mute_events(), [], "{form:?}: the seek changes nothing");
        for plane in 0..2 {
            assert_eq!(
                first_difference(&out[plane], &twin[plane]),
                None,
                "{form:?}: plane {plane}"
            );
        }
        assert_ne!(
            twin[0][seek_at as usize].to_bits(),
            1.0_f32.to_bits(),
            "the seek lands inside the ramp"
        );

        // The discriminating partner: a seek into the muted span.
        let mut rig = Rig::new(&model, form);
        while rig.rendered < seek_at {
            rig.block();
        }
        let _ = builtins_compiler::test_only_take_fader_events();
        rig.seek(1_500);
        let r = rig.block();
        assert_eq!(
            mute_events(),
            [(0, r, true, 0), (1, r, true, 0)],
            "{form:?}: the seek sets the lane muted exactly"
        );
        for plane in &rig.out {
            assert!(
                plane[r as usize..]
                    .iter()
                    .all(|sample| sample.to_bits() == 0),
                "{form:?}: muted exactly from the seek"
            );
        }
    }
}

/// #1651 D1, the drain branch (OQ1 = C): on a lane whose mute is automated, a `Mute` record of the
/// strip's live cell is the lane's `terms`, and render composes `curve || terms`. Track 0's curve
/// is unmuted until 2,000, muted on [2,000, 3,000) and unmuted after. A hand mute (`terms` on,
/// window 300) at the block entry 1,024 mutes the open curve there; the curve's mute step at 2,000
/// changes nothing (no event); a hand unmute (`terms` off) at 2,560 leaves the closed curve
/// closed; the curve's unmute step at 3,000 opens the lane over `M`. Bit for bit the stage driven
/// directly with `SetMute(true, 300)` at 1,024 and `SetMute(false, M)` at 3,000, on every live
/// form.
///
/// Test value: red if the drain hands an automated lane's mute value to the stage (the hand unmute
/// would open the closed curve at 2,560), if a curve event ignores `terms` (the step at 2,000 would
/// emit, the one at 3,000 would not compose), or if a terms write that changes nothing retargets.
#[test]
fn a_hand_mute_on_an_automated_lane_composes_with_the_curve() {
    const FRAMES: u64 = 4_096;
    let mut model = plain(1, 128);
    model.automation = vec![mute_entry(
        &model,
        0,
        ParameterChannel::Both,
        0.0,
        &[(2_000, 1.0), (3_000, 0.0)],
    )];
    let expected = reference(
        &[(1_024, set_mute(true, 300)), (3_000, set_mute(false, M))],
        FRAMES,
    );
    for form in forms().into_iter().filter(|form| form.live) {
        let _ = builtins_compiler::test_only_take_fader_events();
        let mut rig = Rig::new(&model, form);
        while rig.rendered < FRAMES {
            match rig.rendered {
                1_024 => rig.live_mute(true, 300),
                2_560 => rig.live_mute(false, 300),
                _ => {}
            }
            rig.block();
        }
        assert_eq!(
            mute_events(),
            [(0, 3_000, false, M), (1, 3_000, false, M)],
            "{form:?}: only the step that changes `curve || terms` emits"
        );
        for plane in &rig.out {
            assert_eq!(first_difference(plane, &expected), None, "{form:?}");
        }
    }
}

/// #1651 D1: the mute cells follow every fader cell in the plan's one program, so a fader ride
/// renders the same bits with or without a mute entry beside it: track 0's fader steps from `0` to
/// `-6` dB at 1,000 on its left lane, and a mute entry on track 1 (and on track 0's right lane)
/// changes no bit of track 0's left lane on any form.
///
/// Test value: red if a mute cell takes a fader cell's index or slot (the fader lane would read
/// the mute table), or if a lane's mute cells reach its fader channel.
#[test]
fn a_mute_entry_leaves_a_fader_ride_as_it_was() {
    const FRAMES: u64 = 2_048;
    let mut fader_only = plain(2, 128);
    fader_only.automation = vec![Automation {
        id: id("ride-0"),
        target: ControlAddress {
            owner: Owner::Strip(fader_only.tracks[0].id.clone()),
            section: Section::Builtins,
            control: FADER_DB,
            lane: ParameterChannel::Left,
        },
        segments: vec![
            AutomationSegment {
                shape: AutomationShape::Linear,
                start_sample: 0,
                end_sample: 700,
                start_value: -20.0,
                end_value: 0.0,
                unit: ParameterUnit::Db,
            },
            AutomationSegment {
                shape: AutomationShape::Step,
                start_sample: 700,
                end_sample: 1_000,
                start_value: 0.0,
                end_value: -6.0,
                unit: ParameterUnit::Db,
            },
        ],
        offset: session::AutomationOffset::default(),
    }];
    // The output sums the tracks, so track 1 is muted statically and its curve never opens it
    // (`terms` holds it); the mute entry on track 0's right lane moves only that lane.
    fader_only.tracks[1].fader.left_mute = true;
    fader_only.tracks[1].fader.right_mute = true;
    let mut with_mute = fader_only.clone();
    with_mute.automation.push(mute_entry(
        &with_mute,
        1,
        ParameterChannel::Both,
        0.0,
        &[(500, 1.0)],
    ));
    let mut right = mute_entry(&with_mute, 0, ParameterChannel::Right, 0.0, &[(900, 1.0)]);
    right.id = id("mute-0-right");
    with_mute.automation.push(right);
    for form in forms() {
        let (base, _) = Rig::new(&fader_only, form).run(FRAMES);
        let (out, _) = Rig::new(&with_mute, form).run(FRAMES);
        assert_eq!(first_difference(&out[0], &base[0]), None, "{form:?}: left");
        assert_ne!(
            base[0][0].to_bits(),
            base[0][1_500].to_bits(),
            "the ride moves"
        );
        assert_eq!(
            out[1][1_500].to_bits(),
            0,
            "{form:?}: the right lane's own mute"
        );
    }
}

/// The fader jump length `J` of every test here: `fader_ms` 5 at 48 kHz.
const J: u32 = 240;

/// The fader gain of `db` with no VCA (`S = +0.0`, #1646 D1).
fn gain(db: f32) -> f32 {
    builtins::checked_fader_gain(session::vca_compose_db(db, 0.0)).expect("fader domain")
}

/// A fader entry on both lanes of track `track`, from its `(shape, start, end, from dB, to dB)`
/// segments.
fn fader_entry(
    model: &SessionModel,
    track: usize,
    segments: &[(AutomationShape, u64, u64, f32, f32)],
) -> Automation {
    Automation {
        id: id(&format!("fader-{track}")),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[track].id.clone()),
            section: Section::Builtins,
            control: FADER_DB,
            lane: ParameterChannel::Both,
        },
        segments: segments
            .iter()
            .map(
                |&(shape, start_sample, end_sample, start_value, end_value)| AutomationSegment {
                    shape,
                    start_sample,
                    end_sample,
                    start_value,
                    end_value,
                    unit: ParameterUnit::Db,
                },
            )
            .collect(),
        offset: session::AutomationOffset::default(),
    }
}

/// Fader steps at each `(sample, dB)` of `steps`, from `from` dB.
fn fader_steps(from: f32, steps: &[(u64, f32)]) -> Vec<(AutomationShape, u64, u64, f32, f32)> {
    let mut at = (0_u64, from);
    steps
        .iter()
        .map(|&(sample, value)| {
            let segment = (AutomationShape::Step, at.0, sample, at.1, value);
            at = (sample, value);
            segment
        })
        .collect()
}

/// One left-lane fader event the driver pushed: `(sample, kind, held, value, ramp)`.
type FaderEvent = (u64, automation::EventKind, bool, f32, u32);

/// The left-lane fader events the driver pushed since the last take.
fn fader_events() -> Vec<FaderEvent> {
    let (events, overflow) = builtins_compiler::test_only_take_fader_events();
    assert!(!overflow);
    events
        .iter()
        .filter(|event| !event.mute && event.channel == 0)
        .map(|event| {
            (
                event.sample,
                event.kind,
                event.held,
                event.value,
                event.ramp,
            )
        })
        .collect()
}

/// The stage operation of one pushed fader event (#1638 D1).
fn operation(event: &FaderEvent) -> builtins::FaderOperation {
    let &(_, kind, held, value, ramp) = event;
    let gain = gain(value);
    if held {
        builtins::FaderOperation::RememberGain { gain }
    } else if kind == automation::EventKind::Set {
        builtins::FaderOperation::SetGain { gain }
    } else {
        builtins::FaderOperation::RetargetGain {
            gain,
            samples: ramp,
        }
    }
}

/// The stage driven directly from `start` dB with `operations` (in order at each sample).
fn reference_from(
    start: f32,
    operations: &[(u64, builtins::FaderOperation)],
    frames: u64,
) -> Vec<f32> {
    let mut all = vec![(0, builtins::FaderOperation::SetGain { gain: gain(start) })];
    all.extend_from_slice(operations);
    reference(&all, frames)
}

/// The spec's model of a lane with a fader ride and curve mute steps (#1647 D1.6, #1651 T3b),
/// written apart from the driver: the ride's own events `ride` (the events a fader-only plan of
/// the same curve pushes) and each mute step `(sample, muted)` over `mute` samples. At each sample
/// the gain operation comes first, under the hold before the sample's mute step. An event while
/// the gain is held (muted, or inside an unmute ramp `[u, u + M)`) is only remembered by the
/// stage and owes a move; at `E = u + M` an owed move lands in place of the ride's event there:
/// a `Jump` over `J` to the curve at `E + J - 1` (`curve`, flat there in every case). Returns the
/// events the driver must push, the stage operations of the whole lane and the number of moves.
fn compose_model(
    ride: &[FaderEvent],
    steps: &[(u64, bool)],
    mute: u32,
    curve: impl Fn(i64) -> f32,
) -> (Vec<FaderEvent>, Vec<(u64, builtins::FaderOperation)>, usize) {
    let mut times: Vec<u64> = ride
        .iter()
        .map(|event| event.0)
        .chain(steps.iter().map(|step| step.0))
        .chain(steps.iter().map(|step| step.0 + u64::from(mute)))
        .collect();
    times.sort_unstable();
    times.dedup();
    let mut events = Vec::new();
    let mut operations = Vec::new();
    let mut free_from = 0_u64;
    let mut end: Option<u64> = None;
    let mut owed = false;
    let mut moves = 0;
    for t in times {
        let own = ride.iter().find(|event| event.0 == t);
        if end == Some(t) && owed {
            let value = curve(t as i64 + i64::from(J) - 1);
            let landing = (t, automation::EventKind::Jump, false, value, J);
            operations.push((t, operation(&landing)));
            events.push(landing);
            owed = false;
            moves += 1;
        } else if let Some(event) = own {
            // A move changes no later discontinuity jump of the ride; its grid is flat there.
            assert!(
                moves == 0 || event.1 == automation::EventKind::Jump,
                "the ride is flat after the move, at {t}"
            );
            operations.push((t, operation(event)));
            events.push(*event);
            if t < free_from {
                owed = true;
            }
        }
        if end == Some(t) {
            end = None;
        }
        if let Some(&(_, muted)) = steps.iter().find(|step| step.0 == t) {
            operations.push((t, set_mute(muted, mute)));
            if muted {
                free_from = u64::MAX;
                end = None;
            } else {
                free_from = t + u64::from(mute);
                end = Some(free_from);
            }
        }
    }
    (events, operations, moves)
}

/// Run one fader-and-mute case (#1651 gate 2, T3b) on every form at quanta 128, 100, 64 and 37:
/// track 0 has the fader entry `fader` and a mute entry with steps `steps` (alternating from
/// unmuted) under a session mute length of `mute_ms`. The driver pushes exactly the events of
/// [`compose_model`] (the fader-only plan's events, with the owed move at `E`), and both planes are
/// bit for bit the stage driven directly with that model's operations. Returns how many moves
/// the model lands, so a case can say it reaches the move.
fn compose_case(
    label: &str,
    fader: &[(AutomationShape, u64, u64, f32, f32)],
    steps: &[u64],
    mute_ms: f32,
    frames: u64,
) -> usize {
    let mut moves = 0;
    for quantum in [128, 100, 64, 37] {
        let mut ride_only = plain_with(1, quantum, mute_ms);
        let mute = controls::LiveRamps::for_session(&ride_only).mute_samples;
        let entry = fader_entry(&ride_only, 0, fader);
        let segments = controls::stored_segments(&entry);
        let start = automation::value_at(&segments, 0);
        ride_only.automation = vec![entry];
        let mut model = ride_only.clone();
        let mute_steps: Vec<(u64, f32)> = steps
            .iter()
            .enumerate()
            .map(|(index, &sample)| (sample, if index % 2 == 0 { 1.0 } else { 0.0 }))
            .collect();
        model.automation.push(mute_entry(
            &model,
            0,
            ParameterChannel::Both,
            0.0,
            &mute_steps,
        ));
        let model_steps: Vec<(u64, bool)> = mute_steps
            .iter()
            .map(|&(sample, value)| (sample, value != 0.0))
            .collect();
        for form in forms() {
            let tag = format!("{label} q={quantum} {form:?}");
            let _ = builtins_compiler::test_only_take_fader_events();
            let _ = Rig::new(&ride_only, form).run(frames);
            let ride = fader_events();
            let (events, operations, landed) = compose_model(&ride, &model_steps, mute, |t| {
                automation::value_at(&segments, t)
            });
            let expected = reference_from(start, &operations, frames);
            let (out, _) = Rig::new(&model, form).run(frames);
            let pushed = fader_events();
            for plane in &out {
                assert_eq!(first_difference(plane, &expected), None, "{tag}");
            }
            assert_eq!(pushed, events, "{tag}: the pushed fader events");
            moves = landed;
        }
    }
    moves
}

/// #1651 gate 2 (T3b): a fader ride and curve mute steps on one lane. The ride moves while the
/// curve holds the lane muted ([1,000, 2,000): grid events on [1,200, 1,600)) and while its unmute
/// ramp runs ([2,000, 2,480): grid events on [2,100, 2,300)), then rests at `-6` dB. The events
/// inside the mute and the unmute ramp are only remembered; the move lands at `E = 2,480` over
/// `J` to `-6` dB; every sample is the stage driven directly with those operations, on every form
/// at quanta 128, 100, 64 and 37.
///
/// Test value: red if a fader event restarts or cuts the mute ramp, if the move at `E` is missing
/// or late (the lane would rest at the `-10` dB the unmute ramp aimed for), or if the ride's
/// events change under the mute (the cursor must push the fader-only plan's events).
#[test]
fn a_fader_ride_under_a_curve_mute_is_remembered_and_moves_at_the_unmute_end() {
    let moves = compose_case(
        "ride",
        &[
            (AutomationShape::Linear, 0, 1_200, -20.0, -20.0),
            (AutomationShape::Linear, 1_200, 1_600, -20.0, -10.0),
            (AutomationShape::Linear, 1_600, 2_100, -10.0, -10.0),
            (AutomationShape::Linear, 2_100, 2_300, -10.0, -6.0),
        ],
        &[1_000, 2_000],
        10.0,
        3_200,
    );
    assert_eq!(moves, 1, "the case reaches the move at E");
}

/// #1651 T3b, holds that start and end inside a block: a session mute length of 1 ms (`M = 48`)
/// so the unmute ramp's end `E` falls inside blocks. Each case puts a fader step inside the
/// unmute ramp, which owes the move at `E`; the cases are placed against the 128-sample block
/// grid (and run at quanta 128, 100, 64 and 37 on every form):
/// - `E` inside the unmute's block (unmute 1,300, `E` 1,348, fader step 1,320);
/// - `E` exactly at a block's end (unmute 1,360, `E` 1,408 = the next block's first sample);
/// - `E` in the next block (unmute 1,390, `E` 1,438, step 1,400);
/// - an unmute and a re-mute in the same block, after `E` (unmute 1,290, `E` 1,338, re-mute
///   1,354, last unmute 1,600, steps 1,310 and 1,620: two moves), and, with `M = 96`, inside the
///   unmute ramp (unmute 1,300, re-mute 1,364, last unmute 1,600, steps 1,320 and 1,650): the
///   first `E` (1,396) is dropped, the owed move is kept through the mute (constraint 4) and
///   lands at 1,696;
/// - a mute at a block's offset 0 (1,280) and an unmute at offset `q - 1` (1,535);
/// - a mute at offset `q - 1` (1,407) and an unmute at offset 0 (1,664);
/// - fader steps exactly at the mute (1,000: a free retarget, then the mute) and at the unmute
///   (1,300: remembered under the old hold, gain before mute, so it owes the move at 1,348).
///
/// Test value: red if the fader piece is not cut at the mute event (events after an in-block mute
/// would not owe the move, or an in-block unmute's `E` would get no cut), if `E` is off by one
/// (the move would land on a held stage and be only remembered, or a sample late), if an owed
/// event below `E` is dropped, if a re-mute keeps the old cut, or if the hold at a block's end
/// does not carry into the next block (`due_at_entry`, `held_through`).
#[test]
fn a_hold_that_starts_and_ends_inside_a_block_moves_at_its_end() {
    const MS: f32 = 1.0;
    assert_eq!(
        controls::LiveRamps::for_session(&plain_with(1, 128, MS)).mute_samples,
        48
    );
    // `(label, fader steps (sample, dB), mute step samples)`.
    type Case = (&'static str, Vec<(u64, f32)>, &'static [u64]);
    let cases: [Case; 7] = [
        ("E inside the block", vec![(1_320, -8.0)], &[1_000, 1_300]),
        ("E at the block end", vec![(1_380, -8.0)], &[1_000, 1_360]),
        ("E in the next block", vec![(1_400, -8.0)], &[1_000, 1_390]),
        (
            "unmute and re-mute after E",
            vec![(1_310, -8.0), (1_620, -3.0)],
            &[1_000, 1_290, 1_354, 1_600],
        ),
        (
            "mute at 0, unmute at q-1",
            vec![(1_290, -8.0), (1_560, -3.0)],
            &[1_280, 1_535],
        ),
        (
            "mute at q-1, unmute at 0",
            vec![(1_450, -8.0), (1_700, -3.0)],
            &[1_407, 1_664],
        ),
        (
            "steps at the mute and the unmute",
            vec![(1_000, -8.0), (1_300, -3.0)],
            &[1_000, 1_300],
        ),
    ];
    for (label, steps, mutes) in cases {
        let moves = compose_case(label, &fader_steps(-20.0, &steps), mutes, MS, 2_560);
        let expected = if label == "unmute and re-mute after E" {
            2
        } else {
            1
        };
        assert_eq!(moves, expected, "{label}: the case reaches the move at E");
    }
    let moves = compose_case(
        "re-mute inside the unmute ramp",
        &fader_steps(-20.0, &[(1_320, -8.0), (1_650, -3.0)]),
        &[1_000, 1_300, 1_364, 1_600],
        2.0,
        2_560,
    );
    assert_eq!(
        moves, 1,
        "the dropped cut lands no move; the last unmute's does"
    );
}

// #1651 T4, gate 8: the strip carry of mute automation.

/// The block a carry case's successor is swapped in at.
const SWAP_BLOCK: usize = 8;

/// Gate 8's quanta.
const CARRY_QUANTA: [u32; 2] = [128, 100];

/// How a carry case's plans are prepared: no live control (the join cannot write a record, so a
/// strip whose effective mute moves restarts), or live controls (the C ABI's form).
#[derive(Clone, Copy, Debug)]
struct CarryForm {
    backend: graph_compiler::Backend,
    live: bool,
}

/// Every bank width of the target and the scalar oracle (R17-4), each with and without live
/// controls. The scalar oracle binds the fader as a graph node, which no plan swap carries: its
/// strip restarts at the swap and its leg holds the post-ramp claim only.
fn carry_forms() -> Vec<CarryForm> {
    let backends = [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
        graph_compiler::Backend::Scalar,
    ];
    backends
        .into_iter()
        .flat_map(|backend| [false, true].map(|live| CarryForm { backend, live }))
        .collect()
}

/// A signed two-plane source that is never zero (a zero's sign and a gain's bits both show).
fn carry_pcm(frames: usize) -> [Vec<f32>; 2] {
    let plane = |channel: u32| {
        (0..frames as u32)
            .map(|frame| {
                let step = frame
                    .wrapping_mul(7_919)
                    .wrapping_add(channel * 31)
                    .wrapping_add(17)
                    % 1_999;
                (step as f32 - 999.5) / 2_000.0
            })
            .collect::<Vec<f32>>()
    };
    [plane(0), plane(1)]
}

/// What a carry run rendered.
struct CarryRun {
    out: [Vec<f32>; 2],
    /// The swap block's `(allocations, reallocations, frees)` on the render thread.
    swap_allocations: (u64, u64, u64),
    /// The successor's `restarted_strips()`.
    restarted: Vec<Box<str>>,
}

/// `model` compiled with this file's caps.
fn compiled(model: &SessionModel) -> host_core::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    host_core::compile_host_session(&document, &caps(model))
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())))
}

/// `a` prepared in `form` and rendered for [`SWAP_BLOCK`] blocks (`edit` runs before each block,
/// writing through its live handles and gate), then, with `b = Some((next, committed))`, `next`
/// prepared as its successor on the committed model `committed`, swapped in and rendered to
/// `frames` (`edit` then writes through the successor's handles and gate: a record written at the
/// swap block is one the shared commit wrote for the transaction that rebuilt); with `b = None`,
/// `a` alone for every block. Fed `pcm` from frame 0.
fn carry_run(
    a: &SessionModel,
    b: Option<(&SessionModel, &SessionModel)>,
    form: CarryForm,
    pcm: &[Vec<f32>; 2],
    frames: usize,
    mut edit: impl FnMut(
        usize,
        &mut host_core::HostLiveControlHandles,
        &mut engine::realtime::PlanRevisionGate,
    ),
) -> CarryRun {
    use engine::realtime::{PlanExchangeConfig, SwapOutcome, plan_exchange};
    let quantum = a.quantum_frames as usize;
    let caps = caps(a);
    let request = host_core::HostLiveControlRequest::default();
    let fail = |failure: host_core::PrepareDiagnostics| -> ! {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    };
    let first = compiled(a);
    let (prepared, mut handles) = if form.live {
        let (prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &first,
            &caps,
            &request,
            form.backend,
        )
        .unwrap_or_else(|failure| fail(failure));
        (prepared, Some(handles))
    } else {
        (
            host_core::test_only_prepare_host_runtime_on(&first, &caps, form.backend)
                .unwrap_or_else(|failure| fail(failure)),
            None,
        )
    };
    let PreparedHost {
        mut plan,
        mut sources,
        inventory,
        ..
    } = prepared;
    let mut gate = plan.take_revision_gate();
    let (mut publisher, mut owner, _retirer) = plan_exchange(
        plan,
        PlanExchangeConfig {
            retirement_capacity: core::num::NonZeroUsize::MIN,
        },
    )
    .unwrap_or_else(|_| panic!("plan exchange"));
    bench_support::alloc::assert_installed();
    let submit = |sources: &mut host_core::SourceControlSet, at: usize| {
        sources
            .submit(
                NINE_SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: at as u64,
                    sample_rate_hz: RATE,
                    planes: &[&pcm[0][at..at + quantum], &pcm[1][at..at + quantum]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
    };
    let mut out = [Vec::new(), Vec::new()];
    let mut swap_allocations = (0, 0, 0);
    let mut restarted = Vec::new();
    let mut successor_sources = None;
    let mut successor_live = None;
    let mut block = 0;
    while block * quantum < frames {
        if block == SWAP_BLOCK
            && let Some((next, committed)) = b
        {
            let next = compiled(next);
            let committed = compiled(committed).normalized_model().clone();
            let revision = gate.as_ref().map_or(1, |gate| gate.last_published()) + 1;
            let base = host_core::SuccessorBase::new(&inventory, &committed, revision);
            let prepared = if form.live {
                host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                    &next,
                    &caps,
                    &request,
                    base,
                    form.backend,
                )
                .map(|(prepared, handles)| (prepared, Some(handles)))
            } else {
                host_core::test_only_prepare_host_runtime_successor_on(
                    &next,
                    &caps,
                    base,
                    form.backend,
                )
                .map(|prepared| (prepared, None))
            }
            .unwrap_or_else(|failure| fail(failure));
            let (mut prepared, next_handles) = prepared;
            prepared.sources.adopt_persisting(&mut sources);
            restarted = prepared.restarted_strips().to_vec();
            let PreparedHost {
                mut plan,
                sources: next_sources,
                ..
            } = prepared;
            let mut next_gate = plan.take_revision_gate().expect("the successor's gate");
            next_gate.publish(revision);
            successor_live = next_handles.map(|handles| (handles, next_gate));
            publisher
                .reserve_replacement(plan, engine::realtime::PlanAdoption::Next)
                .unwrap_or_else(|_| panic!("reserve the successor"))
                .commit(revision);
            successor_sources = Some(next_sources);
        }
        let at = block * quantum;
        submit(successor_sources.as_mut().unwrap_or(&mut sources), at);
        if let Some((handles, gate)) = successor_live.as_mut() {
            edit(block, handles, gate);
        } else if let (Some(handles), Some(gate)) = (handles.as_mut(), gate.as_mut())
            && successor_sources.is_none()
        {
            edit(block, handles, gate);
        }
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        let report = owner
            .render_contiguous(RenderIo { output }, at as u64)
            .unwrap_or_else(|error| panic!("render: {error:?}"));
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        let swapped = report.swap == SwapOutcome::Applied;
        assert_eq!(
            swapped,
            b.is_some() && block == SWAP_BLOCK,
            "the successor is adopted at the swap block"
        );
        if swapped {
            swap_allocations = (delta.allocations, delta.reallocations, delta.deallocations);
        }
        out[0].extend_from_slice(&samples[..quantum]);
        out[1].extend_from_slice(&samples[quantum..]);
        block += 1;
    }
    drop(handles);
    drop(successor_live);
    CarryRun {
        out: out.map(|mut plane| {
            plane.truncate(frames);
            plane
        }),
        swap_allocations,
        restarted,
    }
}

/// `model` prepared fresh in `form` and fed `pcm` from frame 0: the reference.
fn fresh_run(
    model: &SessionModel,
    form: CarryForm,
    pcm: &[Vec<f32>; 2],
    frames: usize,
) -> [Vec<f32>; 2] {
    carry_run(model, None, form, pcm, frames, |_, _, _| {}).out
}

/// Gate 8 on one case: `a` rendered, `b` swapped in at [`SWAP_BLOCK`] on `a`'s committed model,
/// on every carry form and at both quanta. From the first block boundary at or after the
/// adoption ramp's end (`M` samples from the swap) each plane equals a fresh plan of `b` fed the
/// same PCM, bit for bit; before the swap it equals `a`'s; the swap block allocates and frees
/// nothing; a bank's strip carries. `ramps` asserts that the swap ramps (a bank's carried lane
/// differs from the fresh plan inside the ramp). Returns the mute events the bank legs' successors
/// pushed at the swap sample, per quantum: `(channel, sample, muted, ramp)`.
fn carry_case(
    label: &str,
    a: &SessionModel,
    b: &SessionModel,
    ramps: bool,
) -> Vec<Vec<(usize, u64, bool, u32)>> {
    let mut swap_events = Vec::new();
    for quantum in CARRY_QUANTA {
        let (mut a, mut b) = (a.clone(), b.clone());
        a.quantum_frames = quantum;
        b.quantum_frames = quantum;
        let q = quantum as usize;
        let frames = 4_096_usize.div_ceil(q) * q;
        let pcm = carry_pcm(frames);
        let at = SWAP_BLOCK * q;
        let settled = (at + M as usize).div_ceil(q) * q;
        for form in carry_forms() {
            let what = format!("{label}, {form:?}, quantum {quantum}");
            let bank = form.backend != graph_compiler::Backend::Scalar;
            let fresh = fresh_run(&b, form, &pcm, frames);
            let before = fresh_run(&a, form, &pcm, frames);
            let _ = mute_events();
            let swapped = carry_run(&a, Some((&b, &a)), form, &pcm, frames, |_, _, _| {});
            let events = mute_events();
            assert_eq!(
                swapped.swap_allocations,
                (0, 0, 0),
                "{what}: the swap block"
            );
            if bank && form.live {
                assert!(swapped.restarted.is_empty(), "{what}: the strip carries");
            }
            for plane in 0..2 {
                assert_eq!(
                    first_difference(&swapped.out[plane][..at], &before[plane][..at]),
                    None,
                    "{what}: plane {plane} before the swap"
                );
                assert_eq!(
                    first_difference(&swapped.out[plane][settled..], &fresh[plane][settled..])
                        .map(|index| index + settled),
                    None,
                    "{what}: plane {plane} from the first block boundary after the ramp"
                );
                if ramps && bank && swapped.restarted.is_empty() {
                    assert!(
                        first_difference(
                            &swapped.out[plane][at..settled],
                            &fresh[plane][at..settled]
                        )
                        .is_some(),
                        "{what}: plane {plane} ramps at the swap, it is not set"
                    );
                }
            }
            if bank && form.live {
                swap_events.push(
                    events
                        .into_iter()
                        .filter(|event| event.1 == at as u64)
                        .collect(),
                );
            }
        }
    }
    swap_events
}

/// A two-track [`plain`] model (two bank lanes) with track 0's mute curve `entry`, if any.
fn with_mute(quantum: u32, entry: Option<(f32, &[(u64, f32)])>) -> SessionModel {
    let mut model = plain(2, quantum);
    if let Some((from, steps)) = entry {
        model.automation = vec![mute_entry(&model, 0, ParameterChannel::Both, from, steps)];
    }
    model
}

/// Every swap-sample mute event of a bank leg is one `SetMute` per channel to `muted` over `M`.
fn assert_adoption_jumps(events: &[Vec<(usize, u64, bool, u32)>], muted: bool, label: &str) {
    assert!(!events.is_empty());
    for events in events {
        let mut channels: Vec<usize> = events.iter().map(|event| event.0).collect();
        channels.sort_unstable();
        assert_eq!(channels, [0, 1], "{label}: one adoption jump per channel");
        assert!(
            events.iter().all(|event| event.2 == muted && event.3 == M),
            "{label}: to {muted} over the mute length: {events:?}"
        );
    }
}

/// #1651 gate 8, a gained mute cell: a rebuild at block 8 adds a mute entry on track 0 (both
/// lanes) that holds the lane muted until timeline 2,600. The carried lane composes `curve ||
/// terms` at the swap block's node time and mutes over `M` from the swap (one `SetMute` per
/// channel), then follows the curve's own unmute at 2,600: from the first block boundary after
/// the ramp the output equals a fresh plan of the new model.
///
/// Test value: red if a carried strip lane does not take on a gained mute curve (it stays open
/// until the curve's next event), sets it exactly instead of ramping, ramps over another length,
/// or misplaces the gained cell's cursor (its unmute at 2,600 is missed or repeated); and red if
/// a `terms` record at the swap block composes with the gained cell's prepared seed instead of
/// the curve at the swap (the drain unmutes and the driver mutes again: a jump the stage did not
/// owe).
#[test]
fn a_rebuild_that_adds_a_mute_entry_mutes_from_the_swap_over_the_mute_length() {
    let a = with_mute(128, None);
    let b = with_mute(128, Some((1.0, &[(2_600, 0.0)])));
    let events = carry_case("added", &a, &b, true);
    assert_adoption_jumps(&events, true, "added");
    // The same rebuild also opens the lane's static mute: the join's `Mute` record (the new
    // `terms`) reaches the gained cell at the swap block's drain, before the driver repositions
    // it. The curve is muted at the swap (300 to 2,600), so `curve || terms` equals the stage's
    // (statically muted) mute and the swap lands nothing.
    let mut a = with_mute(128, None);
    (a.tracks[0].fader.left_mute, a.tracks[0].fader.right_mute) = (true, true);
    let b = with_mute(128, Some((0.0, &[(300, 1.0), (2_600, 0.0)])));
    let events = carry_case("added, terms open", &a, &b, false);
    assert!(
        events.iter().all(Vec::is_empty),
        "added, terms open: no jump at the swap: {events:?}"
    );
}

/// #1651 gate 8, a lost mute cell: the predecessor's curve holds track 0 muted until timeline
/// 3,600, and a rebuild at block 8 removes the entry. The join sends the lane a `Mute` record of
/// its effective (static) mute, open, so the carried lane unmutes over `M` from the swap and
/// equals a fresh plan of the new model from the first block boundary after the ramp.
///
/// Test value: red if a lost mute cell keeps the curve's state (the lane stays muted until the
/// next live mute record), or if the release ramps over another length.
#[test]
fn a_rebuild_that_removes_a_mute_entry_holds_the_static_mute() {
    let a = with_mute(128, Some((1.0, &[(3_600, 0.0)])));
    let b = with_mute(128, None);
    carry_case("removed", &a, &b, true);
}

/// #1651 gate 8, a changed curve: (a) the predecessor's curve mutes at 3,000 and the new one is
/// muted from 500 to 2,600, so at the swap the repositioned cell composes muted and jumps over
/// `M`, then unmutes at 2,600; (b) the new curve differs only after the swap (a mute from 2,000
/// to 3,200 instead of from 3,000), so the swap lands no jump and the lane follows the new
/// table's steps. Each equals a fresh plan of the new model from the first block boundary after
/// the ramp.
///
/// Test value: red if a changed table continues its carried cursor (the old table's position in
/// the new one misses or repeats a step), if the adoption jump is missing, repeated, or lands
/// when the composed mute equals the stage's.
#[test]
fn a_rebuild_that_changes_a_mute_curve_jumps_only_when_the_composed_mute_changes() {
    let a = with_mute(128, Some((0.0, &[(3_000, 1.0)])));
    let b = with_mute(128, Some((0.0, &[(500, 1.0), (2_600, 0.0)])));
    let events = carry_case("changed, muted now", &a, &b, true);
    assert_adoption_jumps(&events, true, "changed, muted now");
    let b = with_mute(128, Some((0.0, &[(2_000, 1.0), (3_200, 0.0)])));
    let events = carry_case("changed later", &a, &b, false);
    assert!(
        events.iter().all(Vec::is_empty),
        "changed later: no jump at the swap: {events:?}"
    );
}

/// #1651 T4: a live fader move in flight on track 0 across a rebuild that adds a curve
/// mute which is on at the swap. A `FaderDb` of -12 dB over 2,000 samples is written before block
/// 5 and is still ramping at the swap (block 8); the successor's committed model holds the -12
/// dB and a mute curve muted until timeline 2,600. The adoption mutes the lane over `M` (dual-mono
/// lanes have separate stages, so the mute retargets only the lane's gain to `0`), and the stage
/// remembers the fader gain the move aimed for (#1638 D2, D1.6). After the curve's unmute at
/// 2,600 and its ramp the lane equals the uninterrupted plan (the move completed, never muted),
/// bit for bit.
///
/// Test value: red if the mute adoption stretches or cuts the live fader move (the lane comes
/// back at the gain the move had reached at the swap, or ramps on from it), or if the stage
/// remembers a gain other than the move's target.
#[test]
fn a_mute_adoption_keeps_the_fader_move_in_flight_for_the_unmute() {
    const DB: f32 = -12.0;
    const MOVE: u32 = 2_000;
    let write = |block: usize,
                 handles: &mut host_core::HostLiveControlHandles,
                 gate: &mut engine::realtime::PlanRevisionGate| {
        use builtins_compiler::TestOnlyWriteGate;
        if block == 5 {
            gate.write_published(|stamp| {
                handles.strip_controls[0].fader.write(
                    stamp,
                    builtins_compiler::TrackFaderRecord::FaderDb {
                        lanes: builtins::BuiltinLaneSelector::Both,
                        db: DB,
                        smoothing_samples: MOVE,
                    },
                );
            });
        }
    };
    for quantum in CARRY_QUANTA {
        // One track, so the output is track 0's lane.
        let a = plain(1, quantum);
        let mut committed = a.clone();
        committed.tracks[0].fader.left_db = DB;
        committed.tracks[0].fader.right_db = DB;
        let mut b = committed.clone();
        b.automation = vec![mute_entry(
            &b,
            0,
            ParameterChannel::Both,
            1.0,
            &[(2_600, 0.0)],
        )];
        let q = quantum as usize;
        let frames = 4_096_usize.div_ceil(q) * q;
        assert!(
            5 * q + MOVE as usize > SWAP_BLOCK * q,
            "the move is in flight at the swap"
        );
        let pcm = carry_pcm(frames);
        let open = (2_600 + M as usize).div_ceil(q) * q;
        for form in carry_forms().into_iter().filter(|form| form.live) {
            let what = format!("{form:?}, quantum {quantum}");
            let uninterrupted = carry_run(&a, None, form, &pcm, frames, write).out;
            let swapped = carry_run(&a, Some((&b, &committed)), form, &pcm, frames, write);
            assert_eq!(
                swapped.swap_allocations,
                (0, 0, 0),
                "{what}: the swap block"
            );
            if form.backend != graph_compiler::Backend::Scalar {
                assert!(swapped.restarted.is_empty(), "{what}: the strip carries");
            }
            for (plane, (out, uninterrupted)) in swapped.out.iter().zip(&uninterrupted).enumerate()
            {
                let muted = &out[(SWAP_BLOCK * q + M as usize)..2_600];
                assert_eq!(
                    muted.iter().position(|sample| sample.to_bits() != 0),
                    None,
                    "{what}: plane {plane} is muted after the adoption ramp"
                );
                assert_eq!(
                    first_difference(&out[open..], &uninterrupted[open..])
                        .map(|index| index + open),
                    None,
                    "{what}: plane {plane} after the unmute equals the uninterrupted move"
                );
            }
        }
    }
}

/// #1651 U1 fold 1 (MAJOR-2), every bank width: a hand mute of track 0 committed in the same
/// transaction as a mute curve edit lands as committed. The predecessor's curve closes at 1,500
/// and opens at 2,500; at block 8 (the curve open) a rebuild moves the closing step to 1,600 (the
/// adopted cell repositions) and the successor's committed model holds the hand mute on, whose
/// `Mute` record
/// (the shared commit's terms write, over the edit's ramp word: `M`, `0` or `2,000`) reaches the
/// successor's first block. A twin that never swaps takes the same record at the same block. The
/// curve's change lies inside the span the hand mute holds, so the swapped run equals the twin,
/// bit for bit, on every sample (live forms; the scalar oracle restarts its strip at a swap, so
/// it has no carry to hold).
///
/// Test value: red if the swap block's adoption drops the record's ramp word and lands the hand
/// mute over the mute length instead (a `0` step becomes a 480-sample ramp, a 2,000-sample fade
/// becomes 480), or lands it twice.
#[test]
fn a_hand_mute_in_a_curve_rebuild_lands_over_its_own_ramp() {
    for ramp in [M, 0, 2_000] {
        let mute = |block: usize,
                    handles: &mut host_core::HostLiveControlHandles,
                    gate: &mut engine::realtime::PlanRevisionGate| {
            use builtins_compiler::TestOnlyWriteGate;
            if block == SWAP_BLOCK {
                gate.write_published(|stamp| {
                    handles.strip_controls[0].fader.write(
                        stamp,
                        builtins_compiler::TrackFaderRecord::Mute {
                            lanes: builtins::BuiltinLaneSelector::Both,
                            muted: true,
                            smoothing_samples: ramp,
                        },
                    );
                });
            }
        };
        for quantum in CARRY_QUANTA {
            let a = with_mute(quantum, Some((0.0, &[(1_500, 1.0), (2_500, 0.0)])));
            let mut b = with_mute(quantum, Some((0.0, &[(1_600, 1.0), (2_500, 0.0)])));
            (b.tracks[0].fader.left_mute, b.tracks[0].fader.right_mute) = (true, true);
            let q = quantum as usize;
            let frames = 4_096_usize.div_ceil(q) * q;
            let pcm = carry_pcm(frames);
            for form in carry_forms()
                .into_iter()
                .filter(|form| form.live && form.backend != graph_compiler::Backend::Scalar)
            {
                let what = format!("ramp {ramp}, {form:?}, quantum {quantum}");
                let twin = carry_run(&a, None, form, &pcm, frames, mute).out;
                let swapped = carry_run(&a, Some((&b, &a)), form, &pcm, frames, mute);
                assert_eq!(
                    swapped.swap_allocations,
                    (0, 0, 0),
                    "{what}: the swap block"
                );
                assert!(swapped.restarted.is_empty(), "{what}: the strip carries");
                for (plane, (out, twin)) in swapped.out.iter().zip(&twin).enumerate() {
                    assert_eq!(
                        first_difference(out, twin),
                        None,
                        "{what}: plane {plane} equals the hand mute alone"
                    );
                }
            }
        }
    }
}

/// #1651 U1 fold 1 (MAJOR-1): a curve unmute ends the cell's hold of an older ramp. Track 0 rides
/// a linear fade from `-30` to `0` dB over `[0, 6_400]` and its mute curve closes at 2,001 and
/// opens at 2,570. A hand mute (exact) is drained at the entry at or after 512 and a hand unmute
/// over 4,000 samples at the entry at or after 1,024; under that unmute ramp an offsets write
/// `S = -6` dB (ramp 200) at the entry at or after 1,536 takes the held branch of the offsets
/// jump, which holds the cell until the stage's ramp end (about 5,023). The curve's mute at 2,001
/// replaces the hand unmute ramp, and its unmute at 2,570 over the mute length `M` frees the stage
/// at `E = 2_570 + M`, long before that hold ends. With `M = 0` (`mute_ms` 0) and with `M = 48`
/// (`mute_ms` 1: below the 64-sample grid, with no grid point in `(2_570, E)`), from `E` plus `J`
/// the lane follows the composed curve: at each completion sample `64k + 63` it equals
/// `checked_fader_gain(vca_compose_db(curve, S))` exactly, on every live form (Simd8 on AVX2,
/// Simd4, the scalar oracle) at quanta 128, 100 and 37.
///
/// Test value: red at `M = 0` if the unmute leaves the cell's older hold in place: the exact
/// unmute owes nothing, so its grid events after `E` are only remembered on a free stage and the
/// ride freezes at `-23.7` dB until the hold's end. At `M = 48` the grid events of the muted span
/// leave a move owed at `E`, whose landing re-commits the cell, so that leg holds the claim but is
/// green without the release too.
#[test]
fn a_curve_unmute_ends_an_older_hold_of_the_fader_cell() {
    for mute_ms in [0.0_f32, 1.0] {
        for quantum in [128, 100, 37] {
            let mut model = plain_with(1, quantum, mute_ms);
            let m = controls::LiveRamps::for_session(&model).mute_samples;
            let ride = fader_entry(
                &model,
                0,
                &[(AutomationShape::Linear, 0, 6_400, -30.0, 0.0)],
            );
            model.automation = vec![
                ride.clone(),
                mute_entry(
                    &model,
                    0,
                    ParameterChannel::Both,
                    0.0,
                    &[(2_001, 1.0), (2_570, 0.0)],
                ),
            ];
            let free = 2_570 + u64::from(m);
            let check_from = (free + u64::from(J)).div_ceil(64) * 64 + 63;
            for form in forms().into_iter().filter(|form| form.live) {
                let what = format!("M {m}, quantum {quantum}, {form:?}");
                let mut rig = Rig::new(&model, form);
                let q = rig.quantum as u64;
                while rig.rendered < 4_800 {
                    let entry = |at: u64| (at..at + q).contains(&rig.rendered);
                    if entry(512) {
                        rig.live_mute(true, 0);
                    } else if entry(1_024) {
                        rig.live_mute(false, 4_000);
                    } else if entry(1_536) {
                        rig.live_offsets(-6.0, 200);
                    }
                    rig.block();
                }
                for plane in &rig.out {
                    for t in (check_from..4_800).step_by(64) {
                        let value =
                            automation::value_at(&controls::stored_segments(&ride), t as i64);
                        let expected =
                            builtins::checked_fader_gain(session::vca_compose_db(value, -6.0))
                                .expect("fader domain");
                        assert_eq!(
                            plane[t as usize].to_bits(),
                            expected.to_bits(),
                            "{what}: completion sample {t}"
                        );
                    }
                }
            }
        }
    }
}
