//! Issue #1581 gate 1: every preparation entry meters the same strips the same way.
//!
//! One `PostMatrix` meter per strip, in strip order (the tracks, then the submixes), whose
//! snapshots carry the sample peak alone ([`host_core::meter_selection`]). No entry keeps a meter
//! set of its own, so two hosts or two boot shapes cannot meter different strips.

use core::num::NonZeroUsize;

use builtins::{MeterMetricSet, MeterTap};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps,
    HostShapePolicy, LiveValuesSuperseded, PrepareDiagnostics, PreparedHost,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumPreparation, SpectrumTarget, SuccessorBase, compile_host_session,
    meter_selection, prepare_host_runtime, prepare_host_runtime_with_live_controls,
    prepare_host_runtime_with_live_controls_successor, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_live_lanes_successor, prepare_host_runtime_with_selected_meters,
    prepare_host_runtime_with_selected_meters_successor, prepare_host_session_with_live_controls,
    spectrum_capture_collection_resources, spectrum_capture_resources_for,
};
use session::{
    RouteDestination, RouteSource, SendTap, StableId, Submix, canonical_session_json,
    parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/observation-frame-shape.json");
const QUANTUM: usize = 128;
/// The strips in strip order: the three tracks, then the two submixes.
const STRIPS: [&str; 5] = ["t0", "t1", "t2", "aaa-bus", "zzz-bus"];

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
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

/// The three-track observation fixture plus two submixes declared out of order (`zzz-bus`, then
/// `aaa-bus`); `t0` feeds `zzz-bus`, `t1` feeds `aaa-bus`, and both buses feed the output.
fn document() -> String {
    let mut model = parse_session_json(FIXTURE).expect("observation fixture parses");
    for (bus, feeder) in [("zzz-bus", 0_usize), ("aaa-bus", 1)] {
        let id = StableId::parse(bus).expect("bus id");
        model
            .submixes
            .push(Submix::unity(id.clone(), &model.console));
        let mut out = model.routes[feeder].clone();
        model.routes[feeder].destination = RouteDestination::SubmixInput {
            submix_id: id.clone(),
        };
        out.id = StableId::parse(&format!("{bus}-main")).expect("route id");
        out.source = RouteSource::Submix {
            submix_id: id,
            tap: SendTap::PostPan,
        };
        model.routes.push(out);
    }
    canonical_session_json(&model).expect("canonical bus session")
}

/// A one-block window, so one rendered block publishes one snapshot per meter.
fn request() -> HostLiveControlRequest {
    HostLiveControlRequest {
        meter_window_blocks: 1,
        meter_queue_depth: NonZeroUsize::new(2).expect("depth"),
        ..HostLiveControlRequest::default()
    }
}

fn single() -> SpectrumCaptureRequest {
    let target = SpectrumTarget::TrackPostMatrix("t0".into());
    SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    }
}

fn collection() -> SpectrumCaptureCollectionRequest {
    let entries = vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix("t0".into()),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::Output("main-out".into()),
            channels: SpectrumChannels::Stereo,
        },
    ];
    SpectrumCaptureCollectionRequest {
        maximum_capture_bytes: spectrum_capture_collection_resources(&entries)
            .expect("collection resources")
            .retained_bytes,
        entries,
    }
}

/// `entry`'s meters are one `PostMatrix` reader per strip in strip order, and one rendered block
/// publishes, on each, a snapshot whose only metric is the sample peak.
fn assert_core_meters(
    entry: &str,
    prepared: Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics>,
) {
    let (mut prepared, mut handles) = prepared.unwrap_or_else(|failure| {
        panic!("{entry}: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let meters: Vec<(&str, MeterTap)> = handles
        .meters
        .iter()
        .map(|meter| (meter.track_id.as_ref(), meter.tap))
        .collect();
    assert_eq!(
        meters,
        STRIPS.map(|strip| (strip, MeterTap::PostMatrix)),
        "{entry}: one post-matrix meter per strip, in strip order"
    );
    let mut samples = [0.0_f32; QUANTUM * 2];
    let output = PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("planes");
    prepared
        .plan
        .render(RenderIo { output }, RenderTime { absolute_sample: 0 })
        .unwrap_or_else(|error| panic!("{entry}: render: {error:?}"));
    for meter in &mut handles.meters {
        let snapshot = meter
            .consumer
            .try_pop()
            .unwrap_or_else(|_| panic!("{entry}: {}: one window per block", meter.track_id));
        assert_eq!(
            snapshot.present_metrics,
            MeterMetricSet::SAMPLE_PEAK,
            "{entry}: {}: the sample peak alone",
            meter.track_id
        );
    }
}

/// Red if any entry keeps a meter set of its own: the per-track all-metric default, a host's own
/// list, or none at all.
#[test]
fn every_preparation_entry_meters_every_strip_with_the_core_selection() {
    let document = document();
    let caps = caps();
    let compiled = compile_host_session(&document, &caps).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let model = compiled.normalized_model();
    let live = request();
    let lanes = HostLiveLanes::ALL;
    let superseded = LiveValuesSuperseded::default();
    let selection = meter_selection(model);
    // Every successor displaces one plan: the plain entry's, which has no live controls.
    let predecessor = prepare_host_runtime(&compiled, &caps).expect("the predecessor prepares");
    let base = || SuccessorBase::new(&predecessor.inventory, model, 2);

    assert_core_meters(
        "session_with_live_controls",
        prepare_host_session_with_live_controls(&document, &caps, &live)
            .map(|(_, prepared, handles)| (prepared, handles)),
    );
    assert_core_meters(
        "with_live_controls",
        prepare_host_runtime_with_live_controls(&compiled, &caps, &live),
    );
    assert_core_meters(
        "with_live_controls_successor",
        prepare_host_runtime_with_live_controls_successor(&compiled, &caps, &live, base()),
    );
    assert_core_meters(
        "with_live_lanes",
        prepare_host_runtime_with_live_lanes(&compiled, &caps, &live, lanes, &superseded),
    );
    assert_core_meters(
        "with_live_lanes_successor",
        prepare_host_runtime_with_live_lanes_successor(
            &compiled,
            &caps,
            &live,
            lanes,
            base(),
            &superseded,
        ),
    );
    for (label, spectrum) in [
        (
            "with_live_lanes_and_spectrum(single)",
            SpectrumPreparation::Single(single()),
        ),
        (
            "with_live_lanes_and_spectrum(collection)",
            SpectrumPreparation::Collection(collection()),
        ),
    ] {
        assert_core_meters(
            label,
            prepare_host_runtime_with_live_lanes_and_spectrum(
                &compiled,
                &caps,
                &live,
                lanes,
                Some(&spectrum),
                None,
                &superseded,
            )
            .map(|(prepared, handles, _)| (prepared, handles)),
        );
    }
    assert_core_meters(
        "with_selected_meters(meter_selection)",
        prepare_host_runtime_with_selected_meters(
            &compiled,
            &caps,
            &live,
            &selection,
            lanes,
            &superseded,
        ),
    );
    assert_core_meters(
        "with_selected_meters_successor(meter_selection)",
        prepare_host_runtime_with_selected_meters_successor(
            &compiled,
            &caps,
            &live,
            &selection,
            lanes,
            &superseded,
            base(),
        ),
    );
}

/// The builtin meter's sanitized magnitude, scalar: `|x|` for a finite, non-subnormal `x`, else
/// `+0.0` (the browser's `meter_sample_magnitude`).
fn magnitude(sample: f32) -> f32 {
    let magnitude = sample.abs();
    if (f32::MIN_POSITIVE..f32::INFINITY).contains(&magnitude) {
        magnitude
    } else {
        0.0
    }
}

/// The sample-serial scalar fold of [`magnitude`] over `samples`, from `+0.0`.
fn scalar_peak(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0_f32, |peak, &sample| {
        let magnitude = magnitude(sample);
        if magnitude > peak { magnitude } else { peak }
    })
}

/// Sample `index` of plane `plane`: a deterministic signal below 0.5 in magnitude, a rare larger
/// kept sample, and every kind of value the sanitizer drops (NaN, both infinities, the largest
/// subnormal and a negative one), scattered over lanes and tails.
fn output_sample(plane: usize, index: u64) -> f32 {
    let special = [
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(0x007f_ffff),
        -f32::from_bits(1),
    ];
    let mixed = index
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add(plane as u64 * 0x632b_e59b);
    match mixed % 61 {
        0..=4 => special[(mixed % 61) as usize],
        // A kept sample larger than the rest, rarely, so windows differ.
        5 => {
            (if index.is_multiple_of(2) {
                0.75_f32
            } else {
                -0.875
            }) * (1.0 + plane as f32 / 8.0)
        }
        _ => ((mixed >> 11) as f32 / (1_u64 << 53) as f32 - 0.5) * 0.9,
    }
}

fn output_block(first: u64, frames: usize) -> [Vec<f32>; 2] {
    [0, 1].map(|plane| {
        (0..frames as u64)
            .map(|frame| output_sample(plane, first + frame))
            .collect()
    })
}

/// #1581 gate 3. Windows tile `[start, end)` from the first observed sample with no gap, each at
/// least the window long and shorter than a window plus a block, and each window's peak pair is
/// the scalar sample-serial fold of the sanitized rule over its samples, bit for bit, at the
/// browser's quanta and at an odd block length that leaves a tail after the lanes.
///
/// Test value: red if a window leaves a gap or overlaps the next, if a sanitized NaN, infinity or
/// subnormal reaches a peak, or if the lane fold skips its tail or its upper lanes.
#[test]
fn output_windows_tile_and_peak_as_the_scalar_fold() {
    const WINDOW: u32 = 300;
    for quantum in [64_usize, 128, 512, 13] {
        let (mut observer, mut reader, gate) =
            host_core::output_peak_meter(core::num::NonZeroU32::new(WINDOW).expect("window"))
                .expect("output meter");
        gate.activate(0);
        let mut expected_start = 0_u64;
        let mut windows = 0;
        for block in 0..(4 * WINDOW as usize).div_ceil(quantum) as u64 + 1 {
            let first = block * quantum as u64;
            let [left, right] = output_block(first, quantum);
            observer.observe(&left, &right, first, first + quantum as u64);
            while let Ok(window) = reader.windows_mut().try_pop() {
                assert_eq!(window.start_sample, expected_start, "{quantum}: no gap");
                let frames = window.end_sample - window.start_sample;
                assert!(
                    frames >= u64::from(WINDOW) && frames < u64::from(WINDOW) + quantum as u64,
                    "{quantum}: whole blocks, at least the window: {frames}"
                );
                let [left, right] = output_block(window.start_sample, frames as usize);
                assert_eq!(
                    window.peak.map(f32::to_bits),
                    [scalar_peak(&left), scalar_peak(&right)].map(f32::to_bits),
                    "{quantum}: window at {}",
                    window.start_sample
                );
                expected_start = window.end_sample;
                windows += 1;
            }
        }
        assert!(windows >= 3, "{quantum}: {windows} windows closed");
        assert_eq!(reader.dropped_windows(), 0);
    }
}

/// #1581 gate 3. While the gate is inactive, or before its first sample, a block folds nothing and
/// closes nothing; deactivation discards the open window, and the next activation opens a clean
/// one at its first block.
///
/// Test value: red if the meter measures while inactive, folds a block that starts before the
/// activation sample, or carries an open window across a deactivation.
#[test]
fn an_inactive_output_gate_folds_nothing() {
    const QUANTUM: u64 = 64;
    let (mut observer, mut reader, gate) =
        host_core::output_peak_meter(core::num::NonZeroU32::new(128).expect("window"))
            .expect("output meter");
    let loud = vec![0.5_f32; QUANTUM as usize];
    let quiet = vec![0.125_f32; QUANTUM as usize];
    let mut block = 0_u64;
    let mut observe = |samples: &[f32]| {
        observer.observe(samples, samples, block * QUANTUM, (block + 1) * QUANTUM);
        block += 1;
    };
    for _ in 0..4 {
        observe(&loud);
    }
    assert_eq!(
        reader.windows_mut().available_at_entry(),
        0,
        "an inactive gate closes no window"
    );
    // Activated mid-block: the block that straddles the activation sample is not observed.
    gate.activate(4 * QUANTUM + 1);
    observe(&loud);
    observe(&quiet);
    // Deactivated with a window open: it is discarded.
    gate.deactivate();
    observe(&loud);
    // The same activation sample again: only the deactivation can have closed the open window.
    gate.activate(4 * QUANTUM + 1);
    observe(&quiet);
    observe(&quiet);
    let window = reader
        .windows_mut()
        .try_pop()
        .expect("one window after reactivation");
    assert_eq!(
        (window.start_sample, window.end_sample, window.peak),
        (7 * QUANTUM, 9 * QUANTUM, [0.125, 0.125])
    );
    assert!(reader.windows_mut().try_pop().is_err());
}

/// #1581 gate 3. A full queue keeps the oldest windows, drops each newer one and counts it.
///
/// Test value: red if a full queue overwrites a queued window, or drops a window without counting
/// it.
#[test]
fn a_full_output_queue_drops_the_newest_window_and_counts_it() {
    const QUANTUM: u64 = 64;
    let (mut observer, mut reader, gate) =
        host_core::output_peak_meter(core::num::NonZeroU32::new(64).expect("window"))
            .expect("output meter");
    gate.activate(0);
    let block = vec![0.25_f32; QUANTUM as usize];
    let blocks = host_core::METER_QUEUE_DEPTH as u64 + 2;
    for index in 0..blocks {
        observer.observe(&block, &block, index * QUANTUM, (index + 1) * QUANTUM);
    }
    assert_eq!(reader.dropped_windows(), 2);
    let starts: Vec<u64> = core::iter::from_fn(|| reader.windows_mut().try_pop().ok())
        .map(|window| window.start_sample)
        .collect();
    assert_eq!(
        starts,
        (0..host_core::METER_QUEUE_DEPTH as u64)
            .map(|index| index * QUANTUM)
            .collect::<Vec<_>>()
    );
}

/// #1582 gate 3. Turning the meter level off shuts the output meter's gate.
///
/// Test value: red if a level-off leaves the output meter observing (the browser tests release
/// the lease only to take it again, which reopens the gate and hides a gate left open).
#[test]
fn a_level_off_shuts_the_output_gate() {
    const QUANTUM: u64 = 64;
    let (mut observer, reader, gate) =
        host_core::output_peak_meter(core::num::NonZeroU32::new(64).expect("window"))
            .expect("output meter");
    let tables = host_core::MeterTables::new(0, 0).expect("tables");
    let mut delivery = host_core::MeterDelivery::new(reader, gate, QUANTUM, tables);
    let block = vec![0.25_f32; QUANTUM as usize];
    assert!(delivery.set_level(true, 0));
    observer.observe(&block, &block, 0, QUANTUM);
    assert_eq!(delivery.output_windows_queued(), 1, "the gate is open");
    assert!(delivery.set_level(false, QUANTUM));
    observer.observe(&block, &block, QUANTUM, 2 * QUANTUM);
    assert_eq!(delivery.output_windows_queued(), 0, "the gate is shut");
}

/// #1588 gate 4. A ring with no free frame drops the newest frame and counts it, and the frames
/// it kept come out in order, element for element as they were pushed.
///
/// Test value: red if a full ring drops a frame without counting it (the browser's lost-window
/// counter would then miss windows that never reached a batch), or overwrites a frame the control
/// half has not released.
#[test]
fn a_full_meter_frame_ring_drops_the_newest_frame_and_counts_it() {
    const STRIPS: usize = 3;
    let host_core::MeterFrameRing {
        mut producer,
        mut consumer,
    } = host_core::MeterFrameRing::new(STRIPS).expect("ring");
    let frame = |sequence: u64| {
        let mut frame = host_core::MeterFrame::new(STRIPS).expect("frame");
        frame.sequence = sequence;
        frame.first_sample = sequence * 256;
        frame.end_sample = (sequence + 1) * 256;
        frame.plan_epoch = 4;
        frame.validity = host_core::MeterValidity::COMPLETE;
        frame.output_peak = [0.5, 0.25];
        for (strip, peak) in frame.strip_peaks.iter_mut().enumerate() {
            *peak = [sequence as f32, strip as f32];
        }
        frame.strip_gain_reduction_db[1] = 3.0;
        frame.strip_gain_reduction_present[1] = true;
        frame
    };
    let pushed: Vec<bool> = (1..=host_core::METER_FRAME_RING_DEPTH as u64 + 1)
        .map(|sequence| producer.push(&frame(sequence)))
        .collect();
    assert_eq!(pushed.iter().filter(|kept| !**kept).count(), 1);
    assert!(
        !pushed[host_core::METER_FRAME_RING_DEPTH],
        "the ninth frame is the one dropped"
    );
    assert_eq!(consumer.dropped_frames(), 1);
    for sequence in 1..=host_core::METER_FRAME_RING_DEPTH as u64 {
        assert_eq!(consumer.peek(), Some(&frame(sequence)), "frame {sequence}");
        consumer.release();
    }
    assert_eq!(consumer.peek(), None);
    assert!(producer.push(&frame(10)), "a released frame is free again");
    assert_eq!(consumer.dropped_frames(), 1);
}

/// #1588 D1, R3. A restart while the level stays on begins a new generation and opens the output
/// gate at the grid's first boundary at or after the restart; with the level off it does nothing.
///
/// Test value: red if a restart keeps the generation (the batch fold would then join windows
/// across one client's change of the shared level) or leaves the gate on the old activation.
#[test]
fn a_restart_begins_a_new_generation_on_the_next_grid_boundary() {
    const WINDOW: u64 = 256;
    let (_observer, reader, gate) =
        host_core::output_peak_meter(core::num::NonZeroU32::new(256).expect("window"))
            .expect("output meter");
    let tables = host_core::MeterTables::new(0, 0).expect("tables");
    let mut delivery = host_core::MeterDelivery::new(reader, gate, WINDOW, tables);
    assert!(!delivery.restart(0), "a level that is off does not restart");
    assert_eq!(delivery.generation(), 0);
    assert!(delivery.set_level(true, 0));
    assert_eq!((delivery.generation(), delivery.output_gate_from()), (1, 0));
    assert!(delivery.restart(300));
    assert!(delivery.level());
    assert_eq!(delivery.generation(), 2);
    assert_eq!(delivery.frame().generation, 2);
    assert_eq!(delivery.output_gate_from(), 2 * WINDOW);
}
