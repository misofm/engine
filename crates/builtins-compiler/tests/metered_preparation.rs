//! A 256-track preparation with 56 meters: exact counts and an address-free, repeatable result.
//!
//! Moved here by #1026 from the retired builtins benchmark's untimed
//! `preparation_projection_is_complete_address_free_and_deterministic`, whose `prepare_256_tracks`
//! row was the only preparation that spread meters over several tracks of a large session. The
//! shape is that row's checked fixture (`fixtures/builtins/v1/benchmark/prepare_256_tracks-*.toml`):
//! the canonical track with empty racks 256 times, one post-matrix route from track 0, and a meter
//! at each of the seven taps on tracks 0 to 7, with a four-window queue.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use builtins::{MeterConfig, MeterHandle, MeterTap};
use builtins_compiler::{
    BuiltinCompileCaps, MeterRequest, PreparedBuiltinsSession, prepare_session_builtins,
};
use session::{CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_json};

const SESSION: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const TRACKS: usize = 256;
const METERED_TRACKS: usize = 8;
const TAPS: [MeterTap; 7] = [
    MeterTap::Input,
    MeterTap::PostInputBuiltins,
    MeterTap::PostSimd1,
    MeterTap::PostDynamic,
    MeterTap::PostSimd2PreFader,
    MeterTap::PostFader,
    MeterTap::PostMatrix,
];

fn prepare(rate_hz: u32) -> PreparedBuiltinsSession {
    let mut model = parse_session_json(SESSION).expect("fixture");
    let mut template = model.tracks[0].clone();
    template.console.clear();
    template.inserts.effects.clear();
    model.automation.clear();
    model.tracks = (0..TRACKS)
        .map(|index| {
            let mut track = template.clone();
            track.id = StableId::parse(&format!("track-{index}")).expect("generated ID");
            track
        })
        .collect();
    model.sample_rate_hz = rate_hz;
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostPan,
    };
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("session");
    let config = MeterConfig {
        period_frames: NonZeroU32::new(128).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).expect("constant"),
        reset_generation: 7,
    };
    let requests: Vec<_> = (0..METERED_TRACKS)
        .flat_map(|track| {
            TAPS.into_iter().enumerate().map(move |(tap_index, tap)| {
                let handle = u64::try_from(track * TAPS.len() + tap_index).expect("bounded") + 1;
                MeterRequest {
                    handle: MeterHandle(NonZeroU64::new(handle).expect("one-based handle")),
                    track_id: format!("track-{track}"),
                    tap,
                    config,
                }
            })
        })
        .collect();
    prepare_session_builtins(
        &session,
        &requests,
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
    .expect("prepared")
}

/// Everything preparation reports, and nothing that holds an address.
fn projection(
    prepared: &PreparedBuiltinsSession,
) -> (
    [usize; 4],
    Vec<(String, effect_contract::TailSamples)>,
    builtins_compiler::BuiltinResourceEstimate,
) {
    (
        [
            prepared.processor_count(),
            prepared.tail_count(),
            prepared.observer_count(),
            prepared.meter_consumer_count(),
        ],
        prepared
            .tails()
            .map(|(track_id, tail)| (track_id.to_owned(), tail))
            .collect(),
        prepared.resource_report(),
    )
}

#[test]
fn metered_256_track_preparation_is_exact_and_repeatable() {
    let meters = METERED_TRACKS * TAPS.len();
    for rate_hz in [48_000, 96_000] {
        let first = prepare(rate_hz);
        let (counts, tails, report) = projection(&first);
        // Three strip stages and one tail per track; one observer and one consumer per meter.
        assert_eq!(counts, [TRACKS * 3, TRACKS, meters, meters], "{rate_hz} Hz");
        assert_eq!(tails.len(), TRACKS);
        // Each meter's SPSC ring has one slot more than its four-window capacity.
        assert_eq!(report.meter_items, (meters * 5) as u64, "{rate_hz} Hz");
        assert!(!report.retained_layouts().is_empty());
        assert_eq!(
            projection(&prepare(rate_hz)),
            (counts, tails, report),
            "{rate_hz} Hz: a second preparation reports the same shape"
        );
    }
}
