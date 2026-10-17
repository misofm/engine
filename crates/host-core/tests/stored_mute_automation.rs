//! Stored mute automation on the strip (#1651 T3a): a mute cell renders its steps on the fader
//! stage at their exact samples over the session mute length, through the one event driver, on
//! every bank width and on the scalar oracle (R17-4); a seek that leaves the stage mute unchanged
//! lets a ramp toward that state finish (root ruling R18-1); and a mute value the strip's live cell
//! carries is the lane's `terms`, which render ORs with the curve (OQ1 = C).
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
        maximum_automation_spans_per_block: 128,
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
        mute_ms: 10.0,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    assert_eq!(controls::LiveRamps::for_session(&model).mute_samples, M);
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

/// How a [`Rig`] is prepared: a backend, live controls, and delivery between render calls.
#[derive(Clone, Copy, Debug)]
struct Form {
    backend: graph_compiler::Backend,
    live: bool,
    between: bool,
}

/// Every form of the target: Simd8 (on an AVX2 target only), Simd4 and the scalar oracle, each
/// with no live control, with live controls, and with live controls between render calls (the
/// fused fader-matrix pair).
fn forms() -> Vec<Form> {
    let backends = [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
        graph_compiler::Backend::Scalar,
    ];
    let mut forms = Vec::new();
    for backend in backends {
        for (live, between) in [(false, false), (true, false), (true, true)] {
            forms.push(Form {
                backend,
                live,
                between,
            });
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
            let (prepared, handles) = if form.between {
                host_core::prepare::test_only_prepare_host_runtime_between_render_calls_on(
                    &compiled,
                    &caps,
                    &request,
                    form.backend,
                )
            } else {
                host_core::test_only_prepare_host_runtime_with_live_controls_on(
                    &compiled,
                    &caps,
                    &request,
                    form.backend,
                )
            }
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
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

    /// A plain session seek to timeline sample `timeline` (#1641).
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.prepared
            .sources
            .seek_session(self.generation, timeline, None)
            .expect("the seek is admitted");
        self.frame = timeline;
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

    /// Render one block; returns its first render sample.
    fn block(&mut self) -> u64 {
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
/// with no live control, with live controls, and between render calls) and at quanta 128, 100 and
/// 37, the driver pushes `SetMute` at exactly those samples over the session mute length `M`, and
/// the output is bit for bit the stage driven directly with those two operations: each ramp
/// starts at its step sample, the lane is the exact `+0.0` of a static mute from the end of the
/// mute ramp, and back at the fader from the end of the unmute ramp. Render allocates nothing.
///
/// Test value: red if a mute step snaps to the grid or a block boundary, ramps over another row's
/// length (the fader's 240), fires twice or not at all, depends on the quantum or the form, or if
/// the scalar oracle or the fused pair skips the mute cells.
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
