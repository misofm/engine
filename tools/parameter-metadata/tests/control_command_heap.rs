//! #1615: the peak heap of one live-only `control_command`, native.
//!
//! The browser runs the control plane in the AudioWorklet's message handler, so the peak of heap a
//! live-only transaction needs at once is what can grow the wasm memory (`memory.grow`) there. The
//! test is the qualification's nine-edit transaction (seven parametric-EQ edits on console slot
//! `eq-simd1`, then the two input-filter cutoffs of the left lane) on `observation-session.json`,
//! and the same nine edits on sessions of 64 and 256 tracks. It measures with the audited
//! allocator's thread-scoped counters and live-heap window (`bench_support::alloc`), relative to the
//! heap live at the call's entry.
//!
//! The ceilings are the numbers measured on this head with a few KB of slack. The steady call is
//! the same at every track count, and so is the first (#1620): a live-only commit must not depend
//! on the session's size. The numbers print with `--nocapture`.
//!
//! It lives in `parameter-metadata`, which links host-web and the audited allocator already:
//! `scripts/check-bench-policy.sh` keeps `bench-support` out of every `hosts/` package.

use bench_support::alloc::{
    current_thread_counters, current_thread_delta_since, current_thread_peak,
    current_thread_peak_reset,
};
use controls::{ControlAddress, Owner, Section};
use host_web::{EVENT_LANE_LOSSY, EVENT_LANE_RELIABLE, RESULT_OK, WebBootOptions, native_staging};
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    ParameterChannel, RouteSource, SendTap, StableId, canonical_session_json, parse_session_json,
};

/// What one measured call took: allocations, requested bytes, peak live bytes above the entry, and
/// the live bytes left when it returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Heap {
    allocations: usize,
    requested: usize,
    peak: usize,
    left: usize,
}

fn measured<T>(operation: impl FnOnce() -> T) -> (T, Heap) {
    let mark = current_thread_counters();
    current_thread_peak_reset();
    let result = operation();
    let window = current_thread_peak();
    let delta = current_thread_delta_since(mark);
    let heap = Heap {
        allocations: usize::try_from(delta.allocations).expect("count fits"),
        requested: usize::try_from(delta.requested_bytes).expect("bytes fit"),
        peak: usize::try_from(window.peak_bytes).expect("bytes fit"),
        left: usize::try_from(window.live_bytes).expect("bytes fit"),
    };
    (result, heap)
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

/// The console EQ's parameter IDs (`parametric-eq` contract: band 1 gain is 4, the high-pass is
/// 65 to 67 and the low-pass 81 to 83) and the builtin filter cutoffs (`builtins`: 3 and 4).
const EQ_PARAMETERS: [(u32, f32); 7] = [
    (4, -3.0),
    (66, 400.0),
    (67, 0.8),
    (65, 1.0),
    (82, 6_000.0),
    (83, 0.9),
    (81, 1.0),
];
const HPF_HZ: u32 = 3;
const LPF_HZ: u32 = 4;

fn nine_edits(flip: bool) -> Vec<SessionEdit> {
    let sign = if flip { 1.0 } else { 0.5 };
    let strip = || Owner::Strip(id("track"));
    let mut edits: Vec<SessionEdit> = EQ_PARAMETERS
        .iter()
        .map(|&(control, value)| SessionEdit::SetControl {
            expected_effect: None,
            address: ControlAddress {
                owner: strip(),
                section: Section::Console(id("eq-simd1")),
                control,
                lane: ParameterChannel::Both,
            },
            value: if control == 65 || control == 81 {
                if flip { 1.0 } else { 0.0 }
            } else {
                value * sign
            },
            ramp_samples: None,
        })
        .collect();
    for (control, value) in [(HPF_HZ, 80.0_f32), (LPF_HZ, 12_000.0)] {
        edits.push(SessionEdit::SetControl {
            expected_effect: None,
            address: ControlAddress {
                owner: strip(),
                section: Section::Builtins,
                control,
                lane: ParameterChannel::Left,
            },
            value: value * sign,
            ramp_samples: None,
        });
    }
    edits
}

fn frame(request: u64, revision: u64, edits: &[SessionEdit]) -> Vec<u8> {
    let mut bytes = vec![0_u8; 8_192];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    bytes
}

/// The qualification's session at `tracks` tracks: the first is `track`, the rest are copies of it
/// with their own IDs and routes.
fn document(tracks: usize) -> (String, u64) {
    let mut model = parse_session_json(include_str!(
        "../../../hosts/host-web/qualification/observation-session.json"
    ))
    .expect("the observation session parses");
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    for index in 1..tracks {
        let mut track = template.clone();
        track.id = id(&format!("t{index:04}"));
        // A small track, so 256 of them stay inside the staging ceiling for a document.
        track.inserts.effects.clear();
        for entry in &mut track.console {
            entry.params.clear();
        }
        let mut route = route.clone();
        route.id = id(&format!("t{index:04}-main"));
        route.source = RouteSource::Track {
            track_id: track.id.clone(),
            tap: SendTap::PostPan,
        };
        model.tracks.push(track);
        model.routes.push(route);
    }
    let revision = model.revision;
    (canonical_session_json(&model).expect("canonical"), revision)
}

fn drain(handle: u32) {
    for lane in [EVENT_LANE_RELIABLE, EVENT_LANE_LOSSY] {
        loop {
            let (result, event) = native_staging::control_event(handle, lane);
            assert_eq!(result, RESULT_OK, "event lane {lane}");
            if event.is_empty() {
                break;
            }
        }
    }
}

/// The first call and the steady state of the nine-edit transaction on a session of `tracks`.
fn run(tracks: usize) -> (Heap, Heap) {
    let (text, revision) = document(tracks);
    let options = WebBootOptions {
        source_ring_frames: 2_048,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    };
    let length = native_staging::boot(options, text.as_bytes()).expect("the session stages");
    let handle = host_web::miso_engine_web_v1_boot(length);
    assert_ne!(handle, 0, "the session boots");
    // Boot reads the strip order through the control interface, which draws request IDs from the
    // endpoint's own allocator; the first transaction's ID must sit above them.
    let mut request = 1_000_u64;
    let mut first = None;
    let mut steady = Vec::new();
    for call in 0..6_u64 {
        request += 1;
        let bytes = frame(request, revision + call, &nine_edits(call % 2 == 0));
        let ((result, response), heap) =
            measured(|| native_staging::control_command(handle, &bytes).expect("a live handle"));
        assert_eq!(result, RESULT_OK, "call {call}");
        assert!(!response.is_empty(), "call {call} answers");
        drain(handle);
        if call == 0 {
            first = Some(heap);
        } else if call >= 2 {
            steady.push(heap);
        }
    }
    assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    let steady_first = steady[0];
    assert!(
        steady.iter().all(|heap| *heap == steady_first),
        "the steady calls differ: {steady:?}"
    );
    (first.expect("a first call"), steady_first)
}

/// Ceilings, measured on this head (decision record of #1615): bytes of heap live at the peak above
/// the call's entry, and bytes requested over the call, with a few KB of slack. Debug and release
/// builds give the same numbers (the debug stage check is off, above).
///
/// One track: the first call peaks at 76,011 bytes and every later call at 71,252, requesting
/// 101,025 bytes in 753 allocations. Before #1615 they were 172,790 and 168,031, requesting 293,477
/// in 786.
///
/// #1620: the first call is in place too. It used to take the full path, because its edits add
/// parameter entries to the sparse EQ list (peak 77,600 / 198,182 / 663,179 bytes at 1 / 64 / 256
/// tracks, 839 / 1,853 / 4,927 allocations); in place, it peaks at 72,865 bytes, requesting 103,033
/// in 757 allocations, at every size. Its ceiling was 78,000 at one track and none above.
const FIRST_PEAK_CEILING: usize = 74_000;
const FIRST_REQUESTED_CEILING: usize = 105_000;
const STEADY_PEAK_CEILING: usize = 73_000;
const STEADY_REQUESTED_CEILING: usize = 104_000;

#[test]
fn a_live_only_nine_edit_transaction_has_a_small_peak_at_every_size() {
    std::thread::spawn(|| {
        // A debug build checks each values-only stage against a full catalog build; that check is
        // what this test is not measuring.
        host_core::verify_stages(false);
        let mut peaks = Vec::new();
        let mut firsts = Vec::new();
        for tracks in [1_usize, 64, 256] {
            let (first, steady) = run(tracks);
            println!("tracks {tracks}: first {first:?}; steady {steady:?}");
            // A call keeps only its 80-byte response.
            assert!(
                steady.left <= 256 && first.left <= 256,
                "a call leaves heap live"
            );
            assert!(
                first.peak <= FIRST_PEAK_CEILING && first.requested <= FIRST_REQUESTED_CEILING,
                "{tracks}: first {first:?}"
            );
            assert!(
                steady.peak <= STEADY_PEAK_CEILING,
                "{tracks}: steady {steady:?}"
            );
            assert!(
                steady.requested <= STEADY_REQUESTED_CEILING,
                "{tracks}: steady {steady:?}"
            );
            peaks.push(steady);
            firsts.push(first);
        }
        // #1620: nor does the first call, which adds parameter entries.
        assert!(
            firsts
                .iter()
                .all(|first| first.allocations == firsts[0].allocations),
            "the first call's allocations depend on the track count: {firsts:?}"
        );
        // A live-only commit does not depend on the session's size.
        assert_eq!(
            peaks[1].peak, peaks[2].peak,
            "64 and 256 tracks differ: {peaks:?}"
        );
        assert_eq!(peaks[1].requested, peaks[2].requested, "{peaks:?}");
    })
    .join()
    .expect("the run completes");
}
