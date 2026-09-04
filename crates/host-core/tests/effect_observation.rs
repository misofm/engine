//! Issue #143 P3: the binding — E1, E2, E3, E5 and E13 against a real compiled plan.
//!
//! The fixture is eight compressor chains in one SIMD rack, so the cohort planner forms a real
//! homogeneous bank and the observation path under test is the banked one. Every track's builtins
//! are the identity and only the compressor's **threshold** differs, which keeps the eight
//! instances in one program key (so they still bank) while giving each lane a different reduction
//! that an independently prepared scalar instance can reproduce to the bit.

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::MeterTap;
use effect_contract::{
    AutomationSpanKind, EffectControlRecord, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ObservationSample, ParameterChannel,
    PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    EffectRack, HostConsoleHandles, HostConsoleRequest, HostPrepareCaps, HostShapePolicy,
    PreparedHost, SourceSubmission, prepare_host_session_with_console,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/compressor-bank-observation.json");
/// The same fixture with every compressor moved into the dynamic rack (issue #163 phase 1b).
const DYNAMIC_BANK_SESSION: &str =
    include_str!("../../../fixtures/session/v1/compressor-dynamic-bank-observation.json");
const QUANTUM: usize = 128;
const TRACKS: usize = 8;
/// Blocks per published observation window, and per meter window: they are the same window.
const WINDOW_BLOCKS: u32 = 4;
/// The declared thresholds, in track order. Two tracks share each value.
const THRESHOLDS: [f32; TRACKS] = [0.0, -18.0, -24.0, -30.0, 0.0, -18.0, -24.0, -30.0];
/// `threshold` is parameter id 1 of `miso.compressor`, which is index 0 of its table.
const THRESHOLD_INDEX: u32 = 0;
/// The constant the fixture source carries: -12.04 dBFS.
const LEVEL: f32 = 0.25;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
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

/// The four legs of E1. Every one of them must render the corpus to the same bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Leg {
    /// (a) No console at all: the pre-#137 path.
    NoConsole,
    /// (b) A console, but no observation capacity: level-1 zero.
    ConsoleNoCapacity,
    /// (c) Observation capacity, nothing armed: level-2 zero.
    CapacityUnarmed,
    /// (d) Every declared gain-reduction tap armed.
    AllArmed,
}

fn console(leg: Leg) -> HostConsoleRequest {
    let console = matches!(
        leg,
        Leg::ConsoleNoCapacity | Leg::CapacityUnarmed | Leg::AllArmed
    );
    HostConsoleRequest {
        control_queue_depth: console.then(|| NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: console
            .then(|| NonZeroU32::new(QUANTUM as u32 * WINDOW_BLOCKS).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: match leg {
            Leg::NoConsole | Leg::ConsoleNoCapacity => 0,
            Leg::CapacityUnarmed | Leg::AllArmed => 4,
        },
        master_track: matches!(leg, Leg::CapacityUnarmed | Leg::AllArmed).then_some(0),
    }
}

struct Session {
    prepared: PreparedHost,
    handles: HostConsoleHandles,
    block: usize,
}

fn prepare(leg: Leg) -> Session {
    prepare_from(SESSION, leg)
}

fn prepare_from(document: &str, leg: Leg) -> Session {
    let (_session, prepared, handles) =
        prepare_host_session_with_console(document, &caps(), &console(leg)).unwrap_or_else(
            |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
        );
    assert_eq!(handles.tracks.len(), TRACKS);
    assert!(
        prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound at least one homogeneous bank on this host"
    );
    Session {
        prepared,
        handles,
        block: 0,
    }
}

/// Renders `blocks` quanta of the fixture constant and returns the rendered output words.
fn render(session: &mut Session, blocks: usize) -> Vec<u32> {
    let mut rendered = Vec::with_capacity(blocks * QUANTUM * 2);
    let left = [LEVEL; QUANTUM];
    let right = [LEVEL; QUANTUM];
    for step in 0..blocks {
        let block = session.block + step;
        session
            .prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        session
            .prepared
            .plan
            .render(
                RenderIo {
                    input: None,
                    output,
                },
                RenderTime {
                    absolute_sample: (block * QUANTUM) as u64,
                },
            )
            .expect("render");
        rendered.extend(samples.iter().map(|value| value.to_bits()));
    }
    session.block += blocks;
    rendered
}

/// Arms (or disarms) tap 0 of every observed effect, and returns the sample the change applies at.
fn subscribe_all(session: &mut Session, armed: bool, window_blocks: u32) -> u64 {
    let applied_at = (session.block * QUANTUM) as u64;
    for producer in session.handles.effect_controls.iter_mut() {
        producer
            .producer
            .try_push(EffectControlRecord::Observe {
                tap_index: 0,
                armed,
                window_blocks,
            })
            .expect("room in the bounded queue");
    }
    applied_at
}

/// The reader for one track's compressor tap, in whichever rack the fixture declared it.
fn reader(handles: &HostConsoleHandles, track: usize) -> &engine::realtime::ObservationReader {
    let id = handles.tracks[track].as_ref();
    let handle = handles
        .effect_observations
        .iter()
        .find(|handle| handle.track_id.as_ref() == id && handle.effect_index == 0)
        .expect("an observation handle for the addressed effect");
    // The addressing is `(track, rack, effect_index)`; the fixtures declare exactly one effect per
    // track, so naming the rack here would only restate which fixture is loaded.
    assert!(matches!(
        handle.rack,
        EffectRack::Simd1 | EffectRack::Dynamic
    ));
    &handle.readers[0]
}

/// An independently prepared scalar compressor at `threshold`, fed the same constant.
fn scalar_reference(threshold: f32, blocks: usize) -> ObservationSample {
    let mut values: Vec<InitialParameterValue> = Vec::new();
    for (index, parameter) in compressor::COMPRESSOR_PARAMETERS.iter().enumerate() {
        let value = if index as u32 == THRESHOLD_INDEX {
            threshold
        } else {
            parameter.default_value
        };
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    let request = PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: effect_contract::PortId::new("sidechain-in").expect("port"),
                required: false,
            },
        },
        initial_values: &values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 12,
            maximum_automation_spans_per_block: 128,
        },
    };
    let mut effect: Box<dyn PreparedNativeEffect> = compressor::CompressorFactory
        .prepare(request)
        .expect("scalar prepare");
    let mut peak = ObservationSample::default();
    for block in 0..blocks {
        let mut left = [LEVEL; QUANTUM];
        let mut right = [LEVEL; QUANTUM];
        let process = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            (block * QUANTUM) as u64,
            &[],
            QUANTUM as u32,
        )
        .expect("block");
        let _ = effect.process(process);
        let mut sample = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut sample));
        peak.left = peak.left.max(sample.left.abs());
        peak.right = peak.right.max(sample.right.abs());
    }
    peak
}

/// **E1**: four legs, one corpus, one set of bytes.
///
/// Red mutation: fold the observation read into the compressor's inner loop, or let a `log` touch
/// limiter state -> leg (d) diverges from (a).
#[test]
fn every_observation_leg_renders_the_corpus_to_the_same_bytes() {
    const BLOCKS: usize = 32;
    let mut no_console = prepare(Leg::NoConsole);
    let baseline = render(&mut no_console, BLOCKS);
    assert!(
        baseline.iter().any(|word| *word != 0),
        "the corpus is not silence"
    );

    for leg in [Leg::ConsoleNoCapacity, Leg::CapacityUnarmed, Leg::AllArmed] {
        let mut session = prepare(leg);
        if leg == Leg::AllArmed {
            subscribe_all(&mut session, true, WINDOW_BLOCKS);
        }
        let rendered = render(&mut session, BLOCKS);
        assert_eq!(
            rendered, baseline,
            "{leg:?} rendered different audio from the console-free path"
        );
    }

    // And leg (d) really did observe something, so the equality above is not vacuous.
    let mut armed = prepare(Leg::AllArmed);
    subscribe_all(&mut armed, true, WINDOW_BLOCKS);
    let _ = render(&mut armed, BLOCKS);
    let window = reader(&armed.handles, 3)
        .read()
        .expect("a published window");
    assert!(window.left > 0.0, "leg (d) published a real reduction");
}

/// **E5**: with no observation request, the built plan holds no observation state at all.
///
/// Walked over the built runtime, not inferred from the output: a plan that bound every tap and
/// happened to publish nothing would pass an output comparison and fail this.
///
/// Red mutation: attach lanes whenever the descriptor declares a tap regardless of the request ->
/// the structure fails while the output stays identical, which is the point.
#[test]
fn a_session_that_asked_for_no_observation_holds_none() {
    for leg in [Leg::NoConsole, Leg::ConsoleNoCapacity] {
        let session = prepare(leg);
        assert!(
            session.handles.effect_observations.is_empty(),
            "{leg:?} produced observation handles"
        );
        assert_eq!(
            session.prepared.plan.observation_binding_counts(),
            [0, 0, 0],
            "{leg:?} bound observation state into the runtime"
        );
        assert_eq!(
            session.prepared.report.observation_retained_bytes, 0,
            "{leg:?} retained observation bytes"
        );
        assert_eq!(session.prepared.plan.observation_retained_bytes(), 0);
        assert_eq!(session.handles.master_track, None);
    }

    // A capable plan, in contrast, binds exactly one lane per compressor and exactly one tap per
    // lane -- and arms none of them until it is asked to.
    let mut capable = prepare(Leg::CapacityUnarmed);
    assert_eq!(capable.handles.effect_observations.len(), TRACKS);
    assert_eq!(capable.handles.master_track, Some(0));
    let [stages, declared, armed] = capable.prepared.plan.observation_binding_counts();
    assert_eq!(stages, TRACKS as u64, "one observed lane per compressor");
    assert_eq!(declared, TRACKS as u64, "one declared tap each");
    assert_eq!(armed, 0, "capacity is not a subscription");
    // The retained bytes the report states are exactly the bytes the runtime holds.
    assert!(capable.prepared.report.observation_retained_bytes > 0);
    assert_eq!(
        capable.prepared.report.observation_retained_bytes,
        capable.prepared.plan.observation_retained_bytes()
    );

    // Arming changes the armed count and **not** the retained bytes: a subscription is a flag.
    let retained = capable.prepared.plan.observation_retained_bytes();
    subscribe_all(&mut capable, true, WINDOW_BLOCKS);
    let _ = render(&mut capable, 1);
    let [_, _, armed] = capable.prepared.plan.observation_binding_counts();
    assert_eq!(armed, TRACKS as u64, "every tap armed");
    assert_eq!(
        capable.prepared.plan.observation_retained_bytes(),
        retained,
        "subscribe allocates nothing"
    );

    // And unsubscribing puts it back, without disturbing the retained bytes either way.
    subscribe_all(&mut capable, false, 0);
    let _ = render(&mut capable, 1);
    assert_eq!(capable.prepared.plan.observation_binding_counts()[2], 0);
    assert_eq!(capable.prepared.plan.observation_retained_bytes(), retained);
}

/// **E2**: each bank lane publishes its own reduction, equal to an independent scalar run.
///
/// Red mutation: broadcast lane 0's reading to every lane -> lanes 1..W stop matching their scalar
/// twins, and the ordering assertion fails as well.
#[test]
fn every_bank_lane_publishes_its_own_reduction() {
    let mut session = prepare(Leg::AllArmed);
    subscribe_all(&mut session, true, WINDOW_BLOCKS);
    // Many attack constants, then exactly one window, so the published peak is a settled value the
    // scalar reference reproduces by folding the same blocks the same way.
    const SETTLE: usize = 64;
    let _ = render(&mut session, SETTLE);
    let mut published = Vec::new();
    for track in 0..TRACKS {
        published.push(reader(&session.handles, track).read().expect("a window"));
    }

    // The bit-exact half: an independent scalar compressor at the same threshold, fed the same
    // constant, folded the same way, reaches the same peak.
    for track in 0..TRACKS {
        let expected = scalar_reference(THRESHOLDS[track], SETTLE);
        assert_eq!(
            published[track].left.to_bits(),
            expected.left.to_bits(),
            "lane {track} (threshold {}) published its own reduction",
            THRESHOLDS[track]
        );
        assert_eq!(
            published[track].right.to_bits(),
            expected.right.to_bits(),
            "lane {track} right"
        );
    }

    // The structural half, which a broadcast cannot satisfy: 0 dB does not bite, and each deeper
    // threshold bites strictly harder.
    assert_eq!(published[0].left, 0.0, "threshold 0 dB does not bite");
    assert!(published[1].left > 0.0);
    assert!(
        published[2].left > published[1].left,
        "-24 dB bites harder than -18 dB"
    );
    assert!(
        published[3].left > published[2].left,
        "-30 dB bites harder than -24 dB"
    );
    for track in 0..4 {
        assert_eq!(
            published[track].left.to_bits(),
            published[track + 4].left.to_bits(),
            "two lanes at one threshold agree to the bit"
        );
    }
}

/// **E3**: the first window whose `first_sample >= applied_at_sample` reflects the new threshold,
/// the one before it does not, and the windows tile with no gap.
///
/// Red mutation: publish before `process` (the #137-E1 mirror) -> the reading is one block stale
/// and the first post-command window still carries the old threshold's reduction.
#[test]
fn the_first_window_at_or_after_applied_at_sample_reflects_the_command() {
    let mut session = prepare(Leg::AllArmed);
    // Track 0 starts at threshold 0 dB, which does not bite at this level.
    subscribe_all(&mut session, true, WINDOW_BLOCKS);
    let _ = render(&mut session, WINDOW_BLOCKS as usize * 4);
    let quiet = reader(&session.handles, 0).read().expect("a window");
    assert_eq!(quiet.left, 0.0, "before the command there is no reduction");
    assert_eq!(
        quiet.end_sample - quiet.first_sample,
        u64::from(WINDOW_BLOCKS) * QUANTUM as u64
    );

    // One batch, drained by one `stage` call at the top of one block: the threshold retarget and
    // the window it lands in share a timeline by construction rather than by two clocks agreeing.
    let applied_at = (session.block * QUANTUM) as u64;
    let producer = session
        .handles
        .effect_controls
        .iter_mut()
        .find(|producer| producer.track_id.as_ref() == "comp0")
        .expect("a control channel");
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        producer
            .producer
            .try_push(EffectControlRecord::Parameter {
                parameter_index: THRESHOLD_INDEX,
                channel,
                value: -30.0,
            })
            .expect("room");
    }

    // Collect every window boundary from here on, so tiling is checked rather than assumed.
    let mut boundaries = vec![(quiet.first_sample, quiet.end_sample)];
    let mut first_after = None;
    for _ in 0..8 {
        let _ = render(&mut session, WINDOW_BLOCKS as usize);
        let window = reader(&session.handles, 0).read().expect("a window");
        if window.first_sample == boundaries[boundaries.len() - 1].0 {
            continue;
        }
        boundaries.push((window.first_sample, window.end_sample));
        if window.first_sample >= applied_at && first_after.is_none() {
            first_after = Some(window);
        }
    }
    for pair in boundaries.windows(2) {
        assert_eq!(
            pair[0].1, pair[1].0,
            "windows tile with no gap: {:?} then {:?}",
            pair[0], pair[1]
        );
    }
    let first_after = first_after.expect("a window at or after the command");
    assert!(
        first_after.first_sample >= applied_at,
        "the window opens at or after the applied sample"
    );
    assert!(
        first_after.left > 0.0,
        "the first window at or after `applied_at_sample` reflects the new threshold ({})",
        first_after.left
    );
    // The window *before* the command is the quiet one, which is the other half of the statement.
    assert!(quiet.first_sample < applied_at);
}

/// **E13** (D7): a plan replacement drops every subscription, and re-subscribing works.
///
/// Red mutation: carry armed flags across the replacement -> the absence assertion fails.
#[test]
fn a_plan_replacement_drops_every_subscription() {
    let mut first = prepare(Leg::AllArmed);
    subscribe_all(&mut first, true, WINDOW_BLOCKS);
    let _ = render(&mut first, WINDOW_BLOCKS as usize * 2);
    assert_eq!(
        first.prepared.plan.observation_binding_counts()[2],
        TRACKS as u64,
        "the first plan is fully subscribed"
    );
    let before = reader(&first.handles, 3).read().expect("a window");
    assert!(before.left > 0.0);

    // The structural edit: a replacement plan, prepared from the same session. Subscriptions are
    // per-plan, so the new plan's lanes exist and are **unarmed** -- there is nowhere for an old
    // subscription to have been carried to.
    let mut second = prepare(Leg::AllArmed);
    assert_eq!(
        second.prepared.plan.observation_binding_counts(),
        [TRACKS as u64, TRACKS as u64, 0],
        "the replacement plan carries capacity and no subscription"
    );
    let _ = render(&mut second, WINDOW_BLOCKS as usize * 2);
    for track in 0..TRACKS {
        assert_eq!(
            reader(&second.handles, track).read(),
            None,
            "track {track} published nothing against an unsubscribed replacement plan"
        );
    }

    // Re-subscribing against the new plan works, and acks a fresh map: the sequence restarts at 1.
    subscribe_all(&mut second, true, WINDOW_BLOCKS);
    // Exactly one window, so the sequence number is checked rather than merely bounded.
    let _ = render(&mut second, WINDOW_BLOCKS as usize);
    let after = reader(&second.handles, 3).read().expect("a window");
    assert_eq!(after.sequence, 1, "the replacement plan starts a fresh map");
    assert!(
        after.first_sample >= before.end_sample,
        "and a fresh window, opened where the subscription was applied"
    );
    assert!(after.left > 0.0, "and it publishes again");
    // The retired plan's own readers are untouched by any of this; they simply stop advancing.
    assert_eq!(reader(&first.handles, 3).read(), Some(before));
}

const DYNAMIC_SESSION: &str =
    include_str!("../../../fixtures/session/v1/compressor-dynamic-observation.json");

/// A scalar compressor stepped block by block, with one threshold retarget, folded into windows.
///
/// The plan-side reader is conflating, so a window is read once per `WINDOW_BLOCKS` render batch;
/// this produces the same sequence from an independently prepared instance, applying the command
/// at the same block boundary the plan applies it at.
fn scalar_reference_windows(
    initial: f32,
    retarget: Option<(usize, f32)>,
    blocks: usize,
) -> Vec<ObservationSample> {
    let mut values: Vec<InitialParameterValue> = Vec::new();
    for (index, parameter) in compressor::COMPRESSOR_PARAMETERS.iter().enumerate() {
        let value = if index as u32 == THRESHOLD_INDEX {
            initial
        } else {
            parameter.default_value
        };
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    let request = PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: effect_contract::PortId::new("sidechain-in").expect("port"),
                required: false,
            },
        },
        initial_values: &values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 12,
            maximum_automation_spans_per_block: 128,
        },
    };
    let mut effect: Box<dyn PreparedNativeEffect> = compressor::CompressorFactory
        .prepare(request)
        .expect("scalar prepare");
    let mut windows = Vec::new();
    let mut peak = ObservationSample::default();
    for block in 0..blocks {
        let first_sample = (block * QUANTUM) as u64;
        let spans: Vec<PreparedAutomationSpan> = match retarget {
            Some((at, value)) if at == block => [ParameterChannel::Left, ParameterChannel::Right]
                .into_iter()
                .map(|channel| PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index: THRESHOLD_INDEX,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: value,
                    end_value: value,
                })
                .collect(),
            _ => Vec::new(),
        };
        let mut left = [LEVEL; QUANTUM];
        let mut right = [LEVEL; QUANTUM];
        let process = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            first_sample,
            &spans,
            QUANTUM as u32,
        )
        .expect("block");
        let report = effect.process(process);
        assert_eq!(
            report.invalid_spans, 0,
            "the reference accepted the retarget"
        );
        let mut sample = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut sample));
        peak.left = peak.left.max(sample.left.abs());
        peak.right = peak.right.max(sample.right.abs());
        if (block + 1) % WINDOW_BLOCKS as usize == 0 {
            windows.push(peak);
            peak = ObservationSample::default();
        }
    }
    windows
}

/// The per-node scalar publish site, which the banked fixture never reaches.
///
/// One track cannot fill a cohort, so this instance renders through
/// `graph::runtime::NodeKind::ConsoleEffect` rather than through a bank stage. Everything the
/// banked evals assert is asserted here for that path: the reading is the block's own, the window
/// tiles, and the value equals an independent scalar run to the bit.
///
/// Red mutation: publish **before** `process` in the `ConsoleEffect` arm of `execute_op` (the
/// #137-E1 mirror) -> the published window is one block stale and the bit comparison fails.
#[test]
fn the_per_node_scalar_path_publishes_its_own_block() {
    let request = HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32 * WINDOW_BLOCKS).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 4,
        master_track: Some(0),
    };
    let (_session, prepared, handles) =
        prepare_host_session_with_console(DYNAMIC_SESSION, &caps(), &request).unwrap_or_else(
            |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
        );
    assert_eq!(
        prepared.report.effect_bank_scratch_bytes, 0,
        "a single track cannot fill a cohort, so this is the per-node scalar path"
    );
    let mut session = Session {
        prepared,
        handles,
        block: 0,
    };
    assert_eq!(
        session.prepared.plan.observation_binding_counts(),
        [1, 1, 0],
        "one observed scalar stage, one declared tap, unarmed"
    );

    subscribe_all(&mut session, true, WINDOW_BLOCKS);

    // Four windows, read one at a time, with a threshold retarget in the middle. A settled reading
    // would not discriminate a publish that ran one block early -- once the reduction stops moving,
    // folding blocks `n-1..n+2` and `n..n+3` give the same peak. Retargeting mid-run keeps two of
    // the four windows on a moving envelope, where a one-block shift is visible.
    const RETARGET_BLOCK: usize = 8;
    const BLOCKS: usize = 16;
    let mut published = Vec::new();
    for window in 0..BLOCKS / WINDOW_BLOCKS as usize {
        if window * WINDOW_BLOCKS as usize == RETARGET_BLOCK {
            let producer = session
                .handles
                .effect_controls
                .iter_mut()
                .find(|producer| producer.track_id.as_ref() == "comp0")
                .expect("a control channel");
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                producer
                    .producer
                    .try_push(EffectControlRecord::Parameter {
                        parameter_index: THRESHOLD_INDEX,
                        channel,
                        value: -30.0,
                    })
                    .expect("room");
            }
        }
        let _ = render(&mut session, WINDOW_BLOCKS as usize);
        published.push(reader(&session.handles, 0).read().expect("a window"));
    }

    let expected = scalar_reference_windows(0.0, Some((RETARGET_BLOCK, -30.0)), BLOCKS);
    assert_eq!(published.len(), expected.len());
    for (index, (window, reference)) in published.iter().zip(&expected).enumerate() {
        assert_eq!(
            window.left.to_bits(),
            reference.left.to_bits(),
            "window {index} published its own blocks, not the previous block's state"
        );
        assert_eq!(window.right.to_bits(), reference.right.to_bits());
        assert_eq!(window.sequence, index as u64 + 1, "window {index} sequence");
        assert_eq!(
            window.first_sample,
            (index * WINDOW_BLOCKS as usize * QUANTUM) as u64,
            "window {index} opens where the last one closed"
        );
        assert_eq!(
            window.end_sample - window.first_sample,
            u64::from(WINDOW_BLOCKS) * QUANTUM as u64
        );
    }
    // The case is sharp: nothing before the retarget, a moving envelope after it.
    assert_eq!(published[0].left, 0.0, "threshold 0 dB does not bite");
    assert_eq!(published[1].left, 0.0);
    assert!(published[2].left > 0.0, "the retarget took effect");
    assert!(
        published[3].left > published[2].left,
        "the envelope is still attacking across the fourth window, so a one-block shift shows"
    );

    // Unsubscribing stops the traffic without disturbing the last window.
    subscribe_all(&mut session, false, 0);
    let _ = render(&mut session, WINDOW_BLOCKS as usize * 2);
    assert_eq!(session.prepared.plan.observation_binding_counts()[2], 0);
    let after = reader(&session.handles, 0).read().expect("the last window");
    assert_eq!(
        Some(after),
        published.last().copied(),
        "a disarmed tap publishes nothing new"
    );
}

/// Issue #143 R7: the retained-byte formula, asserted rather than pinned.
///
/// `observation_retained_bytes` is one row of `HostPrepareReport`, and a row that is only ever
/// compared against itself proves nothing. What is checked here is the *shape* of the number: it
/// is one accumulator row plus one conflating cell per declared tap per observed instance, and
/// nothing else. A future field that grows either one moves this equality rather than moving a
/// report row silently.
#[test]
fn observation_retained_bytes_are_the_declared_menu_times_one_row_and_one_slot() {
    let capable = prepare(Leg::CapacityUnarmed);
    let taps: usize = capable
        .handles
        .effect_observations
        .iter()
        .map(|handle| handle.readers.len())
        .sum();
    assert_eq!(taps, TRACKS, "eight compressors, one declared tap each");

    let retained = capable.prepared.report.observation_retained_bytes;
    let slot = engine::realtime::observation_slot_retained_bytes() as u64;
    assert!(retained > 0);
    assert_eq!(
        retained % taps as u64,
        0,
        "the total is a whole number of per-tap rows"
    );
    let per_tap = retained / taps as u64;
    assert!(
        per_tap > slot,
        "a tap is one conflating cell ({slot} B) plus its accumulator row, not just the cell"
    );
    // The measured decomposition, so a change to either half has to move this line.
    assert_eq!(retained, taps as u64 * per_tap);
    assert_eq!(slot, 48, "the conflating cell is eight atomic words");
    assert_eq!(
        per_tap, 104,
        "one 56-byte accumulator row plus one 48-byte cell"
    );
    assert_eq!(retained, 832);

    // And the row that reports it is the row the runtime actually holds.
    assert_eq!(retained, capable.prepared.plan.observation_retained_bytes());

    // The graph's own reported bytes did not move: a session with capacity and one without report
    // the same plan and metadata sizes, because the lane is behind a pointer in a boxed variant.
    let bare = prepare(Leg::ConsoleNoCapacity);
    assert_eq!(
        capable.prepared.report.graph_session_plus_plan_bytes,
        bare.prepared.report.graph_session_plus_plan_bytes,
        "observation capacity does not move the graph's reported plan bytes"
    );
    assert_eq!(
        capable.prepared.report.graph_metadata_bytes,
        bare.prepared.report.graph_metadata_bytes
    );
}

/// Observation capacity without a console is a rejection, not a silent half-attach.
#[test]
fn observation_without_a_control_channel_is_refused() {
    let request = HostConsoleRequest {
        control_queue_depth: None,
        meter_period_frames: Some(NonZeroU32::new(QUANTUM as u32 * WINDOW_BLOCKS).expect("period")),
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        meter_tap: MeterTap::PostMatrix,
        observation_taps: 4,
        master_track: None,
    };
    let failure = prepare_host_session_with_console(SESSION, &caps(), &request)
        .err()
        .expect("a subscription has no delivery path without a command queue");
    assert!(
        String::from_utf8_lossy(failure.as_bytes()).contains("host.observation.console"),
        "the diagnostic names the rule"
    );

    // And a designated master must name a track this session has.
    let request = HostConsoleRequest {
        control_queue_depth: Some(NonZeroUsize::new(8).expect("depth")),
        master_track: Some(TRACKS as u32),
        ..request
    };
    let failure = prepare_host_session_with_console(SESSION, &caps(), &request)
        .err()
        .expect("an out-of-range master designation is refused");
    assert!(
        String::from_utf8_lossy(failure.as_bytes()).contains("host.observation.master_track"),
        "the diagnostic names the rule"
    );
}

/// Issue #143 E7: the cost classes are what they claim.
///
/// Two halves, because "cost" has two honest meanings and only one of them is a stopwatch.
///
/// **The deterministic half** (this test). Level-2 zero says an unarmed tap's effect state is
/// *never read*. That is a statement about how many times the effect is asked, and it is counted
/// exactly here: the lane's `wants` gate is driven the way `graph::runtime::publish_observations`
/// drives it, against an effect that counts every call. No capacity means the loop does not
/// exist; capacity unarmed means `wants` refuses before the effect is touched; armed means
/// exactly one call per tap per block and not one more.
///
/// **The descriptive half** moved to
/// [`observation_cost_classes_are_separated_from_a_computed_scan_in_release`] below, `#[ignore]`d
/// for nightly, release-mode measurement: a wall clock on a shared debug-mode CI runner has no
/// fixed relationship to the shipped profile's speed (issue #359 WP-2, §10).
#[test]
fn observation_cost_classes_are_what_they_claim() {
    use effect_contract::{
        ObservationCadence, ObservationChannels, ObservationCost, ObservationDescriptor,
        ObservationFold, ObservationKind, ObservationLane, ObservationTapId, ParameterUnit,
    };
    use engine::realtime::observation_slot;
    use std::cell::Cell;

    static MENU: [ObservationDescriptor; 1] = [ObservationDescriptor {
        id: ObservationTapId(1),
        display_name: "Gain Reduction",
        display_unit: "dB",
        kind: ObservationKind::GainReductionDb,
        unit: ParameterUnit::Db,
        cost: ObservationCost::Resident,
        cadence: ObservationCadence::PerBlock,
        fold: ObservationFold::PeakMagnitude,
        channels: ObservationChannels::PerLane,
        minimum: 0.0,
        maximum: 100.0,
    }];

    // The counting stand-in for a prepared effect: exactly the surface the publish step touches.
    struct Counting {
        reads: Cell<u64>,
    }
    impl Counting {
        fn observe_resident(&self, _tap: u32, out: &mut ObservationSample) -> bool {
            self.reads.set(self.reads.get() + 1);
            out.left = -6.0;
            out.right = -6.0;
            true
        }
    }

    // The publish step, transcribed from `graph::runtime::publish_observations`.
    fn publish(lane: &mut ObservationLane, effect: &Counting, first_sample: u64, frames: u64) {
        let mut sample = ObservationSample::default();
        for tap in 0..lane.len() {
            if !lane.wants(tap) {
                continue;
            }
            if effect.observe_resident(tap as u32, &mut sample) {
                lane.accumulate(tap, sample, first_sample, frames);
            }
        }
    }

    const BLOCKS: u64 = 4_096;
    let effect = Counting {
        reads: Cell::new(0),
    };
    let (publisher, _reader) = observation_slot();
    let mut lane = ObservationLane::new(&MENU, vec![publisher], 4).expect("lane");

    // Capacity, unarmed: the loop runs and the effect is never asked.
    for block in 0..BLOCKS {
        publish(&mut lane, &effect, block * 128, 128);
    }
    assert_eq!(
        effect.reads.get(),
        0,
        "an unarmed tap's state is never read: that is the whole of level-2 zero"
    );

    // Armed: exactly one call per tap per block.
    lane.arm(0, true, 4, 0);
    for block in 0..BLOCKS {
        publish(&mut lane, &effect, block * 128, 128);
    }
    assert_eq!(
        effect.reads.get(),
        BLOCKS,
        "one read per tap per block, and not one more"
    );

    // Disarmed again: back to zero, immediately.
    let before = effect.reads.get();
    lane.arm(0, false, 0, 0);
    for block in 0..BLOCKS {
        publish(&mut lane, &effect, block * 128, 128);
    }
    assert_eq!(
        effect.reads.get(),
        before,
        "disarming stops the reads at once"
    );
}

/// Release-mode half of the E7 cost-class claim above (issue #143): a real eight-compressor plan
/// rendered in all four legs, timed, with a synthetic per-sample ring scan as the separating
/// negative control -- the shape a `Computed` tap would have if one shipped. The scan must be
/// *measurably* slower than the marginal cost of arming a tap over an unarmed console, or the
/// measurement is too coarse to have said anything, and that is the assertion. The three
/// observation legs are reported (printed) rather than pinned: a wall clock on a shared machine
/// is evidence, not a gate.
///
/// Red mutation: declare the limiter tap `Resident` but implement it as a per-sample ring scan ->
/// the armed row separates from the others, which is what the negative control calibrates for.
///
/// Debug-mode runner variance makes both wall-clock assertions below a coin flip at P95 on a
/// shared 4-vCPU CI runner, so this runs only in release, nightly, `--ignored` (issue #359 WP-2,
/// §10). The separating-control assertion compares `AllArmed` against `CapacityUnarmed`, not
/// `NoConsole`: in an optimized build, merely having a console object attached (present in
/// `ConsoleNoCapacity`, `CapacityUnarmed` and `AllArmed` alike) costs measurably more than the
/// baseline with no console at all, and that fixed cost can exceed the true marginal cost of
/// arming a tap by an order of magnitude. Subtracting `NoConsole` folded both costs into one
/// delta and made this a false red the first time it was actually run in release; `CapacityUnarmed`
/// isolates the cost this test is about.
#[test]
#[ignore = "release-mode budget; runs nightly"]
fn observation_cost_classes_are_separated_from_a_computed_scan_in_release() {
    use std::time::Instant;

    // Four legs of a real eight-compressor plan.
    const RENDER_BLOCKS: usize = 256;
    let mut measured = Vec::new();
    for leg in [
        Leg::NoConsole,
        Leg::ConsoleNoCapacity,
        Leg::CapacityUnarmed,
        Leg::AllArmed,
    ] {
        let mut session = prepare(leg);
        if leg == Leg::AllArmed {
            subscribe_all(&mut session, true, WINDOW_BLOCKS);
        }
        let _ = render(&mut session, 16); // warm the caches and settle the envelopes
        let start = Instant::now();
        let _ = render(&mut session, RENDER_BLOCKS);
        measured.push((leg, start.elapsed()));
    }

    // The separating negative control: what a `Computed` tap's shape actually costs. One pass per
    // sample per track over a ring the size of the plan's, which is the cheapest honest sketch of
    // an analysis pass -- and it must be measurably slower than every observation leg, or the
    // clock on this machine is too coarse for the comparison above to have meant anything.
    let mut ring = vec![0.0_f32; 8 * 128 * 4];
    let scan_start = Instant::now();
    let mut sink = 0.0_f32;
    for block in 0..RENDER_BLOCKS {
        for (index, slot) in ring.iter_mut().enumerate() {
            // Deliberately not a fused multiply-add: fusion belongs to
            // `lane` alone (D3), and this is a stopwatch control, not a kernel.
            *slot = index as f32 * 1e-6 + block as f32;
        }
        for slot in &ring {
            sink = sink.max(slot.abs());
        }
    }
    let scan = scan_start.elapsed();
    assert!(sink > 0.0, "the control is not optimised away");

    for (leg, elapsed) in &measured {
        println!(
            "E7 {leg:?}: {RENDER_BLOCKS} blocks in {:?} ({:.3} us/block)",
            elapsed,
            elapsed.as_secs_f64() * 1e6 / RENDER_BLOCKS as f64
        );
    }
    println!(
        "E7 synthetic computed scan: {scan:?} ({:.3} us/block)",
        scan.as_secs_f64() * 1e6 / RENDER_BLOCKS as f64
    );

    let armed = measured
        .iter()
        .find(|(leg, _)| *leg == Leg::AllArmed)
        .expect("armed leg")
        .1;
    let baseline = measured
        .iter()
        .find(|(leg, _)| *leg == Leg::NoConsole)
        .expect("baseline leg")
        .1;
    // The negative control isolates the cost of *arming* a tap, so it must be compared against a
    // leg that already pays for having a console attached but has not armed anything --
    // `CapacityUnarmed`, not `NoConsole`. In an optimized build the fixed cost of a console being
    // present at all (present in `ConsoleNoCapacity`, `CapacityUnarmed` and `AllArmed` alike) can
    // exceed the true per-tap arming cost by an order of magnitude, which made `NoConsole` an
    // unreliable zero-point here in release: it folded "having a console" and "arming a tap" into
    // one delta and measurably failed this assertion even though no tap was ever scanned per
    // sample. `NoConsole` remains the right zero-point for the coarse product-level gate below.
    let unarmed_with_console = measured
        .iter()
        .find(|(leg, _)| *leg == Leg::CapacityUnarmed)
        .expect("unarmed-with-console leg")
        .1;
    assert!(
        scan > armed
            .saturating_sub(unarmed_with_console)
            .max(core::time::Duration::from_nanos(1)),
        "the negative control ({scan:?}) must separate from the marginal cost of arming, or the \
         clock is too coarse for this comparison to say anything: armed={armed:?} \
         unarmed_with_console={unarmed_with_console:?}"
    );
    // A gate, not a pin: eight resident reads and eight `abs`/compare pairs per block cannot
    // plausibly double a plan that runs eight compressors. A regression that does is a regression.
    assert!(
        armed.as_secs_f64() < baseline.as_secs_f64() * 4.0 + 5e-3,
        "arming every tap cost far more than a copy out of state: armed={armed:?} \
         baseline={baseline:?}"
    );
}

/// Issue #163 phase 1b: the same eight compressors observe identically wherever they are placed.
///
/// `DYNAMIC_BANK_SESSION` is `SESSION` with every compressor moved from SIMD-1 into the dynamic
/// rack and nothing else changed. Before phase 1b that placement banked nothing, so this pair
/// would have compared a banked publish site against a per-node one; now both bank, and the #143
/// gain-reduction taps must resolve through `ConsoleEffectBankStage` on both sides and publish the
/// same windows to the bit.
///
/// This is what would catch a bank that meters its lanes in the wrong order: the eight thresholds
/// are all different, so a lane permutation inside the dynamic bank shows up as a permuted set of
/// reductions here even while the summed PCM stays identical.
#[test]
fn observation_is_identical_across_rack_placement() {
    let mut simd1 = prepare(Leg::AllArmed);
    let mut dynamic = prepare_from(DYNAMIC_BANK_SESSION, Leg::AllArmed);

    // Both placements bank: the fixture is eight identical program keys either way.
    assert!(simd1.prepared.report.effect_bank_scratch_bytes > 0);
    assert_eq!(
        dynamic.prepared.report.effect_bank_scratch_bytes,
        simd1.prepared.report.effect_bank_scratch_bytes,
        "the dynamic placement retains the same bank scratch as the SIMD-1 placement"
    );
    assert!(
        dynamic
            .handles
            .effect_observations
            .iter()
            .all(|handle| matches!(handle.rack, EffectRack::Dynamic)),
        "the taps address the rack the fixture actually declares"
    );

    subscribe_all(&mut simd1, true, WINDOW_BLOCKS);
    subscribe_all(&mut dynamic, true, WINDOW_BLOCKS);
    // The same settle the banked-lane test uses, so the published peak is a settled value rather
    // than a point on the attack ramp.
    const SETTLE: usize = 64;
    let simd1_pcm = render(&mut simd1, SETTLE);
    let dynamic_pcm = render(&mut dynamic, SETTLE);
    assert_eq!(
        simd1_pcm, dynamic_pcm,
        "rack placement must not change a rendered sample"
    );

    for (track, threshold) in THRESHOLDS.iter().enumerate() {
        let expected = reader(&simd1.handles, track).read().expect("a window");
        let observed = reader(&dynamic.handles, track).read().expect("a window");
        assert_eq!(
            observed, expected,
            "track {track}: the gain-reduction window must not depend on the rack"
        );
        // And it is a real reduction for the tracks whose threshold bites, not a shared zero.
        // Reduction is published as a positive magnitude in dB.
        if *threshold < 0.0 {
            assert!(
                observed.left > 0.0 && observed.right > 0.0,
                "track {track}: threshold {threshold} must reduce"
            );
        } else {
            assert_eq!(observed.left, 0.0, "threshold 0 dB does not bite");
        }
    }
    // Distinct thresholds must still produce distinct reductions through the dynamic bank, or a
    // lane permutation would pass the comparison above vacuously.
    let reductions: Vec<u32> = (0..TRACKS)
        .map(|track| {
            reader(&dynamic.handles, track)
                .read()
                .expect("a window")
                .left
                .to_bits()
        })
        .collect();
    let mut distinct = reductions.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(
        distinct.len(),
        4,
        "the four declared thresholds must give four distinct reductions: {reductions:?}"
    );
}
