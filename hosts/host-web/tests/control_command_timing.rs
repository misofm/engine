//! #1612 and #1613, descriptive: the time of one whole `control_command` (the browser's one entry
//! for a `SET_CONTROL` transaction, which is the control plane's `SessionState::command` and so the
//! C ABI's too) for a fader edit on a session of many tracks, native.
//!
//! Ignored: it prints one line per size and asserts nothing about time.
//! `cargo test --release -p host-web --features test-support --test control_command_timing --
//! --ignored --nocapture`
//!
//! Recorded on this box (release, x86-64): after #1612 (the classifier from the rows, the model still
//! cloned and compiled) 267 us at 64 tracks and 1.11 ms at 256; after #1613 (the edit committed in
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
        expected_effect: None,
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

const VCA_OFFSET_DB: u32 = 1;
/// The bound of ruling V-Q3 (`host_core::MAXIMUM_VCAS_BOUNDED`, `MAXIMUM_VCA_REACH_PAIRS`): 256
/// VCAs and 16,384 (strip, VCA) pairs, here 64 tracks under a chain of 256 VCAs.
const BOUND_TRACKS: usize = 64;
const BOUND_VCAS: usize = 256;

/// Batch 7 review M1 and M2: the whole `control_command` of a VCA offset move on a session at the
/// bound, native, in release. A VCA move composes every strip it reaches on the control thread, and
/// the browser runs that thread in the AudioWorklet's message handler, so the bound exists to keep
/// it inside one render quantum (128 frames at 48 kHz, 2.67 ms).
///
/// The median of the calls must fit a quantum. That is a loose ceiling on a box the test does not
/// control (the planner measured 0.15 ms natively for the old browser path): the claim is "a move
/// at the bound is a fraction of a quantum", not a benchmark.
///
/// Ignored, because it asserts time: the nightly runs it in release.
/// `cargo test --release -p host-web --features test-support --test control_command_timing --
/// --ignored --exact a_vca_move_at_the_bound_fits_a_quantum_in_release --nocapture`
#[test]
#[ignore = "release timing"]
fn a_vca_move_at_the_bound_fits_a_quantum_in_release() {
    std::thread::spawn(|| {
        let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        let template = model.tracks[0].clone();
        let route = model.routes[0].clone();
        model.tracks.clear();
        model.routes.clear();
        for index in 0..BOUND_TRACKS {
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
        for index in 0..BOUND_VCAS {
            let members = if index == 0 {
                model.tracks.iter().map(|track| track.id.clone()).collect()
            } else {
                vec![StableId::parse(&format!("v{:04}", index - 1)).expect("VCA ID")]
            };
            model.vcas.push(session::Vca {
                id: StableId::parse(&format!("v{index:04}")).expect("VCA ID"),
                fader: session::DualMonoFader {
                    left_db: 0.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                },
                members,
            });
        }
        assert_eq!(
            model.vca_reach_pairs(u64::MAX),
            host_core::MAXIMUM_VCA_REACH_PAIRS,
            "the fixture sits exactly at the pair bound"
        );
        let document = canonical_session_json(&model).expect("canonical");
        let length = native_staging::boot(WebBootOptions::explicit_defaults(), document.as_bytes())
            .expect("the session stages");
        let handle = host_web::miso_engine_web_v1_boot(length);
        assert_ne!(handle, 0, "a session at the VCA bound boots");

        let mut times = Vec::new();
        for (revision, call) in (model.revision..).zip(0..CALLS) {
            let edits = [SessionEdit::SetControl {
                expected_effect: None,
                address: ControlAddress {
                    owner: Owner::Vca(StableId::parse("v0000").expect("VCA ID")),
                    section: Section::Own,
                    control: VCA_OFFSET_DB,
                    lane: ParameterChannel::Both,
                },
                value: if call % 2 == 0 { -6.0 } else { -3.0 },
                ramp_samples: None,
            }];
            let mut bytes = vec![0_u8; 4_096];
            let length = ProtocolCodec::default()
                .encode_command_frame_into(
                    &TypedCommandFrame {
                        request_id: RequestId::new(call + 1).expect("nonzero request"),
                        expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                        payload: CommandPayload::SessionTransactionApply(&edits),
                    },
                    &mut bytes,
                )
                .expect("a transaction frame");
            let start = Instant::now();
            let (result, _) = native_staging::control_command(handle, &bytes[..length])
                .expect("a live handle");
            times.push(start.elapsed());
            assert_eq!(result, RESULT_OK, "call {call}");
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
        times.sort();
        let median = times[times.len() / 2];
        let quantum = std::time::Duration::from_secs_f64(128.0 / 48_000.0);
        println!(
            "control_command VCA move at the bound: median {median:?}, p99 {:?}, quantum {quantum:?}",
            times[times.len() * 99 / 100]
        );
        assert!(median < quantum, "a VCA move at the bound outlasts a quantum");
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the run completes");
}
