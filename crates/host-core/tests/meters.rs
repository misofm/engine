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
    SpectrumChannels, SpectrumTarget, SuccessorBase, compile_host_session, meter_selection,
    prepare_host_runtime, prepare_host_runtime_with_live_controls,
    prepare_host_runtime_with_live_controls_and_spectrum,
    prepare_host_runtime_with_live_controls_and_spectrum_collection,
    prepare_host_runtime_with_live_controls_and_spectrum_collection_successor,
    prepare_host_runtime_with_live_controls_and_spectrum_successor,
    prepare_host_runtime_with_live_controls_successor, prepare_host_runtime_with_live_lanes,
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
    assert_core_meters(
        "with_spectrum",
        prepare_host_runtime_with_live_controls_and_spectrum(
            &compiled,
            &caps,
            &live,
            &single(),
            lanes,
            &superseded,
        )
        .map(|(prepared, handles, _)| (prepared, handles)),
    );
    assert_core_meters(
        "with_spectrum_successor",
        prepare_host_runtime_with_live_controls_and_spectrum_successor(
            &compiled,
            &caps,
            &live,
            &single(),
            lanes,
            &superseded,
            base(),
        )
        .map(|(prepared, handles, _)| (prepared, handles)),
    );
    assert_core_meters(
        "with_spectrum_collection",
        prepare_host_runtime_with_live_controls_and_spectrum_collection(
            &compiled,
            &caps,
            &live,
            &collection(),
            lanes,
            &superseded,
        )
        .map(|(prepared, handles, _)| (prepared, handles)),
    );
    assert_core_meters(
        "with_spectrum_collection_successor",
        prepare_host_runtime_with_live_controls_and_spectrum_collection_successor(
            &compiled,
            &caps,
            &live,
            &collection(),
            lanes,
            &superseded,
            base(),
        )
        .map(|(prepared, handles, _)| (prepared, handles)),
    );
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
