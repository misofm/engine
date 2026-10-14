//! TBD9 and TBD10, descriptive: the time of one whole `control_command` (the browser's one entry
//! for a `SET_CONTROL` transaction, which is the control plane's `SessionState::command` and so the
//! C ABI's too) for a fader edit on a session of many tracks, native.
//!
//! Ignored: it prints one line per size and asserts nothing about time.
//! `cargo test --release -p host-web --features test-support --test control_command_timing --
//! --ignored --nocapture`
//!
//! Recorded on this box (release, x86-64): after TBD9 (the classifier from the rows, the model still
//! cloned and compiled) 267 us at 64 tracks and 1.11 ms at 256; after TBD10 (the edit committed in
//! place) about 19 us at both. The C ABI's pair, with the full preparation forced on the same
//! session, is `in_place_command_timing_line` in `crates/capi/src/runtime/live_tests.rs`.

#![cfg(feature = "test-support")]

use std::time::Instant;

use controls::{ControlAddress, Owner, Section};
use host_web::{RESULT_OK, WebBootOptions, native_staging};
use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{ParameterChannel, StableId, canonical_session_json, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const FADER_DB: u32 = 5;
const CALLS: u64 = 200;

fn frame(request: u64, revision: u64, strip: &str, db: f32) -> Vec<u8> {
    let edits = [SessionEdit::SetControl {
        address: ControlAddress {
            owner: Owner::Strip(StableId::parse(strip).expect("strip ID")),
            section: Section::Builtins,
            control: FADER_DB,
            lane: ParameterChannel::Both,
        },
        value: db,
        ramp_samples: None,
    }];
    let mut bytes = vec![0_u8; 4_096];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    bytes
}

#[test]
#[ignore = "descriptive timing"]
fn control_command_timing_line() {
    std::thread::spawn(|| {
        for tracks in [64_usize, 256] {
            let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
            model.console.pre_insert.clear();
            model.console.post_insert.clear();
            let template = model.tracks[0].clone();
            let route = model.routes[0].clone();
            model.tracks.clear();
            model.routes.clear();
            for index in 0..tracks {
                let mut track = template.clone();
                track.id = StableId::parse(&format!("t{index:04}")).expect("track ID");
                track.console.clear();
                track.inserts.effects.clear();
                let mut route = route.clone();
                route.id = StableId::parse(&format!("t{index:04}-main")).expect("route ID");
                route.source = session::RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: session::SendTap::PostPan,
                };
                model.tracks.push(track);
                model.routes.push(route);
            }
            let document = canonical_session_json(&model).expect("canonical");
            let options = WebBootOptions::explicit_defaults();
            let length =
                native_staging::boot(options, document.as_bytes()).expect("the session stages");
            let handle = host_web::miso_engine_web_v1_boot(length);
            assert_ne!(handle, 0, "the session boots");
            let mut spent = std::time::Duration::ZERO;
            for (revision, call) in (model.revision..).zip(0..CALLS) {
                let db = if call % 2 == 0 { -6.0 } else { -3.0 };
                let bytes = frame(call + 1, revision, "t0003", db);
                let start = Instant::now();
                let (result, _) =
                    native_staging::control_command(handle, &bytes).expect("a live handle");
                spent += start.elapsed();
                assert_eq!(result, RESULT_OK, "call {call}");
                // The reliable lane holds two commit events; an undrained third commit would be
                // refused with backpressure, so drain the way the worklet does.
                for lane in [host_web::EVENT_LANE_RELIABLE, host_web::EVENT_LANE_LOSSY] {
                    loop {
                        let (result, event) = native_staging::control_event(handle, lane);
                        assert_eq!(result, RESULT_OK, "event lane {lane}");
                        if event.is_empty() {
                            break;
                        }
                    }
                }
            }
            println!(
                "control_command fader edit, {tracks} tracks: {:?} per call",
                spent / u32::try_from(CALLS).expect("calls")
            );
            assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        }
    })
    .join()
    .expect("the run completes");
}
