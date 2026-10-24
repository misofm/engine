//! #1386 (a2): `SESSION_REPLACE` through the exported `miso_engine_v1_submit_document_command`.
//!
//! The core path's rules (the diff, every document refusal, replay, the identical document) are
//! tested once, in `crates/control-plane/tests/session_replace.rs`. This file holds the C ABI's own
//! rows: the export renders what the equal `SESSION_TRANSACTION_APPLY` renders on a twin, it
//! borrows the document and keeps nothing of it, a null document is an argument error, the plain
//! `submit_command` carries no document, and a document four times the control frame commits.

#![allow(unsafe_code)]

use core::ptr;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use capi::*;
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionReplaceRequest,
    SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{RouteSource, SessionModel, StableId, canonical_session_json, parse_session_json};
use sha2::{Digest, Sha256};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const QUANTUM: u32 = 128;
const FRAME_BYTES: u64 = 16_384;

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 200,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 30,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: FRAME_BYTES,
        maximum_replay_bytes: FRAME_BYTES * 16,
        maximum_replay_entries: 64,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

fn json(model: &SessionModel) -> String {
    canonical_session_json(model).expect("a valid document")
}

/// `UpsertTrack` of a muted copy of the first track named `name`, and `UpsertRoute` of its route.
fn added_track(model: &SessionModel, name: &str) -> (session::Track, session::Route) {
    let mut track = model.tracks[0].clone();
    track.id = id(name);
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = id(&format!("{name}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    (track, route)
}

/// The decoded fields of a reply.
#[derive(Debug, PartialEq)]
struct Reply {
    status: StatusCode,
    revision: u64,
    applied: Option<(u32, EditPath)>,
    codes: Vec<String>,
}

fn reply(bytes: &[u8]) -> Reply {
    let mut fields = [0_u16; 256];
    match ProtocolCodec::default()
        .decode_typed_response(bytes, &mut DecodeScratch::new(&mut fields))
        .expect("a typed reply")
    {
        DecodedTypedResponseFrame::Success { header, payload } => {
            let applied = match payload {
                DecodedSuccessResponsePayload::SessionReplaced(value)
                | DecodedSuccessResponsePayload::SessionTransactionApplied(value) => value,
                _ => panic!("an applied reply"),
            };
            Reply {
                status: header.status,
                revision: header.revision.0,
                applied: Some((applied.applied_operations, applied.edit_path)),
                codes: Vec::new(),
            }
        }
        DecodedTypedResponseFrame::NonOk { header, payload } => Reply {
            status: header.status,
            revision: header.revision.0,
            applied: None,
            codes: payload.diagnostics.into_iter().map(|d| d.code).collect(),
        },
    }
}

/// One compiled session over the C ABI.
struct Endpoint {
    session: *mut Session,
    plan: *mut Plan,
    next_request: u64,
    next_frame: u64,
}

impl Endpoint {
    fn new(document: &str) -> Self {
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut diagnostic_storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: diagnostic_storage.as_mut_ptr(),
            capacity_bytes: diagnostic_storage.len() as u64,
            required_bytes: 0,
        };
        let compile_limits = limits();
        // SAFETY: Every descriptor and output location remains live for the complete call.
        unsafe {
            assert_eq!(
                miso_engine_v1_engine_create(&config, &mut engine),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &compile_limits,
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                RESULT_OK,
                "{}",
                String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
            );
            miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            next_request: 1,
            next_frame: 0,
        }
    }

    fn frame(&mut self, revision: u64, payload: CommandPayload<'_>) -> Vec<u8> {
        let request_id = RequestId::new(self.next_request).expect("request ID");
        self.next_request += 1;
        let mut bytes = vec![0_u8; FRAME_BYTES as usize];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id,
                    expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                    payload,
                },
                &mut bytes,
            )
            .expect("command frame");
        bytes.truncate(length);
        bytes
    }

    fn replace_frame(&mut self, revision: u64, document: &[u8]) -> Vec<u8> {
        self.frame(
            revision,
            CommandPayload::SessionReplace(SessionReplaceRequest {
                document_bytes: u32::try_from(document.len()).expect("document length"),
                document_sha256: Sha256::digest(document).into(),
            }),
        )
    }

    /// One call of the document export: its result and its reply bytes.
    fn submit_document(&self, request: &[u8], document: *const u8, bytes: u64) -> (u32, Vec<u8>) {
        let mut response = vec![0_u8; FRAME_BYTES as usize];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; every buffer is complete
        // for the call, and `document` is readable for `bytes` (or null with `bytes` 0, or the
        // null-argument case the export refuses).
        let code = unsafe {
            miso_engine_v1_submit_document_command(
                self.session,
                request.as_ptr(),
                request.len() as u64,
                document,
                bytes,
                &mut output,
            )
        };
        response.truncate(if code == RESULT_OK {
            output.required_bytes as usize
        } else {
            0
        });
        (code, response)
    }

    fn replace(&self, request: &[u8], document: &[u8]) -> Vec<u8> {
        let (code, bytes) = self.submit_document(request, document.as_ptr(), document.len() as u64);
        assert_eq!(code, RESULT_OK);
        bytes
    }

    fn submit(&self, request: &[u8]) -> Vec<u8> {
        let mut response = vec![0_u8; FRAME_BYTES as usize];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; both buffers are complete.
        let code = unsafe {
            miso_engine_v1_submit_command(
                self.session,
                request.as_ptr(),
                request.len() as u64,
                &mut output,
            )
        };
        assert_eq!(code, RESULT_OK);
        response.truncate(output.required_bytes as usize);
        response
    }

    /// The canonical snapshot and the revision, read in chunks.
    fn state(&mut self) -> (Vec<u8>, u64) {
        let mut text = Vec::new();
        loop {
            let bytes = self.frame(
                0,
                CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                    offset: text.len() as u64,
                    maximum_bytes: 8_192,
                }),
            );
            // A snapshot read takes any revision; rewrite the header's flag to `Any`.
            let request = with_any_revision(bytes);
            let response = self.submit(&request);
            let mut fields = [0_u16; 64];
            let DecodedTypedResponseFrame::Success {
                header,
                payload: DecodedSuccessResponsePayload::SessionSnapshot(snapshot),
            } = ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("snapshot")
            else {
                panic!("a snapshot");
            };
            text.extend_from_slice(snapshot.canonical_json_chunk);
            if snapshot.eof {
                return (text, header.revision.0);
            }
        }
    }

    /// Reliable events waiting, drained.
    fn drain_events(&self) -> usize {
        let mut count = 0;
        let mut event = vec![0_u8; FRAME_BYTES as usize];
        loop {
            let mut out = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: event.as_mut_ptr(),
                capacity_bytes: event.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: The session is live and used by this thread alone; the buffer is owned.
            let code = unsafe {
                miso_engine_v1_dequeue_event(self.session, EVENT_LANE_RELIABLE, &mut out)
            };
            assert_eq!(code, RESULT_OK, "dequeue");
            if out.required_bytes == 0 {
                return count;
            }
            count += 1;
        }
    }

    /// Feed one deterministic block of the fixture source and render it; the block's bits.
    fn block(&mut self) -> Vec<u32> {
        let frame = self.next_frame;
        self.next_frame += u64::from(QUANTUM);
        let left: Vec<f32> = (0..QUANTUM)
            .map(|i| ((frame + u64::from(i)) as f32 * 0.013).sin() * 0.5)
            .collect();
        let right: Vec<f32> = left.iter().map(|sample| -sample * 0.75).collect();
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 1,
            start_frame: frame,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: QUANTUM,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut report = SubmitReport {
            struct_size: SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        let source = b"fixture-source";
        let mut pcm = vec![0.0_f32; 2 * QUANTUM as usize];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM,
            plane_stride_samples: QUANTUM,
            reserved: [0; 2],
        };
        // SAFETY: The handles are live, every buffer outlives the calls, one thread uses them.
        unsafe {
            assert_eq!(
                miso_engine_v1_source_submit_planar_f32(
                    self.session,
                    source.as_ptr(),
                    source.len() as u64,
                    &chunk,
                    &mut report,
                ),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_render_f32_planar(self.plan, frame, &output),
                RESULT_OK
            );
        }
        pcm.iter().map(|sample| sample.to_bits()).collect()
    }
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        // SAFETY: The exact live handles `miso_engine_v1_compile_session` returned, destroyed once.
        unsafe {
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

/// A command frame re-encoded with revision `Any` (the header flag and a zero carrier).
fn with_any_revision(mut bytes: Vec<u8>) -> Vec<u8> {
    let mut fields = [0_u16; 64];
    let decoded = ProtocolCodec::default()
        .decode_typed_command(&bytes, &mut DecodeScratch::new(&mut fields))
        .expect("command");
    let protocol::DecodedCommandPayload::SessionSnapshotGet(request) = decoded.payload else {
        panic!("a snapshot request")
    };
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: decoded.header.request_id,
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::SessionSnapshotGet(request),
            },
            &mut bytes,
        )
        .expect("snapshot request");
    bytes.truncate(length);
    bytes
}

/// Test value: red if the export takes a second edit path (its plan renders other bits than the
/// equal `SESSION_TRANSACTION_APPLY` on a twin, or its reply, revision or event differ), or if
/// the export commits an identical document.
#[test]
fn the_export_renders_what_the_equal_transaction_renders() {
    let base = parse_session_json(FIXTURE).expect("fixture");
    let mut replaced = Endpoint::new(FIXTURE);
    let mut twin = Endpoint::new(FIXTURE);
    for _ in 0..4 {
        assert_eq!(replaced.block(), twin.block());
    }
    // A rebuild: one muted track and its route.
    let (track, route) = added_track(&base, "a-added");
    let mut next = base.clone();
    next.tracks.push(track.clone());
    next.routes.push(route.clone());
    let document = json(&next);
    let edits = [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ];
    let (_, revision) = replaced.state();
    let frame = replaced.replace_frame(revision, document.as_bytes());
    let answer = reply(&replaced.replace(&frame, document.as_bytes()));
    let frame = twin.frame(revision, CommandPayload::SessionTransactionApply(&edits));
    let expected = reply(&twin.submit(&frame));
    assert_eq!(answer, expected);
    assert_eq!(answer.applied, Some((2, EditPath::Rebuild)));
    assert_eq!(answer.revision, revision + 1);
    assert_eq!((replaced.drain_events(), twin.drain_events()), (1, 1));
    for block in 0..8 {
        assert_eq!(replaced.block(), twin.block(), "rebuild block {block}");
    }
    assert_eq!(replaced.state(), twin.state());

    // A live change: one fader.
    let mut faded = next.clone();
    faded.tracks[0].fader.left_db = -6.0;
    let document = json(&faded);
    let frame = replaced.replace_frame(revision + 1, document.as_bytes());
    let answer = reply(&replaced.replace(&frame, document.as_bytes()));
    let edits = [SessionEdit::UpsertTrack {
        track: faded.tracks[0].clone(),
    }];
    let frame = twin.frame(
        revision + 1,
        CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(answer, reply(&twin.submit(&frame)));
    assert_eq!(answer.applied, Some((1, EditPath::Live)));
    for block in 0..8 {
        assert_eq!(replaced.block(), twin.block(), "live block {block}");
    }

    // The identical document: unchanged, no event, nothing renders differently.
    replaced.drain_events();
    twin.drain_events();
    let before = replaced.state();
    let frame = replaced.replace_frame(revision + 2, document.as_bytes());
    assert_eq!(
        reply(&replaced.replace(&frame, document.as_bytes())),
        Reply {
            status: StatusCode::Ok,
            revision: revision + 2,
            applied: Some((0, EditPath::Unchanged)),
            codes: Vec::new(),
        }
    );
    assert_eq!(replaced.drain_events(), 0);
    assert_eq!(replaced.state(), before);
    for block in 0..8 {
        assert_eq!(replaced.block(), twin.block(), "unchanged block {block}");
    }
}

/// Test value: red if the export reads a null document as an empty one (or dereferences it),
/// if the plain `submit_command` passes some document, or if a retry through it re-reads one
/// instead of answering the cached reply.
#[test]
fn the_export_refuses_a_null_document_and_submit_command_carries_none() {
    let base = parse_session_json(FIXTURE).expect("fixture");
    let mut faded = base.clone();
    faded.tracks[0].fader.left_db = -3.0;
    let document = json(&faded);
    let mut endpoint = Endpoint::new(FIXTURE);
    let before = endpoint.state();

    let frame = endpoint.replace_frame(before.1, document.as_bytes());
    let (code, _) = endpoint.submit_document(&frame, ptr::null(), document.len() as u64);
    assert_eq!(
        code, RESULT_INVALID_ARGUMENT,
        "a null document with a length"
    );
    assert_eq!(endpoint.state(), before);

    // The plain export passes no document: the core's length refusal, nothing changed.
    let plain = endpoint.replace_frame(before.1, document.as_bytes());
    let refused = reply(&endpoint.submit(&plain));
    assert_eq!(refused.status, StatusCode::InvalidField);
    assert_eq!(refused.codes, ["session.replace.document_length"]);
    assert_eq!(endpoint.state(), before);
    assert_eq!(endpoint.drain_events(), 0);

    // A replace through the document export, then its retry through the plain one.
    let frame = endpoint.replace_frame(before.1, document.as_bytes());
    let first = endpoint.replace(&frame, document.as_bytes());
    assert_eq!(reply(&first).applied, Some((1, EditPath::Live)));
    assert_eq!(endpoint.submit(&frame), first, "the cached reply");
    assert_eq!(endpoint.state().1, before.1 + 1, "committed once");
    assert_eq!(endpoint.drain_events(), 1);
}

/// Test value: red if a replace is bounded by the control frame instead of
/// `maximum_document_bytes`, or if the C ABI keeps any byte of the document: a live replace of a
/// document more than four times the frame leaves the same live bytes as the equal transaction
/// on a twin.
#[test]
fn a_document_four_times_the_frame_commits_and_nothing_of_it_is_kept() {
    assert_installed();
    let base = parse_session_json(FIXTURE).expect("fixture");
    let mut large = base.clone();
    for index in 0..48 {
        let (track, route) = added_track(&base, &format!("wide{index:02}"));
        large.tracks.push(track);
        large.routes.push(route);
    }
    let document = json(&large);
    assert!(
        document.len() > 4 * FRAME_BYTES as usize,
        "{} bytes",
        document.len()
    );
    let mut endpoint = Endpoint::new(FIXTURE);
    let (_, revision) = endpoint.state();
    let frame = endpoint.replace_frame(revision, document.as_bytes());
    let answer = reply(&endpoint.replace(&frame, document.as_bytes()));
    assert_eq!(answer.status, StatusCode::Ok, "{answer:?}");
    assert_eq!(answer.applied, Some((96, EditPath::Rebuild)));
    let mut booted = large.clone();
    booted.revision = revision + 1;
    assert_eq!(endpoint.state().0, json(&booted).into_bytes());

    // The large live replace, against a twin booted from the same document that applies the equal
    // transaction: the same reply and the same bytes left live by the call.
    let mut replaced = Endpoint::new(&document);
    let mut twin = Endpoint::new(&document);
    let mut faded = large.clone();
    faded.tracks[3].fader.right_db = -4.5;
    let next = json(&faded);
    let (_, start) = replaced.state();
    let replace_frame = replaced.replace_frame(start, next.as_bytes());
    let edits = [SessionEdit::UpsertTrack {
        track: faded.tracks[3].clone(),
    }];
    let transaction = twin.frame(start, CommandPayload::SessionTransactionApply(&edits));
    let mark = current_thread_counters();
    let replaced_reply = replaced.replace(&replace_frame, next.as_bytes());
    let replaced_bytes = current_thread_delta_since(mark);
    let mark = current_thread_counters();
    let twin_reply = twin.submit(&transaction);
    let twin_bytes = current_thread_delta_since(mark);
    assert_eq!(reply(&replaced_reply), reply(&twin_reply));
    assert_eq!(reply(&replaced_reply).applied, Some((1, EditPath::Live)));
    let live = |counters: bench_support::alloc::Counters| {
        counters.requested_bytes - counters.released_bytes
    };
    assert_eq!(
        live(replaced_bytes),
        live(twin_bytes),
        "the replace leaves what the transaction leaves: 0 bytes of the document"
    );
    assert_eq!(replaced.state(), twin.state());
}
