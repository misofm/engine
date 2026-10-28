//! Issue #143 P3: the binding — E1, E2, E3, E5 and E13 against a real compiled plan.
//!
//! The fixture is eight compressor chains in one SIMD rack, so the cohort planner forms a real
//! homogeneous bank and the observation path under test is the banked one. Every track's builtins
//! are the identity and only the compressor's **threshold** differs, which keeps the eight
//! instances in one program key (so they still bank) while giving each lane a different reduction
//! that an independently prepared scalar instance can reproduce to the bit.

use core::num::NonZeroUsize;

use effect_contract::{
    EffectControlRecord, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ObservationSample, ParameterChannel, PrepareEffectLimits,
    PrepareEffectRequest, PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts,
    PreparedSidechainPort,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps,
    HostShapePolicy, LiveEffectRack, LiveValuesSuperseded, PreparedHost, SourceSubmission,
    compile_host_session, prepare_host_runtime_with_live_lanes, prepare_host_session,
    prepare_host_session_with_live_controls,
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
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
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
    /// (a) No live controls at all: the pre-#137 path.
    NoLiveControls,
    /// (b) Live controls, but no observation capacity: level-1 zero.
    LiveControlsNoCapacity,
    /// (c) Observation capacity, nothing armed: level-2 zero.
    CapacityUnarmed,
    /// (d) Every declared gain-reduction tap armed.
    AllArmed,
}

fn live_controls(leg: Leg) -> HostLiveControlRequest {
    HostLiveControlRequest {
        meter_window_blocks: WINDOW_BLOCKS,
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        observation_taps: match leg {
            Leg::NoLiveControls | Leg::LiveControlsNoCapacity => 0,
            Leg::CapacityUnarmed | Leg::AllArmed => 4,
        },
    }
}

struct Session {
    prepared: PreparedHost,
    /// The plan's gate handle: live values are cells, written under a stamp of it.
    gate: engine::realtime::PlanRevisionGate,
    handles: HostLiveControlHandles,
    block: usize,
}

/// One transaction: write `record` to the addressed effect's cell under a stamp of the plan's
/// gate and publish the revision.
fn write_cell(session: &mut Session, track_id: &str, record: EffectControlRecord) {
    let Session { handles, gate, .. } = session;
    let producer = handles
        .effect_controls
        .iter_mut()
        .find(|producer| producer.track_id.as_ref() == track_id)
        .expect("a control channel");
    let revision = gate.last_published() + 1;
    let stamp = gate.stamp(revision).expect("a legal revision");
    producer
        .write(stamp, record)
        .expect("the compressor has a cell for the record");
    gate.publish(revision);
}

fn prepare(leg: Leg) -> Session {
    prepare_from(SESSION, leg)
}

fn prepare_from(document: &str, leg: Leg) -> Session {
    // The baseline leg is the plain entry: no live controls at all.
    let (mut prepared, handles) = if leg == Leg::NoLiveControls {
        let (_session, prepared) =
            prepare_host_session(document, &caps()).unwrap_or_else(|failure| {
                panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
            });
        let handles = HostLiveControlHandles {
            strips: Vec::new(),
            track_count: 0,
            strip_controls: Vec::new(),
            effect_controls: Vec::new(),
            meters: Vec::new(),
            effect_observations: Vec::new(),
            route_controls: Vec::new(),
            follow_route_controls: Vec::new(),
            live_values_superseded: LiveValuesSuperseded::default(),
            jump_lengths: None,
        };
        (prepared, handles)
    } else {
        let (_session, prepared, handles) =
            prepare_host_session_with_live_controls(document, &caps(), &live_controls(leg))
                .unwrap_or_else(|failure| {
                    panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
                });
        assert_eq!(handles.strips.len(), TRACKS);
        (prepared, handles)
    };
    assert!(
        prepared.report.effect_bank_scratch_bytes > 0,
        "the cohort planner bound at least one homogeneous bank on this host"
    );
    let gate = prepared
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    Session {
        prepared,
        gate,
        handles,
        block: 0,
    }
}

/// Renders `blocks` quanta of the fixture constant and returns the rendered output words.
fn render(session: &mut Session, blocks: usize) -> Vec<u32> {
    render_at(session, blocks, LEVEL)
}

/// Renders `blocks` quanta of the constant `level` (`0.0` is silence) and returns the rendered
/// output words.
fn render_at(session: &mut Session, blocks: usize, level: f32) -> Vec<u32> {
    let mut rendered = Vec::with_capacity(blocks * QUANTUM * 2);
    let left = [level; QUANTUM];
    let right = [level; QUANTUM];
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
                RenderIo { output },
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
    subscribe_tagged(session, armed, window_blocks, 0)
}

/// [`subscribe_all`] with the records tagged `arm_tag`.
fn subscribe_tagged(session: &mut Session, armed: bool, window_blocks: u32, arm_tag: u64) -> u64 {
    let applied_at = (session.block * QUANTUM) as u64;
    for producer in session.handles.effect_controls.iter_mut() {
        producer
            .try_push_observe(0, armed, window_blocks, arm_tag)
            .expect("room in the observation queue");
    }
    applied_at
}

/// The tag [`bindings_of_all`] gives every binding, which a prepared arm publishes.
const PREPARED_TAG: u64 = 5;

/// The reader for one track's compressor tap, in whichever rack the fixture declared it.
fn reader(handles: &HostLiveControlHandles, track: usize) -> &engine::realtime::ObservationReader {
    let id = handles.strips[track].as_ref();
    let handle = handles
        .effect_observations
        .iter()
        .find(|handle| handle.track_id.as_ref() == id && handle.address.index == 0)
        .expect("an observation handle for the addressed effect");
    // The addressing is `(track, rack, index)`; the fixtures declare exactly one effect per track,
    // a console slot or an insert, so naming the rack here would only restate which fixture is
    // loaded.
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
        tail_bound: effect_compiler::launch_native_effect_registry()
            .expect("launch registry")
            .tail_bound(
                compressor::COMPRESSOR_DESCRIPTOR.id,
                48_000,
                EffectQuality::Normal,
            )
            .expect("a declared row"),
    };
    let mut effect: Box<dyn PreparedNativeEffect> = compressor::CompressorFactory
        .prepare(request)
        .expect("scalar prepare")
        .processor;
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
    let mut no_live_controls = prepare(Leg::NoLiveControls);
    let baseline = render(&mut no_live_controls, BLOCKS);
    assert!(
        baseline.iter().any(|word| *word != 0),
        "the corpus is not silence"
    );

    for leg in [
        Leg::LiveControlsNoCapacity,
        Leg::CapacityUnarmed,
        Leg::AllArmed,
    ] {
        let mut session = prepare(leg);
        if leg == Leg::AllArmed {
            subscribe_all(&mut session, true, WINDOW_BLOCKS);
        }
        let rendered = render(&mut session, BLOCKS);
        assert_eq!(
            rendered, baseline,
            "{leg:?} rendered different audio from the live-control-free path"
        );
        if leg == Leg::AllArmed {
            let window = reader(&session.handles, 3)
                .read()
                .expect("a published window");
            assert!(window.left > 0.0, "leg (d) published a real reduction");
        }
    }
}

/// #1585 gate 1: render, and only render, publishes each tap's arm state in its slot, on every
/// publish site: the console bank, the dynamic-rack bank and the per-node path. An arm pushed and
/// not yet rendered leaves the slot as it was; the block that drains it publishes the tap armed at
/// that block's first sample, before any window closes, and a disarm publishes it unarmed at its
/// own block's first sample, each with its record's tag.
///
/// Test value: red if the arm state is written anywhere but render, if the record's drain does not
/// publish it, or if a bank lane or the per-node path publishes another sample than its block's.
#[test]
fn render_publishes_each_taps_arm_state_on_every_leg() {
    use engine::realtime::ObservationState;
    let request = live_controls(Leg::CapacityUnarmed);
    for (document, tracks, banked) in [
        (SESSION, TRACKS, true),
        (DYNAMIC_BANK_SESSION, TRACKS, true),
        (DYNAMIC_SESSION, 1, false),
    ] {
        let (_session, mut prepared, handles) =
            prepare_host_session_with_live_controls(document, &caps(), &request).unwrap_or_else(
                |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
            );
        assert_eq!(
            prepared.report.effect_bank_scratch_bytes > 0,
            banked,
            "the leg renders through the publish site it names"
        );
        let gate = prepared
            .plan
            .take_revision_gate()
            .expect("the plan's gate handle");
        let mut session = Session {
            prepared,
            gate,
            handles,
            block: 0,
        };
        let states = |session: &Session| -> Vec<ObservationState> {
            (0..tracks)
                .map(|track| {
                    reader(&session.handles, track)
                        .read_state()
                        .expect("a quiescent read is whole")
                })
                .collect()
        };
        let _ = render(&mut session, 3);
        let unarmed = vec![ObservationState::default(); tracks];
        assert_eq!(states(&session), unarmed, "no tap is armed before an arm");

        let armed_at = subscribe_tagged(&mut session, true, WINDOW_BLOCKS, 7);
        assert_ne!(armed_at, 0);
        assert_eq!(states(&session), unarmed, "a queued arm writes no slot");
        let _ = render(&mut session, 1);
        let armed = ObservationState {
            window: None,
            armed: true,
            arm_sample: armed_at,
            arm_tag: 7,
        };
        assert_eq!(
            states(&session),
            vec![armed; tracks],
            "the block that drained the arm published it, at its first sample"
        );

        let disarmed_at = subscribe_tagged(&mut session, false, 0, 8);
        assert_eq!(
            states(&session),
            vec![armed; tracks],
            "a queued disarm writes no slot"
        );
        let _ = render(&mut session, 1);
        assert_eq!(
            states(&session),
            vec![
                ObservationState {
                    window: None,
                    armed: false,
                    arm_sample: disarmed_at,
                    arm_tag: 8,
                };
                tracks
            ],
            "the block that drained the disarm published it"
        );
    }
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
    for leg in [Leg::NoLiveControls, Leg::LiveControlsNoCapacity] {
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
    }

    // A capable plan, in contrast, binds exactly one lane per compressor and exactly one tap per
    // lane -- and arms none of them until it is asked to.
    let mut capable = prepare(Leg::CapacityUnarmed);
    assert_eq!(capable.handles.effect_observations.len(), TRACKS);
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
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        write_cell(
            &mut session,
            "comp0",
            EffectControlRecord::Parameter {
                parameter_index: THRESHOLD_INDEX,
                channel,
                value: -30.0,
            },
        );
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

/// A plan of `compiled` with live controls and observation capacity: fresh with no predecessor, or
/// the successor of `first`, swapped in at `first`'s next block, under the taps `bindings` names.
/// `committed` is the model `first` was prepared from.
fn prepare_successor(
    compiled: &host_core::CompiledSession,
    first: Option<(&mut Session, &session::SessionModel)>,
    bindings: &controls::ObservationBindings,
) -> Session {
    let request = live_controls(Leg::AllArmed);
    let (mut prepared, handles) = match &first {
        Some((first, committed)) => host_core::prepare_host_runtime_with_live_controls_successor(
            compiled,
            &caps(),
            &request,
            host_core::SuccessorBase {
                observation: bindings,
                ..host_core::SuccessorBase::new(&first.prepared.inventory, committed, 2)
            },
        ),
        None => host_core::prepare_host_runtime_with_live_controls(compiled, &caps(), &request),
    }
    .unwrap_or_else(|failure| panic!("prepare: {failure:?}"));
    let gate = prepared
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let mut block = 0;
    if let Some((first, _)) = first {
        assert_eq!(
            prepared
                .sources
                .adopt_persisting(&mut first.prepared.sources),
            1
        );
        // The swap happens at the successor's first block: the exchange runs inside it.
        assert_eq!(
            prepared
                .plan
                .adopt_predecessor_plan(&mut first.prepared.plan),
            engine::realtime::CarryOutcome::Carried
        );
        block = first.block;
    }
    Session {
        prepared,
        gate,
        handles,
        block,
    }
}

/// The table of every observed tap 0 of `session`, each armed with [`WINDOW_BLOCKS`].
fn bindings_of_all(session: &Session) -> controls::ObservationBindings {
    let mut bindings = controls::ObservationBindings::new();
    for handle in &session.handles.effect_observations {
        let effect = session::StableId::parse(&handle.effect_id).expect("a stable effect ID");
        let section = match handle.address.rack {
            LiveEffectRack::Console => controls::TapSection::Console(effect),
            LiveEffectRack::Inserts => controls::TapSection::Inserts(effect),
        };
        let key = controls::TapKey {
            strip: session::StableId::parse(&handle.track_id).expect("a stable strip ID"),
            section,
            tap_id: handle.descriptor.observations[0].id.0,
        };
        bindings.set(key.clone(), WINDOW_BLOCKS);
        assert!(bindings.set_arm_tag(&key, PREPARED_TAG));
    }
    assert_eq!(bindings.len(), TRACKS);
    bindings
}

/// **E13** (D7, rewritten by #1584 D6a): a replacement plan prepared with the taps the host armed
/// keeps every subscription, and one prepared with none starts unarmed.
///
/// The kept subscription opens its first window at the replacement's first block, so its windows
/// equal those of a fresh plan whose record arms the same tap at that block. A replacement prepared
/// from an empty table has the capacity, carries nothing and publishes nothing until the app
/// subscribes again, which works.
///
/// Red mutation: prepare the replacement's taps unarmed whatever the table holds -> the kept
/// replacement publishes nothing. Carry armed flags into a replacement with an empty table -> the
/// unarmed assertions fail.
#[test]
fn a_plan_replacement_keeps_the_subscriptions_the_host_armed() {
    // The replacement changes the console slot's link mode, a prepared setup: every lane restarts,
    // so nothing carries and the taps are the plan's own.
    let committed = compile_host_session(SESSION, &caps()).expect("compile");
    let changed_document = SESSION.replacen(
        "\"link_mode\": \"dual_mono\"",
        "\"link_mode\": \"average\"",
        1,
    );
    assert_ne!(changed_document, SESSION);
    let changed = compile_host_session(&changed_document, &caps()).expect("compile");
    let mut first = prepare_successor(&committed, None, &controls::ObservationBindings::new());
    subscribe_all(&mut first, true, WINDOW_BLOCKS);
    let _ = render(&mut first, WINDOW_BLOCKS as usize * 2);
    assert_eq!(
        first.prepared.plan.observation_binding_counts()[2],
        TRACKS as u64,
        "the first plan is fully subscribed"
    );
    let before = reader(&first.handles, 3).read().expect("a window");
    assert!(before.left > 0.0);
    let bindings = bindings_of_all(&first);

    // Prepared with the table: every tap is armed before the plan renders, and no record was queued.
    let mut kept = prepare_successor(
        &changed,
        Some((&mut first, committed.normalized_model())),
        &bindings,
    );
    assert_eq!(
        kept.prepared.plan.observation_binding_counts(),
        [TRACKS as u64, TRACKS as u64, TRACKS as u64],
        "the replacement is armed at preparation"
    );
    for producer in &kept.handles.effect_controls {
        assert_eq!(
            producer.observe_available_capacity(),
            producer.observe_capacity(),
            "the arm queued no record: a carried tap would be restarted by one"
        );
    }
    let opens_at = (kept.block * QUANTUM) as u64;
    // #1585 R2: render publishes a prepared arm, with its binding's tag, at the plan's first
    // block; until then the slot holds no arm, which a reader that knows the tag reports pending.
    assert!(
        (0..TRACKS).all(|track| !reader(&kept.handles, track)
            .read_state()
            .expect("a whole read")
            .armed),
        "a prepared arm writes no slot before render"
    );
    let _ = render(&mut kept, WINDOW_BLOCKS as usize);
    for track in 0..TRACKS {
        let state = reader(&kept.handles, track)
            .read_state()
            .expect("a whole read");
        assert_eq!(
            (state.armed, state.arm_sample, state.arm_tag),
            (true, opens_at, PREPARED_TAG),
            "track {track}: the first block published the prepared arm"
        );
    }
    assert_eq!(
        kept.prepared.plan.observation_binding_counts()[2],
        TRACKS as u64,
        "and it stays armed through the swap block"
    );

    // The reference: a fresh plan of the same session whose record arms the tap at its first
    // block. The replacement's effects restarted, so both run from rest on the same input.
    let mut reference = prepare_successor(&changed, None, &controls::ObservationBindings::new());
    subscribe_all(&mut reference, true, WINDOW_BLOCKS);
    let _ = render(&mut reference, WINDOW_BLOCKS as usize);
    for track in 0..TRACKS {
        let window = reader(&kept.handles, track)
            .read()
            .unwrap_or_else(|| panic!("track {track} publishes on the replacement"));
        let expected = reader(&reference.handles, track)
            .read()
            .expect("the reference publishes");
        assert_eq!(
            window.first_sample, opens_at,
            "the window opens at the replacement's first block"
        );
        assert_eq!(
            (window.sequence, window.blocks, window.left, window.right),
            (
                expected.sequence,
                expected.blocks,
                expected.left,
                expected.right
            ),
            "track {track}: the prepared arm equals a record at the first block"
        );
    }

    // Prepared with an empty table, as `MonitorBase::NONE` and no subscription leave it: the
    // capacity is there, and no subscription is.
    let mut bare = prepare_successor(
        &committed,
        Some((&mut kept, changed.normalized_model())),
        &controls::ObservationBindings::new(),
    );
    assert_eq!(
        bare.prepared.plan.observation_binding_counts(),
        [TRACKS as u64, TRACKS as u64, 0],
        "the replacement carries capacity and no subscription"
    );
    let _ = render(&mut bare, WINDOW_BLOCKS as usize * 2);
    for track in 0..TRACKS {
        assert_eq!(
            reader(&bare.handles, track).read(),
            None,
            "track {track} published nothing against an unsubscribed replacement plan"
        );
    }

    // Re-subscribing against the new plan works, and acks a fresh map: the sequence restarts at 1.
    let resubscribed_at = subscribe_all(&mut bare, true, WINDOW_BLOCKS);
    // Exactly one window, so the sequence number is checked rather than merely bounded.
    let _ = render(&mut bare, WINDOW_BLOCKS as usize);
    let after = reader(&bare.handles, 3).read().expect("a window");
    assert_eq!(after.sequence, 1, "the replacement plan starts a fresh map");
    assert_eq!(
        after.first_sample, resubscribed_at,
        "and a fresh window, opened where the subscription was applied"
    );
    assert!(after.left > 0.0, "and it publishes again");
    // The retired plan's own readers are untouched by any of this; they simply stop advancing.
    assert_eq!(reader(&first.handles, 3).read(), Some(before));

    // #1585 R2 on a silent strip: the prepared arm is published by the first block's drain, which
    // no silence skip or resident read passes by, so a replacement whose first block is silence
    // reads armed after it.
    let bindings = bindings_of_all(&bare);
    let mut silent = prepare_successor(
        &changed,
        Some((&mut bare, committed.normalized_model())),
        &bindings,
    );
    let silent_at = (silent.block * QUANTUM) as u64;
    let _ = render_at(&mut silent, 1, 0.0);
    for track in 0..TRACKS {
        let state = reader(&silent.handles, track)
            .read_state()
            .expect("a whole read");
        assert_eq!(
            (state.armed, state.arm_sample, state.arm_tag),
            (true, silent_at, PREPARED_TAG),
            "track {track}: a silent first block published the prepared arm"
        );
    }
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
        tail_bound: effect_compiler::launch_native_effect_registry()
            .expect("launch registry")
            .tail_bound(
                compressor::COMPRESSOR_DESCRIPTOR.id,
                48_000,
                EffectQuality::Normal,
            )
            .expect("a declared row"),
    };
    let mut effect: Box<dyn PreparedNativeEffect> = compressor::CompressorFactory
        .prepare(request)
        .expect("scalar prepare")
        .processor;
    let mut windows = Vec::new();
    let mut peak = ObservationSample::default();
    for block in 0..blocks {
        let first_sample = (block * QUANTUM) as u64;
        let spans: Vec<PreparedAutomationSpan> = match retarget {
            Some((at, value)) if at == block => [ParameterChannel::Left, ParameterChannel::Right]
                .into_iter()
                .map(|channel| PreparedAutomationSpan {
                    channel,
                    parameter_index: THRESHOLD_INDEX,
                    sample: first_sample,
                    value,
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
/// `graph::runtime::NodeKind::LiveControlEffect` rather than through a bank stage. Everything the
/// banked evals assert is asserted here for that path: the reading is the block's own, the window
/// tiles, and the value equals an independent scalar run to the bit.
///
/// Red mutation: publish **before** `process` in the `LiveControlEffect` arm of `execute_op` (the
/// #137-E1 mirror) -> the published window is one block stale and the bit comparison fails.
#[test]
fn the_per_node_scalar_path_publishes_its_own_block() {
    let request = HostLiveControlRequest {
        meter_window_blocks: WINDOW_BLOCKS,
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        observation_taps: 4,
    };
    let (_session, mut prepared, handles) =
        prepare_host_session_with_live_controls(DYNAMIC_SESSION, &caps(), &request).unwrap_or_else(
            |failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())),
        );
    assert_eq!(
        prepared.report.effect_bank_scratch_bytes, 0,
        "a single track cannot fill a cohort, so this is the per-node scalar path"
    );
    let gate = prepared
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let mut session = Session {
        prepared,
        gate,
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
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                write_cell(
                    &mut session,
                    "comp0",
                    EffectControlRecord::Parameter {
                        parameter_index: THRESHOLD_INDEX,
                        channel,
                        value: -30.0,
                    },
                );
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

/// Observation capacity without effect lanes is a rejection, not a silent half-attach.
#[test]
fn observation_without_effect_lanes_is_refused() {
    let request = HostLiveControlRequest {
        meter_window_blocks: WINDOW_BLOCKS,
        meter_queue_depth: NonZeroUsize::new(16).expect("meter depth"),
        observation_taps: 4,
    };
    let compiled = compile_host_session(SESSION, &caps()).expect("compiles");
    let failure = prepare_host_runtime_with_live_lanes(
        &compiled,
        &caps(),
        &request,
        HostLiveLanes::FADER_AND_MATRIX,
        &LiveValuesSuperseded::default(),
    )
    .err()
    .expect("a subscription has no delivery path without effect lanes");
    assert!(
        String::from_utf8_lossy(failure.as_bytes()).contains("host.observation.live_controls"),
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
        bypass_observation: Some(0.0),
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
    lane.arm(0, true, 4, 0, 0);
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
    lane.arm(0, false, 0, 0, 0);
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
/// *measurably* slower than the marginal cost of arming a tap over unarmed live controls, or the
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
/// `NoLiveControls`: in an optimized build, merely having a live-control object attached (present
/// in `LiveControlsNoCapacity`, `CapacityUnarmed` and `AllArmed` alike) costs measurably more than
/// the baseline with no live controls at all, and that fixed cost can exceed the true marginal cost
/// of arming a tap by an order of magnitude. Subtracting `NoLiveControls` folded both costs into
/// one delta and made this a false red the first time it was actually run in release;
/// `CapacityUnarmed` isolates the cost this test is about.
#[test]
#[ignore = "release-mode budget; runs nightly"]
fn observation_cost_classes_are_separated_from_a_computed_scan_in_release() {
    use std::time::Instant;

    // Four legs of a real eight-compressor plan.
    const RENDER_BLOCKS: usize = 256;
    let mut measured = Vec::new();
    for leg in [
        Leg::NoLiveControls,
        Leg::LiveControlsNoCapacity,
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

    // A reference figure only, printed below for context: what a `Computed` tap's shape actually
    // costs. One pass per sample per track over a ring the size of the plan's, which is the
    // cheapest honest sketch of an analysis pass. It is not compared against the observation legs
    // (see the load-bearing assertion below, which compares `AllArmed` against
    // `CapacityUnarmed` directly).
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
        .find(|(leg, _)| *leg == Leg::NoLiveControls)
        .expect("baseline leg")
        .1;
    // The negative control isolates the cost of *arming* a tap, so it must be compared against a
    // leg that already pays for having live controls attached but has not armed anything --
    // `CapacityUnarmed`, not `NoLiveControls`. In an optimized build the fixed cost of live
    // controls being present at all (present in `LiveControlsNoCapacity`, `CapacityUnarmed` and
    // `AllArmed` alike) can exceed the true per-tap arming cost by an order of magnitude, which
    // made `NoLiveControls` an unreliable zero-point here in release: it folded "having live
    // controls" and "arming a tap" into one delta and measurably failed this assertion even though
    // no tap was ever scanned per sample. `NoLiveControls` remains the right zero-point for the
    // coarse product-level gate below.
    let unarmed_with_live_controls = measured
        .iter()
        .find(|(leg, _)| *leg == Leg::CapacityUnarmed)
        .expect("unarmed-with-live-controls leg")
        .1;
    // The load-bearing claim: arming eight taps is not measurably slower than attached-but-
    // unarmed live controls. `AllArmed` is not reliably slower than `CapacityUnarmed` at all (the
    // two legs differ by only eight resident reads and eight `abs`/compare pairs per block, well
    // inside run-to-run noise), so comparing the marginal delta against the synthetic scan is
    // inert: a scan-sized regression in arming cost can leave `armed` still faster than
    // `unarmed_with_live_controls`, and a `saturating_sub` clamped to a positive floor would pass
    // no matter what was measured. Assert the bound directly instead.
    assert!(
        armed.as_secs_f64() <= unarmed_with_live_controls.as_secs_f64() * 1.10 + 50e-6,
        "arming eight taps must not measurably exceed the cost of attached-but-unarmed \
         live controls: armed={armed:?} \
         unarmed_with_live_controls={unarmed_with_live_controls:?} (allowed up to 10% + 50us \
         over unarmed_with_live_controls)"
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
/// gain-reduction taps must resolve through `LiveControlEffectBankStage` on both sides and publish
/// the same windows to the bit.
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
            .all(|handle| handle.address.rack == LiveEffectRack::Inserts),
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

/// #1586 D3, D4: host-core's read checks the whole batch before it writes a row. A batch whose
/// last selection is bad, or whose last selection names an earlier one's tap with other lanes,
/// leaves every row as it was and names that selection; a good batch reads each tap's own window.
///
/// Test value: red if the read writes a row before the whole batch passed (no browser test gives
/// a partly bad batch a caller-owned output), if the duplicate rule tells two selections of one tap
/// apart by their lanes, or if a refusal names another selection.
#[test]
fn a_read_checks_the_whole_batch_before_it_writes_a_row() {
    use host_core::observation::{
        ObservationIndex, ObservationIndexEntry, ObservationReadChannels, ObservationReadError,
        ObservationReadStatus, ObservationRefusal, ObservationRow, ObservationSelection, read_into,
    };
    let mut session = prepare(Leg::AllArmed);
    subscribe_all(&mut session, true, WINDOW_BLOCKS);
    let _ = render(&mut session, WINDOW_BLOCKS as usize * 2);
    let observations = &session.handles.effect_observations;
    let index = ObservationIndex::new(
        observations
            .iter()
            .enumerate()
            .map(|(position, handle)| ObservationIndexEntry {
                strip_id: handle.track_id.clone(),
                section: handle.address.rack,
                effect_id: handle.effect_id.clone(),
                handle: Some(u32::try_from(position).expect("a small plan")),
                effect: handle.descriptor,
            })
            .collect(),
        observations,
    )
    .expect("one entry per instance");
    let tap_id = observations[0].descriptor.observations[0].id.0;
    let select = |track: usize, channels| {
        let handle = &observations[track];
        ObservationSelection {
            strip_id: &handle.track_id,
            section: handle.address.rack,
            effect_id: &handle.effect_id,
            tap_id,
            channels,
        }
    };
    let both = ObservationReadChannels::Both;
    let sentinel = ObservationRow {
        status: ObservationReadStatus::Ready,
        left: Some(-7.0),
        ..ObservationRow::default()
    };
    let refused = |selections: &[ObservationSelection<'_>]| {
        let mut rows = [sentinel; 3];
        let error =
            read_into(&index, selections, |_| (0, false), &mut rows).expect_err("a refused batch");
        assert_eq!(rows, [sentinel; 3], "a refused batch writes no row");
        error
    };
    let unknown_tap = ObservationSelection {
        tap_id: 99,
        ..select(2, both)
    };
    assert_eq!(
        refused(&[select(0, both), select(1, both), unknown_tap]),
        ObservationReadError {
            selection: 2,
            refusal: ObservationRefusal::UnknownTap,
        }
    );
    assert_eq!(
        refused(&[
            select(0, ObservationReadChannels::Left),
            select(1, both),
            select(0, ObservationReadChannels::Right),
        ]),
        ObservationReadError {
            selection: 2,
            refusal: ObservationRefusal::Duplicate,
        }
    );
    let unknown_strip = ObservationSelection {
        strip_id: "no-such-strip",
        ..select(1, both)
    };
    assert_eq!(
        refused(&[select(0, both), unknown_strip]),
        ObservationReadError {
            selection: 1,
            refusal: ObservationRefusal::UnknownAddress,
        }
    );

    let mut rows = [ObservationRow::default(); 2];
    read_into(
        &index,
        &[select(0, both), select(5, ObservationReadChannels::Right)],
        |_| (0, false),
        &mut rows,
    )
    .expect("a good batch");
    for (row, track) in rows.iter().zip([0, 5]) {
        let window = observations[track].readers[0].read().expect("a window");
        assert_eq!(row.status, ObservationReadStatus::Ready);
        assert_eq!(row.window, Some(window));
    }
    assert_eq!(rows[1].left, None);
    assert_eq!(rows[1].right, Some(rows[1].window.expect("a window").right));
}
