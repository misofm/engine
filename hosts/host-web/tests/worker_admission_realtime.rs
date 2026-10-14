//! Issue #1562a gate 5, in the single-thread form of the one instance (decision 17, "single
//! instance first"): control frames run in the message handler's place, between render calls, and
//! never allocate or free inside one.
//!
//! The file keeps the name of the full issue's integration binary (`worker_admission_realtime`),
//! whose two-thread form (the Worker committing while the render thread renders) arrives with the
//! Worker (#1332). Every call goes through the module's exports on a fresh thread, as the worklet
//! makes them, so a render call runs inside its render-locked window and the instance's own
//! counters read what it allocated: the render-locked count must not move across any render,
//! and the control count must move across the control frames, which is where the engine allocates
//! on an audio thread in this mode (`singleModeControlAllocations`).
//!
//! Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
//! never in `src/tests.rs`, which registers its own allocator. The workspace has no
//! `bench_support::alloc`, so this binary registers the module's own counting allocator, as
//! `plan_exchange_threads.rs` does.

#![cfg(feature = "test-support")]

use std::alloc::System;

use controls::{ControlAddress, Owner, Section};
use host_web::{RESULT_OK, RenderLockedAllocator, WebBootOptions, native_staging};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{ParameterChannel, StableId, canonical_session_json, parse_session_json};

mod common;

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM_FRAMES: u32 = 128;
const TRANSACTIONS: u64 = 1_000;
const FED_BLOCKS: u64 = 300;
const FADER_DB: u32 = 5;

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
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; 4_096];
    let length = codec
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

fn committed(response: &[u8]) -> (EditPath, u64) {
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success { header, payload } = ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a transaction response")
    else {
        panic!("the transaction was refused")
    };
    let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload else {
        panic!("a transaction success")
    };
    (applied.edit_path, header.revision.0)
}

/// 1,000 `SET_CONTROL` transactions between 1,000 renders.
///
/// Test value: red if control work runs inside, or allocates inside, a render call (the
/// render-locked count would move), or if the control frames' allocations go uncounted (the
/// control count would not move: `singleModeControlAllocations` would read zero).
#[test]
fn a_thousand_control_frames_between_renders_never_allocate_in_a_render_call() {
    std::thread::spawn(|| {
        let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
        // Rendering needs no console or insert to prove the counts; the fixture's tracks are what
        // the frames address.
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        for track in &mut model.tracks {
            track.console.clear();
            track.inserts.effects.clear();
        }
        let strip = model.tracks[0].id.as_str().to_owned();
        let document = canonical_session_json(&model).expect("canonical");
        let options = WebBootOptions::explicit_defaults();
        let length =
            native_staging::boot(options, document.as_bytes()).expect("the fixture stages");
        let handle = host_web::miso_engine_web_v1_boot(length);
        assert_ne!(handle, 0, "the fixture boots");
        let control_before = host_web::miso_engine_web_v1_control_allocation_count();
        let mut revision = model.revision;
        let mut render_allocations = 0;
        let mut control_calls_between = 0;
        for block in 0..TRANSACTIONS {
            // The control frame, in the message handler's place: before the block it affects.
            let control = host_web::miso_engine_web_v1_control_allocation_count();
            let db = if block % 2 == 0 { -6.0 } else { -3.0 };
            let (result, response) =
                native_staging::control_command(handle, &frame(block + 1, revision, &strip, db))
                    .expect("a live handle");
            assert_eq!(result, RESULT_OK, "block {block}: the control call");
            let (path, next) = committed(&response);
            assert_eq!(path, EditPath::Live, "block {block}: the edit is live");
            assert_eq!(next, revision + 1, "block {block}: the revision advances");
            revision = next;
            // The control half's drain (#1570b D4), as the worklet's message handler runs it: the
            // commit event is read so the lane never fills.
            for lane in [host_web::EVENT_LANE_RELIABLE, host_web::EVENT_LANE_LOSSY] {
                loop {
                    let (result, event) = native_staging::control_event(handle, lane);
                    assert_eq!(
                        result, RESULT_OK,
                        "block {block}: the event drain, lane {lane}"
                    );
                    if event.is_empty() {
                        break;
                    }
                }
            }
            control_calls_between +=
                host_web::miso_engine_web_v1_control_allocation_count() - control;

            // The fixture's source holds 300 blocks; past them a render underruns, which emits
            // silence and a counter and is still a render call the counts must cover.
            if block < FED_BLOCKS {
                assert!(native_staging::source_id(handle, SOURCE_ID));
                assert!(native_staging::source_pcm(handle, 0.1));
                let fed = host_web::miso_engine_web_v1_source_submit(
                    handle,
                    SOURCE_ID.len() as u32,
                    1,
                    block * u64::from(QUANTUM_FRAMES),
                    2,
                    QUANTUM_FRAMES,
                    0,
                );
                assert_eq!(fed, RESULT_OK, "block {block}: the source feed");
            }
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            let rendered = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
            render_allocations += host_web::miso_engine_web_v1_render_allocation_count() - before;
            assert_eq!(rendered, RESULT_OK, "block {block}: the render");
        }
        assert_eq!(render_allocations, 0, "a render call allocated or freed");
        assert!(
            control_calls_between > 0,
            "the control frames' allocations are control allocations, and they are counted"
        );
        assert!(
            host_web::miso_engine_web_v1_control_allocation_count() > control_before,
            "the status reports the control allocations"
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the run completes");
}

/// One live edit of each owner kind, by `kind % 5`: a strip fader, an insert parameter, a console
/// slot parameter, a send's gain and a VCA offset, over the parity session of `common`.
fn owner_edit(kind: u64, high: bool) -> SessionEdit {
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    let (owner, section, control, lane, low, up) = match kind % 5 {
        0 => (
            Owner::Strip(id("t1")),
            Section::Builtins,
            common::FADER_DB,
            ParameterChannel::Both,
            -6.0,
            -3.0,
        ),
        1 => (
            Owner::Strip(id("t1")),
            Section::Inserts(id("comp")),
            1,
            ParameterChannel::Left,
            -30.0,
            -24.0,
        ),
        2 => (
            Owner::Strip(id("t1")),
            Section::Console(id(common::PRE_SLOT)),
            1,
            ParameterChannel::Left,
            -30.0,
            -24.0,
        ),
        3 => (
            Owner::Route(id("t1-b")),
            Section::Own,
            common::ROUTE_GAIN_DB,
            ParameterChannel::Both,
            -6.0,
            -3.0,
        ),
        _ => (
            Owner::Vca(id("v")),
            Section::Own,
            common::VCA_OFFSET_DB,
            ParameterChannel::Both,
            -3.0,
            -1.0,
        ),
    };
    SessionEdit::SetControl {
        address: ControlAddress {
            owner,
            section,
            control,
            lane,
        },
        value: if high { up } else { low },
        ramp_samples: None,
    }
}

/// 640 live `SET_CONTROL` transactions between renders, one owner kind at a time (a strip, an
/// insert, a console slot, a send, a VCA), on the parity session: the control frames allocate in
/// the control count, and no render call allocates or frees.
///
/// Test value: red if a live edit of an insert, a console slot, a send or a VCA does any control
/// work inside the next render call (the plan's lanes, a catalog row or a retired table dropped on
/// the render side), which the strip-fader loop above does not reach.
#[test]
fn live_edits_of_every_owner_kind_between_renders_never_allocate_in_a_render_call() {
    const SOURCES: [&str; 3] = ["src-t1", "src-t2", "src-t3"];
    const BLOCKS: u64 = 640;
    std::thread::spawn(|| {
        let model = common::model_at(48_000);
        let document = canonical_session_json(&model).expect("canonical");
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: QUANTUM_FRAMES,
            source_ring_frames: QUANTUM_FRAMES * 2,
            ..WebBootOptions::explicit_defaults()
        };
        let length =
            native_staging::boot(options, document.as_bytes()).expect("the session stages");
        let handle = host_web::miso_engine_web_v1_boot(length);
        assert_ne!(handle, 0, "the parity session boots");
        let mut revision = model.revision;
        let mut render_allocations = 0;
        let mut control_calls = 0;
        for block in 0..BLOCKS {
            let control = host_web::miso_engine_web_v1_control_allocation_count();
            let edits = [owner_edit(block, (block / 5) % 2 == 1)];
            let codec = ProtocolCodec::default();
            let mut bytes = vec![0_u8; 4_096];
            let length = codec
                .encode_command_frame_into(
                    &TypedCommandFrame {
                        request_id: RequestId::new(block + 1).expect("nonzero request"),
                        expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                        payload: CommandPayload::SessionTransactionApply(&edits),
                    },
                    &mut bytes,
                )
                .expect("a transaction frame");
            let (result, response) =
                native_staging::control_command(handle, &bytes[..length]).expect("a live handle");
            assert_eq!(result, RESULT_OK, "block {block}: the control call");
            let (path, next) = committed(&response);
            assert_eq!(path, EditPath::Live, "block {block}: the edit is live");
            revision = next;
            for lane in [host_web::EVENT_LANE_RELIABLE, host_web::EVENT_LANE_LOSSY] {
                loop {
                    let (result, event) = native_staging::control_event(handle, lane);
                    assert_eq!(result, RESULT_OK, "block {block}: the event drain");
                    if event.is_empty() {
                        break;
                    }
                }
            }
            control_calls += host_web::miso_engine_web_v1_control_allocation_count() - control;
            for source in SOURCES {
                assert!(native_staging::source_id(handle, source.as_bytes()));
                assert!(native_staging::source_pcm(handle, 0.1));
                let fed = host_web::miso_engine_web_v1_source_submit(
                    handle,
                    source.len() as u32,
                    1,
                    block * u64::from(QUANTUM_FRAMES),
                    2,
                    QUANTUM_FRAMES,
                    0,
                );
                // The sources hold 72 blocks; a later feed is refused at the end of the region.
                if block < 64 {
                    assert_eq!(fed, RESULT_OK, "block {block}: the feed of {source}");
                }
            }
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            let rendered = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
            render_allocations += host_web::miso_engine_web_v1_render_allocation_count() - before;
            assert_eq!(rendered, RESULT_OK, "block {block}: the render");
        }
        assert_eq!(render_allocations, 0, "a render call allocated or freed");
        assert!(
            control_calls > 0,
            "the control frames' allocations are counted"
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the run completes");
}
