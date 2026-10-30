//! Control-plane and C-boundary gates for the runtime modules.

use super::*;
use protocol::{ExpectedRevision, RequestId, SessionRevision, StatusCode};
use session::parse_session_json;

/// The seek runway `R` of a ring of [`limits`]'s 1,024 frames at 48 kHz and quantum 128 (#2205):
/// a live plain seek of a source fed before it lands at the first block at which `R` blocks of
/// its generation are queued.
pub(super) fn runway_blocks() -> u64 {
    u64::from(source::seek_runway_blocks(48_000, 128, 1_024 / 128))
}

/// Region end of the single fixture source, read through the facade's accessor.
fn source_region_end(sources: &SourceControlSet) -> u64 {
    sources
        .region(b"fixture-source")
        .expect("fixture source region")
        .end
}

pub(super) const SESSION: &str =
    include_str!("../../../../fixtures/session/v1/parametric-eq-nine-track.json");

pub(super) fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: crate::COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        reserved1: 0,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
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
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

pub(super) fn command_bytes(request_id: u64, payload: protocol::CommandPayload<'_>) -> Vec<u8> {
    command_bytes_at_revision(request_id, ExpectedRevision::Any, payload)
}

pub(super) fn command_bytes_at_revision(
    request_id: u64,
    expected_revision: ExpectedRevision,
    payload: protocol::CommandPayload<'_>,
) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = protocol::TypedCommandFrame {
        request_id: RequestId::new(request_id).expect("nonzero request"),
        expected_revision,
        payload,
    };
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let len = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("typed command");
    bytes.truncate(len);
    bytes
}

fn pinned_hex(value: &str) -> Vec<u8> {
    assert!(value.len().is_multiple_of(2));
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let digit = |value: u8| match value {
                b'0'..=b'9' => value - b'0',
                b'a'..=b'f' => value - b'a' + 10,
                _ => panic!("pinned lowercase hexadecimal"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

const ALL_COMMAND_RESPONSE_VECTORS: [&str; 10] = [
    // #1627 re-pin: `AUTOMATION_ENQUEUE` is retired. The capabilities record drops fields 9, 12,
    // 19 and 20 (23 fields, was 27; the length 0x188, was 0x1c8) and the flags are 0x7fdf as
    // before; the earlier #1315 re-pin already listed no command `0x0006` and cleared bit 5.
    // #1386 re-pin: the command list adds `SESSION_REPLACE` (`0x0013`): field 25's length is 0x18
    // (was 0x16) and the ID takes the two padding bytes, so the frame length is unchanged.
    // #1595 re-pin: the C ABI advertises the spectrum commands (`0x0010`-`0x0012`) and flag bit 16:
    // field 25's length is 0x1e (two padding bytes follow), the payload 0x190 (was 0x188) and the
    // flags 0x17fdf.
    // #1587a re-pin: field 15, the lossy lane's slots, is 256 (was 2): one whole meter batch.
    // #1586 re-pin: the C ABI advertises `OBSERVATION_READ` (`0x000f`) and flag bit 15: field 25's
    // length is 0x20 (the ID takes the two padding bytes, so the frame length is unchanged) and the
    // flags 0x1ffdf.
    // #1583 re-pin: the lossy lane holds one meter batch whole, two records: telemetry slots
    // (field 15) is 2, was 1.
    // #2115 re-pin: the C ABI advertises `LOUDNESS` (`0x0014`) and flag bit 17: field 25's length
    // is 0x22 (six padding bytes follow), the payload 0x198 (was 0x190) and the flags 0x3ffdf.
    "4d49534f43544c000100000030000200010000009801000001000000000000002a00000000000000170000000000000001000201020000000100000000000000020002010200000000000000000000000300020102000000010000000000000004000201020000000000000000000000050004010800000000100000000000000600030104000000000800000000000007000401080000000010000000000000080001010100000004000000000000000a0004010800000001000000000000000b0004010800000000100000000000000d0004010800000001000000000000000e0004010800000002000000000000000f0004010800000000010000000000001000040108000000100000000000000011000401080000000020000000000000120004010800000000100000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01220000000100020003000700080009000a000b000c000d000e000f00100011001200130014000000000000001a000c010e000000018010801180208021803080408000001b00040108000000dfff030000000000",
    // #338 re-pin: canonical JSON snapshot bytes were 16,712 (0x4148), beginning with `{`.
    // #1093 re-pin: 13,729 (0x35a1) after the fixture moved to decision 12's console shape.
    // #1216 re-pin: 13,918 (0x365e), route field `mute` added, value false (21 bytes, 9 routes).
    // #1218 re-pin: 14,179 (0x3763), route field `follows_mute` added, value false (29 bytes,
    // 9 routes).
    // #1240 re-pin: 14,193 (0x3771), root key `vcas` added, empty (`  "vcas": [],` and its
    // newline, 14 bytes).
    // #1054 re-pin: 14,245 (0x37a5), D1a adds the required `control_smoothing` key, `default`
    // (`  "control_smoothing": {`, `    "kind": "default"` and `  },`, each with its newline, 52
    // bytes). Only the snapshot length moves; the key's bytes are not otherwise pinned here.
    // #1369 re-pin: 14,524 (0x38bc), each of the fixture's nine console entries gains
    // `"link_mode": "slot",` and its newline (31 bytes each, 279 bytes).
    "4d49534f43544c000100000030000200020000004000000002000000000000002a0000000000000004000000000000000100040108000000bc380000000000000200040108000000000000000000000003000a01010000007b0000000000000004000801010000000000000000000000",
    // #1093 re-pin: the first descriptor's rack is `console` (5), was `simd1` (1).
    "4d49534f43544c0001000000300002000c000000d000000003000000000000002a00000000000000050000000000000001000401080000002a0000000000000002000401080000000100000000000000030003010400000001000000000000000400080101000000000000000000000005000b01880000000500000000000000010003010400000001000000000000000200010101000000010000000000000003000b013800000003000000000000000100010101000000020000000000000002000900030000006571300000000000030001010100000006000000000000000400010001000000010000000000000005000900030000006571300000000000",
    "4d49534f43544c0001000000300002000d0000006800000004000000000000002a00000000000000070000000000000001000401080000002a00000000000000020004010800000001000000000000000300030104000000010000000000000004000801010000000000000000000000050002010200000000000000000000000600020102000000080000000000000007000a0100000000",
    // #1635 re-pin: the transport snapshots (`TRANSPORT_GET`, `TRANSPORT_SET`) drop the retired
    // position, field 2: two fields, payload length 0x20 (was 0x30).
    "4d49534f43544c000100000030000200070000002000000005000000000000002a0000000000000002000000000000000100010101000000010000000000000003000401080000000000000000000000",
    "4d49534f43544c000100000030000200080000002000000006000000000000002a0000000000000002000000000000000100010101000000020000000000000003000401080000000000000000000000",
    // #1583 re-pin: the telemetry echo drops the retired meter handles (field 1) and carries the
    // C ABI's meter window, 12 blocks (field 8): the length 0x50 became 0x58, six fields still.
    "4d49534f43544c000100000030000200090000005800000007000000000000002a0000000000000006000000000000000200030104000000000000000000000003000d010000000004000301040000000000000000000000050008010100000000000000000000000600010101000000010000000000000008000300040000000c00000000000000",
    // #1312 re-pin: the counter snapshot gains `LIVE_VALUES_SUPERSEDED` (id 16), value 0, as a
    // third value (a 40-byte `CounterValue` message): the length 0x70 became 0xa0 and the
    // frame's TLV count 3 became 4.
    // #1629 re-pin: the served set loses `CANCELED_AUTOMATION` (id 12), whose 40-byte value
    // message leaves: the length 0x100 became 0xd0 and the TLV count 6 became 5.
    // #1768 re-pin: the served set gains `STRIPS_RESTARTED` (id 18), value 0, as a sixth value
    // (a 40-byte `CounterValue` message): the length 0xd0 became 0x100 and the TLV count 5 became
    // 6.
    // #1521 re-pin: the served set gains `LIVE_BLOCK_ERRORS` (id 19), value 0, as a seventh value
    // (a 40-byte `CounterValue` message): the length 0x100 became 0x130 and the TLV count 6 became
    // 7.
    // #1537 re-pin: the served set gains `SOURCE_UNDERRUN_BLOCKS` (id 20), value 0, as an eighth
    // value (a 40-byte `CounterValue` message): the length 0x130 became 0x160 and the TLV count 7
    // became 8.
    // #1699 re-pin: the served set gains `SOURCE_FRAMES_DISCARDED` (id 21), value 0, as a ninth
    // value (a 40-byte `CounterValue` message): the length 0x160 became 0x190 and the TLV count 8
    // became 9.
    // #1583 re-pin: the served set gains `METER_WINDOWS_LOST` (id 22), value 0, as a tenth value
    // (a 40-byte `CounterValue` message): the length 0x190 became 0x1c0 and the TLV count 9
    // became 10.
    "4d49534f43544c0001000000300002000a000000c001000008000000000000002a000000000000000a000000000000000100040108000000000000000000000002000b01280000000200000000000000010003010400000005000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000006000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000010000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000011000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000012000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000013000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000014000000000000000200040108000000000000000000000002000b01280000000200000000000000010003010400000015000000000000000200040108000000000000000000000002000b012800000002000000000000000100030104000000160000000000000002000401080000000000000000000000",
    "4d49534f43544c0001000000300002000b0000002000000009000000000000002a0000000000000002000000000000000100040108000000000000000000000002000801010000000100000000000000",
    "4d49534f43544c00010000003000020003000000200000000a000000000000002b0000000000000002000000000000000100030104000000010000000000000002000101010000000300000000000000",
];

/// A structural edit that renders identically, for a test that needs a plan rebuild (#1260 D3).
///
/// It changes only the first source's `content` identity, to `blake3:` and `tag` repeated as hex,
/// keeping its length. A session ID edit is model-only and commits without a rebuild, so it can no
/// longer trigger one. Successive rebuilds of one session need distinct tags: rewriting the
/// committed content is a live, record-free delta.
pub(super) fn rebuild_edit(document: &str, tag: u8) -> protocol::SessionEdit {
    let model = parse_session_json(document).expect("rebuild edit session");
    let source = &model.sources[0];
    let content = format!("blake3:{}", format!("{tag:02x}").repeat(32));
    assert_ne!(
        content, source.content,
        "the rebuild edit changes the content"
    );
    protocol::SessionEdit::SetSourceContent {
        source_id: source.id.clone(),
        content,
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    }
}

pub(super) fn generated_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(SESSION).expect("accepted parity base");
    model.sample_rate_hz = sample_rate_hz;
    model.sources[0].frames = 192;
    if track_count == 1 {
        model.tracks.truncate(1);
        model.routes.truncate(1);
    } else {
        assert_eq!(track_count, 10);
        let mut track = model.tracks[8].clone();
        track.id = session::StableId::parse("eq9").expect("tenth track");
        // The console EQ runs on every track (decision 12), so the tenth track's own effect, a
        // bypassed limiter, is an insert after its bypassed EQ.
        track.console[0].bypass = true;
        let mut effect = model.lower_track(&track).pre_insert[0].clone();
        effect.id = session::StableId::parse("limiter").expect("limiter slot");
        effect.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.true-peak-limiter").expect("limiter id"),
        };
        effect.params.clear();
        effect.bypass = true;
        track.inserts.effects = vec![effect];
        let mut route = model.routes[8].clone();
        route.id = session::StableId::parse("eq9-main").expect("tenth route");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("track route")
        };
        *track_id = track.id.clone();
        model.tracks.push(track);
        model.routes.push(route);
    }
    session::canonical_session_json(&model).expect("canonical parity session")
}

pub(super) fn submit_c(
    session: *mut crate::Session,
    generation: u64,
    start_frame: u64,
    sample_rate_hz: u32,
    left: &[f32],
    right: &[f32],
    final_chunk: bool,
) {
    submit_c_to(
        session,
        b"fixture-source",
        generation,
        start_frame,
        sample_rate_hz,
        left,
        right,
        final_chunk,
    );
}

#[allow(clippy::too_many_arguments)]
fn submit_c_to(
    session: *mut crate::Session,
    source_id: &[u8],
    generation: u64,
    start_frame: u64,
    sample_rate_hz: u32,
    left: &[f32],
    right: &[f32],
    final_chunk: bool,
) {
    let planes = [left.as_ptr(), right.as_ptr()];
    let chunk = crate::SourceChunk {
        struct_size: crate::SOURCE_CHUNK_SIZE,
        sample_rate_hz,
        generation,
        start_frame,
        planes: planes.as_ptr(),
        plane_count: 2,
        frames: left.len() as u32,
        end_of_region: u32::from(final_chunk),
        reserved0: 0,
    };
    let mut report = crate::SubmitReport {
        struct_size: crate::SUBMIT_REPORT_SIZE,
        reserved0: 0,
        accepted_frames: 0,
        cumulative_written_frames: 0,
        active_generation: 0,
    };
    assert_eq!(left.len(), right.len());
    assert_eq!(
        crate::ffi::test_source_submit(session, source_id, &chunk, &mut report),
        crate::RESULT_OK
    );
    assert_eq!(report.accepted_frames, left.len() as u64);
}

pub(super) fn boxed_c_children(session: &str) -> (*mut crate::Session, *mut crate::Plan) {
    boxed_c_children_with_limits(session, limits())
}

pub(super) fn boxed_c_children_with_limits(
    session: &str,
    limits: CompileLimits,
) -> (*mut crate::Session, *mut crate::Plan) {
    let children = compile_children(session, limits).expect("C children");
    (
        Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        ))),
        Box::into_raw(Box::new(crate::Plan::new(children.plan))),
    )
}

pub(super) fn command_c(session: *mut crate::Session, request: &[u8]) -> (u32, Vec<u8>) {
    let (result, _, storage) = command_c_capacity(session, request, 4_096);
    (result, storage)
}

pub(super) fn command_c_capacity(
    session: *mut crate::Session,
    request: &[u8],
    capacity: usize,
) -> (u32, u64, Vec<u8>) {
    let mut storage = vec![0xa5_u8; capacity];
    let mut output = crate::BytesOut {
        struct_size: crate::BYTES_OUT_SIZE,
        reserved0: 0,
        data: if storage.is_empty() {
            core::ptr::null_mut()
        } else {
            storage.as_mut_ptr()
        },
        capacity_bytes: storage.len() as u64,
        required_bytes: u64::MAX,
    };
    let result = crate::ffi::test_submit_command(session, request, &mut output);
    if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
        storage.truncate(output.required_bytes as usize);
    }
    (result, output.required_bytes, storage)
}

/// A complete command frame under the retired message ID `0x0006` (`AUTOMATION_ENQUEUE`, #1627)
/// at `revision`, carrying the payload the command had: `count` (field 1), `record_bytes`
/// (field 2) and `records` (field 3, one valid 32-byte point record).
pub(super) fn retired_automation_frame(request_id: u64, revision: u64) -> Vec<u8> {
    let mut bytes = command_bytes(request_id, protocol::CommandPayload::CapabilitiesGet);
    assert_eq!(bytes.len(), protocol::OUTER_HEADER_BYTES);
    bytes[15] = 0; // the revision precondition is exact, as the command's was
    bytes[32..40].copy_from_slice(&revision.to_le_bytes());
    bytes[16..18].copy_from_slice(&0x0006_u16.to_le_bytes());
    // One 32-byte point record: kind 1, three reserved zero bytes, handle 1, start and end
    // 1 << 20 (two `u64`), then both values 0.0 (two `f32`).
    let mut record = [0_u8; 32];
    record[0] = 1;
    record[4] = 1;
    record[8..16].copy_from_slice(&(1_u64 << 20).to_le_bytes());
    record[16..24].copy_from_slice(&(1_u64 << 20).to_le_bytes());
    for (id, wire, value) in [
        (1_u16, 2_u8, &1_u16.to_le_bytes()[..]),
        (2, 2, &32_u16.to_le_bytes()[..]),
        (3, 10, &record[..]),
    ] {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.extend_from_slice(&[wire, 1]);
        bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
        bytes.extend_from_slice(value);
        bytes.resize(bytes.len().next_multiple_of(8), 0);
    }
    let payload = (bytes.len() - protocol::OUTER_HEADER_BYTES) as u32;
    bytes[20..24].copy_from_slice(&payload.to_le_bytes());
    bytes[40..44].copy_from_slice(&3_u32.to_le_bytes());
    bytes
}

/// One decoded success page, or the status and diagnostic codes of a non-OK response.
type PageResult<T> = Result<T, (StatusCode, Vec<String>)>;

/// One `CONTROL_LIST_GET` page through the C ABI, under request ID `request_id`.
pub(super) fn control_list_c(
    session: *mut crate::Session,
    request_id: u64,
    request: &protocol::ControlListRequest,
) -> PageResult<protocol::ControlListPage> {
    let bytes = command_bytes(
        request_id,
        protocol::CommandPayload::ControlListGet(request),
    );
    control_response_c(session, &bytes, |payload| match payload {
        protocol::DecodedSuccessResponsePayload::ControlListPage(page) => page,
        _ => panic!("expected a control list page"),
    })
}

/// One `CONTROL_STATE_GET` page through the C ABI, under request ID `request_id`.
pub(super) fn control_state_c(
    session: *mut crate::Session,
    request_id: u64,
    request: &protocol::ControlListRequest,
) -> PageResult<protocol::ControlStatePage> {
    let bytes = command_bytes(
        request_id,
        protocol::CommandPayload::ControlStateGet(request),
    );
    control_response_c(session, &bytes, |payload| match payload {
        protocol::DecodedSuccessResponsePayload::ControlStatePage(page) => page,
        _ => panic!("expected a control state page"),
    })
}

fn control_response_c<T>(
    session: *mut crate::Session,
    request: &[u8],
    page: impl FnOnce(protocol::DecodedSuccessResponsePayload<'_>) -> T,
) -> PageResult<T> {
    let (result, _, response) = command_c_capacity(session, request, 1 << 20);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = vec![0_u16; 1 << 16];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("control response")
    {
        protocol::DecodedTypedResponseFrame::Success { payload, .. } => Ok(page(payload)),
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => Err((
            header.status,
            payload
                .diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.code)
                .collect(),
        )),
    }
}

/// The whole control list of `session`, in pages of `limit` rows, with the structure epoch and
/// the revision of its first page. Request IDs count up from `request_id`.
pub(super) fn control_list_all_c(
    session: *mut crate::Session,
    request_id: &mut u64,
    limit: u16,
) -> (Vec<protocol::ControlEntry>, u64, u64) {
    let mut entries = Vec::new();
    let mut after_handle = 0;
    let mut first = None;
    loop {
        *request_id += 1;
        let page = control_list_c(
            session,
            *request_id,
            &protocol::ControlListRequest {
                structure_epoch: first.map_or(0, |(epoch, _)| epoch),
                after_handle,
                limit,
                owner: None,
                headers_only: false,
            },
        )
        .expect("a list page");
        first.get_or_insert((page.structure_epoch, page.revision));
        entries.extend(page.entries);
        after_handle = page.last_handle;
        if page.eof {
            let (epoch, revision) = first.expect("a first page");
            return (entries, epoch, revision);
        }
    }
}

/// Every control value of `session`, `(handle, value bits)`, in pages of `limit` rows.
pub(super) fn control_state_all_c(
    session: *mut crate::Session,
    request_id: &mut u64,
    limit: u16,
) -> Vec<(u32, u32)> {
    let mut values = Vec::new();
    let mut after_handle = 0;
    // A cursor names the structure it belongs to: the first page asks for the current one (`0`)
    // and the later pages for the epoch it answered.
    let mut epoch = 0;
    loop {
        *request_id += 1;
        let page = control_state_c(
            session,
            *request_id,
            &protocol::ControlListRequest {
                structure_epoch: epoch,
                after_handle,
                limit,
                owner: None,
                headers_only: false,
            },
        )
        .expect("a state page");
        epoch = page.structure_epoch;
        values.extend(
            page.records
                .iter()
                .map(|record| (record.handle, record.value.to_bits())),
        );
        after_handle = page.last_handle;
        if page.eof {
            return values;
        }
    }
}

#[test]
fn c_commands_observe_the_rendered_plan_sample() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let codec = ProtocolCodec::default();
    let initial_transport = command_bytes(1, protocol::CommandPayload::TransportGet);
    let (result, response) = command_c(c_session, &initial_transport);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::TransportGetSnapshot(initial),
        ..
    } = codec
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("initial transport response")
    else {
        panic!("expected initial transport success")
    };
    assert_eq!(initial.effective_sample, protocol::SampleTime(0));

    let mut pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );

    let transport = command_bytes(3, protocol::CommandPayload::TransportGet);
    let (result, response) = command_c(c_session, &transport);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::TransportGetSnapshot(transport),
        ..
    } = codec
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("rendered transport response")
    else {
        panic!("expected rendered transport success")
    };
    assert_eq!(transport.effective_sample, protocol::SampleTime(128));

    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

pub(super) fn event_c(session: *mut crate::Session, lane: u32) -> (u32, Vec<u8>) {
    let (result, _, storage) = event_c_capacity(session, lane, 4_096);
    (result, storage)
}

pub(super) fn event_c_capacity(
    session: *mut crate::Session,
    lane: u32,
    capacity: usize,
) -> (u32, u64, Vec<u8>) {
    let mut storage = vec![0xa5_u8; capacity];
    let mut output = crate::BytesOut {
        struct_size: crate::BYTES_OUT_SIZE,
        reserved0: 0,
        data: if storage.is_empty() {
            core::ptr::null_mut()
        } else {
            storage.as_mut_ptr()
        },
        capacity_bytes: storage.len() as u64,
        required_bytes: u64::MAX,
    };
    let result = crate::ffi::test_dequeue_event(session, lane, &mut output);
    if result == crate::RESULT_OK && output.required_bytes <= storage.len() as u64 {
        storage.truncate(output.required_bytes as usize);
    }
    (result, output.required_bytes, storage)
}

fn event_c_exact_retry(session: *mut crate::Session, lane: u32, oracle: &[u8]) {
    let (query_result, required, query) = event_c_capacity(session, lane, 0);
    assert_eq!(query_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(required, oracle.len() as u64);
    assert!(query.is_empty());
    let (short_result, short_required, short) = event_c_capacity(session, lane, oracle.len() - 1);
    assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(short_required, oracle.len() as u64);
    assert!(short.iter().all(|byte| *byte == 0xa5));
    let (exact_result, exact_required, exact) = event_c_capacity(session, lane, oracle.len());
    assert_eq!(exact_result, crate::RESULT_OK);
    assert_eq!(exact_required, oracle.len() as u64);
    assert_eq!(exact, oracle);
}

fn render_parity_shape(track_count: usize, sample_rate_hz: u32) {
    let session = generated_parity_session(track_count, sample_rate_hz);
    let mut direct = compile_children(&session, limits()).expect("direct children");
    let wrapped = compile_children(&session, limits()).expect("C children");
    let c_session = Box::into_raw(Box::new(crate::Session::new(
        wrapped.session,
        wrapped.session_error,
    )));
    let c_plan = Box::into_raw(Box::new(crate::Plan::new(wrapped.plan)));
    let quantum = 128_usize;
    let mut first_left = vec![0.0_f32; quantum];
    let mut first_right = vec![0.0_f32; quantum];
    first_left[0] = -0.0;
    first_right[0] = 0.0;
    first_left[1] = 0.25;
    first_right[1] = -0.5;
    let final_left = vec![0.125_f32; 64];
    let final_right = vec![-0.25_f32; 64];

    for block in 0..8_u64 {
        match block {
            0 | 3 => {
                let generation = if block == 0 { 1 } else { 2 };
                if block == 3 {
                    direct
                        .session
                        .seek(b"fixture-source", generation, 0)
                        .expect("direct seek");
                    assert_eq!(
                        crate::ffi::test_source_seek(c_session, b"fixture-source", generation, 0,),
                        crate::RESULT_OK
                    );
                }
                direct
                    .session
                    .submit(
                        b"fixture-source",
                        SourceSubmission {
                            generation,
                            start_frame: 0,
                            sample_rate_hz,
                            planes: &[&first_left, &first_right],
                            frames: quantum as u32,
                            end_of_region: false,
                        },
                    )
                    .expect("direct full chunk");
                submit_c(
                    c_session,
                    generation,
                    0,
                    sample_rate_hz,
                    &first_left,
                    &first_right,
                    false,
                );
            }
            1 | 4 => {
                let generation = if block == 1 { 1 } else { 2 };
                direct
                    .session
                    .submit(
                        b"fixture-source",
                        SourceSubmission {
                            generation,
                            start_frame: 128,
                            sample_rate_hz,
                            planes: &[&final_left, &final_right],
                            frames: 64,
                            end_of_region: true,
                        },
                    )
                    .expect("direct partial final");
                submit_c(
                    c_session,
                    generation,
                    128,
                    sample_rate_hz,
                    &final_left,
                    &final_right,
                    true,
                );
            }
            _ => {}
        }

        let mut direct_pcm = vec![f32::NAN; quantum * 2];
        direct
            .plan
            .render(
                block * quantum as u64,
                PlanarBufferMut::try_new(&mut direct_pcm, 2, quantum, quantum)
                    .expect("direct output"),
            )
            .expect("direct render");
        let mut c_pcm = vec![f32::NAN; quantum * 2];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: c_pcm.as_mut_ptr(),
            sample_capacity: c_pcm.len() as u64,
            frames: quantum as u32,
            plane_stride_samples: quantum as u32,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, block * quantum as u64, &output),
            crate::RESULT_OK
        );
        assert_eq!(
            c_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            direct_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "direct/C parity for {track_count} tracks at {sample_rate_hz} Hz block {block}"
        );
    }
    if track_count == 1 {
        crate::ffi::test_session_destroy(c_session);
        crate::ffi::test_plan_destroy(c_plan);
    } else {
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }
}

/// The caller's meter caps bound every meter stream (#1581, root caps ruling): one per strip plus
/// the output. A cap of one stream per strip refuses the session, with the builtin compiler's own
/// diagnostic, and one more stream admits it.
///
/// Test value: red if the core's strip meters or its output meter escape the caller's caps, or if
/// the refusal loses its code.
#[test]
fn a_meter_cap_below_the_strip_count_is_refused() {
    let strips = parse_session_json(SESSION)
        .expect("session")
        .strips()
        .count();
    assert!(strips > 1, "the reference session has several strips");
    let mut small = limits();
    small.maximum_meter_streams = strips as u64;
    let failure = compile_children(SESSION, small)
        .err()
        .expect("one meter stream per strip leaves none for the output");
    let diagnostics = String::from_utf8_lossy(&failure.diagnostics);
    assert!(
        diagnostics.contains("builtin.resource.limit\t$.meter_requests"),
        "{diagnostics}"
    );
    small.maximum_meter_streams += 1;
    assert!(
        compile_children(SESSION, small).is_ok(),
        "strips plus the output fit"
    );
}

/// #1581 gate 6 on the C ABI: a two-strip plan's provider epoch keeps one post-matrix meter
/// consumer per strip, in strip order, and its resource row counts their bytes. (Gate 7, the
/// output meter on the C ABI render call, is #1583's windowed-output-peaks test and the race
/// test's allocation count.)
///
/// Test value: red if the C ABI still drops its meter handles.
#[test]
fn a_c_abi_plan_keeps_its_strip_meters() {
    bench_support::alloc::assert_installed();
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    assert!(model.submixes.is_empty());
    let document = session::canonical_session_json(&model).expect("two-strip session");
    let mut children = compile_children(&document, limits()).expect("children");
    let meters: Vec<(String, builtins::MeterTap)> = children
        .session
        .test_providers()
        .test_meters()
        .iter()
        .map(|meter| (meter.track_id.to_string(), meter.tap))
        .collect();
    let strips: Vec<(String, builtins::MeterTap)> = model
        .strips()
        .map(|strip| (strip.id.as_str().to_owned(), builtins::MeterTap::PostMatrix))
        .collect();
    assert_eq!(meters, strips, "one post-matrix meter per strip");
    assert!(children.plan.resources().builtin_meter_payload_bytes > 0);

    // #1583: the session owns the output meter's delivery from boot; the adapter takes nothing.
    assert!(children.session.take_output_meter().is_none());
}

/// #1674 gate 1: across a carried C ABI rebuild (a muted track whose ID sorts first), the strip
/// meter consumer the running epoch holds for the old strip 0, now strip 1, is the one it held
/// before: the window that opened on the predecessor and closes on the successor arrives on it.
/// (A strip queue holds one window and nothing pops it, so the window spans the swap.)
///
/// Test value: red if the promoted epoch keeps the successor's fresh consumer for a carried strip,
/// whose producer the swap block moved into the retiring plan; no other test reads the C ABI's
/// strip consumers across a rebuild.
#[test]
fn a_carried_strip_meter_keeps_its_reader_across_a_c_abi_rebuild() {
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    let document = session::canonical_session_json(&model).expect("two-strip session");
    let mut children = compile_children(&document, limits()).expect("children");
    // #882: strip meters observe only the blocks the output meter's gate admits, so the level is
    // on from block 0: a meter table configured before any render (#1583).
    let configure = meter_telemetry(1, 1);
    children
        .session
        .command(&configure, 4_096)
        .expect("the meter table is accepted");
    let window = u64::from(host_core::default_meter_window_blocks(48_000, 128).get());
    // Feeds one block ahead and renders `blocks` blocks from `first`; every submit runs the
    // service step first.
    let run = |children: &mut super::CompiledChildren, first: u64, blocks: u64| {
        let mut feed = |frame: u64| {
            let (left, right) = continuity_block(frame, 0);
            children
                .session
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: frame * 128,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: 128,
                        end_of_region: false,
                    },
                )
                .expect("submit");
        };
        if first == 0 {
            feed(0);
        }
        for block in first..first + blocks {
            feed(block + 1);
            let mut pcm = [0.0_f32; 256];
            children
                .plan
                .render(
                    block * 128,
                    PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
                )
                .expect("render");
        }
    };
    // The queued windows of `meter`, peeked in order: (sequence, start, end).
    let windows = |meter: &host_core::MeterConsumer| -> Vec<(u64, u64, u64)> {
        (0..)
            .map_while(|index| meter.consumer.peek(index))
            .map(|snapshot| {
                (
                    snapshot.window_sequence,
                    snapshot.start_sample,
                    snapshot.end_sample,
                )
            })
            .collect()
    };
    let swap = window / 2;
    run(&mut children, 0, swap);
    let meter = &children.session.test_providers().test_meters()[0];
    let strip = meter.track_id.clone();
    assert!(windows(meter).is_empty(), "window 0 is open at the rebuild");

    let transaction = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&add_muted_track(&model, "a-muted")),
    );
    children
        .session
        .command(&transaction, 4_096)
        .expect("the rebuild commits");
    run(&mut children, swap, window - swap + 1);
    children.session.service().expect("service");
    assert_eq!(children.session.test_providers().test_epoch(), 1);
    assert!(children.session.test_pending_providers().is_empty());
    let meter = &children.session.test_providers().test_meters()[1];
    assert_eq!(meter.track_id, strip, "the old strip 0 is strip 1");
    assert_eq!(
        windows(meter),
        [(0, 0, window * 128)],
        "the reader the epoch held receives the carried window 0, opened before the swap"
    );
    let snapshot = meter.consumer.peek(0).expect("window 0");
    assert_eq!(
        (
            snapshot.cumulative_dropped_snapshots,
            snapshot.cumulative_discontinuities
        ),
        (0, 0),
        "no loss is counted across the swap"
    );
}

#[test]
fn ring_zero_derives_from_the_document_and_matches_the_explicit_value() {
    let model = parse_session_json(SESSION).expect("session");
    let derived = host_core::default_source_ring_frames(model.sample_rate_hz, model.quantum_frames);
    assert_eq!(derived, 7_296);

    let mut zero = limits();
    zero.source_ring_frames = 0;
    let zero = compile_children(SESSION, zero).expect("zero derives");

    let mut explicit = limits();
    explicit.source_ring_frames = derived;
    let explicit = compile_children(SESSION, explicit).expect("explicit derived value");

    assert_eq!(zero.plan.resources(), explicit.plan.resources());
    assert_eq!(
        zero.session
            .test_providers()
            .test_sources()
            .retained_bytes(),
        explicit
            .session
            .test_providers()
            .test_sources()
            .retained_bytes()
    );
}

#[test]
fn structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic() {
    let mut children = compile_children(SESSION, limits()).expect("children");
    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("old provider source block");
    let mut pcm = [0.0_f32; 256];
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("old output"),
        )
        .expect("old plan block");
    assert!(pcm.iter().any(|sample| *sample != 0.0), "old provider PCM");
    // A rebuild that leaves the source's declaration alone, so its ring carries (#1273 D1).
    let edits = add_muted_track(&parse_session_json(SESSION).expect("model"), "a-muted");
    let first_request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert!(matches!(
        children.session.command(&first_request, 0),
        Err(CommandError::BufferTooSmall { required: 4_096 })
    ));
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(42)
    );
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    assert_eq!(children.plan.owner().active_epoch().0, 0);

    let first_len = children
        .session
        .command(&first_request, 4_096)
        .expect("first structural command");
    let first_response = children.session.command_response(first_len).to_vec();
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(43)
    );
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    assert_eq!(children.session.test_pending_providers()[0].test_epoch(), 1);
    assert_eq!(children.plan.owner().active_epoch().0, 0);
    assert_eq!(children.session.test_controller().replay().len(), 1);
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 128,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("a submission after the commit feeds the newest committed session");

    let required = match children.session.dequeue_event(EventLane::Reliable, 0) {
        Err(EventError::BufferTooSmall { required }) => required,
        other => panic!("expected reliable query length, got {other:?}"),
    };
    let event_len = children
        .session
        .dequeue_event(EventLane::Reliable, required)
        .expect("reliable retry")
        .expect("session event");
    let event = children.session.event_response(event_len).to_vec();
    let mut fields = [0_u16; 64];
    assert!(matches!(
        ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("session event"),
        protocol::DecodedTypedEventFrame {
            header,
            payload: protocol::DecodedEventPayload::SessionCommitted(_),
        } if header.revision == SessionRevision(43)
    ));
    assert_eq!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 0)
            .expect("empty reliable lane"),
        None
    );

    let model = parse_session_json(SESSION).expect("source-changing model");
    let source = &model.sources[0];
    let second_edit = protocol::SessionEdit::SetSourceContent {
        source_id: model.sources[0].id.clone(),
        content: source.content.clone(),
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: 512,
    };
    let second_request = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&second_edit)),
    );
    // The swap block's continuity is `a_c_abi_structural_transaction_keeps_the_source_playing`'s.
    children
        .plan
        .render(
            128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("replacement boundary");
    assert_eq!(children.plan.owner().active_epoch().0, 1);
    assert_eq!(children.session.test_providers().test_epoch(), 0);
    children
        .session
        .service()
        .expect("control promotion and retirement");
    assert_eq!(children.session.test_providers().test_epoch(), 1);
    assert!(children.session.test_pending_providers().is_empty());

    let second_len = children
        .session
        .command(&second_request, 4_096)
        .expect("source-changing replacement after reclaim");
    assert!(second_len > 0);
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(44)
    );
    assert_eq!(
        source_region_end(children.session.test_providers().test_sources()),
        48_000
    );
    assert_eq!(children.session.test_pending_providers()[0].test_epoch(), 2);
    assert_eq!(
        source_region_end(children.session.test_pending_providers()[0].test_sources()),
        512
    );
    children
        .plan
        .render(
            256,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("second replacement boundary");
    children
        .session
        .service()
        .expect("second provider promotion and retirement");
    // #1273 D1: the successor is diffed against the committed model before the transaction, so
    // the source whose declaration changed (frames 512) restarts in a new ring at generation 1,
    // frame 0, rather than carrying its old ring (written to frame 256) into the new plan.
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 1,
                start_frame: 0,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: false,
            },
        )
        .expect("a changed source restarts its feed at generation 1, frame 0");
    assert_eq!(children.session.test_providers().test_epoch(), 2);
    assert_eq!(
        source_region_end(children.session.test_providers().test_sources()),
        512
    );
    assert!(children.session.test_pending_providers().is_empty());
    assert!(children.session.test_retired_providers().is_empty());
    children
        .session
        .seek(b"fixture-source", 2, 384)
        .expect("seek new source-changing provider");
    children
        .session
        .submit(
            b"fixture-source",
            SourceSubmission {
                generation: 2,
                start_frame: 384,
                sample_rate_hz: 48_000,
                planes: &[&left, &right],
                frames: 128,
                end_of_region: true,
            },
        )
        .expect("new source-changing provider PCM");
    pcm.fill(f32::NAN);
    children
        .plan
        .render(
            384,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("new provider output"),
        )
        .expect("new provider render");
    assert!(
        pcm.iter().any(|sample| *sample != 0.0),
        "source-changing provider produces submitted PCM"
    );

    let replay_len = children
        .session
        .command(&first_request, first_len as u64)
        .expect("exact structural replay");
    assert_eq!(
        children.session.command_response(replay_len),
        first_response
    );
    assert_eq!(
        children.session.test_controller().session().revision(),
        SessionRevision(44)
    );
    assert!(children.session.test_pending_providers().is_empty());
}

/// A test signal with no exact-zero sample (issue #1269's acceptance shape): every frame and
/// channel differs, so a block rendered from the wrong source position cannot match.
fn continuity_signal(frame: u64, channel: usize, salt: u64) -> f32 {
    let step = ((frame + salt * 131) % 997) as f32 * 1.0e-4;
    if channel == 0 {
        0.125 + step
    } else {
        -0.25 - step
    }
}

/// One block of [`continuity_signal`] at source frame `block * 128`.
pub(super) fn continuity_block(block: u64, salt: u64) -> (Vec<f32>, Vec<f32>) {
    let start = block * 128;
    (
        (start..start + 128)
            .map(|frame| continuity_signal(frame, 0, salt))
            .collect(),
        (start..start + 128)
            .map(|frame| continuity_signal(frame, 1, salt))
            .collect(),
    )
}

/// The gap-free acceptance session: tracks `eq0`-`eq2` on the fixture source, empty console
/// sections, no inserts, and the input filters off, so no unchanged path holds DSP state.
pub(super) fn stateless_session(sample_rate_hz: u32) -> session::SessionModel {
    let mut model = parse_session_json(SESSION).expect("fixture");
    model.sample_rate_hz = sample_rate_hz;
    model.tracks.truncate(3);
    model.routes.truncate(3);
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for builtins in [&mut track.builtins.left, &mut track.builtins.right] {
            builtins.hpf_hz = 0.0;
            builtins.lpf_hz = 0.0;
        }
    }
    model
}

/// `UpsertTrack` of a muted copy of the first track, whose ID sorts before every other, and
/// `UpsertRoute` of its route: the added strip contributes exact zeros.
pub(super) fn add_muted_track(
    model: &session::SessionModel,
    id: &str,
) -> [protocol::SessionEdit; 2] {
    let mut track = model.tracks[0].clone();
    track.id = session::StableId::parse(id).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse(&format!("{id}-main")).expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        protocol::SessionEdit::UpsertTrack { track },
        protocol::SessionEdit::UpsertRoute { route },
    ]
}

/// `RemoveRoute` and `RemoveTrack` of a track [`add_muted_track`] added.
fn remove_track(id: &str) -> [protocol::SessionEdit; 2] {
    [
        protocol::SessionEdit::RemoveRoute {
            route_id: session::StableId::parse(&format!("{id}-main")).expect("route ID"),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: session::StableId::parse(id).expect("track ID"),
        },
    ]
}

/// The committed session's canonical JSON, read through `SESSION_SNAPSHOT_GET` in chunks.
fn snapshot_c(session: *mut crate::Session) -> String {
    let mut json = Vec::new();
    for request_id in 1_000_u64.. {
        let request = command_bytes(
            request_id,
            protocol::CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                offset: json.len() as u64,
                maximum_bytes: 2_048,
            }),
        );
        let (result, response) = command_c(session, &request);
        assert_eq!(result, crate::RESULT_OK);
        let mut fields = [0_u16; 64];
        let protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::SessionSnapshot(chunk),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("snapshot response")
        else {
            panic!("expected a snapshot chunk")
        };
        json.extend_from_slice(chunk.canonical_json_chunk);
        if chunk.eof {
            break;
        }
    }
    String::from_utf8(json).expect("canonical JSON")
}

/// Renders one 128-frame block through the exported entry point.
fn render_c(plan: *mut crate::Plan, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(plan, block * 128, &output),
        crate::RESULT_OK
    );
    pcm
}

/// [`feed_and_render_c`] from `first` until the watermark reports `revision` (#1325 D3: a strip
/// removal is adopted at `S`); the next block.
fn feed_and_render_until_c(
    session: *mut crate::Session,
    plan: *mut crate::Plan,
    sample_rate_hz: u32,
    first: u64,
    revision: u64,
) -> u64 {
    let mut block = first;
    while crate::ffi::test_plan_watermark(plan).1.revision < revision {
        assert!(block < first + 64, "revision {revision} is adopted");
        feed_and_render_c(session, plan, sample_rate_hz, block, 1);
        block += 1;
    }
    // The service step promotes the adopted successor's epoch, as the next control call would.
    assert_eq!(crate::ffi::test_service(session), crate::RESULT_OK);
    block
}

/// Feeds `fixture-source` one block ahead and renders `blocks` blocks from `first`.
fn feed_and_render_c(
    session: *mut crate::Session,
    plan: *mut crate::Plan,
    sample_rate_hz: u32,
    first: u64,
    blocks: u64,
) -> Vec<Vec<f32>> {
    (first..first + blocks)
        .map(|block| {
            let (left, right) = continuity_block(block + 1, 0);
            submit_c(
                session,
                1,
                (block + 1) * 128,
                sample_rate_hz,
                &left,
                &right,
                false,
            );
            render_c(plan, block)
        })
        .collect()
}

/// Issue #1273 gate 1: across a C ABI structural transaction (`UpsertTrack` of a muted track and
/// its route), every block is bit-identical to the committed post-edit session compiled fresh and
/// fed the same PCM from frame 0. The source keeps its ring, generation and position: no seek, no
/// refill.
///
/// Test value: red if the successor allocates a fresh ring for the unchanged source, if the
/// producer stays in the old set, or if a submit after the commit is routed to the old set
/// (`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` pinned the
/// opposite, a silent swap block, before this issue).
fn structural_transaction_keeps_the_source_playing_at(sample_rate_hz: u32) {
    let model = stateless_session(sample_rate_hz);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, sample_rate_hz, &left, &right, false);
    let mut swapped = feed_and_render_c(c_session, c_plan, sample_rate_hz, 0, 6);
    let edits = add_muted_track(&model, "a-muted");
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(command_c(c_session, &transaction).0, crate::RESULT_OK);
    swapped.extend(feed_and_render_c(c_session, c_plan, sample_rate_hz, 6, 6));
    let carried = crate::ffi::test_plan_carry_counts(c_plan);
    assert_eq!(carried, (1, 0), "one swap, and it carried");
    let committed = snapshot_c(c_session);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);

    let (r_session, r_plan) = boxed_c_children(&committed);
    let (left, right) = continuity_block(0, 0);
    submit_c(r_session, 1, 0, sample_rate_hz, &left, &right, false);
    let reference = feed_and_render_c(r_session, r_plan, sample_rate_hz, 0, 12);
    crate::ffi::test_plan_destroy(r_plan);
    crate::ffi::test_session_destroy(r_session);

    for (block, (swapped, reference)) in swapped.iter().zip(&reference).enumerate() {
        assert!(
            reference.iter().all(|sample| *sample != 0.0),
            "{sample_rate_hz} Hz block {block}: the reference plays"
        );
        let swapped_bits = swapped
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>();
        let reference_bits = reference
            .iter()
            .map(|sample| sample.to_bits())
            .collect::<Vec<_>>();
        assert_eq!(
            swapped_bits, reference_bits,
            "{sample_rate_hz} Hz block {block}: bit-identical to the fresh post-edit session"
        );
    }
}

#[test]
fn a_c_abi_structural_transaction_keeps_the_source_playing() {
    structural_transaction_keeps_the_source_playing_at(48_000);
    structural_transaction_keeps_the_source_playing_at(96_000);
}

/// Issue #1275 gate 1: a stem a C ABI structural transaction adds (`UpsertSource`, `UpsertTrack`
/// on it and its route) starts in exact time through `miso_engine_v1_source_seek_at`. The host
/// anchors generation 2 at frame `A` to render sample `A`, two blocks past the next render (which
/// also swaps the plan in), and submits it from `A`. Every block is bit-identical to the post-edit
/// session compiled fresh, fed the first stem from frame 0 and the added stem zeros for frames
/// below `A` and the same PCM from `A`.
///
/// Test value: red if the export forwards to the plain seek (observation timing): the seek then
/// applies at the swap block, two blocks before the anchor, and the stem is out of time from the
/// anchor block on (block 8 differs); red as well if the anchored seek goes to the running plan's
/// source set, which does not hold the added source until the swap. With `source_frame` (`F`)
/// unequal to the anchor (`A`), red too if the export or the session swaps the two.
fn an_added_source_starts_at_its_anchor_at(sample_rate_hz: u32, source_frame: u64) {
    const ADDED: &[u8] = b"s2-source";
    const ANCHOR_BLOCK: u64 = 8;
    let anchor = ANCHOR_BLOCK * 128;
    // The stem's frame `F + n` holds the content the reference plays at render frame `A + n`.
    let stem_frame = |render_frame: u64| render_frame - anchor + source_frame;
    let mut model = stateless_session(sample_rate_hz);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, sample_rate_hz, &left, &right, false);
    let mut swapped = feed_and_render_c(c_session, c_plan, sample_rate_hz, 0, 6);

    let mut source = model.sources[0].clone();
    source.id = session::StableId::parse("s2-source").expect("source ID");
    let mut track = model.tracks[0].clone();
    track.id = session::StableId::parse("t2").expect("track ID");
    track.source_id = source.id.clone();
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse("t2-main").expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    let edits = [
        protocol::SessionEdit::UpsertSource { source },
        protocol::SessionEdit::UpsertTrack { track },
        protocol::SessionEdit::UpsertRoute { route },
    ];
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    assert_eq!(command_c(c_session, &transaction).0, crate::RESULT_OK);
    assert_eq!(
        crate::ffi::test_source_seek_at(c_session, ADDED, 2, source_frame, anchor),
        crate::RESULT_OK
    );
    for block in 6..16 {
        let next = block + 1;
        let (left, right) = continuity_block(next, 0);
        submit_c(
            c_session,
            1,
            next * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
        if next >= ANCHOR_BLOCK {
            let (left, right) = continuity_block(next, 1);
            submit_c_to(
                c_session,
                ADDED,
                2,
                stem_frame(next * 128),
                sample_rate_hz,
                &left,
                &right,
                false,
            );
        }
        swapped.push(render_c(c_plan, block));
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(c_plan),
        (1, 0),
        "one swap, and it carried the first stem"
    );
    let committed = snapshot_c(c_session);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);

    // #1288: `t2` fades in at its first played block, the anchor (no latency before its fader).
    let (committed, revision, unmute) = with_added_track_muted(&committed, "t2");
    let (r_session, r_plan) = boxed_c_children(&committed);
    let feed_reference = |block: u64| {
        let (left, right) = continuity_block(block, 0);
        submit_c(
            r_session,
            1,
            block * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
        let (left, right) = if block < ANCHOR_BLOCK {
            (vec![0.0; 128], vec![0.0; 128])
        } else {
            continuity_block(block, 1)
        };
        submit_c_to(
            r_session,
            ADDED,
            1,
            block * 128,
            sample_rate_hz,
            &left,
            &right,
            false,
        );
    };
    feed_reference(0);
    let reference = (0..16)
        .map(|block| {
            feed_reference(block + 1);
            if block == ANCHOR_BLOCK {
                unmute_c(r_session, revision, &unmute);
            }
            render_c(r_plan, block)
        })
        .collect::<Vec<_>>();
    crate::ffi::test_plan_destroy(r_plan);
    crate::ffi::test_session_destroy(r_session);
    assert_bit_identical(
        &format!("{sample_rate_hz} Hz added stem from frame {source_frame}"),
        0,
        &swapped,
        &reference,
    );
}

#[test]
fn an_added_c_abi_source_starts_at_its_anchored_render_sample() {
    an_added_source_starts_at_its_anchor_at(48_000, 8 * 128);
    an_added_source_starts_at_its_anchor_at(96_000, 8 * 128);
    // `F != A`: the stem's frame 0 enters the graph at the anchor block.
    an_added_source_starts_at_its_anchor_at(48_000, 0);
}

/// The seek report of `id` through the exported entry point, as `(generation, first_sample,
/// source_frame, cumulative_underrun_frames, cumulative_held_frames)`.
fn seek_report_c(session: *mut crate::Session, id: &[u8]) -> (u64, u64, u64, u64, u64) {
    let (result, report) = crate::ffi::test_source_seek_report(session, id);
    assert_eq!(result, crate::RESULT_OK);
    (
        report.generation,
        report.first_sample,
        report.source_frame,
        report.cumulative_underrun_frames,
        report.cumulative_held_frames,
    )
}

/// Issue #1316 gates 3 and 4: scrub one stem, then start another aligned to it.
///
/// One track plays stem X. After block 5 the host plain-seeks X to generation 2 at frame 4096 and
/// reads its report `(g, s, f)` after block 6. A transaction then adds stem Y on its own track;
/// before the swap block renders, Y reads generation 0. The host anchors Y at `A` = 1280 (block
/// 10) on frame `f + (A - s)` and submits it from there. From A on, every block is bit-identical
/// to the committed session compiled fresh and fed both stems from the aligned frames. X's report
/// is unchanged by the swap that carries it; Y reads `(1, 896, 0)` after the swap block (held
/// while its seek waits) and `(2, A, f + (A - s))` from A. One starved block after the run shows
/// the carried ring still publishes to the host's reader.
///
/// Test value: red if the report's sample and frame are not on the anchor's clock (Y is then out of
/// time with X from A), if the record lives on the plan instead of the ring (X's report is lost at
/// the swap, or no longer moves after it), or if an added ring inherits a predecessor's record.
#[test]
fn a_scrubbed_stem_and_a_stem_started_from_its_report_stay_aligned() {
    const X: &[u8] = b"fixture-source";
    const Y: &[u8] = b"s2-source";
    const ANCHOR_BLOCK: u64 = 10;
    const BLOCKS: u64 = 16;
    let anchor = ANCHOR_BLOCK * 128;
    let mut model = stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (session, plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(session, 1, 0, 48_000, &left, &right, false);
    let _ = feed_and_render_c(session, plan, 48_000, 0, 6);

    // Scrub X during playback; its runway lands it at the next block (#2205).
    assert_eq!(
        crate::ffi::test_source_seek(session, X, 2, 4_096),
        crate::RESULT_OK
    );
    let runway = runway_blocks();
    for block in 32..32 + runway {
        submit_block_c(session, X, 2, block, 0, 48_000);
    }
    let _ = render_c(plan, 6);
    let (g, s, f, _, _) = seek_report_c(session, X);
    assert_eq!((g, s, f), (2, 768, 4_096));
    // X's frame at render sample `t` is `f + (t - s)`, in blocks of 128.
    let x_block = |block: u64| (f + block * 128 - s) / 128;

    transact_c(
        session,
        1,
        42,
        &add_source_and_track(&model, "s2-source", "t2"),
    );
    assert_eq!(
        seek_report_c(session, Y),
        (0, 0, 0, 0, 0),
        "an added ring has not rendered"
    );
    let y_frame = f + (anchor - s);
    assert_eq!(
        crate::ffi::test_source_seek_at(session, Y, 2, y_frame, anchor),
        crate::RESULT_OK
    );
    let mut rendered = Vec::new();
    for block in 7..BLOCKS {
        // X stays its runway ahead.
        submit_block_c(session, X, 2, x_block(block) + runway - 1, 0, 48_000);
        if block >= ANCHOR_BLOCK {
            submit_block_c(session, Y, 2, x_block(block), 1, 48_000);
        }
        rendered.push(render_c(plan, block));
        assert_eq!(
            seek_report_c(session, X),
            (2, 768, 4_096, 0, 0),
            "block {block}"
        );
        let held = 128 * (block.min(ANCHOR_BLOCK - 1) - 6);
        let expected_y = if block < ANCHOR_BLOCK {
            (1, 896, 0, 0, held)
        } else {
            (2, anchor, y_frame, 0, held)
        };
        assert_eq!(seek_report_c(session, Y), expected_y, "block {block}");
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(plan),
        (1, 0),
        "one swap, and it carried X"
    );
    // A starved block after the swap, once X's runway is played: the carried ring still
    // publishes to the host's reader.
    for block in BLOCKS..BLOCKS + runway {
        let _ = render_c(plan, block);
    }
    assert_eq!(seek_report_c(session, X), (2, 768, 4_096, 128, 0));
    let committed = snapshot_c(session);
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);

    // The reference reads both stems from the aligned frames, from its own block 0, which is the
    // anchor: `t2` fades in there (#1288).
    let (committed, revision, unmute) = with_added_track_muted(&committed, "t2");
    let reference = render_fresh_reference(&committed, BLOCKS - ANCHOR_BLOCK, &|session, block| {
        if block == 0 {
            unmute_c(session, revision, &unmute);
        }
        submit_c_to_block(session, X, x_block(ANCHOR_BLOCK + block), block, 0);
        submit_c_to_block(session, Y, x_block(ANCHOR_BLOCK + block), block, 1);
    });
    assert_bit_identical(
        "scrubbed X with Y aligned from its report",
        ANCHOR_BLOCK,
        &rendered[(ANCHOR_BLOCK - 7) as usize..],
        &reference,
    );
}

/// Submits the content of block `content` (salt `salt`) as generation 1 at source frame
/// `block * 128` of `source`.
fn submit_c_to_block(
    session: *mut crate::Session,
    source: &[u8],
    content: u64,
    block: u64,
    salt: u64,
) {
    let (left, right) = continuity_block(content, salt);
    submit_c_to(
        session,
        source,
        1,
        block * 128,
        48_000,
        &left,
        &right,
        false,
    );
}

/// Submits block `block` of each of `ids`' signals (salted by position) through the session state.
pub(super) fn submit_direct(children: &mut CompiledChildren, ids: &[&[u8]], block: u64) {
    for (salt, id) in ids.iter().enumerate() {
        let (left, right) = continuity_block(block, salt as u64);
        let report = children
            .session
            .submit(
                id,
                SourceSubmission {
                    generation: 1,
                    start_frame: block * 128,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("block {block}: {:?}", error.report()));
        assert_eq!(report.accepted_frames, 128);
    }
}

/// Renders block `block` through the plan state.
pub(super) fn render_direct(children: &mut CompiledChildren, block: u64) -> Vec<f32> {
    let mut pcm = vec![f32::NAN; 256];
    children
        .plan
        .render(
            block * 128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("block");
    pcm
}

/// Feeds `ids` one block ahead and renders `blocks` blocks from `first`.
pub(super) fn feed_and_render_direct(
    children: &mut CompiledChildren,
    ids: &[&[u8]],
    first: u64,
    blocks: u64,
) -> Vec<Vec<f32>> {
    (first..first + blocks)
        .map(|block| {
            submit_direct(children, ids, block + 1);
            render_direct(children, block)
        })
        .collect()
}

/// Feeds `ids` one block ahead and renders from `first` until the watermark reports `revision`:
/// a transaction that removes a strip is adopted at its scheduled sample `S`, after the strip's
/// ramp (#1325 D3). The blocks rendered and the next block.
pub(super) fn feed_and_render_until_direct(
    children: &mut CompiledChildren,
    ids: &[&[u8]],
    first: u64,
    revision: u64,
) -> (Vec<Vec<f32>>, u64) {
    let mut blocks = Vec::new();
    let mut block = first;
    while children
        .plan
        .queries()
        .watermark()
        .expect("watermark")
        .revision
        < revision
    {
        assert!(block < first + 64, "revision {revision} is adopted");
        submit_direct(children, ids, block + 1);
        blocks.push(render_direct(children, block));
        block += 1;
    }
    // The service step promotes the adopted successor's epoch, as the next control call would.
    children.session.service().expect("service");
    (blocks, block)
}

/// Renders through the plan owner alone, the first half of `PlanState::render`, block by block
/// from `sample` until a block applies a swap: a rebuild that restarts a strip is a duck-swap,
/// adopted at its `S` (#1324a), so the swap window may open blocks after the commit. The swap
/// block's report and its first sample. The blocks before it swap nothing, so the epoch the second
/// half would publish for them is the one already published.
pub(super) fn render_owner_until_swap(
    children: &mut CompiledChildren,
    pcm: &mut [f32; 256],
    sample: u64,
) -> (engine::realtime::RealtimeRenderReport, u64) {
    let mut at = sample;
    loop {
        assert!(at < sample + 64 * 128, "a swap within 64 blocks");
        let report = children
            .plan
            .test_owner_mut()
            .render_contiguous(
                RenderIo {
                    output: PlanarBufferMut::try_new(pcm, 2, 128, 128).expect("output"),
                },
                at,
            )
            .expect("a block");
        if report.swap == engine::realtime::SwapOutcome::Applied {
            return (report, at);
        }
        at += 128;
    }
}

fn assert_bit_identical(label: &str, first: u64, actual: &[Vec<f32>], expected: &[Vec<f32>]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let block = first + index as u64;
        assert!(
            expected.iter().all(|sample| *sample != 0.0),
            "{label} block {block}: the reference plays"
        );
        assert_eq!(
            actual
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "{label} block {block}: bit-identical to the reference"
        );
    }
}

/// Submits block `block` of `signal` salt `salt` to `source` as generation `generation`, at the
/// source frame `block * 128`.
fn submit_block_c(
    session: *mut crate::Session,
    source: &[u8],
    generation: u64,
    block: u64,
    salt: u64,
    sample_rate_hz: u32,
) {
    let (left, right) = continuity_block(block, salt);
    submit_c_to(
        session,
        source,
        generation,
        block * 128,
        sample_rate_hz,
        &left,
        &right,
        false,
    );
}

/// Commits `edits` at `base` through the C entry point and returns the path the response reports.
/// Panics unless the transaction commits.
fn transact_c(
    session: *mut crate::Session,
    request_id: u64,
    base: u64,
    edits: &[protocol::SessionEdit],
) -> protocol::EditPath {
    let request = command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(base)),
        protocol::CommandPayload::SessionTransactionApply(edits),
    );
    let (result, response) = command_c(session, &request);
    assert_eq!(result, crate::RESULT_OK, "request {request_id}");
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("transaction response")
    {
        protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
            header,
        } => {
            assert_eq!(header.revision, SessionRevision(base + 1));
            applied.edit_path
        }
        protocol::DecodedTypedResponseFrame::Success { .. } => panic!("not a transaction response"),
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => {
            panic!(
                "request {request_id}: {:?} {:?}",
                header.status, payload.diagnostics
            )
        }
    }
}

/// `UpsertSource` of `source_id` (a copy of the first source), `UpsertTrack` of `track_id` on it
/// and `UpsertRoute` of its route.
fn add_source_and_track(
    model: &session::SessionModel,
    source_id: &str,
    track_id: &str,
) -> [protocol::SessionEdit; 3] {
    let mut source = model.sources[0].clone();
    source.id = session::StableId::parse(source_id).expect("source ID");
    let mut track = model.tracks[0].clone();
    track.id = session::StableId::parse(track_id).expect("track ID");
    track.source_id = source.id.clone();
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse(&format!("{track_id}-main")).expect("route ID");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("track route")
    };
    *track_id = track.id.clone();
    [
        protocol::SessionEdit::UpsertSource { source },
        protocol::SessionEdit::UpsertTrack { track },
        protocol::SessionEdit::UpsertRoute { route },
    ]
}

/// A structural insert edit that changes the audio only on `track`: a compressor at its default
/// parameters, put at the head of the track's inserts.
fn put_compressor(model: &session::SessionModel, track: usize) -> protocol::SessionEdit {
    use session::{EffectIdentity, EffectQuality, LinkMode, SidechainDeclaration, StableId};
    protocol::SessionEdit::PutTrackEffect {
        track_id: model.tracks[track].id.clone(),
        rack_name: session::RackName::Inserts,
        final_position: 0,
        effect: session::Effect {
            id: StableId::parse("comp").expect("insert ID"),
            identity: EffectIdentity::Native {
                effect_id: StableId::parse("miso.compressor").expect("effect ID"),
            },
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            params: Vec::new(),
            sidechain: SidechainDeclaration::None,
        },
    }
}

/// The reference of the held-seek tests: a fresh session of `committed`, fed by `feed(session,
/// block)` one block ahead of each rendered block, rendering `blocks` blocks.
/// Issue #1288: a track a transaction adds fades in from its first played block, so the fresh
/// reference of `committed` holds the added `track` muted and unmutes it live at that block. Returns
/// the muted document, its revision, and the edit whose transaction is that unmute: a live record
/// over the session mute ramp, the ramp the fade runs over.
fn with_added_track_muted(committed: &str, track: &str) -> (String, u64, protocol::SessionEdit) {
    let mut model = session::parse_session_json(committed).expect("committed document");
    let strip = model
        .tracks
        .iter_mut()
        .find(|candidate| candidate.id.as_str() == track)
        .expect("the added track");
    let unmuted = strip.clone();
    strip.fader.left_mute = true;
    strip.fader.right_mute = true;
    (
        session::canonical_session_json(&model).expect("canonical"),
        model.revision,
        protocol::SessionEdit::UpsertTrack { track: unmuted },
    )
}

/// The reference's live unmute of [`with_added_track_muted`].
fn unmute_c(session: *mut crate::Session, base: u64, edit: &protocol::SessionEdit) {
    assert_eq!(
        transact_c(session, 900, base, core::slice::from_ref(edit)),
        protocol::EditPath::Live,
        "the unmute is a live record"
    );
}

fn render_fresh_reference(
    committed: &str,
    blocks: u64,
    feed: &dyn Fn(*mut crate::Session, u64),
) -> Vec<Vec<f32>> {
    let (session, plan) = boxed_c_children(committed);
    feed(session, 0);
    let rendered = (0..blocks)
        .map(|block| {
            feed(session, block + 1);
            render_c(plan, block)
        })
        .collect::<Vec<_>>();
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);
    rendered
}

/// Issue #1319 D1 (the #1275 verifier's probe P5): a seek the predecessor plan already popped, and
/// holds, moves with its source's consumer across a plan swap and applies at its anchor.
///
/// At 48 kHz the fixture source plays generation 1 for blocks 0-7. After block 3 the host anchors
/// generation 2 at render sample 1024 (block 8) and submits it from frame 0. Blocks 4 and 5 render
/// on the predecessor, whose consumer pops the seek and holds it while the queued generation-1
/// blocks play. A structural transaction (a muted track on the same source) commits after block 5,
/// block 6 is the swap block, and block 8 applies the seek on the successor. All 16 blocks equal a
/// fresh session of the committed document fed one stream that switches content at block 8, and
/// the swap carried the consumer.
///
/// Test value: red if the swap resets, rebuilds or drops the carried consumer's held seek (block 8
/// then finds no seek: the generation-2 blocks are discarded as unobserved and block 8 on is
/// silent), or if the successor applies a held seek before its anchor or after it; no other
/// committed test seeks before a swap (`successor_swap.rs` asserts `seek_before >= SWAP_BLOCK`).
#[test]
fn a_held_anchored_seek_crosses_a_plan_swap() {
    const RATE: u32 = 48_000;
    const FIXTURE_ID: &[u8] = b"fixture-source";
    const ANCHOR_BLOCK: u64 = 8;
    let model = stateless_session(RATE);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (session, plan) = boxed_c_children(&document);
    let mut swapped = Vec::new();
    submit_block_c(session, FIXTURE_ID, 1, 0, 0, RATE);
    swapped.extend(feed_and_render_c(session, plan, RATE, 0, 4));
    // Generation 1 is queued through block 7 (block 4 is queued by the feed above).
    for block in 5..8 {
        submit_block_c(session, FIXTURE_ID, 1, block, 0, RATE);
    }
    assert_eq!(
        crate::ffi::test_source_seek_at(session, FIXTURE_ID, 2, 0, ANCHOR_BLOCK * 128),
        crate::RESULT_OK
    );
    for block in 0..4 {
        submit_block_c(session, FIXTURE_ID, 2, block, 1, RATE);
    }
    swapped.push(render_c(plan, 4));
    swapped.push(render_c(plan, 5));
    let edits = add_muted_track(&model, "a-muted");
    assert_eq!(
        transact_c(session, 1, 42, &edits),
        protocol::EditPath::Rebuild,
        "a structural transaction rebuilds"
    );
    for block in 6..16 {
        // The ring holds eight blocks: generation 2 block `m` fits once `m - 3` blocks played.
        match block {
            6 => {
                submit_block_c(session, FIXTURE_ID, 2, 4, 1, RATE);
                submit_block_c(session, FIXTURE_ID, 2, 5, 1, RATE);
            }
            7 => submit_block_c(session, FIXTURE_ID, 2, 6, 1, RATE),
            8 => submit_block_c(session, FIXTURE_ID, 2, 7, 1, RATE),
            _ => {}
        }
        swapped.push(render_c(plan, block));
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(plan),
        (1, 0),
        "one swap, and it carried the source"
    );
    let committed = snapshot_c(session);
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);

    let reference = render_fresh_reference(&committed, 16, &|session, block| {
        let (salt, content_block) = if block < ANCHOR_BLOCK {
            (0, block)
        } else {
            (1, block - ANCHOR_BLOCK)
        };
        let (left, right) = continuity_block(content_block, salt);
        submit_c(session, 1, block * 128, RATE, &left, &right, false);
    });
    assert_bit_identical("held seek across a swap", 0, &swapped, &reference);
}

/// Issue #1319 D2: an anchored seek queued on a source that persists from the running plan through
/// a withdrawn candidate into its replacement survives the supersession.
///
/// P0 plays the fixture source for blocks 0-3 and has generation 1 queued through block 7. With no
/// render between them, T1 (a compressor on the second track) commits and candidate A is
/// published; the seek to generation 2 at block 8 goes to A's producer set, so it is queued in the
/// command slot of the ring P0 owns; T2 (a compressor on the third track) withdraws A and commits
/// B against P0. Generation 2 is then submitted through the producer that is now B's, and blocks
/// 4-11 render. The source switches at the anchor, and every block equals a twin that submits
/// T1 and T2 as one transaction at the same point and seeks the same way. The supersession is one
/// withdrawal (and one drop of A), and each transaction took the rebuild path.
///
/// Test value: red if the supersession rebuilds a persisting source's ring for B (its queued
/// blocks are lost) or loses the seek in the ring's command slot while A is withdrawn (block 8 on
/// then finds generation 2 unobserved and is silent); the #1310 tests seek only on a source the
/// withdrawn candidate added, whose ring is donated.
#[test]
fn a_queued_seek_on_a_persisting_source_survives_supersession() {
    const RATE: u32 = 48_000;
    const FIXTURE_ID: &[u8] = b"fixture-source";
    const ANCHOR_BLOCK: u64 = 8;
    let model = stateless_session(RATE);
    let document = session::canonical_session_json(&model).expect("canonical");
    let first = [put_compressor(&model, 1)];
    let second = [put_compressor(&model, 2)];
    let combined = [put_compressor(&model, 1), put_compressor(&model, 2)];

    let run = |superseding: bool| {
        let (session, plan) = boxed_c_children(&document);
        let before = crate::ffi::test_owner_counters(session);
        submit_block_c(session, FIXTURE_ID, 1, 0, 0, RATE);
        let mut blocks = feed_and_render_c(session, plan, RATE, 0, 4);
        for block in 5..8 {
            submit_block_c(session, FIXTURE_ID, 1, block, 0, RATE);
        }
        // The seek goes to the pending candidate's producer set, whichever transaction left it.
        let seek = || {
            assert_eq!(
                crate::ffi::test_source_seek_at(session, FIXTURE_ID, 2, 0, ANCHOR_BLOCK * 128),
                crate::RESULT_OK
            );
        };
        if superseding {
            assert_eq!(
                transact_c(session, 1, 42, &first),
                protocol::EditPath::Rebuild
            );
            seek();
            assert_eq!(
                transact_c(session, 2, 43, &second),
                protocol::EditPath::Rebuild
            );
        } else {
            assert_eq!(
                transact_c(session, 1, 42, &combined),
                protocol::EditPath::Rebuild
            );
            seek();
        }
        for block in 0..4 {
            submit_block_c(session, FIXTURE_ID, 2, block, 1, RATE);
        }
        for block in 4..12 {
            blocks.push(render_c(plan, block));
        }
        let counters = crate::ffi::test_owner_counters(session);
        let delta = (
            counters.candidate_withdrawn - before.candidate_withdrawn,
            counters.candidate_superseded - before.candidate_superseded,
            counters.candidate_taken - before.candidate_taken,
        );
        let carried = crate::ffi::test_plan_carry_counts(plan);
        crate::ffi::test_plan_destroy(plan);
        crate::ffi::test_session_destroy(session);
        (blocks, delta, carried)
    };
    let (superseded, delta, carried) = run(true);
    assert_eq!(
        delta,
        (1, 1, 0),
        "one withdrawal, whose candidate A was dropped"
    );
    assert_eq!(carried, (1, 0), "one swap, B alone, and it carried");
    let (twin, twin_delta, twin_carried) = run(false);
    assert_eq!(twin_delta, (0, 0, 0));
    assert_eq!(twin_carried, (1, 0));
    assert_bit_identical("supersession across a queued seek", 0, &superseded, &twin);
}

/// Issue #1319 D3: a held seek on a source a candidate added carries on when a later structural
/// transaction finds that candidate already taken by render.
///
/// T1 adds source `s2` with its track. The host anchors `s2`'s generation 2 at block 8 (frame 0)
/// and submits it, and block 2 renders: render claims T1's candidate A, and A's consumer for `s2`
/// pops the seek and holds it. T2 (a muted track on the fixture source, keeping `s2`) commits: its
/// service step promotes A, so B is prepared against A as the running plan and carries A's `s2`
/// consumer with its held seek, rather than preparing across a withdrawal. Blocks 3-11 render. `s2`
/// is silent until block 8 and plays from there, and every block equals a fresh session of T2's
/// document fed `s2` zeros below the anchor and the same PCM from it. Neither withdrawal outcome
/// is counted: A was neither withdrawn nor found taken by the withdrawal.
///
/// Test value: red if the control plane treats a candidate render has taken as withdrawn and
/// prepares B across it: `s2`, which A added, is then fresh in B, and A's held, acked seek and its
/// queued generation 2 are lost (block 8 on lacks `s2`).
#[test]
fn a_held_seek_carries_from_a_taken_candidate() {
    const RATE: u32 = 48_000;
    const FIXTURE_ID: &[u8] = b"fixture-source";
    const ADDED: &[u8] = b"s2-source";
    const ANCHOR_BLOCK: u64 = 8;
    let mut model = stateless_session(RATE);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let document = session::canonical_session_json(&model).expect("canonical");
    let (session, plan) = boxed_c_children(&document);
    let before = crate::ffi::test_owner_counters(session);
    submit_block_c(session, FIXTURE_ID, 1, 0, 0, RATE);
    let mut swapped = feed_and_render_c(session, plan, RATE, 0, 2);

    let first = add_source_and_track(&model, "s2-source", "t2");
    assert_eq!(
        transact_c(session, 1, 42, &first),
        protocol::EditPath::Rebuild
    );
    assert_eq!(
        crate::ffi::test_source_seek_at(session, ADDED, 2, 0, ANCHOR_BLOCK * 128),
        crate::RESULT_OK
    );
    for block in 0..4 {
        submit_block_c(session, ADDED, 2, block, 1, RATE);
    }
    submit_block_c(session, FIXTURE_ID, 1, 3, 0, RATE);
    swapped.push(render_c(plan, 2));
    assert_eq!(
        crate::ffi::test_plan_carry_counts(plan),
        (1, 0),
        "render took A"
    );

    let second = add_muted_track(&model, "a-muted");
    assert_eq!(
        transact_c(session, 2, 43, &second),
        protocol::EditPath::Rebuild
    );
    let counters = crate::ffi::test_owner_counters(session);
    assert_eq!(
        (
            counters.candidate_withdrawn - before.candidate_withdrawn,
            counters.candidate_taken - before.candidate_taken,
        ),
        (0, 0),
        "the service step promoted A before the withdrawal ran"
    );
    for block in 3..12 {
        if block > 3 {
            submit_block_c(session, FIXTURE_ID, 1, block, 0, RATE);
        }
        swapped.push(render_c(plan, block));
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(plan),
        (2, 0),
        "A, then B, carried"
    );
    let committed = snapshot_c(session);
    crate::ffi::test_plan_destroy(plan);
    crate::ffi::test_session_destroy(session);

    // `t2`, which A added, fades in at its first played block, the anchor (#1288), and B carries
    // its waiting fade (#1392).
    let (committed, revision, unmute) = with_added_track_muted(&committed, "t2");
    let reference = render_fresh_reference(&committed, 12, &|session, block| {
        // `feed` runs for block `n + 1` just before block `n` renders.
        if block == ANCHOR_BLOCK + 1 {
            unmute_c(session, revision, &unmute);
        }
        submit_block_c(session, FIXTURE_ID, 1, block, 0, RATE);
        let (left, right) = if block < ANCHOR_BLOCK {
            (vec![0.0; 128], vec![0.0; 128])
        } else {
            continuity_block(block - ANCHOR_BLOCK, 1)
        };
        submit_c_to(session, ADDED, 1, block * 128, RATE, &left, &right, false);
    });
    assert_bit_identical("held seek from a taken candidate", 0, &swapped, &reference);
}

/// Issue #1273 gate 2: a transaction that removes a track and its source leaves the remaining
/// source playing from its carried ring, bit-identical to the fresh post-edit session, and a
/// submit for the removed source is refused as unknown. Since #1325 D4 the removed source keeps
/// feeding the running plan until the successor is adopted at `S`, and is refused from then on.
///
/// Test value: red if the carry joins sources by graph index instead of ID (the remaining
/// source's index moves when the first is removed), if a removed source's producer stays
/// reachable after the adoption, or if it is retired at the commit (a phase-1 submit refused).
#[test]
fn removing_a_track_and_its_source_keeps_the_other_source_playing() {
    const AUX: &[u8] = b"aux-source";
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    // The removed source sorts first, so the remaining one changes graph index.
    let mut aux = model.sources[0].clone();
    aux.id = session::StableId::parse("aux-source").expect("source ID");
    model.sources.insert(0, aux);
    model.tracks[0].source_id = session::StableId::parse("aux-source").expect("source ID");
    let both: [&[u8]; 2] = [b"fixture-source", AUX];
    let document = session::canonical_session_json(&model).expect("canonical");
    let mut children = compile_children(&document, limits()).expect("children");
    submit_direct(&mut children, &both, 0);
    feed_and_render_direct(&mut children, &both, 0, 6);
    let edits = [
        protocol::SessionEdit::RemoveRoute {
            route_id: model.routes[0].id.clone(),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: model.tracks[0].id.clone(),
        },
        protocol::SessionEdit::RemoveSource {
            source_id: session::StableId::parse("aux-source").expect("source ID"),
        },
    ];
    children
        .session
        .command(
            &command_bytes_at_revision(
                1,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::SessionTransactionApply(&edits),
            ),
            4_096,
        )
        .expect("removal transaction");
    // #1325 D4: until the adoption both sources are fed, and neither submit is refused.
    let (_, next) = feed_and_render_until_direct(&mut children, &both, 6, 43);
    let (left, right) = continuity_block(next + 1, 1);
    let refused = children.session.submit(
        AUX,
        SourceSubmission {
            generation: 1,
            start_frame: (next + 1) * 128,
            sample_rate_hz: 48_000,
            planes: &[&left, &right],
            frames: 128,
            end_of_region: false,
        },
    );
    let Err(refusal) = refused else {
        panic!("a submit for the removed source is refused after the adoption")
    };
    assert_eq!(
        refusal.report(),
        (crate::RESULT_INVALID_ARGUMENT, &b"source.id.unknown"[..])
    );
    // #1273 D3: a seek, like a submit, addresses the newest committed session from the adoption
    // on (#1325 D4).
    let Err(refusal) = children.session.seek(AUX, 2, 0) else {
        panic!("a seek for the removed source is refused")
    };
    assert_eq!(
        refusal.report(),
        (crate::RESULT_INVALID_ARGUMENT, &b"source.id.unknown"[..])
    );
    let swapped = feed_and_render_direct(&mut children, &both[..1], next, 6);
    assert_eq!(children.plan.owner().carried_count(), 1);

    let committed = session::canonical_session_json(
        children
            .session
            .test_controller()
            .session()
            .compiled()
            .normalized_model(),
    )
    .expect("committed");
    let mut reference = compile_children(&committed, limits()).expect("reference");
    submit_direct(&mut reference, &both[..1], 0);
    let expected = feed_and_render_direct(&mut reference, &both[..1], 0, next + 6);
    assert_bit_identical("removal", next, &swapped, &expected[next as usize..]);
}

// Issue #1323: a host-declared discontinuity resets the latency floors of #1285.

/// The #1285 gate 3 session: three tracks on the fixture source, the first muted behind a
/// true-peak limiter insert, and the `RemoveRoute`/`RemoveTrack` transaction that removes it.
fn limited_session() -> (session::SessionModel, [protocol::SessionEdit; 2]) {
    let mut model = stateless_session(48_000);
    let limited = &mut model.tracks[0];
    limited.fader.left_mute = true;
    limited.fader.right_mute = true;
    limited.inserts.effects.push(session::Effect {
        id: session::StableId::parse("lim").expect("insert ID"),
        identity: session::EffectIdentity::Native {
            effect_id: session::StableId::parse("miso.true-peak-limiter").expect("limiter id"),
        },
        quality: session::EffectQuality::Normal,
        bypass: false,
        link_mode: session::LinkMode::Maximum,
        params: Vec::new(),
        sidechain: session::SidechainDeclaration::None,
    });
    let edits = [
        protocol::SessionEdit::RemoveRoute {
            route_id: model.routes[0].id.clone(),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: model.tracks[0].id.clone(),
        },
    ];
    (model, edits)
}

/// `edits` as one transaction expected at `revision`.
fn transaction_at(request_id: u64, revision: u64, edits: &[protocol::SessionEdit]) -> Vec<u8> {
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(revision)),
        protocol::CommandPayload::SessionTransactionApply(edits),
    )
}

/// Runs `request` on the session state, which must accept it, and drains the reliable events.
fn command_direct(children: &mut CompiledChildren, request: &[u8]) {
    children
        .session
        .command(request, 4_096)
        .unwrap_or_else(|error| panic!("{error:?}"));
    while children
        .session
        .dequeue_event(EventLane::Reliable, 4_096)
        .expect("a reliable event")
        .is_some()
    {}
}

/// The committed model's canonical JSON.
fn committed_document(children: &CompiledChildren) -> String {
    session::canonical_session_json(
        children
            .session
            .test_controller()
            .session()
            .compiled()
            .normalized_model(),
    )
    .expect("canonical")
}

/// `document` compiled fresh, so with no floors, fed the fixture source's signal from source
/// block `from` on, rendered for `blocks` blocks: the reference of a plan that adopts at block
/// `from` with its carried source consumer standing at frame `from * 128`. Returns its latency
/// and its blocks.
fn natural_reference(document: &str, from: u64, blocks: u64) -> (u64, Vec<Vec<f32>>) {
    let mut fresh = compile_children(document, limits()).expect("fresh");
    let latency = fresh.plan.resources().latency_samples;
    let submit = |fresh: &mut CompiledChildren, block: u64| {
        let (left, right) = continuity_block(from + block, 0);
        fresh
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: block * 128,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("block {block}: {:?}", error.report()));
    };
    submit(&mut fresh, 0);
    let rendered = (0..blocks)
        .map(|block| {
            submit(&mut fresh, block + 1);
            render_direct(&mut fresh, block)
        })
        .collect();
    (latency, rendered)
}

fn sample_bits(blocks: &[Vec<f32>]) -> Vec<Vec<u32>> {
    blocks
        .iter()
        .map(|block| block.iter().map(|sample| sample.to_bits()).collect())
        .collect()
}

/// Issue #1323 gate 2: the #1285 removal through the C ABI, then
/// `miso_engine_v1_declare_discontinuity`. Before the declaration
/// `miso_engine_v1_plan_resources` reports A's latency (the floors), and after one render it
/// reports the committed session's natural latency; from that block the output is the committed
/// session compiled fresh, fed from the frame the carried source stood at. The revision and the
/// watermark do not move.
///
/// Test value: red if the declaration clears the floors in an inventory but publishes no plan (the
/// report keeps A's latency), if the C entry does not reach the control plane, or if the
/// successor carries any state but the source (the compensation lines differ from a fresh plan's).
#[test]
fn a_declared_discontinuity_reports_the_natural_latency_on_the_c_abi() {
    let (model, edits) = limited_session();
    let document = session::canonical_session_json(&model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let latency = crate::ffi::test_plan_latency(c_plan);
    assert!(latency > 0, "the limiter delays A");
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, 48_000, &left, &right, false);
    feed_and_render_c(c_session, c_plan, 48_000, 0, 6);
    assert_eq!(
        command_c(c_session, &transaction_at(1, 42, &edits)).0,
        crate::RESULT_OK
    );
    let next = feed_and_render_until_c(c_session, c_plan, 48_000, 6, 43);
    assert_eq!(
        crate::ffi::test_plan_latency(c_plan),
        latency,
        "the removal keeps A's latency (#1285)"
    );
    let summary = crate::ffi::test_session_state_summary(c_session);
    let (_, watermark) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(watermark.revision, 43);

    assert_eq!(
        crate::ffi::test_declare_discontinuity(c_session),
        crate::RESULT_OK
    );
    assert_eq!(
        crate::ffi::test_plan_latency(c_plan),
        latency,
        "the running plan reports until render adopts the successor"
    );
    let reset = feed_and_render_c(c_session, c_plan, 48_000, next, 6);
    let (natural, expected) = natural_reference(&snapshot_c(c_session), next, 6);
    assert!(natural < latency, "B's natural latency is lower");
    assert_eq!(crate::ffi::test_plan_latency(c_plan), natural);
    assert!(
        expected.iter().flatten().any(|sample| *sample != 0.0),
        "the reference plays"
    );
    assert_eq!(sample_bits(&reset), sample_bits(&expected));
    let after = crate::ffi::test_session_state_summary(c_session);
    assert_eq!(after.0, summary.0, "the revision does not move");
    let (_, adopted) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(adopted, watermark, "the watermark does not move");
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1323 gate 3: a declaration with nothing to reset returns `OK`, publishes no plan and
/// leaves the output bit-identical to a run without it: on a session at revision 0 (B15-R6: no
/// successor, so no floor), and after a structural transaction that adds a muted track, whose
/// successor's floors raise no node.
///
/// Test value: red if the declaration always rebuilds (every effect tail and line would restart
/// at each stop: here the compensation lines of the added track's path and the limiter restart,
/// and the output differs from the twin), or if it refuses or moves the revision at revision 0.
#[test]
fn a_declaration_with_nothing_to_reset_publishes_nothing() {
    for (label, revision, structural) in [("revision 0", 0, false), ("added track", 42, true)] {
        let (mut model, _) = limited_session();
        model.revision = revision;
        let document = session::canonical_session_json(&model).expect("canonical");
        let run = |declare: bool| {
            let mut children = compile_children(&document, limits()).expect("children");
            let latency = children.plan.resources().latency_samples;
            submit_direct(&mut children, &[b"fixture-source"], 0);
            if declare {
                let epoch = children.session.newest_plan_epoch();
                children
                    .session
                    .declare_discontinuity()
                    .expect("nothing to reset");
                assert_eq!(children.session.newest_plan_epoch(), epoch, "{label}");
            }
            let mut blocks = feed_and_render_direct(&mut children, &[b"fixture-source"], 0, 4);
            if structural {
                command_direct(
                    &mut children,
                    &transaction_at(
                        1,
                        revision,
                        &add_muted_track(&stateless_session(48_000), "a-muted"),
                    ),
                );
            }
            blocks.extend(feed_and_render_direct(
                &mut children,
                &[b"fixture-source"],
                4,
                2,
            ));
            assert!(
                !children
                    .session
                    .test_providers()
                    .test_inventory()
                    .runs_behind_natural_timing(),
                "{label}: no floor is raised"
            );
            if declare {
                let epoch = children.session.newest_plan_epoch();
                let summary = children.session.test_state_summary();
                children
                    .session
                    .declare_discontinuity()
                    .expect("nothing to reset");
                assert_eq!(children.session.newest_plan_epoch(), epoch, "{label}");
                assert_eq!(children.session.test_state_summary(), summary, "{label}");
            }
            blocks.extend(feed_and_render_direct(
                &mut children,
                &[b"fixture-source"],
                6,
                6,
            ));
            assert_eq!(
                children.plan.resources().latency_samples,
                latency,
                "{label}"
            );
            blocks
        };
        let declared = run(true);
        let twin = run(false);
        assert!(
            twin[8..].iter().flatten().any(|sample| *sample != 0.0),
            "{label}: the twin plays"
        );
        assert_eq!(sample_bits(&declared), sample_bits(&twin), "{label}");
    }
}

/// Issue #1323 gate 4: with the replacement peak cap below any successor, the declaration after
/// the #1285 removal returns the cap's compile rejection, and the plan render runs, its floors,
/// the committed model and the revision are unchanged: the next blocks equal a run without the
/// call. With the cap lifted, the same declaration then resets the floors.
///
/// Test value: red if the declaration drops the floors before its preparation succeeds (the
/// running epoch's inventory loses them and the retry finds nothing to reset, so the latency
/// never comes down), or if a refusal publishes or moves anything.
#[test]
fn a_refused_declaration_changes_nothing() {
    let (model, edits) = limited_session();
    let document = session::canonical_session_json(&model).expect("canonical");
    let run = |declare: bool| {
        let mut children = compile_children(&document, limits()).expect("children");
        let latency = children.plan.resources().latency_samples;
        submit_direct(&mut children, &[b"fixture-source"], 0);
        let mut blocks = feed_and_render_direct(&mut children, &[b"fixture-source"], 0, 6);
        command_direct(&mut children, &transaction_at(1, 42, &edits));
        let (adopted, next) =
            feed_and_render_until_direct(&mut children, &[b"fixture-source"], 6, 43);
        blocks.extend(adopted);
        if declare {
            let floors = children.session.test_providers().test_inventory().clone();
            assert!(floors.runs_behind_natural_timing());
            let epoch = children.session.newest_plan_epoch();
            let summary = children.session.test_state_summary();
            let committed = committed_document(&children);
            children.session.set_replacement_peak_limit(1);
            let refused = children.session.declare_discontinuity();
            assert!(
                matches!(refused, Err(CommandError::CompileRejected(_))),
                "{refused:?}"
            );
            assert_eq!(children.session.newest_plan_epoch(), epoch);
            assert_eq!(children.session.test_state_summary(), summary);
            assert_eq!(committed_document(&children), committed);
            assert_eq!(
                children
                    .session
                    .test_providers()
                    .test_inventory()
                    .input_floors(),
                floors.input_floors(),
                "the running plan keeps its floors"
            );
            children.session.set_replacement_peak_limit(u64::MAX);
        }
        blocks.extend(feed_and_render_direct(
            &mut children,
            &[b"fixture-source"],
            next,
            2,
        ));
        assert_eq!(children.plan.resources().latency_samples, latency);
        (children, blocks, latency, next + 2)
    };
    let (mut refused, blocks, latency, next) = run(true);
    let (_, twin, _, _) = run(false);
    assert_eq!(sample_bits(&blocks), sample_bits(&twin));

    refused
        .session
        .declare_discontinuity()
        .expect("the retry under the lifted cap");
    feed_and_render_direct(&mut refused, &[b"fixture-source"], next, 1);
    let (natural, _) = natural_reference(&committed_document(&refused), next, 1);
    assert!(natural < latency);
    assert_eq!(refused.plan.resources().latency_samples, natural);
}

/// The lifecycle counters of every owner and reservation, balanced once both handles are gone.
fn assert_owners_balanced(counters: TestOwnerCounters) {
    assert_eq!(
        counters.current_provider_constructed + counters.candidate_provider_constructed,
        counters.current_provider_disposed + counters.candidate_provider_disposed,
        "providers"
    );
    assert_eq!(
        counters.current_plan_constructed + counters.candidate_plan_constructed,
        counters.current_plan_disposed + counters.candidate_plan_disposed,
        "plans"
    );
    assert_eq!(
        counters.token_constructed, counters.token_disposed,
        "tokens"
    );
    assert_eq!(
        counters.reservation_constructed,
        counters.reservation_canceled + counters.reservation_committed,
        "reservations"
    );
}

/// Issue #1323 gate 5: a declared stop supersedes a pending warm successor. The #1285 removal B
/// is committed and published `Primed` 64 blocks ahead (floored at A's arrivals, so above its
/// natural ones). After 4 blocks B is still pending; the declaration withdraws it, and the block
/// after the declaration is the discontinuity successor's: from it the output is the committed
/// session compiled fresh, fed from the frame the carried source stood at, at the natural
/// latency. B is never adopted. The watermark advances to the committed revision at that block
/// with `EXACT` and never `TRANSITION_FALLBACK`; with an earlier structural edit B folded in, with
/// `EXACT | SUPERSEDED`, and `superseded` grows by 1. The source's producer moves into the
/// successor, and every owner and reservation balances.
///
/// Test value: red if the successor is published with B's `Primed` kind (it waits for
/// `not_before` and readiness, and the floored B plays past the stop), if B is left to its own
/// adoption, or if the revision is reported as a fallback or loses B's folded revision.
fn a_declared_stop_supersedes_a_pending_warm_successor(fold: bool) {
    const DECLARED_AT: u64 = 8;
    test_reset_lifecycle_observer();
    {
        let (model, edits) = limited_session();
        let document = session::canonical_session_json(&model).expect("canonical");
        let mut children = compile_children(&document, limits()).expect("children");
        let latency = children.plan.resources().latency_samples;
        submit_direct(&mut children, &[b"fixture-source"], 0);
        feed_and_render_direct(&mut children, &[b"fixture-source"], 0, 4);
        let mut revision = 42;
        if fold {
            // An edit committed and left pending: B supersedes it, so B folds in one revision.
            command_direct(
                &mut children,
                &transaction_at(
                    1,
                    revision,
                    &add_muted_track(&stateless_session(48_000), "a-muted"),
                ),
            );
            revision += 1;
        }
        children
            .session
            .test_set_next_adoption(engine::realtime::PlanAdoption::Primed {
                not_before: (4 + 64) * 128,
                lead_blocks: 1,
            });
        command_direct(&mut children, &transaction_at(2, revision, &edits));
        revision += 1;
        assert!(
            children.session.test_pending_providers()[0]
                .test_inventory()
                .runs_behind_natural_timing(),
            "B is floored above its natural arrivals"
        );
        feed_and_render_direct(&mut children, &[b"fixture-source"], 4, 4);
        let before = children.plan.queries().watermark().expect("watermark");
        assert_eq!(before.revision, 42, "B waits, unadopted");
        assert_eq!(children.session.test_pending_providers().len(), 1);
        let counters = children.session.test_owner_counters();

        children
            .session
            .declare_discontinuity()
            .expect("the declaration");
        let after = children.session.test_owner_counters();
        assert_eq!(after.candidate_withdrawn, counters.candidate_withdrawn + 1);
        assert_eq!(
            after.candidate_superseded,
            counters.candidate_superseded + 1
        );
        assert_eq!(after.candidate_republished, counters.candidate_republished);
        assert_eq!(
            after.candidate_provider_published,
            counters.candidate_provider_published + 1
        );
        let newest = children.session.newest_plan_epoch();
        let successor = children
            .session
            .test_epoch_mut(newest)
            .expect("the successor's epoch");
        assert_eq!(
            successor.test_source_vacant(b"fixture-source"),
            Some(false),
            "the source's producer moved into the successor"
        );

        let reset = feed_and_render_direct(&mut children, &[b"fixture-source"], DECLARED_AT, 8);
        let watermark = children.plan.queries().watermark().expect("watermark");
        assert_eq!(
            (watermark.revision, watermark.first_sample),
            (revision, DECLARED_AT * 128),
            "the successor is adopted at the block after the declaration"
        );
        let superseded = u64::from(fold);
        assert_eq!(
            watermark.outcome_flags,
            engine::realtime::OUTCOME_EXACT
                | if fold {
                    engine::realtime::OUTCOME_SUPERSEDED
                } else {
                    0
                }
        );
        assert_eq!(watermark.superseded, before.superseded + superseded);
        assert_eq!(watermark.exact, before.exact + 1);
        assert_eq!(watermark.transition_fallback, before.transition_fallback);
        let (natural, expected) = natural_reference(&committed_document(&children), DECLARED_AT, 8);
        assert!(natural < latency);
        assert_eq!(children.plan.resources().latency_samples, natural);
        assert!(expected.iter().flatten().any(|sample| *sample != 0.0));
        assert_eq!(sample_bits(&reset), sample_bits(&expected));
    }
    assert_owners_balanced(test_lifecycle_counters());
}

#[test]
fn a_declared_stop_supersedes_a_pending_warm_successor_alone_and_folded() {
    a_declared_stop_supersedes_a_pending_warm_successor(false);
    a_declared_stop_supersedes_a_pending_warm_successor(true);
}

/// Issue #1323, the supersession the spec leaves open: a structural edit E committed while the
/// discontinuity successor D waits unadopted supersedes it, and E is prepared at the same
/// discontinuity. Render never played D, so the pair is the one transaction "declare, then edit"
/// against the running plan, whose combined twin (P0 -> E) has no floors: E runs at its natural
/// latency, carries only its source, and renders as its model compiled fresh from the block that
/// adopts it. A second declaration while D waits finds nothing to reset (D has no raised floor)
/// and publishes nothing.
///
/// Test value: red if E is prepared as an ordinary successor across the withdrawn D: its base is
/// the running plan's inventory, so E takes the running plan's floors (A's latency) and the
/// declared stop is lost.
#[test]
fn an_edit_that_supersedes_a_discontinuity_successor_is_prepared_at_it() {
    let (model, edits) = limited_session();
    let document = session::canonical_session_json(&model).expect("canonical");
    let mut children = compile_children(&document, limits()).expect("children");
    let latency = children.plan.resources().latency_samples;
    submit_direct(&mut children, &[b"fixture-source"], 0);
    feed_and_render_direct(&mut children, &[b"fixture-source"], 0, 6);
    command_direct(&mut children, &transaction_at(1, 42, &edits));
    let (_, adopted_at) = feed_and_render_until_direct(&mut children, &[b"fixture-source"], 6, 43);
    assert_eq!(children.plan.resources().latency_samples, latency);
    let before = children.plan.queries().watermark().expect("watermark");

    children
        .session
        .declare_discontinuity()
        .expect("the declaration");
    let declared = children.session.newest_plan_epoch();
    children
        .session
        .declare_discontinuity()
        .expect("a second declaration");
    assert_eq!(
        children.session.newest_plan_epoch(),
        declared,
        "the waiting discontinuity successor has nothing to reset"
    );
    let counters = children.session.test_owner_counters();
    let edit = add_muted_track(
        children
            .session
            .test_controller()
            .session()
            .compiled()
            .normalized_model(),
        "a-muted",
    );
    command_direct(&mut children, &transaction_at(2, 43, &edit));
    let after = children.session.test_owner_counters();
    assert_eq!(after.candidate_withdrawn, counters.candidate_withdrawn + 1);
    assert_eq!(
        after.candidate_superseded,
        counters.candidate_superseded + 1
    );
    assert!(
        !children.session.test_pending_providers()[0]
            .test_inventory()
            .runs_behind_natural_timing(),
        "E has no raised floor"
    );

    let rendered = feed_and_render_direct(&mut children, &[b"fixture-source"], adopted_at, 6);
    let watermark = children.plan.queries().watermark().expect("watermark");
    assert_eq!(
        (watermark.revision, watermark.first_sample),
        (44, adopted_at * 128)
    );
    assert_eq!(watermark.outcome_flags, engine::realtime::OUTCOME_EXACT);
    assert_eq!(watermark.superseded, before.superseded);
    let (natural, expected) = natural_reference(&committed_document(&children), adopted_at, 6);
    assert!(natural < latency);
    assert_eq!(children.plan.resources().latency_samples, natural);
    assert!(expected.iter().flatten().any(|sample| *sample != 0.0));
    assert_eq!(sample_bits(&rendered), sample_bits(&expected));
}

/// Issue #1273 gate 3: a structural transaction refused at any phase, including a protocol
/// commit that returns `Err`, moves no producer. The source still accepts PCM and the running
/// plan renders it, bit-identical to a session that never saw the transaction.
///
/// Test value: red if the producer hand-over runs before any fallible step: the refused
/// candidate takes the producer with it, and the next submit returns `source.ring.vacated`.
#[test]
fn a_refused_structural_transaction_moves_no_producer() {
    use TestStructuralFaultPhase::{
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterRuntimePrepare,
        BeforeProtocolCommit, BeforeRuntimePrepare, ProtocolCommit,
    };
    let source: [&[u8]; 1] = [b"fixture-source"];
    let model = stateless_session(48_000);
    let document = session::canonical_session_json(&model).expect("canonical");
    let mut reference = compile_children(&document, limits()).expect("reference");
    submit_direct(&mut reference, &source, 0);
    let expected = feed_and_render_direct(&mut reference, &source, 0, 4);
    let edits = add_muted_track(&model, "a-muted");
    let transaction = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    for phase in [
        AfterProtocolPrepare,
        BeforeRuntimePrepare,
        AfterRuntimePrepare,
        AfterAdmission,
        AfterPlanReservation,
        BeforeProtocolCommit,
        ProtocolCommit,
    ] {
        let mut children = compile_children(&document, limits()).expect("children");
        submit_direct(&mut children, &source, 0);
        let mut rendered = feed_and_render_direct(&mut children, &source, 0, 2);
        children
            .session
            .test_set_structural_faults([Some(phase), None]);
        let refused = children.session.command(&transaction, 4_096);
        assert!(
            matches!(
                refused,
                Err(CommandError::Backpressure | CommandError::Internal)
            ),
            "{phase:?}: {refused:?}"
        );
        assert_eq!(
            children.session.test_controller().session().revision(),
            SessionRevision(42),
            "{phase:?}"
        );
        assert!(
            children.session.test_pending_providers().is_empty(),
            "{phase:?}"
        );
        rendered.extend(feed_and_render_direct(&mut children, &source, 2, 2));
        assert_eq!(children.plan.owner().carried_count(), 0, "{phase:?}");
        assert_bit_identical(&format!("{phase:?}"), 0, &rendered, &expected);
    }
}

/// The outcome of one control call, reduced to what issue #1042 distinguishes.
fn control_outcome<T, E: core::fmt::Debug>(result: &Result<T, E>) -> String {
    match result {
        Ok(_) => "ok".to_owned(),
        Err(error) => format!("{error:?}"),
    }
}

/// Issue #1042: a control call that lands inside a plan-swapping render call.
///
/// `RealtimePlanOwner::render_contiguous` swaps plans and commits the retired plan to the
/// retirement queue at the *start* of a render call, while `PlanState::render` publishes
/// `active_epoch` only after that call returns. This test runs the two halves of
/// `PlanState::render` separately and makes control calls between them, so the interleaving a
/// second thread reaches by chance is reached here on every round.
///
/// On the unmodified base the first control call in that window returned `Internal`; the next
/// synchronization parked the old provider in `retired_providers` for good; and the old epoch's
/// report row, skipped because it matched the lagging atomic, kept the table full, so every later
/// structural command returned `Backpressure`.
///
/// #1398 gate 1: the structural command inside the window is admitted against the running plan
/// and commits there, while the report table holds three rows (the lagging atomic's, the running
/// plan's and the new candidate's); the next round's swapping block adopts its plan. Red if the
/// lag refusal is kept or the report table is a row short (`Backpressure` in the window).
///
/// Issue #1273 gate 4: the trigger adds and removes a muted track (`UpsertTrack`), so every swap
/// carries the source's ring, and the source is fed twice a round: after the commit, while the
/// candidate is pending, and inside the window. Both submits must land in the carried ring. Test
/// value: red if a submit in either place is routed to the current epoch, whose producer has
/// moved (`source.ring.vacated`), or if the carried ring loses its write position
/// (`source.frame.noncontiguous`).
#[test]
fn control_calls_inside_a_plan_swapping_render_call_keep_replacement_live() {
    let model = parse_session_json(SESSION).expect("fixture");
    let mut children = compile_children(SESSION, limits()).expect("children");
    let left = [0.25_f32; 128];
    let right = [-0.5_f32; 128];
    let mut next_frame = 0_u64;
    let mut feed = |children: &mut CompiledChildren, label: &str| {
        let report = children
            .session
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: next_frame,
                    sample_rate_hz: 48_000,
                    planes: &[&left, &right],
                    frames: 128,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("{label}: {:?}", error.report()));
        assert_eq!(report.accepted_frames, 128, "{label}");
        next_frame += 128;
    };
    let mut pcm = [0.0_f32; 256];
    feed(&mut children, "first block");
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("first block");
    // Even rounds add the muted track, odd rounds remove it.
    let structural = |request_id: u64, revision: u64, round: u64| {
        let edits = if round.is_multiple_of(2) {
            add_muted_track(&model, "a-race").to_vec()
        } else {
            remove_track("a-race").to_vec()
        };
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(&edits),
        )
    };
    let mut request_id = 1;
    let mut revision = 42;
    let mut pending = structural(request_id, revision, 0);
    children
        .session
        .command(&pending, 4_096)
        .expect("first structural command");
    revision += 1;

    let mut sample = 0;
    for round in 1..=4_u64 {
        sample += 128;
        // The candidate is pending: the source's producer has moved to it.
        feed(&mut children, "pending candidate");
        // Drain the round's `SESSION_COMMITTED`, so the reliable lane never backpressures.
        assert!(
            children
                .session
                .dequeue_event(EventLane::Reliable, 4_096)
                .expect("reliable event")
                .is_some()
        );
        let old_epoch = children.session.test_providers().test_epoch();
        let new_epoch = children.session.test_pending_providers()[0].test_epoch();
        assert_eq!(new_epoch, old_epoch + 1);

        // First half of `PlanState::render`: the owner swaps and retires the old plan. A removal
        // is adopted at its scheduled sample `S`, after the strip's ramp (#1325 D3): the blocks
        // before it render whole, fed, and the round's window is the swapping block's.
        let report = loop {
            let report = children
                .plan
                .test_owner_mut()
                .render_contiguous(
                    RenderIo {
                        output: PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
                    },
                    sample,
                )
                .expect("swapping block");
            if report.swap == engine::realtime::SwapOutcome::Applied || round % 2 == 1 {
                break report;
            }
            children
                .plan
                .test_active_epoch()
                .store(report.active_epoch.0, Ordering::Release);
            sample += 128;
            assert!(sample < 128 * 128, "round {round}: the removal is adopted");
            feed(&mut children, "phase 1");
        };
        assert_eq!(report.swap, engine::realtime::SwapOutcome::Applied);
        assert_eq!(report.active_epoch.0, new_epoch);
        assert_eq!(children.plan.owner().carried_count(), round);
        assert!(
            pcm.iter().all(|sample| *sample != 0.0),
            "round {round}: the swap block plays the carried ring"
        );
        assert_eq!(
            children.plan.test_active_epoch().load(Ordering::Acquire),
            old_epoch,
            "the window is open: the atomic still names the retired plan"
        );

        // Inside the window: a source submit, an immediate command, a lossy dequeue, and a
        // structural command.
        feed(&mut children, "swap window");
        request_id += 1;
        let capabilities = command_bytes(request_id, protocol::CommandPayload::CapabilitiesGet);
        let window_immediate = children.session.command(&capabilities, 4_096);
        let window_lossy = children.session.dequeue_event(EventLane::Lossy, 4_096);
        request_id += 1;
        pending = structural(request_id, revision, round);
        let window_structural = children.session.command(&pending, 4_096);
        let window_revision = children.session.test_controller().session().revision().0;
        // The any-thread query reads the lagging atomic's row, which must still be there.
        let window_query = children.plan.queries().resources();
        let window_rows = children.session.test_transaction_snapshot().resource_rows;
        assert_eq!(
            window_rows
                .iter()
                .find_map(|(epoch, report)| (*epoch == old_epoch).then_some(*report)),
            Some(window_query)
        );
        assert_eq!(
            window_rows
                .iter()
                .map(|(epoch, _)| *epoch)
                .collect::<Vec<_>>(),
            vec![old_epoch, new_epoch, new_epoch + 1],
            "round {round}: the lagging row, the running plan's and the window's candidate"
        );

        // Second half of `PlanState::render`: publish the epoch that rendered the block.
        children
            .plan
            .test_active_epoch()
            .store(report.active_epoch.0, Ordering::Release);
        let after_structural = children.session.command(&pending, 4_096);
        let snapshot = children.session.test_transaction_snapshot();
        let rows = snapshot
            .resource_rows
            .iter()
            .map(|(epoch, _)| *epoch)
            .collect::<Vec<_>>();
        assert_eq!(
            (
                control_outcome(&window_immediate),
                control_outcome(&window_lossy),
                control_outcome(&window_structural),
                window_revision,
                control_outcome(&after_structural),
                snapshot.provider_epoch,
                snapshot.retired_provider_epochs,
                snapshot.pending_provider_epochs,
                rows,
            ),
            (
                "ok".to_owned(),
                "ok".to_owned(),
                // #1398: the window's structural command is admitted against the running plan
                // and commits; the same request after the window is its replay.
                "ok".to_owned(),
                revision + 1,
                "ok".to_owned(),
                new_epoch,
                Vec::new(),
                vec![new_epoch + 1],
                vec![new_epoch, new_epoch + 1],
            ),
            "round {round}: a control call inside the swapping render call"
        );
        assert_eq!(
            children.session.test_controller().session().revision().0,
            revision + 1,
            "round {round}: the replay commits nothing more"
        );
        revision += 1;
    }
    // The last window's candidate is adopted at the next block.
    let last = children.session.test_pending_providers()[0].test_epoch();
    feed(&mut children, "last candidate");
    children
        .plan
        .render(
            sample + 128,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("adopting block");
    assert_eq!(children.plan.owner().active_epoch().0, last);
}

/// Issue #1042, attempt 2, and #1398 gate 1: a valid structural edit inside the swap window is
/// admitted against the running plan, never refused.
///
/// The limit here admits exactly the pairs that really coexist (nine EQs with eight, either way
/// round) and refuses nine with nine. The test removes one EQ and puts it back twice: while the
/// eight-EQ plan is still pending, and inside the window after the swap. While the eight-EQ plan
/// is pending the edit would supersede it (#1310), so the running nine-EQ plan, the withdrawn
/// eight-EQ plan and the new nine-EQ plan are held at once: the three-plan peak is over the limit
/// (#1310 gate 4). The edit fits beside the eight-EQ plan alone, which is all a retry meets once
/// render adopts it, so the refusal is retryable `Backpressure`, not a permanent compile rejection
/// (batch 15a review fold, item 2). Inside the window the nine-EQ plan is reclaimed, so the edit
/// sits beside the eight-EQ plan alone and is admitted; its plan is adopted at its duck-swap's
/// `S` (#1324a), and the report table holds three rows meanwhile. Red if the lag refusal is kept (the window
/// answers `Backpressure`), if the admission pairs the candidate with the atomic's retired row
/// (`effect.resource.limit`), or if the three-plan refusal is permanent.
#[test]
fn a_valid_edit_inside_the_swap_window_is_admitted_against_the_running_plan() {
    // The fixture's EQ is a console slot on all nine tracks, which a track cannot drop (decision
    // 12), so the edited EQ is a tenth instance: an insert on the first track. Console session
    // edits are #1094's.
    let mut model = parse_session_json(SESSION).expect("fixture");
    let reduced_session = session::canonical_session_json(&model).expect("canonical");
    let mut eq = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    eq.id = session::StableId::parse("eq-insert").expect("insert slot");
    model.tracks[0].inserts.effects.push(eq.clone());
    let full_session = session::canonical_session_json(&model).expect("canonical");
    let full = compile_children(&full_session, limits())
        .expect("ten EQs")
        .plan
        .resources()
        .effect_scalar_state_bytes;
    let reduced = compile_children(&reduced_session, limits())
        .expect("nine EQs")
        .plan
        .resources()
        .effect_scalar_state_bytes;
    assert!(reduced < full, "removing an EQ frees effect state");
    let mut tight = limits();
    tight.maximum_effect_state_bytes = full + reduced;

    let mut children = compile_children(&full_session, tight).expect("tight children");
    let mut pcm = [0.0_f32; 256];
    children
        .plan
        .render(
            0,
            PlanarBufferMut::try_new(&mut pcm, 2, 128, 128).expect("output"),
        )
        .expect("first block");
    let track_id = model.tracks[0].id.clone();
    let remove = protocol::SessionEdit::RemoveTrackEffect {
        track_id: track_id.clone(),
        rack_name: session::RackName::Inserts,
        effect_id: eq.id.clone(),
    };
    let put_back = protocol::SessionEdit::PutTrackEffect {
        track_id,
        rack_name: session::RackName::Inserts,
        final_position: 0,
        effect: eq,
    };
    let edit = |request_id: u64, revision: u64, edit: &protocol::SessionEdit| {
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(edit)),
        )
    };
    children
        .session
        .command(&edit(1, 42, &remove), 4_096)
        .expect("nine EQs beside eight fit");
    assert!(
        children
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("reliable event")
            .is_some()
    );
    let request = edit(2, 43, &put_back);
    // Before the swap the eight-EQ plan is still pending: superseding it would hold three plans,
    // which the limit refuses until a render adopts it, so the refusal is retryable; the
    // eight-EQ plan is republished and adopted below.
    let pending = children.session.command(&request, 4_096);

    // First half of `PlanState::render`: swap to the eight-EQ plan, at its duck-swap's `S`.
    let (report, swapped_at) = render_owner_until_swap(&mut children, &mut pcm, 128);
    assert_eq!(report.swap, engine::realtime::SwapOutcome::Applied);
    let window = children.session.command(&request, 4_096);
    let window_revision = children.session.test_controller().session().revision().0;
    let window_rows = children
        .session
        .test_transaction_snapshot()
        .resource_rows
        .iter()
        .map(|(epoch, _)| *epoch)
        .collect::<Vec<_>>();
    // Second half: publish the epoch that rendered the block.
    children
        .plan
        .test_active_epoch()
        .store(report.active_epoch.0, Ordering::Release);
    // The same request again is the window's replay.
    let after = children.session.command(&request, 4_096);
    assert_eq!(
        (
            control_outcome(&pending),
            control_outcome(&window),
            window_revision,
            window_rows,
            control_outcome(&after),
            children.session.test_controller().session().revision().0,
        ),
        (
            control_outcome::<(), _>(&Err(CommandError::Backpressure)),
            "ok".to_owned(),
            44,
            vec![0, 1, 2],
            "ok".to_owned(),
            44
        ),
        "the eight-EQ plan rendering beside nine EQs fits; only a retired row says it does not"
    );
    // The window's plan, a duck-swap too, is adopted at its own `S`.
    feed_and_render_until_direct(&mut children, &[], swapped_at / 128 + 1, 44);
    assert_eq!(children.plan.owner().active_epoch().0, 2);
}

/// Issue #1042 on one thread: a *rejected* render call can open the window too.
///
/// `RealtimePlanOwner::enter_block` swaps plans before `render_contiguous` checks the block's
/// time, so a render call with the wrong `absolute_sample` swaps and returns
/// `RESULT_RENDER_REJECTED` without publishing `active_epoch`. Before the fix, the next control
/// call returned `RESULT_INTERNAL` and every later structural command `RESULT_BACKPRESSURE`, with
/// no second thread involved.
#[test]
fn a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let mut c_pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: c_pcm.as_mut_ptr(),
        sample_capacity: c_pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    let structural = |request_id: u64, revision: u64| {
        let edit = rebuild_edit(SESSION, u8::try_from(request_id).expect("small request ID"));
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
        )
    };
    // #1398 gate 1: each round publishes a candidate, a rejected render call swaps to it, and a
    // structural command inside that window is admitted against the running plan and commits; the
    // next good block adopts its plan. The retry of the same request is its replay.
    let mut request_id = 1;
    let mut revision = 42;
    assert_eq!(
        command_c(c_session, &structural(request_id, revision)).0,
        crate::RESULT_OK
    );
    revision += 1;

    for round in 1..=4_u64 {
        let next_sample = round * 128;
        let (reliable, _) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
        let (revision_before, _, provider_before, pending_before) =
            crate::ffi::test_session_state_summary(c_session);
        assert_eq!(pending_before, 1, "round {round}: a candidate is published");
        let rejected = crate::ffi::test_render(c_plan, next_sample + 1, &output);
        let (lossy, _) = event_c(c_session, crate::EVENT_LANE_LOSSY);
        request_id += 1;
        let request = structural(request_id, revision);
        let (window, canary) = command_c(c_session, &request);
        let (window_revision, _, provider, pending) =
            crate::ffi::test_session_state_summary(c_session);
        let window_rows = crate::ffi::test_transaction_snapshot(c_session)
            .resource_rows
            .len();
        let good = crate::ffi::test_render(c_plan, next_sample, &output);
        let (after, _) = command_c(c_session, &request);
        let (after_revision, _, after_provider, after_pending) =
            crate::ffi::test_session_state_summary(c_session);
        // Drain the window's `SESSION_COMMITTED`, then publish the next round's candidate.
        let (window_event, _) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
        revision += 1;
        request_id += 1;
        let (next, _) = command_c(c_session, &structural(request_id, revision));
        revision += 1;
        assert_eq!(
            (
                reliable,
                rejected,
                lossy,
                window,
                canary == vec![0xa5; 4_096],
                window_revision,
                provider,
                pending,
                window_rows,
            ),
            (
                crate::RESULT_OK,
                crate::RESULT_RENDER_REJECTED,
                crate::RESULT_OK,
                crate::RESULT_OK,
                false,
                revision_before + 1,
                provider_before + 1,
                1,
                3,
            ),
            "round {round}: control after a rejected, swapping render call"
        );
        assert_eq!(
            (
                good,
                after,
                after_revision,
                after_provider,
                after_pending,
                window_event,
                next,
            ),
            (
                crate::RESULT_OK,
                crate::RESULT_OK,
                revision_before + 1,
                provider_before + 2,
                0,
                crate::RESULT_OK,
                crate::RESULT_OK,
            ),
            "round {round}: the window's plan is adopted at the next block"
        );
    }
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// #1627 gate 3. A frame under the retired message ID `0x0006` (`AUTOMATION_ENQUEUE`), with the
/// payload the command had, is refused at decode by the C ABI as an unsupported message: the call
/// fails `INVALID_ARGUMENT`, writes nothing and reports no size, and the revision, the queues and
/// the reliable event lane stay as they were (no ack precedes a drop). `CAPABILITIES_GET` neither
/// lists command `0x0006` nor sets flag bit 5, and its record has the 23 fields.
///
/// Red if the C ABI dispatches the retired command again, if the refusal changes state first, or
/// if the capabilities record advertises the command or its flag.
#[test]
fn the_retired_automation_command_is_refused_at_decode_and_not_advertised() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let mut request = 0;
    let (_, _, revision) = control_list_all_c(c_session, &mut request, 64);
    let before = crate::ffi::test_session_state_summary(c_session);
    let snapshot_before = crate::ffi::test_transaction_snapshot(c_session);
    request += 1;
    let frame = retired_automation_frame(request, revision);
    let (result, required, storage) = command_c_capacity(c_session, &frame, 4_096);
    assert_eq!(result, crate::RESULT_INVALID_ARGUMENT);
    assert_eq!(required, u64::MAX, "no size is reported");
    assert_eq!(storage, vec![0xa5; 4_096], "nothing is written");
    let snapshot = crate::ffi::test_transaction_snapshot(c_session);
    assert_eq!(snapshot.revision, revision);
    assert_eq!(snapshot.revision, snapshot_before.revision);
    let after = crate::ffi::test_session_state_summary(c_session);
    assert_eq!((after.0, after.2, after.3), (before.0, before.2, before.3));
    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_RELIABLE),
        (crate::RESULT_OK, Vec::new()),
        "no reliable event"
    );

    // The refusal spent no request ID: the same ID serves the next command.
    let bytes = command_bytes(request, protocol::CommandPayload::CapabilitiesGet);
    let (result, response) = command_c(c_session, &bytes);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = vec![0_u16; 1 << 12];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::Capabilities(capabilities),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("a capabilities record that passes the wire validator")
    else {
        panic!("capabilities are a success response");
    };
    let ids = |bytes: &[u8]| {
        bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
            .collect::<Vec<_>>()
    };
    let commands = ids(capabilities.supported_commands);
    let events = ids(capabilities.supported_events);
    assert!(!commands.contains(&0x0006), "{commands:?}");
    assert_eq!(capabilities.flags.0 & (1 << 5), 0, "flag bit 5");
    assert!(events.contains(&0x8001), "the session family keeps 0x8001");
    assert!(!events.contains(&0x8002), "0x8002 is retired (#1628)");
    assert!(commands.contains(&0x0003), "the session family is served");
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// The C ABI serves five event families: `SESSION_COMMITTED`, `TRANSPORT_STATE`, `DIAGNOSTIC`,
/// `METER_BATCH` and `COUNTER_SNAPSHOT`. `AUTOMATION_CANCELED` (`0x8002`) is retired with
/// `AUTOMATION_ENQUEUE` (#1627, #1628): no batch can be queued, so no path raises it and no
/// capabilities record lists it.
#[test]
fn all_five_event_families_cross_c_dequeue_with_exact_oracle_bytes() {
    // #1583 re-pin: the three meter configurations are the output's peak at a 12-block period
    // (the window), so each echo carries the period 12, the one table entry (field 7) and the
    // window 12 (field 8), and no retired field 1; the third is the second again, not a two-entry
    // table. The counter configuration's echo carries the window and no field 1. The meter
    // events are two windows' batches, and the counter snapshot follows them at sample 3200 with
    // `TELEMETRY_DROPPED` 0: the two-slot lossy lane drops nothing.
    const RESPONSES: [&str; 7] = [
        "4d49534f43544c000100000030000200090000008800000001000000000000002a00000000000000070000000000000002000301040000000c0000000000000003000d010000000004000301040000000000000000000000050008010100000001000000000000000600010101000000010000000000000007000b00280000000200000000000000010001010100000001000000000000000300010101000000010000000000000008000300040000000c00000000000000",
        // #1635 re-pin: the transport snapshot drops the retired position (field 2).
        "4d49534f43544c000100000030000200080000002000000002000000000000002a0000000000000002000000000000000100010101000000020000000000000003000401080000000000000000000000",
        "4d49534f43544c000100000030000200030000002000000004000000000000002b0000000000000002000000000000000100030104000000010000000000000002000101010000000200000000000000",
        "4d49534f43544c000100000030000200090000008800000005000000000000002b00000000000000070000000000000002000301040000000c0000000000000003000d010000000004000301040000000000000000000000050008010100000000000000000000000600010101000000010000000000000007000b00280000000200000000000000010001010100000001000000000000000300010101000000010000000000000008000300040000000c00000000000000",
        "4d49534f43544c000100000030000200090000008800000006000000000000002b00000000000000070000000000000002000301040000000c0000000000000003000d010000000004000301040000000000000000000000050008010100000000000000000000000600010101000000010000000000000007000b00280000000200000000000000010001010100000001000000000000000300010101000000010000000000000008000300040000000c00000000000000",
        // #1386 re-pin: the capabilities command list adds `0x0013`, as above; #1595 adds the
        // spectrum commands and bit 16, #1587a field 15's 256 slots, and #1586 `0x000f` and bit 15,
        // as above; #2115 adds `0x0014` and bit 17, as above.
        "4d49534f43544c000100000030000200010000009801000007000000000000002b00000000000000170000000000000001000201020000000100000000000000020002010200000000000000000000000300020102000000010000000000000004000201020000000000000000000000050004010800000000100000000000000600030104000000000800000000000007000401080000000010000000000000080001010100000004000000000000000a0004010800000001000000000000000b0004010800000000100000000000000d0004010800000001000000000000000e0004010800000002000000000000000f0004010800000000010000000000001000040108000000100000000000000011000401080000000020000000000000120004010800000000100000000000001500020102000000000100000000000016000201020000000001000000000000170002010200000000010000000000001800030104000000000800000000000019000c01220000000100020003000700080009000a000b000c000d000e000f00100011001200130014000000000000001a000c010e000000018010801180208021803080408000001b00040108000000dfff030000000000",
        "4d49534f43544c000100000030000200090000006000000008000000000000002b0000000000000006000000000000000200030104000000000000000000000003000d0104000000060000000000000004000301040000000100000000000000050008010100000000000000000000000600010101000000010000000000000008000300040000000c00000000000000",
    ];
    const EVENTS: [&str; 6] = [
        // #1635 re-pin: the `TRANSPORT_STATE` event drops the retired position (field 3): four
        // fields, payload length 0x40 (was 0x50).
        "4d49534f43544c000100000030000300108000004000000000000000000000002a00000000000000040000000000000001000401080000000100000000000000020001010100000002000000000000000400040108000000000000000000000005000400080000000200000000000000",
        "4d49534f43544c000100000030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000002000000000000000200040108000000040000000000000003000401080000002a0000000000000004000301040000000100000000000000",
        "4d49534f43544c000100000030000300308000006000000000000000000000002b00000000000000010000000000000001000b015800000004000000000000000100090114000000636170692e72656e6465722e616374697669747900000000020001010100000001000000000000000600040008000000800000000000000007000400080000000100000000000000",
        // #1583 re-pin: the output's peak over the first meter window, `[0, 1536)`, sequence 1:
        // two records (left, right; valid, silence), and fields 5 and 6 carry the span's start and
        // the sequence.
        "4d49534f43544c000100000030000300208000007800000000000000000000002b00000000000000060000000000000001000401080000000006000000000000020002010200000002000000000000000300020102000000100000000000000004000a012000000001000000010001000000000000000000010000000200010000000000000000000500040108000000000000000000000006000401080000000100000000000000",
        // The second window, `[1536, 3072)`, sequence 2.
        "4d49534f43544c000100000030000300208000007800000000000000000000002b0000000000000006000000000000000100040108000000000c000000000000020002010200000002000000000000000300020102000000100000000000000004000a012000000001000000010001000000000000000000010000000200010000000000000000000500040108000000000600000000000006000401080000000200000000000000",
        "4d49534f43544c000100000030000300218000004000000000000000000000002b0000000000000002000000000000000100040108000000800c00000000000002000b012800000002000000000000000100030104000000060000000000000002000401080000000000000000000000",
    ];
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let eager_capacities = crate::ffi::test_retained_capacities(c_session);
    let revision = SessionRevision(42);
    // #1583: the output's peak, one batch per meter window.
    let window = fixture_window_blocks();
    let configuration = protocol::TelemetryConfiguration {
        meters: vec![output_peak_entry()],
        meter_period_blocks: u32::try_from(window).expect("period"),
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let configure = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    );
    let (c_result, c_response) = command_c(c_session, &configure);
    assert_eq!(c_result, crate::RESULT_OK);
    assert_eq!(c_response, pinned_hex(RESPONSES[0]));

    let transport = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::TransportSet(protocol::TransportSetRequest {
            state: protocol::TransportState::Playing,
        }),
    );
    assert_eq!(
        command_c(c_session, &transport),
        (crate::RESULT_OK, pinned_hex(RESPONSES[1]))
    );
    event_c_exact_retry(
        c_session,
        crate::EVENT_LANE_RELIABLE,
        &pinned_hex(EVENTS[0]),
    );

    let edit = protocol::SessionEdit::SetSessionId {
        session_id: session::StableId::parse("event-origin").expect("stable ID"),
    };
    let structural = command_bytes_at_revision(
        4,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    assert_eq!(
        command_c(c_session, &structural),
        (crate::RESULT_OK, pinned_hex(RESPONSES[2]))
    );
    for event in &EVENTS[1..=1] {
        event_c_exact_retry(c_session, crate::EVENT_LANE_RELIABLE, &pinned_hex(event));
    }

    let mut pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    event_c_exact_retry(
        c_session,
        crate::EVENT_LANE_RELIABLE,
        &pinned_hex(EVENTS[2]),
    );

    let quiet_meter_configuration = protocol::TelemetryConfiguration {
        diagnostics_enabled: false,
        ..configuration.clone()
    };
    let disable_diagnostics = command_bytes_at_revision(
        5,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&quiet_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &disable_diagnostics),
        (crate::RESULT_OK, pinned_hex(RESPONSES[3]))
    );
    // #1583: two meter windows from the configuration's sample 0, the output's peak of each.
    for block in 1..2 * window {
        assert_eq!(
            crate::ffi::test_render(c_plan, block * 128, &output),
            crate::RESULT_OK
        );
    }
    // The next command's service step stages the two windows' batches (sequences 1 and 2); the
    // configure itself, the same table, then restarts the sequence and keeps the level on.
    let same_meters = command_bytes_at_revision(
        6,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&quiet_meter_configuration),
    );
    assert_eq!(
        command_c(c_session, &same_meters),
        (crate::RESULT_OK, pinned_hex(RESPONSES[4]))
    );
    let nothing_new = command_bytes(7, protocol::CommandPayload::CapabilitiesGet);
    assert_eq!(
        command_c(c_session, &nothing_new),
        (crate::RESULT_OK, pinned_hex(RESPONSES[5]))
    );
    // The lossy lane holds the two batches whole: nothing coalesced or dropped.
    assert_eq!(
        crate::ffi::test_telemetry_counters(c_session),
        protocol::TelemetryCounters {
            telemetry_coalesced: 0,
            telemetry_dropped: 0,
        }
    );
    for event in &EVENTS[3..=4] {
        event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &pinned_hex(event));
    }

    let counter_configuration = protocol::TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids: vec![protocol::CounterId::TelemetryDropped],
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let configure_counters = command_bytes_at_revision(
        8,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::TelemetryConfigure(&counter_configuration),
    );
    assert_eq!(
        command_c(c_session, &configure_counters),
        (crate::RESULT_OK, pinned_hex(RESPONSES[6]))
    );
    assert_eq!(
        crate::ffi::test_render(c_plan, 2 * window * 128, &output),
        crate::RESULT_OK
    );
    event_c_exact_retry(c_session, crate::EVENT_LANE_LOSSY, &pinned_hex(EVENTS[5]));

    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_RELIABLE),
        (crate::RESULT_OK, Vec::new())
    );
    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_LOSSY),
        (crate::RESULT_OK, Vec::new())
    );
    assert_eq!(
        crate::ffi::test_retained_capacities(c_session),
        eager_capacities
    );
    crate::ffi::test_session_destroy(c_session);
    crate::ffi::test_plan_destroy(c_plan);
}

#[test]
fn plan_first_destroy_guards_structural_publication_without_visible_mutation() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    crate::ffi::test_plan_destroy(c_plan);
    let edit = rebuild_edit(SESSION, 0x01);
    let request = command_bytes_at_revision(
        1,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    let before = crate::ffi::test_session_state_summary(c_session);
    let (short_result, required, short_canary) = command_c_capacity(c_session, &request, 0);
    assert_eq!(short_result, crate::RESULT_BUFFER_TOO_SMALL);
    assert_eq!(required, 4_096);
    assert!(short_canary.is_empty());
    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
    let (result, canary) = command_c(c_session, &request);
    assert_eq!(result, crate::RESULT_BACKPRESSURE);
    assert_eq!(canary, vec![0xa5; 4_096]);
    assert_eq!(crate::ffi::test_session_state_summary(c_session), before);
    assert_eq!(
        event_c(c_session, crate::EVENT_LANE_RELIABLE),
        (crate::RESULT_OK, Vec::new())
    );
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits() {
    use TestStructuralFaultPhase::{
        AfterAdmission, AfterPlanReservation, AfterProtocolPrepare, AfterRuntimePrepare,
        BeforeProtocolCommit, BeforeRuntimePrepare,
    };

    const PHASES: [TestStructuralFaultPhase; 6] = [
        AfterProtocolPrepare,
        BeforeRuntimePrepare,
        AfterRuntimePrepare,
        AfterAdmission,
        AfterPlanReservation,
        BeforeProtocolCommit,
    ];
    fn accumulate_fault(counters: &mut TestOwnerCounters, phase: TestStructuralFaultPhase) {
        counters.token_constructed += 1;
        counters.token_disposed += 1;
        counters.replay_candidate_constructed += 1;
        counters.replay_candidate_disposed += 1;
        if matches!(
            phase,
            AfterRuntimePrepare | AfterAdmission | AfterPlanReservation | BeforeProtocolCommit
        ) {
            counters.candidate_provider_constructed += 1;
            counters.candidate_provider_disposed += 1;
            counters.candidate_plan_constructed += 1;
            counters.candidate_plan_disposed += 1;
        }
        if matches!(phase, AfterPlanReservation | BeforeProtocolCommit) {
            counters.reservation_constructed += 1;
            counters.reservation_canceled += 1;
        }
    }
    fn accumulate_success(counters: &mut TestOwnerCounters) {
        counters.token_constructed += 1;
        counters.token_disposed += 1;
        counters.replay_candidate_constructed += 1;
        counters.replay_candidate_published += 1;
        counters.replay_current_disposed += 1;
        counters.candidate_provider_constructed += 1;
        counters.candidate_provider_published += 1;
        counters.candidate_plan_constructed += 1;
        counters.candidate_plan_published += 1;
        counters.reservation_constructed += 1;
        counters.reservation_committed += 1;
    }

    for first in PHASES {
        for second in PHASES {
            crate::ffi::test_reset_lifecycle_observer();
            let (c_session, c_plan) = boxed_c_children(SESSION);
            crate::ffi::test_set_structural_faults(c_session, [Some(first), Some(second)]);
            let edit = rebuild_edit(SESSION, 0x01);
            let request = command_bytes_at_revision(
                1,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
            );
            let before = crate::ffi::test_transaction_snapshot(c_session);
            let plan_before = crate::ffi::test_plan_snapshot(c_plan);
            let mut expected = TestOwnerCounters {
                current_provider_constructed: 1,
                current_plan_constructed: 1,
                replay_current_constructed: 1,
                ..TestOwnerCounters::default()
            };
            for phase in [first, second] {
                let (result, canary) = command_c(c_session, &request);
                assert_eq!(result, crate::RESULT_BACKPRESSURE, "{first:?}/{second:?}");
                assert_eq!(canary, vec![0xa5; 4_096]);
                assert_eq!(
                    crate::ffi::test_transaction_snapshot(c_session),
                    before,
                    "canonical/model/epochs/replay/events/resources/credits {first:?}/{second:?}"
                );
                assert_eq!(
                    crate::ffi::test_plan_snapshot(c_plan),
                    plan_before,
                    "PCM boundary and plan resources {first:?}/{second:?}"
                );
                assert_eq!(
                    event_c(c_session, crate::EVENT_LANE_RELIABLE),
                    (crate::RESULT_OK, Vec::new())
                );
                accumulate_fault(&mut expected, phase);
                assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            }

            let (result, response) = command_c(c_session, &request);
            assert_eq!(result, crate::RESULT_OK);
            assert_ne!(response, vec![0xa5; 4_096]);
            accumulate_success(&mut expected);
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            assert_eq!(
                crate::ffi::test_session_state_summary(c_session),
                (43, 1, 0, 1)
            );

            let mut pcm = [f32::NAN; 256];
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: 128,
                plane_stride_samples: 128,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(c_plan, 0, &output),
                crate::RESULT_OK
            );
            assert_eq!(
                event_c(c_session, crate::EVENT_LANE_RELIABLE).0,
                crate::RESULT_OK
            );
            expected.current_provider_disposed += 1;
            expected.current_plan_disposed += 1;
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);

            let retry_edit = rebuild_edit(SESSION, 0x02);
            let retry = command_bytes_at_revision(
                2,
                ExpectedRevision::Exact(SessionRevision(43)),
                protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(
                    &retry_edit,
                )),
            );
            assert_eq!(command_c(c_session, &retry).0, crate::RESULT_OK);
            accumulate_success(&mut expected);
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            assert_eq!(
                crate::ffi::test_session_state_summary(c_session),
                (44, 2, 1, 1),
                "reclaim released publication and retirement credit"
            );
            assert_eq!(
                crate::ffi::test_render(c_plan, 128, &output),
                crate::RESULT_OK
            );
            let collect = command_bytes(3, protocol::CommandPayload::CapabilitiesGet);
            assert_eq!(command_c(c_session, &collect).0, crate::RESULT_OK);
            expected.current_plan_disposed += 1;
            expected.candidate_provider_disposed += 1;
            assert_eq!(crate::ffi::test_owner_counters(c_session), expected);
            crate::ffi::test_plan_destroy(c_plan);
            expected.current_plan_disposed += 1;
            crate::ffi::test_session_destroy(c_session);
            expected.candidate_provider_disposed += 1;
            expected.replay_current_disposed += 1;
            assert_eq!(crate::ffi::test_lifecycle_counters(), expected);
        }
    }
}

#[test]
fn capi_controller_dispatches_every_advertised_command_family() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let eager_capacities = crate::ffi::test_retained_capacities(c_session);
    assert!(eager_capacities.iter().all(|capacity| *capacity > 0));
    let codec = ProtocolCodec::default();
    let mut request_id = 0_u64;
    macro_rules! dispatch {
        ($expected:expr, $payload:expr, $status:expr, $revision:expr, $events:expr) => {{
            request_id += 1;
            let mut request = vec![0_u8; 4_096];
            let len = codec
                .encode_command_frame_into(
                    &protocol::TypedCommandFrame {
                        request_id: RequestId::new(request_id).expect("request ID"),
                        expected_revision: $expected,
                        payload: $payload,
                    },
                    &mut request,
                )
                .expect("command frame");
            request.truncate(len);
            let pinned = pinned_hex(ALL_COMMAND_RESPONSE_VECTORS[request_id as usize - 1]);
            let (c_result, c_bytes) = command_c(c_session, &request);
            assert_eq!(c_result, crate::RESULT_OK, "C command {request_id}");
            assert_eq!(c_bytes, pinned, "pinned C bytes {request_id}");

            let (c_replay_result, c_replay) = command_c(c_session, &request);
            assert_eq!(c_replay_result, crate::RESULT_OK, "C replay {request_id}");
            assert_eq!(c_replay, pinned, "pinned replay bytes {request_id}");
            let mut fields = [0_u16; 512];
            let response = codec
                .decode_typed_response(&c_bytes, &mut DecodeScratch::new(&mut fields))
                .expect("typed response");
            let header = match response {
                protocol::DecodedTypedResponseFrame::Success { header, .. }
                | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
            };
            assert_eq!(header.status, $status, "accepted status {request_id}");
            assert_eq!(header.revision, SessionRevision($revision));
            let mut event_ids = Vec::new();
            loop {
                let (event_result, c_event) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
                assert_eq!(event_result, crate::RESULT_OK);
                if c_event.is_empty() {
                    break;
                }
                let mut event_fields = [0_u16; 64];
                event_ids.push(
                    codec
                        .decode_typed_event(&c_event, &mut DecodeScratch::new(&mut event_fields))
                        .expect("typed command event")
                        .header
                        .message_id,
                );
            }
            assert_eq!(
                event_c(c_session, crate::EVENT_LANE_RELIABLE),
                (crate::RESULT_OK, Vec::new())
            );
            assert_eq!(event_ids.as_slice(), $events, "events {request_id}");
            let summary = crate::ffi::test_session_state_summary(c_session);
            assert_eq!(summary.0, $revision as u64);
            assert_eq!(summary.1, request_id as usize);
            assert_eq!(summary.2, 0, "pinned active provider epoch");
            assert_eq!(
                summary.3,
                usize::from(request_id == 10),
                "pinned pending provider count"
            );
            header.message_id
        }};
    }

    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::CapabilitiesGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::CapabilitiesGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::SessionSnapshotGet(protocol::SessionSnapshotRequest {
                offset: 0,
                maximum_bytes: 1,
            },),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::SessionSnapshotGet
    );
    // The first page of the control list and of the control states: one row, the first owner's
    // header (`eq0`), and the page after it.
    let control_request = protocol::ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 1,
        owner: None,
        headers_only: false,
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::ControlListGet(&control_request),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::ControlListGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::ControlStateGet(&control_request),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::ControlStateGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::TransportGet,
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::TransportGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::TransportSet(protocol::TransportSetRequest {
                state: protocol::TransportState::Playing,
            },),
            StatusCode::Ok,
            42,
            &[protocol::MessageId::TransportState]
        ),
        protocol::MessageId::TransportSet
    );
    let telemetry = protocol::TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::TelemetryConfigure(&telemetry),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::TelemetryConfigure
    );
    let counters = protocol::CountersRequest {
        all: true,
        ids: Vec::new(),
    };
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::CountersGet(&counters),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::CountersGet
    );
    assert_eq!(
        dispatch!(
            ExpectedRevision::Any,
            protocol::CommandPayload::DiagnosticsGet(protocol::DiagnosticsRequest {
                after_sequence: 0,
                limit: 1,
                minimum_severity: protocol::DiagnosticSeverity::Info,
            },),
            StatusCode::Ok,
            42,
            &[]
        ),
        protocol::MessageId::DiagnosticsGet
    );
    let structural = rebuild_edit(SESSION, 0x01);
    assert_eq!(
        dispatch!(
            ExpectedRevision::Exact(SessionRevision(42)),
            protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&structural),),
            StatusCode::Ok,
            43,
            &[protocol::MessageId::SessionCommitted]
        ),
        protocol::MessageId::SessionTransactionApply
    );
    assert_eq!(request_id, 10);
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 10, 0, 1),
        "pinned revision/replay/provider/pending vector"
    );
    assert_eq!(
        crate::ffi::test_retained_capacities(c_session),
        eager_capacities
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

#[test]
fn exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact() {
    const REUSE: &str = "4d49534f43544c000100000030000200070009004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const EXPIRED: &str = "4d49534f43544c00010000003000020001000a004800000001000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    const STALE: &str = "4d49534f43544c000100000030000200030007004800000013000000000000002a00000000000000020000000000000001000b01300000000200000000000000010009011000000070726f746f636f6c2e6661696c7572650200010101000000030000000000000002000301040000000000000000000000";
    // #1635 re-pin: the transport snapshot and the `TRANSPORT_STATE` event drop the retired position.
    const TRANSPORT_20: &str = "4d49534f43544c000100000030000200080000002000000014000000000000002a0000000000000002000000000000000100010101000000010000000000000003000401080000000000000000000000";
    const TRANSPORT_21: &str = "4d49534f43544c000100000030000200080000002000000015000000000000002a0000000000000002000000000000000100010101000000020000000000000003000401080000000000000000000000";
    const EVENT_FULL: &str = "4d49534f43544c00010000003000020003000b00b000000016000000000000002a00000000000000030000000000000001000b01380000000200000000000000010009011500000070726f746f636f6c2e6261636b7072657373757265000000020001010100000003000000000000000200030104000000000000000000000003000b005800000005000000000000000100010101000000040000000000000002000401080000000200000000000000030004010800000002000000000000000400020102000000010000000000000005000400080000000400000000000000";
    const TRANSPORT_EVENT_20: &str = "4d49534f43544c000100000030000300108000004000000000000000000000002a00000000000000040000000000000001000401080000000100000000000000020001010100000001000000000000000400040108000000000000000000000005000400080000001400000000000000";
    const TRANSPORT_EVENT_21: &str = "4d49534f43544c000100000030000300108000004000000000000000000000002a00000000000000040000000000000001000401080000000200000000000000020001010100000002000000000000000400040108000000000000000000000005000400080000001500000000000000";
    const STRUCTURAL_23: &str = "4d49534f43544c000100000030000200030000002000000017000000000000002b0000000000000002000000000000000100030104000000010000000000000002000101010000000300000000000000";
    const COMMIT_EVENT: &str = "4d49534f43544c000100000030000300018000004000000000000000000000002b000000000000000400000000000000010004010800000003000000000000000200040108000000170000000000000003000401080000002a0000000000000004000301040000000100000000000000";
    const RETRY_24: &str = "4d49534f43544c000100000030000200030000002000000018000000000000002c0000000000000002000000000000000100030104000000010000000000000002000101010000000300000000000000";
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let codec = ProtocolCodec::default();
    let capabilities_response = |request_id: u64| {
        let mut bytes = pinned_hex(ALL_COMMAND_RESPONSE_VECTORS[0]);
        bytes[24..32].copy_from_slice(&request_id.to_le_bytes());
        bytes
    };
    macro_rules! dispatch {
        ($request:expr, $status:expr, $expected:expr) => {{
            let request = $request;
            let (result, bytes) = command_c(c_session, &request);
            assert_eq!(result, crate::RESULT_OK);
            assert_eq!(bytes, $expected);
            let mut fields = [0_u16; 64];
            let decoded = codec
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
                .expect("typed decision");
            let header = match decoded {
                protocol::DecodedTypedResponseFrame::Success { header, .. }
                | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header,
            };
            assert_eq!(header.status, $status);
            bytes
        }};
    }

    let first = command_bytes(1, protocol::CommandPayload::CapabilitiesGet);
    let first_bytes = dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1));
    assert_eq!(
        dispatch!(first.clone(), StatusCode::Ok, capabilities_response(1)),
        first_bytes
    );
    let conflict = command_bytes(1, protocol::CommandPayload::TransportGet);
    dispatch!(conflict, StatusCode::RequestIdReuse, pinned_hex(REUSE));
    for request_id in 2..=18 {
        dispatch!(
            command_bytes(request_id, protocol::CommandPayload::CapabilitiesGet),
            StatusCode::Ok,
            capabilities_response(request_id)
        );
    }
    dispatch!(first, StatusCode::ReplayExpired, pinned_hex(EXPIRED));

    let edit = rebuild_edit(SESSION, 0x01);
    let stale = command_bytes_at_revision(
        19,
        ExpectedRevision::Exact(SessionRevision(41)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(stale, StatusCode::RevisionConflict, pinned_hex(STALE));

    for (request_id, state, expected) in [
        (20, protocol::TransportState::Stopped, TRANSPORT_20),
        (21, protocol::TransportState::Playing, TRANSPORT_21),
    ] {
        dispatch!(
            command_bytes_at_revision(
                request_id,
                ExpectedRevision::Exact(SessionRevision(42)),
                protocol::CommandPayload::TransportSet(protocol::TransportSetRequest { state },),
            ),
            StatusCode::Ok,
            pinned_hex(expected)
        );
    }
    let event_full = command_bytes_at_revision(
        22,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(event_full, StatusCode::Backpressure, pinned_hex(EVENT_FULL));
    for expected in [TRANSPORT_EVENT_20, TRANSPORT_EVENT_21] {
        let (result, bytes) = event_c(c_session, crate::EVENT_LANE_RELIABLE);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(bytes, pinned_hex(expected));
    }

    let first_structural = command_bytes_at_revision(
        23,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    );
    dispatch!(first_structural, StatusCode::Ok, pinned_hex(STRUCTURAL_23));
    let commit_event = event_c(c_session, crate::EVENT_LANE_RELIABLE);
    assert_eq!(commit_event.0, crate::RESULT_OK);
    assert_eq!(commit_event.1, pinned_hex(COMMIT_EVENT));

    // #1310: a second structural edit while the first candidate waits unadopted supersedes it
    // (it was publication backpressure before): it commits at once, the first candidate is
    // withdrawn and dropped, and its replacement is the one pending plan.
    let second_edit = rebuild_edit(SESSION, 0x02);
    let superseding = command_bytes_at_revision(
        24,
        ExpectedRevision::Exact(SessionRevision(43)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&second_edit)),
    );
    let before = crate::ffi::test_session_state_summary(c_session);
    let before_owners = crate::ffi::test_owner_counters(c_session);
    assert_eq!(before.0, 43);
    assert_eq!(before.3, 1);
    let committed = dispatch!(superseding.clone(), StatusCode::Ok, pinned_hex(RETRY_24));
    assert!(!committed.is_empty());
    let mut owners = before_owners;
    owners.token_constructed += 1;
    owners.token_disposed += 1;
    owners.replay_candidate_constructed += 1;
    owners.replay_candidate_published += 1;
    owners.replay_current_disposed += 1;
    owners.candidate_provider_constructed += 1;
    owners.candidate_provider_published += 1;
    owners.candidate_plan_constructed += 1;
    owners.candidate_plan_published += 1;
    owners.reservation_constructed += 1;
    owners.reservation_committed += 1;
    owners.candidate_withdrawn += 1;
    owners.candidate_superseded += 1;
    // The superseded candidate's plan and provider drop on the control thread.
    owners.candidate_plan_disposed += 1;
    owners.candidate_provider_disposed += 1;
    assert_eq!(crate::ffi::test_owner_counters(c_session), owners);
    let superseded = crate::ffi::test_session_state_summary(c_session);
    assert_eq!((superseded.0, superseded.2, superseded.3), (44, 0, 1));

    let mut c_pcm = [f32::NAN; 256];
    let output = crate::PlanarOutput {
        struct_size: crate::PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: c_pcm.as_mut_ptr(),
        sample_capacity: c_pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    assert_eq!(
        crate::ffi::test_render(c_plan, 0, &output),
        crate::RESULT_OK
    );
    assert!(c_pcm.iter().all(|sample| sample.to_bits() == 0));
    // The same request again is its replay, and the replacement was adopted: the running plan's
    // provider and plan retire.
    let replay = dispatch!(superseding, StatusCode::Ok, pinned_hex(RETRY_24));
    assert_eq!(replay, committed);
    owners.current_provider_disposed += 1;
    owners.current_plan_disposed += 1;
    assert_eq!(crate::ffi::test_owner_counters(c_session), owners);
    let after_adoption = crate::ffi::test_session_state_summary(c_session);
    assert_eq!(after_adoption.0, 44);
    assert_eq!(after_adoption.3, 0);

    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// The parity session stripped to a chain whose builtin tail is finite: no console slot, no
/// insert, and both input filters off. Only such a session tells a live input lane apart, because
/// the input lane makes a strip's builtin tail infinite (#1256 gate 1).
fn bare_parity_session(track_count: usize, sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&generated_parity_session(track_count, sample_rate_hz))
        .expect("accepted parity session");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
    }
    session::canonical_session_json(&model).expect("canonical bare session")
}

/// The ten-track parity session with mixed effect cohorts (#1263 verdict NIT 3): an enabled
/// compressor insert on the first and sixth tracks, and a bypassed delay and a bypassed multiband
/// compressor insert on the second. Only an enabled non-EQ effect tells a non-target lane seeded
/// bypassed apart; both bypasses ride lanes (#1339, #1340).
fn mixed_effect_parity_session(sample_rate_hz: u32) -> String {
    let mut model = parse_session_json(&generated_parity_session(10, sample_rate_hz))
        .expect("accepted parity session");
    let template = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
    let native = |instance: &str, effect: &str, bypass: bool| {
        let mut native = template.clone();
        native.id = session::StableId::parse(instance).expect("instance ID");
        native.identity = session::EffectIdentity::Native {
            effect_id: session::StableId::parse(effect).expect("effect ID"),
        };
        native.params.clear();
        native.bypass = bypass;
        native
    };
    for track in [0, 5] {
        model.tracks[track]
            .inserts
            .effects
            .push(native("comp", "miso.compressor", false));
    }
    model.tracks[1].inserts.effects.extend([
        native("dly", "miso.delay", true),
        native("mb", "miso.multiband-compressor", true),
    ]);
    session::canonical_session_json(&model).expect("canonical mixed-effect session")
}

/// The C ABI's lane selection less its effect lanes: the input rows are live (#1556), no route row
/// is.
const INPUT_LANE_ONLY: HostLiveLanes = HostLiveLanes {
    strip_input: true,
    effects: false,
    routes: false,
};

/// One C ABI plan against a lanes-free `host_core::prepare_host_runtime` plan of the same session
/// (#1256 gate 1), and against the C ABI's lane selection without its effect lanes (#1263 gate 1).
fn assert_live_lanes_render_like_lanes_free(document: &str, label: &str, expect_finite: bool) {
    let mut c = compile_children(document, limits()).unwrap_or_else(|failure| {
        panic!(
            "{label}: compile: {}",
            String::from_utf8_lossy(&failure.diagnostics)
        )
    });
    let caps = prepare_caps(limits());
    let compiled = host_core::compile_host_session(document, &caps)
        .unwrap_or_else(|_| panic!("{label}: host compile"));
    let host_core::PreparedHost {
        mut plan,
        mut sources,
        report: host,
        ..
    } = host_core::prepare_host_runtime(&compiled, &caps)
        .unwrap_or_else(|_| panic!("{label}: lanes-free host prepare"));

    // Same latency and tail as the lanes-free plan.
    let resources = c.plan.resources();
    let (tail_kind, tail_samples) = control_plane::tail_words(host.output_tail_every_peak);
    if expect_finite {
        assert_eq!(
            tail_kind, TAIL_FINITE,
            "{label}: the lanes-free tail is finite"
        );
    }
    assert_eq!(resources.latency_samples, host.latency_samples, "{label}");
    // Every strip carries the input lane (#1556), so the input section's tail is the live bound:
    // finite for every history of live values, and never below the prepared design's. A plan with
    // no infinite-tail effect stays finite; one with such an effect stays infinite.
    if tail_kind == TAIL_FINITE {
        assert_eq!(
            resources.tail_kind, TAIL_FINITE,
            "{label}: the live input bound is finite"
        );
        assert!(
            resources.tail_samples >= tail_samples,
            "{label}: the live bound {} covers the design's {tail_samples}",
            resources.tail_samples
        );
    } else {
        assert_eq!(
            resources.tail_kind,
            crate::TAIL_INFINITE,
            "{label}: an infinite-tail effect keeps the plan's tail infinite"
        );
    }

    // #1263 gate 1: attaching the effect lanes moves neither latency nor tail. The reference is
    // the C ABI's selection less its effect lanes, prepared at the same depth.
    let (without_effects, _) = host_core::prepare_host_runtime_with_live_lanes(
        &compiled,
        &caps,
        &HostLiveControlRequest::default(),
        // The C ABI's selection less its effect lanes: the input rows are live (#1556), no route row is.
        INPUT_LANE_ONLY,
        &host_core::LiveValuesSuperseded::default(),
    )
    .unwrap_or_else(|_| panic!("{label}: host prepare without effect lanes"));
    let without_effects = without_effects.report;
    assert_eq!(
        resources.latency_samples, without_effects.latency_samples,
        "{label}: latency without effect lanes"
    );
    assert_eq!(
        (resources.tail_kind, resources.tail_samples),
        control_plane::tail_words(without_effects.output_tail_every_peak),
        "{label}: tail without effect lanes"
    );

    // #1263 gate 1: one effect producer per prepared effect instance -- every strip's console
    // slots and inserts -- and an owner exactly on the parametric EQs.
    let normalized = compiled.normalized_model();
    let mut expected = normalized
        .strips()
        .flat_map(|strip| {
            let racks = normalized.lower_strip(&strip);
            let id = strip.id.as_str().to_owned();
            racks
                .in_chain_order()
                .into_iter()
                .flatten()
                .map(|effect| {
                    let native = match &effect.identity {
                        session::EffectIdentity::Native { effect_id } => effect_id.as_str(),
                        _ => panic!("{label}: a native effect"),
                    };
                    (
                        id.clone(),
                        effect.id.as_str().to_owned(),
                        native == "miso.parametric-eq",
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut kept = c
        .session
        .test_providers()
        .test_effects()
        .iter()
        .map(|producer| {
            (
                producer.track_id.to_string(),
                producer.effect_id.to_string(),
                producer.has_owner(),
            )
        })
        .collect::<Vec<_>>();
    expected.sort();
    kept.sort();
    assert_eq!(
        kept, expected,
        "{label}: one producer per effect instance, owners on the EQs"
    );

    // One producer per strip, in canonical strip order, each with an input lane (#1556).
    let model = compiled.normalized_model();
    let strips = c.session.test_providers();
    assert_eq!(strips.test_track_count(), model.tracks.len(), "{label}");
    assert_eq!(
        strips
            .test_strip_controls()
            .iter()
            .map(|producer| &*producer.track_id)
            .collect::<Vec<_>>(),
        model
            .strips()
            .map(|strip| strip.id.as_str())
            .collect::<Vec<_>>(),
        "{label}: one producer per strip, in canonical order"
    );
    assert!(
        strips
            .test_strip_controls()
            .iter()
            .all(|producer| producer.input.is_some()),
        "{label}: every strip carries an input lane"
    );

    // Eight blocks fed the same source chunks are bit-identical.
    let rate = compiled.sample_rate().0;
    let quantum = 128_usize;
    let mut first_left = vec![0.0_f32; quantum];
    let mut first_right = vec![0.0_f32; quantum];
    first_left[1] = 0.25;
    first_right[1] = -0.5;
    first_left[7] = -1.0;
    first_right[9] = 0.75;
    let final_left = vec![0.125_f32; 64];
    let final_right = vec![-0.25_f32; 64];
    let mut audible = false;
    for block in 0..8_u64 {
        let chunk = match block {
            0 => Some((0, &first_left, &first_right, false)),
            1 => Some((128, &final_left, &final_right, true)),
            _ => None,
        };
        if let Some((start_frame, left, right, end_of_region)) = chunk {
            let planes = [left.as_slice(), right.as_slice()];
            let submission = || SourceSubmission {
                generation: 1,
                start_frame,
                sample_rate_hz: rate,
                planes: &planes,
                frames: left.len() as u32,
                end_of_region,
            };
            c.session
                .submit(b"fixture-source", submission())
                .unwrap_or_else(|_| panic!("{label}: C submit"));
            sources
                .submit(b"fixture-source", submission())
                .unwrap_or_else(|_| panic!("{label}: host submit"));
        }
        let mut c_pcm = vec![f32::NAN; quantum * 2];
        c.plan
            .render(
                block * quantum as u64,
                PlanarBufferMut::try_new(&mut c_pcm, 2, quantum, quantum).expect("C output"),
            )
            .unwrap_or_else(|code| panic!("{label}: C render {code:?}"));
        let mut host_pcm = vec![f32::NAN; quantum * 2];
        plan.render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut host_pcm, 2, quantum, quantum)
                    .expect("host output"),
            },
            block * quantum as u64,
        )
        .unwrap_or_else(|_| panic!("{label}: host render"));
        assert_eq!(
            c_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            host_pcm
                .iter()
                .map(|sample| sample.to_bits())
                .collect::<Vec<_>>(),
            "{label}: block {block}"
        );
        audible |= c_pcm.iter().any(|sample| *sample != 0.0);
    }
    assert!(audible, "{label}: the compared blocks carry signal");
}

#[test]
fn c_abi_plans_with_live_lanes_render_like_lanes_free_plans() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        // #1263 gate 1: the nine-track EQ fixture itself, at the helper's 192-frame source.
        let mut nine = parse_session_json(SESSION).expect("accepted nine-track fixture");
        nine.sample_rate_hz = sample_rate_hz;
        nine.sources[0].frames = 192;
        assert_live_lanes_render_like_lanes_free(
            &session::canonical_session_json(&nine).expect("canonical nine-track session"),
            &format!("nine-track EQ fixture at {sample_rate_hz} Hz"),
            false,
        );
        for track_count in [1, 10] {
            assert_live_lanes_render_like_lanes_free(
                &generated_parity_session(track_count, sample_rate_hz),
                &format!("parity {track_count} tracks at {sample_rate_hz} Hz"),
                false,
            );
            assert_live_lanes_render_like_lanes_free(
                &bare_parity_session(track_count, sample_rate_hz),
                &format!("bare {track_count} tracks at {sample_rate_hz} Hz"),
                true,
            );
        }
        assert_live_lanes_render_like_lanes_free(
            &mixed_effect_parity_session(sample_rate_hz),
            &format!("mixed effects 10 tracks at {sample_rate_hz} Hz"),
            false,
        );
    }
}

#[test]
fn direct_and_c_render_match_one_and_ten_tracks_across_launch_rates() {
    for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
        render_parity_shape(1, sample_rate_hz);
        render_parity_shape(10, sample_rate_hz);
    }
}

/// How long one side of the lockstep below waits for the other before it fails. Each step takes
/// one block's submit or render, microseconds in a debug build.
const LOCKSTEP_DEADLINE: std::time::Duration = std::time::Duration::from_secs(10);

/// Waits until `progress` reaches `target`, for the lockstep below.
///
/// # Panics
///
/// When the peer has ended (normally or by a panic: its `StopOnDrop` set `peer_ended`) short of
/// `target`, or after [`LOCKSTEP_DEADLINE`]; the message names `what` this side waited for and the
/// block, `target - 1` (#1251). The message is formatted only on failure, so the wait allocates
/// nothing on its passing path.
fn await_lockstep(
    progress: &std::sync::atomic::AtomicU64,
    target: u64,
    peer_ended: &std::sync::atomic::AtomicBool,
    what: &'static str,
) {
    use std::sync::atomic::Ordering;
    let deadline = std::time::Instant::now() + LOCKSTEP_DEADLINE;
    loop {
        // Read the peer's end before its progress: a peer that ended published its last
        // progress first, so an ended peer short of `target` will never reach it.
        let ended = peer_ended.load(Ordering::Acquire);
        if progress.load(Ordering::Acquire) >= target {
            return;
        }
        let block = target - 1;
        assert!(
            !ended,
            "lockstep: the peer ended before {what} block {block}"
        );
        assert!(
            std::time::Instant::now() < deadline,
            "lockstep: {what} block {block} did not happen within {LOCKSTEP_DEADLINE:?}"
        );
        std::thread::yield_now();
    }
}

#[test]
fn barrier_schedule_separates_one_source_producer_from_exclusive_render() {
    let mut model =
        parse_session_json(&generated_parity_session(1, 48_000)).expect("concurrency session");
    model.sources[0].frames = 1_024;
    let session = session::canonical_session_json(&model).expect("canonical");
    let children = compile_children(&session, limits()).expect("concurrent children");
    let session = Box::into_raw(Box::new(crate::Session::new(
        children.session,
        children.session_error,
    ))) as usize;
    let plan = Box::into_raw(Box::new(crate::Plan::new(children.plan))) as usize;
    // A lockstep of two counters in place of two barriers: the producer submits block `b` and
    // waits until the renderer has consumed it; the renderer waits until block `b` is submitted,
    // renders it and publishes it consumed. Each side's `StopOnDrop` marks it ended however it
    // leaves its loop, so a failed assertion on one side fails the other's wait instead of
    // leaving it blocked at a barrier (#1251).
    let submitted = std::sync::atomic::AtomicU64::new(0);
    let consumed = std::sync::atomic::AtomicU64::new(0);
    let producer_ended = std::sync::atomic::AtomicBool::new(false);
    let render_ended = std::sync::atomic::AtomicBool::new(false);
    std::thread::scope(|scope| {
        let (submitted, consumed) = (&submitted, &consumed);
        let (producer_ended, render_ended) = (&producer_ended, &render_ended);
        scope.spawn(move || {
            let _ended = bench_support::producer::StopOnDrop(producer_ended);
            let session = session as *mut crate::Session;
            let left = [0.25_f32; 128];
            let right = [-0.5_f32; 128];
            for block in 0..6_u64 {
                let (generation, start_frame) = if block < 3 {
                    (1, block * 128)
                } else {
                    if block == 3 {
                        assert_eq!(
                            crate::ffi::test_source_seek(session, b"fixture-source", 2, 512,),
                            crate::RESULT_OK
                        );
                    }
                    (2, 512 + (block - 3) * 128)
                };
                submit_c(
                    session,
                    generation,
                    start_frame,
                    48_000,
                    &left,
                    &right,
                    false,
                );
                submitted.store(block + 1, std::sync::atomic::Ordering::Release);
                await_lockstep(consumed, block + 1, render_ended, "the renderer consumed");
            }
        });
        scope.spawn(move || {
            let _ended = bench_support::producer::StopOnDrop(render_ended);
            let plan = plan as *mut crate::Plan;
            let mut observed_signal = false;
            for block in 0..6_u64 {
                await_lockstep(
                    submitted,
                    block + 1,
                    producer_ended,
                    "the producer submitted",
                );
                let mut pcm = [f32::NAN; 256];
                let output = crate::PlanarOutput {
                    struct_size: crate::PLANAR_OUTPUT_SIZE,
                    channels: 2,
                    samples: pcm.as_mut_ptr(),
                    sample_capacity: pcm.len() as u64,
                    frames: 128,
                    plane_stride_samples: 128,
                    reserved: [0; 2],
                };
                assert_eq!(
                    crate::ffi::test_render(plan, block * 128, &output),
                    crate::RESULT_OK
                );
                assert!(pcm.iter().all(|sample| sample.is_finite()));
                observed_signal |= pcm.iter().any(|sample| *sample != 0.0);
                consumed.store(block + 1, std::sync::atomic::Ordering::Release);
            }
            assert!(observed_signal);
        });
    });
    crate::ffi::test_session_destroy(session as *mut crate::Session);
    crate::ffi::test_plan_destroy(plan as *mut crate::Plan);
}

/// Issue #146 at the C ABI: the render entry pins the environment and hands the caller's back.
///
/// A native host's audio thread may run with FTZ and DAZ set (see `crates/lane/src/fpenv.rs`), so
/// this is the real exposure. Three arms render one subnormal fade tail through one session:
///
/// * the C entry with the caller's FTZ/DAZ clear -- the reference bytes;
/// * the C entry with the caller's FTZ+DAZ set -- must be the same bytes;
/// * the plan rendered *behind* the entry with FTZ+DAZ set -- must differ, or the second arm is
///   vacuous.
///
/// Red mutation (recorded in `crates/host-core/tests/MUTATIONS.md`, M-146-2): delete the
/// `CanonicalFpEnv::enter()` line from `miso_engine_v1_render_f32_planar`; the second arm collapses
/// onto the third.
#[cfg(target_arch = "x86_64")]
#[allow(unsafe_code)]
mod fp_environment {
    use super::*;
    use lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};

    const QUANTUM: usize = 128;
    const BLOCKS: u64 = 12;
    /// DAZ, the six exception masks, the rounding-control field and FTZ. The low six bits are
    /// sticky status flags that ordinary arithmetic sets and no restore is expected to hold back.
    const MXCSR_CONTROL_BITS: u32 = 0xFFC0;

    struct Restore(u32);

    impl Drop for Restore {
        fn drop(&mut self) {
            // SAFETY: `self.0` is the word the test read from this thread before it changed it, so
            // it sets no reserved bit, and writing it hands the thread its own word back.
            unsafe { write_mxcsr(self.0) };
        }
    }

    /// One block of a fade tail: every sample is a distinct `f32` subnormal, which is exactly what
    /// DAZ reads as zero and FTZ writes as zero.
    fn tail_block(block: u64) -> (Vec<f32>, Vec<f32>) {
        let mut left = vec![0.0_f32; QUANTUM];
        let mut right = vec![0.0_f32; QUANTUM];
        for (frame, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let step = (block * QUANTUM as u64 + frame as u64) as u32;
            *left = f32::from_bits(0x0004_0000_u32.wrapping_add(step.wrapping_mul(1_031)));
            *right = -f32::from_bits(0x0002_0000_u32.wrapping_add(step.wrapping_mul(613)));
        }
        (left, right)
    }

    /// Renders `BLOCKS` quanta through the C ABI render entry.
    fn render_through_the_c_entry(caller: u32) -> (Vec<u32>, u32) {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let children = compile_children(SESSION, limits()).expect("C children");
        let c_session = Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        )));
        let c_plan = Box::into_raw(Box::new(crate::Plan::new(children.plan)));

        // SAFETY: every caller of this function passes a word read from this thread with only
        // FTZ (bit 15) and DAZ (bit 6) set or cleared, so no reserved bit. The test measures the
        // render under it, and the write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(caller) };
        let mut rendered = Vec::with_capacity(BLOCKS as usize * QUANTUM * 2);
        let mut leaked = 0;
        for block in 0..BLOCKS {
            let (left, right) = tail_block(block);
            submit_c(
                c_session,
                1,
                block * QUANTUM as u64,
                48_000,
                &left,
                &right,
                false,
            );
            let mut pcm = vec![f32::NAN; QUANTUM * 2];
            let output = crate::PlanarOutput {
                struct_size: crate::PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: QUANTUM as u32,
                plane_stride_samples: QUANTUM as u32,
                reserved: [0; 2],
            };
            assert_eq!(
                crate::ffi::test_render(c_plan, block * QUANTUM as u64, &output),
                crate::RESULT_OK
            );
            // E2, read the instant the entry returns: every bit of the caller's word, status flags
            // included, because the guard's restore is unconditional.
            leaked |= read_mxcsr() ^ caller;
            rendered.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
        (rendered, leaked)
    }

    /// Renders `BLOCKS` quanta straight into the plan state, behind the C entry and its guard.
    fn render_behind_the_c_entry(caller: u32) -> Vec<u32> {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let mut children = compile_children(SESSION, limits()).expect("direct children");

        // SAFETY: every caller of this function passes a word read from this thread with only
        // FTZ (bit 15) and DAZ (bit 6) set or cleared, so no reserved bit. The test measures the
        // render under it, and the write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(caller) };
        let mut rendered = Vec::with_capacity(BLOCKS as usize * QUANTUM * 2);
        for block in 0..BLOCKS {
            let (left, right) = tail_block(block);
            children
                .session
                .submit(
                    b"fixture-source",
                    SourceSubmission {
                        generation: 1,
                        start_frame: block * QUANTUM as u64,
                        sample_rate_hz: 48_000,
                        planes: &[&left, &right],
                        frames: QUANTUM as u32,
                        end_of_region: false,
                    },
                )
                .expect("direct submit");
            let mut pcm = vec![f32::NAN; QUANTUM * 2];
            children
                .plan
                .render(
                    block * QUANTUM as u64,
                    PlanarBufferMut::try_new(&mut pcm, 2, QUANTUM, QUANTUM).expect("direct output"),
                )
                .expect("direct render");
            rendered.extend(pcm.iter().map(|sample| sample.to_bits()));
        }
        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        rendered
    }

    /// The refusal has its own frozen diagnostic, so a host that hits it is told which rule it
    /// broke rather than reading the catch-all.
    #[test]
    fn the_fp_environment_refusal_names_its_own_check() {
        assert_eq!(
            plan_error::text(plan_error::FP_ENVIRONMENT),
            b"render.fp_environment.invalid"
        );
        assert_ne!(
            plan_error::text(plan_error::FP_ENVIRONMENT),
            plan_error::text(u32::MAX),
            "the refusal must not fall through to render.internal"
        );
    }

    #[test]
    fn the_c_render_entry_is_canonical_under_a_caller_that_set_ftz_and_daz() {
        let saved = read_mxcsr();
        let clear = saved & !(MXCSR_FTZ | MXCSR_DAZ);
        let hostile = clear | MXCSR_FTZ | MXCSR_DAZ;

        let (reference, reference_leak) = render_through_the_c_entry(clear);
        let (guarded, guarded_leak) = render_through_the_c_entry(hostile);
        let behind = render_behind_the_c_entry(hostile);

        assert_eq!(
            reference_leak & MXCSR_CONTROL_BITS,
            0,
            "the entry must restore every control bit"
        );
        assert_eq!(
            guarded_leak, 0,
            "the entry must restore the caller's exact word, status flags included"
        );
        assert_ne!(
            behind, reference,
            "issue #146 is vacuous here: FTZ+DAZ did not move this fixture behind the entry"
        );
        assert_eq!(
            guarded, reference,
            "a caller's FTZ+DAZ reached a render through the C ABI entry"
        );
        assert_eq!(read_mxcsr(), saved, "the test leaked MXCSR state");
    }

    #[test]
    fn a_rejected_c_render_also_restores_the_callers_word() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        // Round-toward-zero and a sticky precision flag alongside FTZ+DAZ: nothing an entry may
        // keep, and nothing it may drop.
        let hostile = (saved & !0x6000) | MXCSR_FTZ | MXCSR_DAZ | 0x6000 | 0x0020;

        let children = compile_children(SESSION, limits()).expect("C children");
        let c_session = Box::into_raw(Box::new(crate::Session::new(
            children.session,
            children.session_error,
        )));
        let c_plan = Box::into_raw(Box::new(crate::Plan::new(children.plan)));

        let mut pcm = vec![f32::NAN; QUANTUM * 2];
        // SAFETY: `hostile` sets FTZ, DAZ, RC and a status flag on a word read from this thread,
        // all below bit 16. The test measures the C entry's rejections under it, and the
        // write-back below (and `_restore`) restores `saved`.
        unsafe { write_mxcsr(hostile) };

        // A malformed descriptor: refused before the plan is touched at all.
        let malformed = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 3,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM as u32,
            plane_stride_samples: QUANTUM as u32,
            reserved: [0; 2],
        };
        assert_eq!(
            crate::ffi::test_render(c_plan, 0, &malformed),
            crate::RESULT_INVALID_ARGUMENT
        );
        assert_eq!(read_mxcsr(), hostile, "a refused descriptor leaked MXCSR");

        // A null plan handle: the earliest return the entry has.
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM as u32,
            plane_stride_samples: QUANTUM as u32,
            reserved: [0; 2],
        };
        assert_ne!(
            crate::ffi::test_render(core::ptr::null_mut(), 0, &output),
            crate::RESULT_OK
        );
        assert_eq!(read_mxcsr(), hostile, "a null handle leaked MXCSR");

        // A discontinuous absolute sample: rejected by the plan, after the guard is installed.
        assert_eq!(
            crate::ffi::test_render(c_plan, 7, &output),
            crate::RESULT_RENDER_REJECTED
        );
        assert_eq!(read_mxcsr(), hostile, "a rejected render leaked MXCSR");

        // SAFETY: `saved` was read from this thread at the top of this function, so it sets no
        // reserved bit; this hands the thread its own word back.
        unsafe { write_mxcsr(saved) };
        crate::ffi::test_plan_destroy(c_plan);
        crate::ffi::test_session_destroy(c_session);
    }
}

/// A `SESSION_TRANSACTION_APPLY` at `revision` that rebuilds the plan ([`rebuild_edit`] with
/// `tag`).
fn rebuild_command(request_id: u64, revision: u64, tag: u8) -> Vec<u8> {
    let edit = rebuild_edit(SESSION, tag);
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(revision)),
        protocol::CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
    )
}

/// The meter window of the 48 kHz, 128-frame fixture, in blocks (#1581 D2): 12.
fn fixture_window_blocks() -> u64 {
    u64::from(host_core::default_meter_window_blocks(48_000, 128).get())
}

/// The output's peak, the one meter entry the C ABI serves before #1587 (#1583 D3).
fn output_peak_entry() -> protocol::MeterEntry {
    protocol::MeterEntry {
        target: protocol::MeterTarget::Output,
        metric: protocol::MeterMetric::Peak,
    }
}

/// Telemetry that stages one meter batch of the output's peak per meter window (one batch of
/// `windows` windows each), configured at the fixture's initial revision.
fn meter_telemetry(request_id: u64, windows: u64) -> Vec<u8> {
    let configuration = protocol::TelemetryConfiguration {
        meters: vec![output_peak_entry()],
        meter_period_blocks: u32::try_from(windows * fixture_window_blocks()).expect("period"),
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    )
}

/// One 128-frame block of silence for `fixture-source` through the exported submit entry point.
fn submit_silence_c(session: *mut crate::Session, generation: u64, start_frame: u64) {
    let silence = [0.0_f32; 128];
    submit_c(
        session,
        generation,
        start_frame,
        48_000,
        &silence,
        &silence,
        false,
    );
}

/// Issue #1348 gate 1: after a committed rebuild and its swap block, `miso_engine_v1_service`
/// alone disposes of the retired plan and promotes the successor's provider, and a second rebuild
/// then reserves.
///
/// Test value: red if `service` does not reclaim or does not promote; no other call does only this
/// step, and every existing reclaim test reaches it through a command or a dequeue.
#[test]
fn service_alone_reclaims_the_retired_plan_and_promotes_its_successor() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        command_c(c_session, &rebuild_command(1, 42, 0x01)).0,
        crate::RESULT_OK
    );
    render_c(c_plan, 0);
    let before = crate::ffi::test_lifecycle_counters();
    // (revision, replay entries, current provider epoch, pending providers)
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 0, 1)
    );

    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let after = crate::ffi::test_lifecycle_counters();
    assert_eq!(
        after.current_plan_disposed,
        before.current_plan_disposed + 1,
        "service disposes of the retired plan"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 1, 0),
        "service promotes the adopted successor's provider"
    );
    assert!(crate::ffi::test_last_error(c_session).is_empty());

    assert_eq!(
        command_c(c_session, &rebuild_command(2, 43, 0x02)).0,
        crate::RESULT_OK,
        "the reclaimed credit lets the next rebuild reserve"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (44, 2, 1, 1)
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 2: a source submission runs the whole service step first: with the output's
/// meter configured, it disposes of the retired plan and stages the meter batch of the window the
/// swap block began.
///
/// Test value: red if a control call skips the telemetry half of the step, as the source submit
/// did before #1348 (only `command` and `dequeue_event` staged render telemetry).
#[test]
fn a_source_submit_services_the_session_first() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        command_c(c_session, &meter_telemetry(1, 1)).0,
        crate::RESULT_OK
    );
    assert_eq!(
        command_c(c_session, &rebuild_command(2, 42, 0x01)).0,
        crate::RESULT_OK
    );
    // The swap block and the rest of the first meter window.
    for block in 0..fixture_window_blocks() {
        render_c(c_plan, block);
    }
    let before = crate::ffi::test_lifecycle_counters();
    let telemetry_before = crate::ffi::test_transaction_snapshot(c_session).telemetry;

    // The rebuild changed the source's content, so it restarts at generation 1, frame 0.
    submit_silence_c(c_session, 1, 0);
    let after = crate::ffi::test_lifecycle_counters();
    assert_eq!(
        after.current_plan_disposed,
        before.current_plan_disposed + 1,
        "the submit's service step disposes of the retired plan"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 2, 1, 0)
    );
    let telemetry_after = crate::ffi::test_transaction_snapshot(c_session).telemetry;
    assert_eq!(
        telemetry_after.occupancy,
        telemetry_before.occupancy + 2,
        "the submit's service step stages the window's left and right records"
    );
    let (result, event) = event_c(c_session, crate::EVENT_LANE_LOSSY);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    assert!(matches!(
        ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("lossy event"),
        protocol::DecodedTypedEventFrame {
            payload: protocol::DecodedEventPayload::MeterBatch(_),
            ..
        }
    ));
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 3: the provider's telemetry counters are refreshed by every control call's
/// service step, after its staging. Drops staged by a source submission are in the provider's
/// counter snapshot as soon as the submit returns, equal to the controller's own count, and
/// `COUNTERS_GET` reports them.
///
/// Test value: red if a control call other than `command` leaves the provider's counters stale
/// (the refresh kept in `command` only), or if the step refreshes only before it stages, either of
/// which #1312 and #1351's snapshot readers would then report wrongly.
#[test]
fn every_control_call_refreshes_the_provider_counters() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    // Three counter records per observed render overflow the lossy lane's two slots.
    assert_eq!(
        command_c(
            c_session,
            &counter_telemetry(
                1,
                vec![
                    protocol::CounterId::TelemetryCoalesced,
                    protocol::CounterId::TelemetryDropped,
                    protocol::CounterId::LiveValuesSuperseded,
                ],
                1,
                false
            )
        )
        .0,
        crate::RESULT_OK
    );
    let provider_dropped = |session: *mut crate::Session| {
        crate::ffi::test_provider_counters(session)
            .iter()
            .find(|value| value.id == protocol::CounterId::TelemetryDropped)
            .expect("telemetry-dropped counter")
            .value
    };
    for block in 0..4 {
        render_c(c_plan, block);
        submit_silence_c(c_session, 1, block * 128);
        let controller = crate::ffi::test_telemetry_counters(c_session);
        assert_eq!(
            provider_dropped(c_session),
            controller.telemetry_dropped,
            "block {block}: the submit's own service step counted its drops"
        );
    }
    let dropped = crate::ffi::test_telemetry_counters(c_session).telemetry_dropped;
    assert!(dropped > 0, "the telemetry lane dropped records");

    let request = command_bytes(
        2,
        protocol::CommandPayload::CountersGet(&protocol::CountersRequest {
            all: false,
            ids: vec![protocol::CounterId::TelemetryDropped as u32],
        }),
    );
    let (result, response) = command_c(c_session, &request);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    let protocol::DecodedTypedResponseFrame::Success {
        payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
        ..
    } = ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("counters response")
    else {
        panic!("expected a counter snapshot")
    };
    assert_eq!(
        snapshot.values,
        vec![protocol::CounterValue {
            id: protocol::CounterId::TelemetryDropped,
            value: dropped,
        }]
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// A `TELEMETRY_CONFIGURE` command for the counter snapshot alone, at `revision`.
pub(super) fn counter_telemetry_for(
    request_id: u64,
    revision: SessionRevision,
    counter_ids: Vec<protocol::CounterId>,
    counter_period_blocks: u32,
) -> Vec<u8> {
    let configuration = protocol::TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
        counter_ids,
        counter_period_blocks,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(revision),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    )
}

/// A `TELEMETRY_CONFIGURE` command for the counter snapshot (and, when `meters` is set, the output's
/// meter batch, one per window), at the fixture's revision 42.
fn counter_telemetry(
    request_id: u64,
    counter_ids: Vec<protocol::CounterId>,
    counter_period_blocks: u32,
    meters: bool,
) -> Vec<u8> {
    let configuration = protocol::TelemetryConfiguration {
        meter_period_blocks: if meters {
            u32::try_from(fixture_window_blocks()).expect("period")
        } else {
            0
        },
        meters: if meters {
            vec![output_peak_entry()]
        } else {
            Vec::new()
        },
        meter_window_blocks: None,
        counter_ids,
        counter_period_blocks,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    )
}

/// The status of one command response.
fn response_status(response: &[u8]) -> StatusCode {
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("typed response")
    {
        protocol::DecodedTypedResponseFrame::Success { header, .. }
        | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => header.status,
    }
}

/// `COUNTERS_GET` for `ids`, through the C command path, as `(id, value)` in the order asked; the
/// status when the provider refuses.
fn counters_get_c(
    session: *mut crate::Session,
    request_id: u64,
    ids: &[protocol::CounterId],
) -> Result<Vec<protocol::CounterValue>, StatusCode> {
    let request = command_bytes(
        request_id,
        protocol::CommandPayload::CountersGet(&protocol::CountersRequest {
            all: ids.is_empty(),
            ids: ids.iter().map(|&id| id as u32).collect(),
        }),
    );
    let (result, response) = command_c(session, &request);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("counters response")
    {
        protocol::DecodedTypedResponseFrame::Success {
            payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
            ..
        } => Ok(snapshot.values),
        protocol::DecodedTypedResponseFrame::NonOk { header, .. } => Err(header.status),
        protocol::DecodedTypedResponseFrame::Success { .. } => panic!("expected counters"),
    }
}

/// Every counter snapshot waiting on the lossy lane, oldest first; every other lossy event is
/// dropped from the result.
pub(super) fn lossy_counter_snapshots(
    session: *mut crate::Session,
) -> Vec<protocol::CounterSnapshot> {
    let mut snapshots = Vec::new();
    loop {
        let (result, event) = event_c(session, crate::EVENT_LANE_LOSSY);
        assert_eq!(result, crate::RESULT_OK);
        if event.is_empty() {
            return snapshots;
        }
        let mut fields = [0_u16; 64];
        if let protocol::DecodedTypedEventFrame {
            payload: protocol::DecodedEventPayload::CounterSnapshot(snapshot),
            ..
        } = ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("lossy event")
        {
            snapshots.push(snapshot);
        }
    }
}

/// Issue #1351 gate 1: every staged counter snapshot carries the configured counters' own values,
/// the ones `COUNTERS_GET` returns, never the render-call count, whichever control call stages
/// it; one call is a `source_submit`.
///
/// A snapshot reads the provider after the step's first refresh, so it equals `COUNTERS_GET` as
/// the previous call left it (counters change only inside a service step), which is the value the
/// test reads before each block.
///
/// Test value: red if the snapshot is filled with anything but the counter's own value (the
/// render-call count it carried), or if only `command` refreshes the provider so that a submit
/// stages a stale value.
#[test]
fn a_counter_snapshot_carries_each_configured_counters_own_value() {
    let ids = [
        protocol::CounterId::TelemetryCoalesced,
        protocol::CounterId::TelemetryDropped,
        protocol::CounterId::LiveValuesSuperseded,
    ];
    let (c_session, c_plan) = boxed_c_children(SESSION);
    // A snapshot of three counters per observed render overflows the lossy lane's two slots.
    assert_eq!(
        command_c(c_session, &counter_telemetry(1, ids.to_vec(), 1, false)).0,
        crate::RESULT_OK
    );
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let mut request = 1;
    let mut expected = Vec::new();
    let mut staged = Vec::new();
    for block in 0..8_u64 {
        request += 1;
        expected.push(counters_get_c(c_session, request, &ids).expect("served counters"));
        render_c(c_plan, block);
        // The control call that stages the block's snapshot is a `source_submit`.
        submit_silence_c(c_session, 1, block * 128);
        let snapshots = lossy_counter_snapshots(c_session);
        assert!(
            snapshots.len() <= 1,
            "block {block}: one snapshot per block"
        );
        staged.push((block, snapshots.into_iter().next()));
    }
    let counters = crate::ffi::test_telemetry_counters(c_session);
    assert!(
        counters.telemetry_coalesced + counters.telemetry_dropped > 0,
        "the lossy lane coalesced or dropped a record"
    );
    // The lossy lane may keep only part of a snapshot, so each kept value is compared by ID.
    let mut differs_from_render_calls = false;
    let mut kept = 0;
    for (block, snapshot) in staged {
        for value in snapshot.into_iter().flat_map(|snapshot| snapshot.values) {
            kept += 1;
            let own = expected[block as usize]
                .iter()
                .find(|own| own.id == value.id)
                .expect("a configured counter");
            assert_eq!(
                value, *own,
                "block {block}: the snapshot value is COUNTERS_GET's"
            );
            differs_from_render_calls |= value.value != block + 1;
        }
    }
    assert!(kept >= 4, "the lane kept values to compare: {kept}");
    assert!(differs_from_render_calls);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// One `TELEMETRY_CONFIGURE` of `meters` at `period` blocks (no counter, no diagnostic), at
/// revision 42, through the C command path: the status and, when refused, the one diagnostic's
/// code and path.
fn configure_meters_c(
    session: *mut crate::Session,
    request_id: u64,
    meters: Vec<protocol::MeterEntry>,
    period: u64,
) -> (StatusCode, Option<(String, Vec<protocol::PathSegment>)>) {
    let configuration = protocol::TelemetryConfiguration {
        meters,
        meter_period_blocks: u32::try_from(period).expect("period"),
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let request = command_bytes_at_revision(
        request_id,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::TelemetryConfigure(&configuration),
    );
    let (result, response) = command_c(session, &request);
    assert_eq!(result, crate::RESULT_OK);
    let mut fields = [0_u16; 64];
    match ProtocolCodec::default()
        .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
        .expect("telemetry response")
    {
        protocol::DecodedTypedResponseFrame::Success { header, .. } => (header.status, None),
        protocol::DecodedTypedResponseFrame::NonOk { header, payload } => {
            let [diagnostic] = payload.diagnostics.as_slice() else {
                panic!("one diagnostic: {payload:?}")
            };
            (
                header.status,
                Some((diagnostic.code.clone(), diagnostic.path.clone())),
            )
        }
    }
}

/// One dequeued `METER_BATCH`: `(first_sample, end_sample, window_sequence, records)`.
type MeterBatchRow = (u64, u64, u64, Vec<protocol::MeterRecord>);

/// Every `METER_BATCH` on the lossy lane, in order; other lossy events are dropped.
fn meter_batches_c(session: *mut crate::Session) -> Vec<MeterBatchRow> {
    let mut batches = Vec::new();
    loop {
        let (result, event) = event_c(session, crate::EVENT_LANE_LOSSY);
        assert_eq!(result, crate::RESULT_OK);
        if event.is_empty() {
            return batches;
        }
        let mut fields = [0_u16; 64];
        if let protocol::DecodedTypedEventFrame {
            payload: protocol::DecodedEventPayload::MeterBatch(batch),
            ..
        } = ProtocolCodec::default()
            .decode_typed_event(&event, &mut DecodeScratch::new(&mut fields))
            .expect("lossy event")
        {
            batches.push((
                batch.first_sample.0,
                batch.observed_sample.0,
                batch.window_sequence,
                (0..usize::from(batch.count))
                    .map(|index| batch.record(index).expect("meter record"))
                    .collect(),
            ));
        }
    }
}

/// The `METER_WINDOWS_LOST` counter, through `COUNTERS_GET`.
fn meter_windows_lost_c(session: *mut crate::Session, request_id: u64) -> u64 {
    counters_get_c(
        session,
        request_id,
        &[protocol::CounterId::MeterWindowsLost],
    )
    .expect("served counter")[0]
        .value
}

/// #1583 gate 2: each refused meter table is typed (status, code, path) and changes nothing: no
/// batch is staged, and the configuration accepted before keeps its fold, so its first batch
/// still spans from sample 0 and no window is counted lost.
///
/// Test value: red if a refused configure is half stored (the controller stores the table, or the
/// level or the fold restarts on a refusal, which discards the open batch) or answers untyped.
#[test]
fn a_refused_meter_table_is_typed_and_changes_nothing() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let window = fixture_window_blocks();
    let strip = |id: &str, metric| protocol::MeterEntry {
        target: protocol::MeterTarget::Strip(id.into()),
        metric,
    };
    assert_eq!(
        configure_meters_c(c_session, 1, vec![output_peak_entry()], 2 * window),
        (StatusCode::Ok, None)
    );
    // One of the batch's two windows is folded before the refusals.
    for block in 0..window {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let path = |index| {
        vec![
            protocol::PathSegment::Field("meters".to_owned()),
            protocol::PathSegment::Index(index),
        ]
    };
    let refusals = [
        (
            vec![
                output_peak_entry(),
                strip("nobody", protocol::MeterMetric::Peak),
            ],
            2 * window,
            StatusCode::NotFound,
            "meter.strip.unknown",
            path(1),
        ),
        (
            vec![output_peak_entry()],
            5,
            StatusCode::InvalidField,
            "meter.period",
            vec![protocol::PathSegment::Field(
                "meter_period_blocks".to_owned(),
            )],
        ),
    ];
    for (request, (meters, period, status, code, at)) in (2..).zip(refusals) {
        assert_eq!(
            configure_meters_c(c_session, request, meters, period),
            (status, Some((code.to_owned(), at))),
            "{code}"
        );
        assert!(meter_batches_c(c_session).is_empty(), "{code}: no batch");
    }
    for block in window..2 * window {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let batches = meter_batches_c(c_session);
    assert_eq!(
        batches
            .iter()
            .map(|(first, end, sequence, _)| (*first, *end, *sequence))
            .collect::<Vec<_>>(),
        [(0, 2 * window * 128, 1)],
        "the accepted table's first batch, whole"
    );
    assert_eq!(meter_windows_lost_c(c_session, 100), 0);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Tracks `eq0` and `eq1` on the fixture source and submix `bus`: `eq0` at unity into the output,
/// `eq1` with a 0.5 matrix and a post-fader send (route `eq1-main`) into `bus`, and `bus` with a
/// 0.25 matrix into the output. Every gain is a power of two, so each post-matrix peak is exact:
/// `eq0` is the source's, `eq1` half of it, `bus` a quarter, and the output one and a quarter.
fn strip_meter_session() -> session::SessionModel {
    let matrix = |gain: f32| session::MatrixOrPan::Matrix {
        ll: gain,
        lr: 0.0,
        rl: 0.0,
        rr: gain,
        smoothing_samples: 16,
    };
    let mut model = stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    model.tracks[0].matrix_or_pan = matrix(1.0);
    let bus = session::StableId::parse("bus").expect("bus ID");
    let mut b = model.tracks[0].clone();
    b.id = session::StableId::parse("eq1").expect("track ID");
    b.matrix_or_pan = matrix(0.5);
    let mut send = model.routes[0].clone();
    send.id = session::StableId::parse("eq1-main").expect("route ID");
    send.source = session::RouteSource::Track {
        track_id: b.id.clone(),
        tap: session::SendTap::PostFader,
    };
    send.destination = session::RouteDestination::SubmixInput {
        submix_id: bus.clone(),
    };
    let mut out = model.routes[0].clone();
    out.id = session::StableId::parse("bus-main").expect("route ID");
    out.source = session::RouteSource::Submix {
        submix_id: bus.clone(),
        tap: session::SendTap::PostPan,
    };
    let mut submix = session::Submix::unity(bus, &model.console);
    submix.matrix_or_pan = matrix(0.25);
    model.tracks.push(b);
    model.routes.extend([send, out]);
    model.submixes.push(submix);
    model
}

/// #1583 gates 3 and 4 and #1587a gates 1, 3 and 4: with the table `[{eq0, peak}, {eq1, peak},
/// {bus, peak}, {output, peak}]` at twice the window, each batch is two windows: its span is the
/// first window's start to the second's end, its sequence counts from 1, and each entry's left and
/// right records are its own strip's post-matrix maxima of each plane over the batch, `VALID`, with
/// `CLIPPED` above 1.0 (`strip_meter_session`'s exact gains are the offline reference). Across
/// three rebuilds, the batches stay consecutive and nothing is lost: a muted track that sorts first,
/// adopted in the middle of a batch, shifts every strip's position but no handle; while `eq1` is removed its records have flags 0
/// (and `bus`, fed by nothing, is a valid 0); when `eq1` returns it is `VALID` again under handle 2.
///
/// Test value: red on the defect #1583 replaced (one combined block peak per handle), if a record
/// carries another strip's or the output's peak (a strip resolved by position, or a frame folded
/// against the retired epoch's strips after a rebuild), if a plane lands on the other plane, if the
/// fold loses a window's peak, if adoption drops the carried windows, the peaks a batch folded
/// before a swap in its middle, or restarts the sequence, if
/// an absent strip reports a stale value as valid, or if a returned strip moves its handle.
#[test]
fn strip_and_output_meter_batches_are_each_entrys_own_peak_across_rebuilds() {
    let model = strip_meter_session();
    let document = session::canonical_session_json(&model).expect("strip meter session");
    let (c_session, c_plan) = boxed_c_children(&document);
    let window = fixture_window_blocks();
    let strip = |id: &str| protocol::MeterEntry {
        target: protocol::MeterTarget::Strip(id.into()),
        metric: protocol::MeterMetric::Peak,
    };
    assert_eq!(
        configure_meters_c(
            c_session,
            1,
            vec![
                strip("eq0"),
                strip("eq1"),
                strip("bus"),
                output_peak_entry()
            ],
            2 * window
        ),
        (StatusCode::Ok, None)
    );
    // Window `w`'s source left and right peaks, each at one sample of the window's third block.
    let peaks = [
        [0.5_f32, 0.25],
        [0.75, 0.125],
        [1.5, 0.5],
        [0.25, 0.375],
        // `eq1` ramps out over this batch before its removal swaps in (#1325).
        [0.5, 0.5],
        [0.25, 0.25],
        [0.5, 0.5],
        [0.25, 0.25],
        // A returned strip fades in over its first window, so the batch after `eq1` returns
        // takes its peaks from its second window.
        [0.25, 0.25],
        [0.75, 0.75],
    ];
    let feed = |block: u64| {
        let peak = peaks[(block / window) as usize];
        let mut left = [0.0625_f32; 128];
        let mut right = [-0.0625_f32; 128];
        if block % window == 2 {
            left[17] = -peak[0];
            right[90] = peak[1];
        }
        submit_c(c_session, 1, block * 128, 48_000, &left, &right, false);
        render_c(c_plan, block);
    };
    // Batch `n`'s source peaks, the maxima of its two windows' peaks.
    let source = |batch: usize| {
        let [first, second] = [peaks[2 * batch], peaks[2 * batch + 1]];
        [first[0].max(second[0]), first[1].max(second[1])]
    };
    let records = |handle: u32, peak: Option<[f32; 2]>| {
        [
            protocol::MeterComponent::Left,
            protocol::MeterComponent::Right,
        ]
        .into_iter()
        .zip(peak.unwrap_or([0.0; 2]))
        .map(move |(component, value)| protocol::MeterRecord {
            handle,
            component,
            flags: match peak {
                None => 0,
                Some(_) if value > 1.0 => 1 | 2,
                Some(_) => 1,
            },
            value,
        })
    };
    let scaled = |peak: [f32; 2], gain: f32| [peak[0] * gain, peak[1] * gain];
    let expected = |batch: usize, eq1_present: bool| {
        let peak = source(batch);
        let bus = if eq1_present { 0.25 } else { 0.0 };
        let span = window * 128;
        (
            2 * batch as u64 * span,
            2 * (batch as u64 + 1) * span,
            batch as u64 + 1,
            records(1, Some(peak))
                .chain(records(2, eq1_present.then(|| scaled(peak, 0.5))))
                .chain(records(3, Some(scaled(peak, bus))))
                .chain(records(4, Some(scaled(peak, 1.0 + bus))))
                .collect::<Vec<_>>(),
        )
    };
    // Each commit is at the revision the one before it left: 42 at boot.
    let commit = |request: u64, edits: &[protocol::SessionEdit]| {
        let transaction = command_bytes_at_revision(
            request,
            ExpectedRevision::Exact(SessionRevision(40 + request)),
            protocol::CommandPayload::SessionTransactionApply(edits),
        );
        let (result, response) = command_c(c_session, &transaction);
        assert_eq!(
            (result, response_status(&response)),
            (crate::RESULT_OK, StatusCode::Ok),
            "commit {request}"
        );
        // The commit's `SESSION_COMMITTED`, so the reliable lane has room for the next one.
        while !event_c(c_session, crate::EVENT_LANE_RELIABLE).1.is_empty() {}
    };
    let render_batch = |index: usize| {
        for block in 2 * index as u64 * window..2 * (index as u64 + 1) * window {
            feed(block);
        }
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
        meter_batches_c(c_session)
    };
    let batch = |index: usize, eq1_present: bool| {
        assert_eq!(
            render_batch(index),
            [expected(index, eq1_present)],
            "batch {index}"
        );
    };
    batch(0, true);
    // Gate 3: a rebuild by a muted track whose ID sorts first, so every strip's position shifts,
    // adopted in the middle of a batch: the batch's first window is folded before the swap, its
    // second after it.
    let render_window = |index: u64| {
        for block in index * window..(index + 1) * window {
            feed(block);
        }
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
        meter_batches_c(c_session)
    };
    assert_eq!(render_window(2), [], "half a batch");
    commit(2, &add_muted_track(&model, "a-muted"));
    assert_eq!(render_window(3), [expected(1, true)], "batch 1");
    // Gate 4: `eq1` and its send removed, then returned.
    commit(3, &remove_track("eq1"));
    // While `eq1` ramps out, `bus` and the output carry its tail; `eq0` and the absent `eq1` are
    // exact already.
    let ramping = render_batch(2);
    let (first, end, sequence, want) = expected(2, false);
    assert_eq!(
        ramping
            .iter()
            .map(|(first, end, sequence, records)| (*first, *end, *sequence, records[..4].to_vec()))
            .collect::<Vec<_>>(),
        [(first, end, sequence, want[..4].to_vec())],
        "batch 2"
    );
    batch(3, false);
    commit(
        4,
        &[
            protocol::SessionEdit::UpsertTrack {
                track: model.tracks[1].clone(),
            },
            protocol::SessionEdit::UpsertRoute {
                route: model.routes[1].clone(),
            },
        ],
    );
    batch(4, true);
    assert_eq!(meter_windows_lost_c(c_session, 100), 0, "no window lost");
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// #1587a: a rebuild that carries no strip meter (the one track replaced by another ID) restarts
/// the meter grid at its adoption block, mid-window. Render moves the output meter's gate there
/// at the swap (`OutputPeakGateRelay::restart_at`), the control thread's adoption asks for the same
/// value (`MeterDelivery::adopt`, `align` at the adoption sample), and every window after it pairs
/// with the new plan's strip windows: the batches tile from the adoption sample, `VALID`, with
/// consecutive sequences, and no window is lost.
///
/// Test value: red if render does not restart the gate at an uncarried swap, or the control
/// thread's request is not the restart's value (either opens the output windows off the new strip
/// grid, so no batch pairs again), or if the adoption reads a stale adoption sample.
#[test]
fn an_uncarried_rebuild_restarts_the_meter_grid_at_its_adoption_block() {
    let mut model = stateless_session(48_000);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let document = session::canonical_session_json(&model).expect("one-track session");
    let (c_session, c_plan) = boxed_c_children(&document);
    let window = fixture_window_blocks();
    assert_eq!(
        configure_meters_c(c_session, 1, vec![output_peak_entry()], window),
        (StatusCode::Ok, None)
    );
    let feed = |block: u64| {
        let left = [0.25_f32; 128];
        let right = [-0.125_f32; 128];
        submit_c(c_session, 1, block * 128, 48_000, &left, &right, false);
        render_c(c_plan, block);
    };
    let swap_block = window + 2;
    for block in 0..swap_block {
        feed(block);
    }
    let mut replacement = model.tracks[0].clone();
    replacement.id = session::StableId::parse("eq9").expect("track ID");
    let mut route = model.routes[0].clone();
    route.id = session::StableId::parse("eq9-main").expect("route ID");
    route.source = session::RouteSource::Track {
        track_id: replacement.id.clone(),
        tap: session::SendTap::PostPan,
    };
    let edits = [
        protocol::SessionEdit::RemoveRoute {
            route_id: model.routes[0].id.clone(),
        },
        protocol::SessionEdit::RemoveTrack {
            track_id: model.tracks[0].id.clone(),
        },
        protocol::SessionEdit::UpsertTrack { track: replacement },
        protocol::SessionEdit::UpsertRoute { route },
    ];
    let transaction = command_bytes_at_revision(
        2,
        ExpectedRevision::Exact(SessionRevision(42)),
        protocol::CommandPayload::SessionTransactionApply(&edits),
    );
    let (result, response) = command_c(c_session, &transaction);
    assert_eq!(
        (result, response_status(&response)),
        (crate::RESULT_OK, StatusCode::Ok)
    );
    // The removed track ramps out first (#1325), so the swap lands a few blocks later.
    let end_block = swap_block + 5 * window;
    let mut swapped = None;
    for block in swap_block..end_block {
        feed(block);
        let (_, watermark) = crate::ffi::test_plan_watermark(c_plan);
        if watermark.revision == 43 {
            swapped.get_or_insert(watermark.first_sample);
        }
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let span = window * 128;
    let batches: Vec<_> = meter_batches_c(c_session)
        .into_iter()
        .map(|(first, end, sequence, records)| {
            (
                first,
                end,
                sequence,
                records
                    .iter()
                    .map(|record| record.flags)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    // The watermark names the block that adopted the successor (#1325 D4).
    let adoption = swapped.expect("render adopted the successor");
    assert_ne!(adoption % span, 0, "the swap lands mid-window");
    let after = (end_block * 128 - adoption) / span;
    assert_eq!(
        batches,
        core::iter::once((0, span, 1, vec![1, 1]))
            .chain((0..after).map(|k| (
                adoption + k * span,
                adoption + (k + 1) * span,
                k + 2,
                vec![1, 1]
            )))
            .collect::<Vec<_>>()
    );
    assert_eq!(meter_windows_lost_c(c_session, 100), 0);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Two tracks on one source, a compressor insert (`comp`) on the second only (#1587b).
fn compressor_pair_session() -> session::SessionModel {
    let mut model = stateless_session(48_000);
    model.tracks.truncate(2);
    model.routes.truncate(2);
    model.tracks[1].source_id = model.tracks[0].source_id.clone();
    let protocol::SessionEdit::PutTrackEffect { effect, .. } = put_compressor(&model, 1) else {
        panic!("an insert edit")
    };
    model.tracks[1].inserts.effects.insert(0, effect);
    model
}

/// `UpsertTrack` of a muted copy of `model.tracks[index]` named `id`, effects included, and
/// `UpsertRoute` of its route into the output.
fn add_muted_copy(
    model: &session::SessionModel,
    index: usize,
    id: &str,
) -> [protocol::SessionEdit; 2] {
    let [protocol::SessionEdit::UpsertTrack { mut track }, route] = add_muted_track(model, id)
    else {
        panic!("a track and its route")
    };
    let copy = model.tracks[index].clone();
    track.inserts = copy.inserts;
    track.source_id = copy.source_id;
    [protocol::SessionEdit::UpsertTrack { track }, route]
}

/// The `Observe` arm of `track`'s compressor tap (#1587b).
fn observe_compressor(
    track: &session::StableId,
    armed: bool,
    window_blocks: u32,
) -> protocol::SessionEdit {
    protocol::SessionEdit::Observe {
        strip_id: track.clone(),
        section: protocol::control_core::TapSection::Inserts(
            session::StableId::parse("comp").expect("compressor"),
        ),
        tap_id: 1,
        armed,
        window_blocks,
        arm_tag: 0,
    }
}

/// #1587b attempt 4: a candidate promoted before render's swap block has run (as
/// `withdraw_pending_candidate` promotes one render has claimed) finds the carry record not yet
/// written, marks its observation pairing pending, and settles it at the next service step once
/// render has carried, with reclamation held so the reclaim fallback cannot do it. The rebuild adds
/// a muted copy of the compressor's track that sorts first, so the carried handle moves from
/// position 0 to 1. The running epoch's reader then holds the carried tap's windows, its sequence
/// continuing from before the swap.
///
/// Test value: red if a promotion that reads an unwritten record pairs nothing for good (attempt
/// 3), or if the settle pairs the wrong positions.
#[test]
fn a_promotion_before_the_carry_pairs_the_carried_tap_once_render_carries() {
    let model = compressor_pair_session();
    let track = model.tracks[1].id.clone();
    let document = session::canonical_session_json(&model).expect("compressor session");
    let mut children = compile_children(&document, limits()).expect("children");
    let window = fixture_window_blocks();
    let source = model.sources[0].id.as_str().as_bytes().to_vec();
    command_direct(
        &mut children,
        &transaction_at(
            1,
            42,
            &[observe_compressor(
                &track,
                true,
                u32::try_from(window).expect("window"),
            )],
        ),
    );
    submit_direct(&mut children, &[&source], 0);
    feed_and_render_direct(&mut children, &[&source], 0, 2 * window);
    children.session.service().expect("service");
    let held = |children: &CompiledChildren| {
        let snapshot = children.session.test_transaction_snapshot();
        snapshot
            .observation_windows
            .iter()
            .find(|(epoch, strip, _, _)| {
                *epoch == snapshot.provider_epoch && **strip == *track.as_str()
            })
            .expect("the compressor's readers")
            .3[0]
            .map(|(sequence, first, ..)| (sequence, first))
    };
    let before = held(&children).expect("a window before the rebuild");
    command_direct(
        &mut children,
        &transaction_at(2, 42, &add_muted_copy(&model, 1, "a-muted")),
    );
    control_plane::test_hold_reclamation(true);
    children
        .session
        .test_promote_pending_before_carry()
        .expect("promoted before the carry");
    feed_and_render_direct(&mut children, &[&source], 2 * window, 2 * window);
    children.session.service().expect("service");
    let after = held(&children);
    control_plane::test_hold_reclamation(false);
    children.session.service().expect("service");
    assert_eq!(
        after,
        Some((before.0 + 2, 3 * window * 128)),
        "the carried tap's windows, in the running epoch's reader"
    );
}

/// #1587b gate 2: two tracks on one source, a compressor insert on the second only, driven past
/// its threshold; `{first, gain reduction}` (handle 1) and `{second, gain reduction}` (handle 2)
/// beside the output's peak, one batch per window. The first track has no tap, so its record is
/// `{1, Aggregate, 0, 0.0}` throughout. For the second: unarmed, `{2, Aggregate, 0, 0.0}`.
/// Armed at a window's first block, the batch of that window is `VALID` and reads the tap's
/// window, which opened at the arm: `strip_reduction_db` of the window the session's reader
/// holds. Disarmed, it is `{2, Aggregate, 0, 0.0}` again, though the reader still holds that
/// window. Re-armed in the middle of a window, the batch at that window's end is still flags `0`:
/// the reader's newest window opened before the re-arm. The next batch reads the re-armed tap's
/// first window.
///
/// Test value: red if a window from before the tap's arm sample reaches a record (the fold
/// ignores `arm_sample`), if a disarmed tap still reports (it ignores the slot's arm state), if a
/// record carries another value than its tap's window's reduction, if a tap's reading lands on
/// another strip's entry (a handle mapped to the wrong strip), or if gain reduction is still
/// refused.
#[test]
fn gain_reduction_records_read_only_windows_since_the_taps_arm() {
    let model = compressor_pair_session();
    let quiet = model.tracks[0].id.clone();
    let track = model.tracks[1].id.clone();
    let document = session::canonical_session_json(&model).expect("compressor session");
    let (c_session, c_plan) = boxed_c_children(&document);
    let window = fixture_window_blocks();
    let gain_reduction = |id: &session::StableId| protocol::MeterEntry {
        target: protocol::MeterTarget::Strip(id.as_str().into()),
        metric: protocol::MeterMetric::GainReduction,
    };
    assert_eq!(
        configure_meters_c(
            c_session,
            1,
            vec![
                gain_reduction(&quiet),
                gain_reduction(&track),
                output_peak_entry()
            ],
            window
        ),
        (StatusCode::Ok, None)
    );
    let mut request = 2;
    // A disarm names its arm's tag (#2087): the session's first arm takes tag 1.
    let mut observe = |armed: bool| {
        let edit = protocol::SessionEdit::Observe {
            strip_id: track.clone(),
            section: protocol::control_core::TapSection::Inserts(
                session::StableId::parse("comp").expect("compressor"),
            ),
            tap_id: 1,
            armed,
            window_blocks: u32::try_from(window).expect("window"),
            arm_tag: if armed { 0 } else { 1 },
        };
        let (result, response) = command_c(c_session, &transaction_at(request, 42, &[edit]));
        assert_eq!(
            (result, response_status(&response)),
            (crate::RESULT_OK, StatusCode::Ok),
            "observe {armed}"
        );
        request += 1;
    };
    // A full-scale square wave, far above the compressor's default threshold.
    let render_until = |end: u64, block: &mut u64| {
        while *block < end {
            let left: Vec<f32> = (0..128)
                .map(|i| if i % 2 == 0 { 0.9 } else { -0.9 })
                .collect();
            submit_c(c_session, 1, *block * 128, 48_000, &left, &left, false);
            render_c(c_plan, *block);
            *block += 1;
        }
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
        meter_batches_c(c_session)
            .into_iter()
            .map(|(_, _, _, records)| [records[0], records[1]])
            .collect::<Vec<_>>()
    };
    let quiet_record = protocol::MeterRecord {
        handle: 1,
        component: protocol::MeterComponent::Aggregate,
        flags: 0,
        value: 0.0,
    };
    let absent = [
        quiet_record,
        protocol::MeterRecord {
            handle: 2,
            component: protocol::MeterComponent::Aggregate,
            flags: 0,
            value: 0.0,
        },
    ];
    // The first sample and reduction of the newest window the session's reader holds.
    let held = || {
        let snapshot = crate::ffi::test_transaction_snapshot(c_session);
        let (_, _, _, taps) = snapshot
            .observation_windows
            .iter()
            .find(|(epoch, strip, _, _)| {
                *epoch == snapshot.provider_epoch && **strip == *track.as_str()
            })
            .expect("the compressor's readers");
        let (_, first, _, [left, right]) = taps[0].expect("a published window");
        // The compressor's tap publishes its reduction in decibels, so its strip word is the
        // deeper lane (`strip_reduction_db`; the readings stay inside its declared range).
        (first, f32::from_bits(left).max(f32::from_bits(right)))
    };
    let valid = |value| {
        [
            quiet_record,
            protocol::MeterRecord {
                flags: 1,
                value,
                ..absent[1]
            },
        ]
    };
    let mut block = 0;
    assert_eq!(render_until(window, &mut block), [absent], "unarmed");
    observe(true);
    assert_eq!(
        render_until(2 * window, &mut block),
        [valid(held().1)],
        "armed"
    );
    let (first, reduction) = held();
    assert_eq!(first, window * 128, "the window opened at the arm");
    assert!(reduction > 1.0, "the compressor reduces: {reduction} dB");
    observe(false);
    assert_eq!(render_until(3 * window, &mut block), [absent], "disarmed");
    assert_eq!(held().0, window * 128, "the reader keeps the window");
    render_until(3 * window + window / 2, &mut block);
    observe(true);
    assert_eq!(
        render_until(4 * window, &mut block),
        [absent],
        "re-armed, before the first window since the arm closes"
    );
    assert_eq!(
        held().0,
        window * 128,
        "the reader holds the window from before the re-arm"
    );
    assert_eq!(
        render_until(5 * window, &mut block),
        [valid(held().1)],
        "re-armed"
    );
    assert_eq!(
        held().0,
        (3 * window + window / 2) * 128,
        "the re-armed tap's window"
    );
    // #1674's observation half: a rebuild that carries the compressor (a muted copy of its track,
    // compressor included, whose ID sorts first, so every strip's position shifts and the
    // carried reader moves from handle 0 to handle 1) keeps its armed tap's reading. The promoted
    // epoch reads the carried slot, so each window after the swap is `VALID` again.
    assert_eq!(
        transact_c(c_session, 100, 42, &add_muted_copy(&model, 1, "a-muted")),
        protocol::EditPath::Rebuild
    );
    for later in 5..11 {
        assert_eq!(
            render_until((later + 1) * window, &mut block),
            [valid(held().1)],
            "window {later}, after a carried rebuild"
        );
    }
    assert_eq!(
        crate::ffi::test_plan_carry_counts(c_plan),
        (1, 0),
        "the rebuild carried"
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// #1690b gate 3: one `{master, gain reduction}` entry, configured once while no strip is the
/// master (accepted), follows the core's `master` row. Two tracks share a source above the
/// compressor's threshold; only the second (`comp`) has the compressor, its tap armed. One batch
/// per window: no master reads `{1, Aggregate, 0, 0.0}`; the master on the compressor's track
/// reads its tap's window; moved to the quiet track it reads absent; moved back it reads the tap
/// again; a rebuild that adds a track sorting first (every strip's index shifts; the new track's
/// compressor is unarmed) still reads the master's tap; and from the master's removal it reads
/// absent. The handle is `1` throughout.
///
/// Test value: red if the entry is bound to a strip index (it reads the unarmed copy after the
/// rebuild) or to the strip that was master when the entry resolved (it reads the compressor after
/// the move to the quiet track, or after the removal).
#[test]
fn a_master_entry_follows_the_master_row_across_moves_a_rebuild_and_a_removal() {
    let model = compressor_pair_session();
    let quiet = model.tracks[0].id.clone();
    let comp = model.tracks[1].id.clone();
    let comp_route = model
        .routes
        .iter()
        .find(|route| {
            matches!(&route.source, session::RouteSource::Track { track_id, .. } if *track_id == comp)
        })
        .expect("the compressor track's route")
        .id
        .clone();
    let document = session::canonical_session_json(&model).expect("compressor session");
    let (c_session, c_plan) = boxed_c_children(&document);
    let window = fixture_window_blocks();
    assert_eq!(
        configure_meters_c(
            c_session,
            1,
            vec![protocol::MeterEntry {
                target: protocol::MeterTarget::Master,
                metric: protocol::MeterMetric::GainReduction,
            }],
            window
        ),
        (StatusCode::Ok, None),
        "a master entry with no master is accepted"
    );
    let mut request = 2;
    let mut monitor = |edit: protocol::SessionEdit| {
        let (result, response) = command_c(c_session, &transaction_at(request, 42, &[edit]));
        assert_eq!(
            (result, response_status(&response)),
            (crate::RESULT_OK, StatusCode::Ok),
            "request {request}"
        );
        request += 1;
    };
    let master = |strip: &session::StableId| protocol::SessionEdit::SetControl {
        address: protocol::control_core::ControlAddress {
            owner: protocol::control_core::Owner::Strip(strip.clone()),
            section: protocol::control_core::Section::Own,
            control: 4,
            lane: protocol::control_core::Lane::Both,
        },
        value: 1.0,
        ramp_samples: None,
        expected_effect: None,
    };
    monitor(observe_compressor(
        &comp,
        true,
        u32::try_from(window).expect("window"),
    ));
    // A full-scale square wave, far above the compressor's default threshold. The last batch of
    // each run.
    let render_until = |end: u64, block: &mut u64| {
        while *block < end {
            let left: Vec<f32> = (0..128)
                .map(|i| if i % 2 == 0 { 0.9 } else { -0.9 })
                .collect();
            submit_c(c_session, 1, *block * 128, 48_000, &left, &left, false);
            render_c(c_plan, *block);
            *block += 1;
        }
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
        let (_, _, _, records) = meter_batches_c(c_session).pop().expect("a batch");
        <[protocol::MeterRecord; 1]>::try_from(records).expect("one record")
    };
    // The reduction of the newest window the compressor's reader holds in the running epoch.
    let held = || {
        let snapshot = crate::ffi::test_transaction_snapshot(c_session);
        let (_, _, _, taps) = snapshot
            .observation_windows
            .iter()
            .find(|(epoch, strip, _, _)| {
                *epoch == snapshot.provider_epoch && **strip == *comp.as_str()
            })
            .expect("the compressor's readers");
        let (_, _, _, [left, right]) = taps[0].expect("a published window");
        f32::from_bits(left).max(f32::from_bits(right))
    };
    let record = |flags, value| protocol::MeterRecord {
        handle: 1,
        component: protocol::MeterComponent::Aggregate,
        flags,
        value,
    };
    let absent = [record(0, 0.0)];
    let mut block = 0;
    assert_eq!(render_until(2 * window, &mut block), absent, "no master");
    monitor(master(&comp));
    let reading = render_until(3 * window, &mut block);
    assert_eq!(reading, [record(1, held())], "the compressor's track");
    assert!(reading[0].value > 1.0, "the compressor reduces");
    monitor(master(&quiet));
    assert_eq!(
        render_until(4 * window, &mut block),
        absent,
        "the quiet track"
    );
    monitor(master(&comp));
    assert_eq!(
        render_until(5 * window, &mut block),
        [record(1, held())],
        "the compressor's track again"
    );
    assert_eq!(
        transact_c(c_session, 100, 42, &add_muted_copy(&model, 1, "a-muted")),
        protocol::EditPath::Rebuild
    );
    assert_eq!(
        render_until(8 * window, &mut block),
        [record(1, held())],
        "after a rebuild that shifts every strip"
    );
    // The commits' `SESSION_COMMITTED` events, so the reliable lane has room for the next one.
    while !event_c(c_session, crate::EVENT_LANE_RELIABLE).1.is_empty() {}
    assert_eq!(
        transact_c(
            c_session,
            101,
            43,
            &[
                protocol::SessionEdit::RemoveRoute {
                    route_id: comp_route
                },
                protocol::SessionEdit::RemoveTrack {
                    track_id: comp.clone()
                },
            ]
        ),
        protocol::EditPath::Rebuild
    );
    assert_eq!(
        render_until(11 * window, &mut block),
        absent,
        "the master removed"
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// #1583 gate 5: nine windows rendered with no service call overflow the output meter's queue of
/// eight. With three windows per batch, the first service stages batches 1 and 2 (windows 1-6),
/// render dropped window 9, and the next window's gap discards the partial batch of windows 7
/// and 8: `METER_WINDOWS_LOST` counts exactly those three windows, and the next batch, windows
/// 10-12, is sequence 3. Emptying the table with window 13 folded counts nothing more.
///
/// Test value: red if a dropped window or a discarded partial batch is not counted (or counted
/// twice), if the sequence skips over the lost windows or a batch spans the gap, or if a turn-off
/// counts its open partial batch as lost.
#[test]
fn lost_meter_windows_are_counted_and_the_sequence_does_not_skip() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let window = fixture_window_blocks();
    assert_eq!(
        configure_meters_c(c_session, 1, vec![output_peak_entry()], 3 * window),
        (StatusCode::Ok, None)
    );
    // No source submit: a submit runs the service step.
    for block in 0..9 * window {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let spans = |batches: Vec<MeterBatchRow>| {
        batches
            .into_iter()
            .map(|(first, end, sequence, _)| {
                (first / (window * 128), end / (window * 128), sequence)
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(spans(meter_batches_c(c_session)), [(0, 3, 1), (3, 6, 2)]);
    assert_eq!(
        meter_windows_lost_c(c_session, 2),
        1,
        "window 9 was dropped"
    );
    for block in 9 * window..12 * window {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(spans(meter_batches_c(c_session)), [(9, 12, 3)]);
    assert_eq!(
        meter_windows_lost_c(c_session, 3),
        3,
        "windows 7 to 9 reached no batch"
    );
    // A turn-off with a batch open (window 13 of 13-15) counts nothing: the client emptied the
    // table.
    for block in 12 * window..13 * window {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(
        configure_meters_c(c_session, 4, Vec::new(), 0),
        (StatusCode::Ok, None)
    );
    assert_eq!(
        meter_windows_lost_c(c_session, 5),
        3,
        "the turn-off loses nothing"
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1351 gate 2 (D4): with period 4 a snapshot is staged once render has completed four
/// blocks since the last one, and not before.
///
/// Test value: red if the snapshot ignores the configured period and is staged at every observed
/// block.
#[test]
fn a_counter_snapshot_waits_for_its_configured_period() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        command_c(
            c_session,
            &counter_telemetry(1, vec![protocol::CounterId::TelemetryDropped], 4, false)
        )
        .0,
        crate::RESULT_OK
    );
    for block in 0..3 {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert!(
        lossy_counter_snapshots(c_session).is_empty(),
        "three blocks stage nothing"
    );
    render_c(c_plan, 3);
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let snapshots = lossy_counter_snapshots(c_session);
    assert_eq!(snapshots.len(), 1, "the fourth block stages one");
    assert_eq!(snapshots[0].observed_sample, protocol::SampleTime(4 * 128));

    // Blocks rendered with no call between yield one snapshot at the next call; skipped periods
    // are not replayed.
    for block in 4..13 {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(lossy_counter_snapshots(c_session).len(), 1);
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert!(lossy_counter_snapshots(c_session).is_empty());
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1351 gate 3 (D2): configuring a counter the C ABI does not serve is `NOT_FOUND`, and
/// the stored configuration and the counters stay as they were.
///
/// Test value: red if a configuration naming a counter that can never report is accepted, or if a
/// refused one still replaces the stored configuration.
#[test]
fn configuring_an_unserved_counter_is_refused_and_changes_nothing() {
    let (c_session, c_plan) = boxed_c_children(SESSION);
    let served = vec![protocol::CounterId::TelemetryDropped];
    let (result, response) = command_c(c_session, &counter_telemetry(1, served.clone(), 1, false));
    assert_eq!(
        (result, response_status(&response)),
        (crate::RESULT_OK, StatusCode::Ok)
    );
    let before = counters_get_c(c_session, 2, &[]).expect("all served counters");

    let mut request = 2;
    for unserved in [
        protocol::CounterId::MalformedFrames,
        protocol::CounterId::ControlCommandBackpressure,
    ] {
        request += 1;
        let refused = counter_telemetry(request, vec![unserved], 1, true);
        let (result, response) = command_c(c_session, &refused);
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(
            response_status(&response),
            StatusCode::NotFound,
            "{unserved:?}"
        );
        request += 1;
        assert_eq!(
            counters_get_c(c_session, request, &[unserved]),
            Err(StatusCode::NotFound),
            "{unserved:?} is not readable either"
        );
    }
    assert_eq!(
        counters_get_c(c_session, 1_000, &[]).expect("counters"),
        before
    );

    // The stored configuration is the accepted one: a snapshot of exactly that counter, and no
    // meter batch from the refused configuration.
    render_c(c_plan, 0);
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let snapshots = lossy_counter_snapshots(c_session);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(
        snapshots[0]
            .values
            .iter()
            .map(|value| value.id)
            .collect::<Vec<_>>(),
        served
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 4 (D7): a published candidate scheduled `NoEarlierThan` 64 blocks ahead
/// survives 1,000 service steps after a rendered block: render still adopts it, exactly at the
/// block that starts at its sample, its provider stays pending until then, no plan or provider is
/// disposed, and the retirement credit it holds comes back only through the adoption's reclaim.
///
/// Test value: red if `synchronize_plan_epochs` treats an unadopted scheduled candidate as stale
/// and disposes of it or promotes its provider early -- for example on the pre-#1311 assumption
/// that any block rendered after a publication adopted it (an acked revision's plan lost, or the
/// control plane addressing a plan render does not run). Every other reclaim test publishes for
/// the next block, where that assumption holds.
#[test]
fn a_scheduled_candidate_survives_service_until_render_adopts_it() {
    const ADOPTION_BLOCK: u64 = 64;
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    crate::ffi::test_set_next_adoption(
        c_session,
        engine::realtime::PlanAdoption::NoEarlierThan(ADOPTION_BLOCK * 128),
    );
    assert_eq!(
        command_c(c_session, &rebuild_command(1, 42, 0x01)).0,
        crate::RESULT_OK
    );
    render_c(c_plan, 0);
    let published = crate::ffi::test_lifecycle_counters();
    // (revision, replay entries, current provider epoch, pending providers)
    let summary = crate::ffi::test_session_state_summary(c_session);
    assert_eq!(summary, (43, 1, 0, 1));

    for _ in 0..1_000 {
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    }
    assert_eq!(
        crate::ffi::test_lifecycle_counters(),
        published,
        "service disposes of no plan and no provider"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        summary,
        "the candidate's provider stays pending"
    );

    for block in 1..ADOPTION_BLOCK {
        render_c(c_plan, block);
    }
    let (result, watermark) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(watermark.revision, 42, "not adopted before its block");
    render_c(c_plan, ADOPTION_BLOCK);
    let (result, watermark) = crate::ffi::test_plan_watermark(c_plan);
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        (watermark.revision, watermark.first_sample),
        (43, ADOPTION_BLOCK * 128),
        "render adopts the candidate service kept, at the block that starts at its sample"
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        summary,
        "no control call yet"
    );
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (43, 1, 1, 0)
    );
    let mut reclaimed = published;
    reclaimed.current_plan_disposed += 1;
    reclaimed.current_provider_disposed += 1;
    assert_eq!(crate::ffi::test_lifecycle_counters(), reclaimed);
    // The reclaim returned the candidate's retirement credit: the next rebuild reserves.
    assert_eq!(
        command_c(c_session, &rebuild_command(2, 43, 0x02)).0,
        crate::RESULT_OK
    );
    assert_eq!(
        crate::ffi::test_session_state_summary(c_session),
        (44, 2, 1, 1)
    );
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1348 gate 5: `miso_engine_v1_service` with nothing pending returns `OK` and changes
/// nothing, and 1,000 consecutive calls with no render change nothing either, even with telemetry
/// configured and one render observation already staged. A null or wrong handle is refused as on
/// every session call.
///
/// Test value: red if the step does work that grows with idle time or mutates state with nothing
/// pending (for example, staging the same render observation again on every call).
#[test]
fn service_with_nothing_pending_changes_nothing() {
    crate::ffi::test_reset_lifecycle_observer();
    let (c_session, c_plan) = boxed_c_children(SESSION);
    assert_eq!(
        crate::ffi::test_service(core::ptr::null_mut()),
        crate::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        crate::ffi::test_service(c_plan.cast()),
        crate::RESULT_WRONG_HANDLE
    );

    let fresh = crate::ffi::test_transaction_snapshot(c_session);
    let lifecycle = crate::ffi::test_lifecycle_counters();
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    assert_eq!(crate::ffi::test_transaction_snapshot(c_session), fresh);
    assert_eq!(crate::ffi::test_lifecycle_counters(), lifecycle);

    assert_eq!(
        command_c(c_session, &meter_telemetry(1, 1)).0,
        crate::RESULT_OK
    );
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    for block in 0..fixture_window_blocks() {
        render_c(c_plan, block);
    }
    assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    let settled = crate::ffi::test_transaction_snapshot(c_session);
    assert_eq!(
        settled.telemetry.occupancy, 2,
        "one meter batch staged, left and right"
    );
    let counters = crate::ffi::test_owner_counters(c_session);
    for _ in 0..1_000 {
        assert_eq!(crate::ffi::test_service(c_session), crate::RESULT_OK);
    }
    assert_eq!(crate::ffi::test_transaction_snapshot(c_session), settled);
    assert_eq!(crate::ffi::test_owner_counters(c_session), counters);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
}

/// Issue #1503 gate 3: the C ABI holds the revision at the plan gate's ceiling. A document one
/// past it is refused as the compiler refuses any document. A session one below it takes one live
/// fader edit and the watermark reads the ceiling; the next edit is refused with the
/// revision-exhausted status, pushes no record, changes nothing the transaction may change, and
/// renders exactly like a twin that never saw it.
///
/// Test value: red if a host path bypasses the store's bound (the edit commits a revision the
/// gate cannot hold, and its watermark is corrupt), if a creation path skips the schema's
/// ceiling, or if a refused edit still pushes its live record.
#[test]
fn the_c_abi_bounds_revisions_at_the_plan_gate_ceiling() {
    const SAMPLE_RATE_HZ: u32 = 48_000;
    let ceiling = engine::realtime::GATE_REVISION_MAX;
    let mut model = stateless_session(SAMPLE_RATE_HZ);
    model.tracks.truncate(1);
    model.routes.truncate(1);
    let track_id = model.tracks[0].id.as_str().to_owned();
    let quiet = session::DualMonoFader {
        left_db: -6.0,
        right_db: -6.0,
        ..model.tracks[0].fader.clone()
    };
    model.revision = ceiling - 1;
    let document = session::canonical_session_json(&model).expect("canonical");

    let above = document.replacen(
        &format!("\"revision\": \"{}\"", ceiling - 1),
        "\"revision\": \"9223372036854775808\"",
        1,
    );
    assert_eq!(
        above.matches("9223372036854775808").count(),
        1,
        "the above-ceiling document differs by its revision alone"
    );
    let at_ceiling = document.replacen(
        &format!("\"revision\": \"{}\"", ceiling - 1),
        &format!("\"revision\": \"{ceiling}\""),
        1,
    );
    let refused = compile_children(&above, limits())
        .err()
        .expect("above the ceiling");
    assert!(
        refused
            .diagnostics
            .starts_with(b"numeric.out_of_schema_range\t$.revision"),
        "{}",
        String::from_utf8_lossy(&refused.diagnostics)
    );
    let (c_session, c_plan) = boxed_c_children(&at_ceiling);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);

    let fader = |request_id: u64, revision: u64, fader: &session::DualMonoFader| {
        command_bytes_at_revision(
            request_id,
            ExpectedRevision::Exact(SessionRevision(revision)),
            protocol::CommandPayload::SessionTransactionApply(
                &protocol::control_core::fader_edits(
                    &session::StableId::parse(&track_id).expect("track ID"),
                    fader,
                ),
            ),
        )
    };
    let status = |response: &[u8]| {
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
            .expect("typed response")
        {
            protocol::DecodedTypedResponseFrame::Success { header, .. }
            | protocol::DecodedTypedResponseFrame::NonOk { header, .. } => {
                (header.status, header.revision)
            }
        }
    };
    let watermark = |plan| {
        let (result, mark) = crate::ffi::test_plan_watermark(plan);
        assert_eq!(result, crate::RESULT_OK);
        mark
    };
    // Two identical sessions one revision below the ceiling, each with the same live edit.
    let start = |_| {
        let (c_session, c_plan) = boxed_c_children(&document);
        let (left, right) = continuity_block(0, 0);
        submit_c(c_session, 1, 0, SAMPLE_RATE_HZ, &left, &right, false);
        let mut blocks = feed_and_render_c(c_session, c_plan, SAMPLE_RATE_HZ, 0, 4);
        let (result, response) = command_c(c_session, &fader(1, ceiling - 1, &quiet));
        assert_eq!(result, crate::RESULT_OK);
        assert_eq!(
            status(&response),
            (StatusCode::Ok, SessionRevision(ceiling)),
            "the edit that reaches the ceiling commits"
        );
        assert_eq!(
            crate::ffi::test_session_state_summary(c_session).3,
            0,
            "a fader edit is live: no rebuild, no pending plan"
        );
        blocks.extend(feed_and_render_c(c_session, c_plan, SAMPLE_RATE_HZ, 4, 4));
        let mark = watermark(c_plan);
        assert_eq!(
            mark.revision, ceiling,
            "render applied the ceiling revision"
        );
        (c_session, c_plan, blocks, mark)
    };
    let (c_session, c_plan, mut blocks, applied) = start(0);
    let (t_session, t_plan, mut twin_blocks, _) = start(1);

    let before = crate::ffi::test_transaction_snapshot(c_session);
    let (result, response) = command_c(
        c_session,
        &fader(
            2,
            ceiling,
            &session::DualMonoFader {
                left_db: -12.0,
                right_db: -12.0,
                ..quiet.clone()
            },
        ),
    );
    assert_eq!(result, crate::RESULT_OK);
    assert_eq!(
        status(&response),
        (StatusCode::RevisionExhausted, SessionRevision(ceiling))
    );
    let mut after = crate::ffi::test_transaction_snapshot(c_session);
    assert_eq!(
        after.replay_entries,
        before.replay_entries + 1,
        "the refusal is replayable"
    );
    after.replay_entries = before.replay_entries;
    assert_eq!(after, before, "no record, no plan, no model change");
    assert_eq!(watermark(c_plan), applied, "the watermark is unchanged");

    blocks.extend(feed_and_render_c(c_session, c_plan, SAMPLE_RATE_HZ, 8, 4));
    twin_blocks.extend(feed_and_render_c(t_session, t_plan, SAMPLE_RATE_HZ, 8, 4));
    assert!(
        blocks[8..].iter().flatten().any(|sample| *sample != 0.0),
        "the twin comparison runs on signal"
    );
    let bits = |blocks: &[Vec<f32>]| {
        blocks
            .iter()
            .map(|block| {
                block
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        bits(&blocks),
        bits(&twin_blocks),
        "the refused edit changed no sample"
    );
    assert_eq!(watermark(c_plan), applied);

    for (session, plan) in [(c_session, c_plan), (t_session, t_plan)] {
        crate::ffi::test_plan_destroy(plan);
        crate::ffi::test_session_destroy(session);
    }
}

/// #1642: the session seek through the C entry points. A two-source session at 48 kHz, quantum
/// 128: track `eq0` on `fixture-source` (10,000 frames) and track `t2` on `short-source` (4,000
/// frames), stateless, played on generation 1.
mod session_seek {
    use super::*;

    const LONG: &[u8] = b"fixture-source";
    const SHORT: &[u8] = b"short-source";
    const Q: u64 = 128;

    fn two_sources() -> session::SessionModel {
        let mut model = stateless_session(48_000);
        model.tracks.truncate(1);
        model.routes.truncate(1);
        model.sources[0].frames = 10_000;
        let [
            protocol::SessionEdit::UpsertSource { mut source },
            protocol::SessionEdit::UpsertTrack { track },
            protocol::SessionEdit::UpsertRoute { route },
        ] = add_source_and_track(&model, "short-source", "t2")
        else {
            panic!("three upserts")
        };
        source.frames = 4_000;
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
        model
    }

    /// The C children of `model`, with `fixture-source` fed and rendered for four blocks.
    struct Rig {
        session: *mut crate::Session,
        plan: *mut crate::Plan,
        block: u64,
    }

    impl Rig {
        fn new(model: &session::SessionModel) -> Self {
            let document = session::canonical_session_json(model).expect("canonical");
            let (session, plan) = boxed_c_children(&document);
            let mut rig = Self {
                session,
                plan,
                block: 0,
            };
            submit_block_c(session, LONG, 1, 0, 0, 48_000);
            for _ in 0..4 {
                submit_block_c(session, LONG, 1, rig.block + 1, 0, 48_000);
                rig.render();
            }
            rig
        }

        /// Renders the next block; returns its output.
        fn render(&mut self) -> Vec<f32> {
            let output = render_c(self.plan, self.block);
            self.block += 1;
            output
        }

        /// The sample of the next block.
        fn next(&self) -> u64 {
            self.block * Q
        }

        fn timeline(&self, sample: u64) -> i64 {
            crate::ffi::test_plan_timeline_at(self.plan, sample)
        }

        /// `(generation, first_sample, source_frame)` of `id`'s seek report.
        fn landed(&self, id: &[u8]) -> (u64, u64, u64) {
            let (generation, first, frame, _, _) = seek_report_c(self.session, id);
            (generation, first, frame)
        }

        fn seek(&self, generation: u64, timeline_sample: u64) -> u32 {
            crate::ffi::test_session_seek(self.session, generation, timeline_sample, None)
        }

        fn seek_at(&self, generation: u64, timeline_sample: u64, anchor: u64) -> u32 {
            crate::ffi::test_session_seek(self.session, generation, timeline_sample, Some(anchor))
        }

        fn last_error(&self) -> Vec<u8> {
            crate::ffi::test_last_error(self.session)
        }

        /// `LONG`'s runway of generation `generation` from source frame `frame` (#2205): the
        /// blocks a live plain seek of it waits for.
        fn feed_long_runway(&self, generation: u64, frame: u64) {
            for block in 0..runway_blocks() {
                self.feed_long(generation, frame + block * Q);
            }
        }

        /// One quantum of `LONG`'s generation `generation` from source frame `frame`.
        fn feed_long(&self, generation: u64, frame: u64) {
            let (left, right) = continuity_block(frame / Q, 0);
            submit_c_to(
                self.session,
                LONG,
                generation,
                frame,
                48_000,
                &left,
                &right,
                false,
            );
        }
    }

    impl Drop for Rig {
        fn drop(&mut self) {
            crate::ffi::test_plan_destroy(self.plan);
            crate::ffi::test_session_destroy(self.session);
        }
    }

    /// Gate 1 up to its anchored seek: the rig after `session_seek(2, 6_000)`, `LONG`'s runway
    /// and the block that applied it (`SHORT` was never fed, so it waits for nothing, #2205).
    fn after_gate_one_plain() -> Rig {
        let mut rig = Rig::new(&two_sources());
        assert_eq!(rig.seek(2, 6_000), crate::RESULT_OK);
        rig.feed_long_runway(2, 6_000);
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG), (2, sample, 6_000));
        assert_eq!(rig.landed(SHORT), (2, sample, 4_000), "clamped to its end");
        assert_eq!(rig.timeline(sample), 6_000);
        rig
    }

    /// Gate 1, both forms move everything at one block.
    ///
    /// Test value: red if the timeline and the sources apply the seek in different blocks, a
    /// source is not clamped to its region, or the anchored form ignores its anchor.
    #[test]
    fn both_forms_move_the_timeline_and_every_source_at_one_block() {
        let mut rig = after_gate_one_plain();
        let anchor = rig.next() + 2 * Q;
        assert_eq!(rig.seek_at(3, 1_000, anchor), crate::RESULT_OK);
        // The plain seek applied at the block before these, at timeline 6,000.
        for block in 1..=2 {
            let sample = rig.next();
            rig.render();
            assert_eq!(rig.landed(LONG).0, 2, "held before the anchor");
            assert_eq!(rig.landed(SHORT).0, 2, "held before the anchor");
            assert_eq!(rig.timeline(sample), (6_000 + block * Q) as i64);
        }
        rig.feed_long(3, 1_000);
        let sample = rig.next();
        assert_eq!(sample, anchor);
        rig.render();
        assert_eq!(rig.landed(LONG), (3, anchor, 1_000));
        assert_eq!(rig.landed(SHORT), (3, anchor, 1_000));
        assert_eq!(rig.timeline(anchor), 1_000);
    }

    /// Gate 2, the published generation alone (batch 16b review, MAJOR 1): a per-source seek of
    /// the short source to 10, then a transaction that removes it, swapped in. No live producer
    /// holds a generation above 1 now, so only the session's published generation (10) refuses a
    /// session seek at 5; one at 11 is accepted.
    ///
    /// Test value: red if a session seek is checked only against the live producers' own
    /// generations and not against the largest the session has published: a source removed at a
    /// higher generation would let a lower one through.
    #[test]
    fn a_removed_sources_generation_keeps_a_session_seek_stale() {
        let model = two_sources();
        let mut rig = Rig::new(&model);
        assert_eq!(
            crate::ffi::test_source_seek(rig.session, SHORT, 10, 0),
            crate::RESULT_OK
        );
        submit_block_c(rig.session, LONG, 1, rig.block + 1, 0, 48_000);
        rig.render();
        assert_eq!(rig.landed(SHORT).0, 10);
        // One unread quantum of the short source, which the removal discards (#1699).
        submit_block_c(rig.session, SHORT, 10, 0, 0, 48_000);
        transact_c(
            rig.session,
            1,
            model.revision,
            &[
                protocol::SessionEdit::RemoveRoute {
                    route_id: model.routes[1].id.clone(),
                },
                protocol::SessionEdit::RemoveTrack {
                    track_id: model.tracks[1].id.clone(),
                },
                protocol::SessionEdit::RemoveSource {
                    source_id: model.sources[1].id.clone(),
                },
            ],
        );
        // #1699 parity row: the C ABI serves the discarded quantum as `SOURCE_FRAMES_DISCARDED`.
        let session = rig.session;
        let discarded = |request_id: u64| {
            assert_eq!(crate::ffi::test_service(session), crate::RESULT_OK);
            let (result, response) = command_c(
                session,
                &command_bytes(
                    request_id,
                    protocol::CommandPayload::CountersGet(&protocol::CountersRequest {
                        all: false,
                        ids: vec![protocol::CounterId::SourceFramesDiscarded as u32],
                    }),
                ),
            );
            assert_eq!(result, crate::RESULT_OK);
            let mut fields = [0_u16; 64];
            let protocol::DecodedTypedResponseFrame::Success {
                payload: protocol::DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
                .expect("counters response")
            else {
                panic!("expected a counter snapshot")
            };
            let [value] = snapshot.values[..] else {
                panic!("one counter")
            };
            assert_eq!(value.id, protocol::CounterId::SourceFramesDiscarded);
            value.value
        };
        // #1325 D4: until the adoption at `S` the removed short source feeds the running plan's
        // ramp, one quantum a block, and nothing is discarded; the adoption retires the plan with
        // the quantum it never read.
        let mut request_id = 2;
        let mut short_block = 1;
        loop {
            submit_block_c(rig.session, LONG, 1, rig.block + 1, 0, 48_000);
            rig.render();
            if crate::ffi::test_plan_watermark(rig.plan).1.revision > model.revision {
                break;
            }
            assert!(rig.block < 64, "the removal is adopted");
            assert_eq!(discarded(request_id), 0, "nothing is discarded before S");
            request_id += 1;
            submit_block_c(rig.session, SHORT, 10, short_block, 0, 48_000);
            short_block += 1;
        }
        assert_eq!(discarded(request_id), Q);
        let (left, right) = continuity_block(short_block, 0);
        let planes = [left.as_ptr(), right.as_ptr()];
        let chunk = crate::SourceChunk {
            struct_size: crate::SOURCE_CHUNK_SIZE,
            sample_rate_hz: 48_000,
            generation: 10,
            start_frame: short_block * Q,
            planes: planes.as_ptr(),
            plane_count: 2,
            frames: 128,
            end_of_region: 0,
            reserved0: 0,
        };
        let mut report = crate::SubmitReport {
            struct_size: crate::SUBMIT_REPORT_SIZE,
            reserved0: 0,
            accepted_frames: 0,
            cumulative_written_frames: 0,
            active_generation: 0,
        };
        assert_eq!(
            crate::ffi::test_source_submit(rig.session, SHORT, &chunk, &mut report),
            crate::RESULT_INVALID_ARGUMENT,
            "the removed source is refused after the adoption"
        );
        assert_eq!(rig.last_error(), b"source.id.unknown");
        assert_eq!(
            rig.landed(LONG).0,
            1,
            "the long source is still at generation 1"
        );
        assert_eq!(rig.seek(5, 0), crate::RESULT_INVALID_ARGUMENT);
        assert_eq!(rig.last_error(), b"source.generation.stale");
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG).0, 1, "the refusal pushed nothing");
        assert_eq!(
            rig.timeline(sample),
            sample as i64,
            "the timeline did not move"
        );
        assert_eq!(rig.seek(11, 0), crate::RESULT_OK);
    }

    /// Gate 3, one full slot refuses all.
    ///
    /// Test value: red if any slot is pushed before every slot is checked: the partial seek the
    /// acked-batch rule forbids.
    #[test]
    fn one_full_slot_refuses_every_consumer() {
        let mut rig = Rig::new(&two_sources());
        assert_eq!(
            crate::ffi::test_source_seek(rig.session, LONG, 2, 640),
            crate::RESULT_OK
        );
        assert_eq!(rig.seek(3, 2_000), crate::RESULT_BACKPRESSURE);
        assert_eq!(rig.last_error(), b"source.seek.backpressure");
        // The short source's slot (and the timeline's) stayed empty.
        assert_eq!(
            crate::ffi::test_source_seek(rig.session, SHORT, 3, 100),
            crate::RESULT_OK
        );
        rig.feed_long_runway(2, 640);
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG), (2, sample, 640));
        assert_eq!(rig.landed(SHORT), (3, sample, 100));
        assert_eq!(
            rig.timeline(sample),
            sample as i64,
            "the timeline did not move"
        );
    }

    /// Gate 4, a session seek while a structural candidate waits for its swap.
    ///
    /// Test value: red if the call seeks the running plan's producers instead of the newest
    /// committed session's: the added source would not be sought, and the carried source's seek
    /// would land in a ring the candidate does not hold.
    #[test]
    fn a_seek_before_the_swap_reaches_the_carried_and_the_added_source_and_the_timeline() {
        let mut model = stateless_session(48_000);
        model.tracks.truncate(1);
        model.routes.truncate(1);
        let mut rig = Rig::new(&model);
        transact_c(
            rig.session,
            1,
            42,
            &add_source_and_track(&model, "added", "t2"),
        );
        assert_eq!(rig.seek(2, 3_000), crate::RESULT_OK);
        rig.feed_long_runway(2, 3_000);
        let swap = rig.next();
        rig.render();
        assert_eq!(
            crate::ffi::test_plan_carry_counts(rig.plan),
            (1, 0),
            "the swap block carried the source"
        );
        assert_eq!(rig.landed(LONG), (2, swap, 3_000));
        assert_eq!(rig.landed(b"added"), (2, swap, 3_000));
        assert_eq!(rig.timeline(swap), 3_000);
    }

    /// #2205 gate (C ABI): a plain session seek during playback, with `LONG` fed and `SHORT`
    /// never fed, waits while `LONG`'s producer is late (its old audio plays on), then lands
    /// `LONG`, `SHORT` and the timeline in one block, the first at which `LONG`'s runway is
    /// queued, with no underrun on `LONG`.
    ///
    /// Test value: red if the seek lands before `LONG` has PCM of its generation (today: at the
    /// next block, `LONG` underruns a quantum), if the never-fed source holds the seek, or if
    /// the sources and the timeline land in different blocks.
    #[test]
    fn a_plain_session_seek_lands_every_source_once_the_fed_one_has_its_runway() {
        let mut rig = Rig::new(&two_sources());
        assert_eq!(rig.seek(2, 2_000), crate::RESULT_OK);
        let held = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG).0, 1, "held: LONG's producer is late");
        assert_eq!(rig.landed(SHORT).0, 1, "held with LONG");
        assert_eq!(rig.timeline(held), held as i64, "the timeline runs on");
        rig.feed_long_runway(2, 2_000);
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG), (2, sample, 2_000));
        assert_eq!(
            rig.landed(SHORT),
            (2, sample, 2_000),
            "the never-fed source"
        );
        assert_eq!(rig.timeline(sample), 2_000);
        assert_eq!(seek_report_c(rig.session, LONG).3, 0, "LONG never underran");
    }

    /// Gate 5, a per-source seek leaves the timeline where it is.
    ///
    /// Test value: red if a per-source seek moves the playhead.
    #[test]
    fn a_source_seek_leaves_the_timeline() {
        let mut rig = Rig::new(&two_sources());
        assert_eq!(
            crate::ffi::test_source_seek(rig.session, LONG, 2, 2_048),
            crate::RESULT_OK
        );
        rig.feed_long_runway(2, 2_048);
        let sample = rig.next();
        rig.render();
        assert_eq!(rig.landed(LONG), (2, sample, 2_048));
        assert_eq!(rig.landed(SHORT).0, 1);
        assert_eq!(rig.timeline(sample), sample as i64);
    }
}

/// Root ruling B16-R9 on the C ABI: a plain session seek declares the discontinuity in the same
/// call, after its check; an anchored one declares nothing. Each case starts from #1285's floors:
/// the limited session, its removal transaction committed and swapped, so the running plan runs
/// behind its natural timing and a declaration has a successor to publish.
mod session_seek_declares {
    use super::*;

    const SOURCE: &[u8] = b"fixture-source";
    const Q: u64 = 128;

    /// The floored C children: the session, the plan, the next block and the floored latency.
    struct Floored {
        session: *mut crate::Session,
        plan: *mut crate::Plan,
        block: u64,
        floored: u64,
    }

    impl Floored {
        /// With `swap` false the removal's candidate still waits for its block.
        fn new(swap: bool) -> Self {
            let (model, edits) = limited_session();
            let document = session::canonical_session_json(&model).expect("canonical");
            let (session, plan) = boxed_c_children(&document);
            let floored = crate::ffi::test_plan_latency(plan);
            assert!(floored > 0, "the limiter delays the plan");
            let (left, right) = continuity_block(0, 0);
            submit_c(session, 1, 0, 48_000, &left, &right, false);
            feed_and_render_c(session, plan, 48_000, 0, 6);
            assert_eq!(
                command_c(session, &transaction_at(1, 42, &edits)).0,
                crate::RESULT_OK
            );
            let mut block = 6;
            if swap {
                feed_and_render_c(session, plan, 48_000, 6, 4);
                block = 10;
                assert_eq!(crate::ffi::test_plan_latency(plan), floored, "#1285 floors");
            }
            Self {
                session,
                plan,
                block,
                floored,
            }
        }

        fn epoch(&self) -> u64 {
            crate::ffi::test_newest_plan_epoch(self.session)
        }

        fn latency(&self) -> u64 {
            crate::ffi::test_plan_latency(self.plan)
        }

        /// Feeds the runway of generation `generation` from `frame` (#2205), without rendering;
        /// returns the frame after it.
        fn runway(&mut self, generation: u64, frame: u64) -> u64 {
            for step in 0..runway_blocks() {
                let (left, right) = continuity_block(self.block + 1 + step, 0);
                submit_c(
                    self.session,
                    generation,
                    frame + step * Q,
                    48_000,
                    &left,
                    &right,
                    false,
                );
            }
            frame + runway_blocks() * Q
        }

        /// Feeds generation `generation` from `frame` and renders `blocks` blocks.
        fn play(&mut self, generation: u64, frame: u64, blocks: u64) {
            for step in 0..blocks {
                let (left, right) = continuity_block(self.block + 1, 0);
                submit_c(
                    self.session,
                    generation,
                    frame + step * Q,
                    48_000,
                    &left,
                    &right,
                    false,
                );
                render_c(self.plan, self.block);
                self.block += 1;
            }
        }
    }

    impl Drop for Floored {
        fn drop(&mut self) {
            crate::ffi::test_plan_destroy(self.plan);
            crate::ffi::test_session_destroy(self.session);
        }
    }

    /// Gate 2: an accepted plain session seek publishes exactly one discontinuity successor and its
    /// seek applies, at the next block; also while the removal's candidate still waits, which the
    /// declaration supersedes before the seek is pushed to the same producers.
    ///
    /// Test value: red if the plain seek does not declare (the latency stays floored), publishes
    /// more than one plan, or pushes to a set other than the one the successor carries (the seek
    /// would not land, or the debug assertion of the re-check fires).
    #[test]
    fn an_accepted_plain_session_seek_publishes_one_successor_and_applies() {
        for swap in [true, false] {
            let mut rig = Floored::new(swap);
            let epoch = rig.epoch();
            assert_eq!(
                crate::ffi::test_session_seek(rig.session, 2, 1_280, None),
                crate::RESULT_OK,
                "swap {swap}"
            );
            assert_eq!(rig.epoch(), epoch + 1, "swap {swap}: one successor");
            let sample = rig.block * Q;
            let next = rig.runway(2, 1_280);
            rig.play(2, next, 3);
            let (generation, first, frame, _, _) = seek_report_c(rig.session, SOURCE);
            assert_eq!(
                (generation, first, frame),
                (2, sample, 1_280),
                "swap {swap}"
            );
            assert!(rig.latency() < rig.floored, "swap {swap}: the floors reset");
            assert_eq!(
                rig.epoch(),
                epoch + 1,
                "swap {swap}: nothing else published"
            );
        }
    }

    /// Gate 1b: a plain session seek whose declaration is refused pushes nothing and returns the
    /// declaration's own error; the same generation is accepted once the fault clears.
    ///
    /// Test value: red if the seek ignores its declaration's error (`let _ =`): the refused
    /// declaration would be followed by a push, the report would carry generation 2 and the seek
    /// would return OK.
    #[test]
    fn a_refused_declaration_pushes_nothing_and_leaves_the_generation_free() {
        use crate::runtime::TestStructuralFaultPhase::BeforeProtocolCommit;
        let mut rig = Floored::new(true);
        let epoch = rig.epoch();
        crate::ffi::test_set_structural_faults(rig.session, [Some(BeforeProtocolCommit), None]);
        assert_eq!(
            crate::ffi::test_session_seek(rig.session, 2, 1_280, None),
            crate::RESULT_BACKPRESSURE,
            "the declaration's refusal is returned"
        );
        assert_eq!(rig.epoch(), epoch, "nothing is published");
        rig.play(1, (rig.block + 1) * Q, 2);
        assert_eq!(rig.latency(), rig.floored, "the floors stay");
        assert_eq!(seek_report_c(rig.session, SOURCE).0, 1, "nothing is pushed");
        // The fault is spent: the same generation is not burned by the refused attempt.
        assert_eq!(
            crate::ffi::test_session_seek(rig.session, 2, 1_280, None),
            crate::RESULT_OK
        );
        assert_eq!(rig.epoch(), epoch + 1, "one successor");
        let sample = rig.block * Q;
        let next = rig.runway(2, 1_280);
        rig.play(2, next, 3);
        let (generation, first, frame, _, _) = seek_report_c(rig.session, SOURCE);
        assert_eq!((generation, first, frame), (2, sample, 1_280));
    }
}

/// #1646 gate 1: the sessions of one case, all on [`stateless_session`] at 48 kHz.
struct FlatFaderCase {
    label: &'static str,
    /// A flat fader automation on one track, whose static fader is another value.
    automated: session::SessionModel,
    /// The same session with the track's static fader at the flat value and no automation.
    reference: session::SessionModel,
}

/// A `linear` segment from `value` to `value` on `lanes` of track `track`'s fader (builtin row 5).
fn flat_fader_entry(
    model: &session::SessionModel,
    track: usize,
    value: f32,
) -> session::Automation {
    session::Automation {
        id: session::StableId::parse(&format!("fader-{track}")).expect("automation ID"),
        target: session::ControlAddress {
            owner: session::Owner::Strip(model.tracks[track].id.clone()),
            section: session::Section::Builtins,
            control: 5,
            lane: session::ParameterChannel::Both,
        },
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: 1 << 20,
            start_value: value,
            end_value: value,
            unit: session::ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// A VCA `id` with offset `offset` dB on both lanes, holding `members`.
fn flat_case_vca(id: &str, offset: f32, members: &[&session::StableId]) -> session::Vca {
    session::Vca {
        id: session::StableId::parse(id).expect("VCA ID"),
        fader: session::DualMonoFader {
            left_db: offset,
            right_db: offset,
            left_mute: false,
            right_mute: false,
        },
        members: members.iter().map(|member| (*member).clone()).collect(),
    }
}

/// Gate 1's crafted three-VCA value and offsets (in ascending VCA-ID order), for which the two
/// composition orders of README F16 round to different gain bits.
const CRAFTED_VALUE: u32 = 0x405a_f905;
const CRAFTED_OFFSETS: [u32; 3] = [0xc1af_fc6c, 24.0_f32.to_bits(), 0x2624_8cd2];

/// The four cases of #1646 gate 1: a flat value on a track no VCA reaches; a `both` entry on a
/// track one `+3` dB VCA reaches; the crafted value on a track three VCAs reach; and `-0.0` dB on
/// a track neither of two VCAs reaches (the cell's `S = +0.0`). Each automated track's own static
/// fader is another value, so the flat value is heard only if the automation prepares it.
fn flat_fader_cases() -> Vec<FlatFaderCase> {
    let base = stateless_session(48_000);
    let case = |label: &'static str,
                track: usize,
                value: f32,
                static_db: f32,
                vcas: &dyn Fn(&session::SessionModel) -> Vec<session::Vca>| {
        let mut reference = base.clone();
        reference.vcas = vcas(&reference);
        let fader = &mut reference.tracks[track].fader;
        (fader.left_db, fader.right_db) = (value, value);
        let mut automated = reference.clone();
        let fader = &mut automated.tracks[track].fader;
        (fader.left_db, fader.right_db) = (static_db, static_db);
        automated.automation = vec![flat_fader_entry(&automated, track, value)];
        FlatFaderCase {
            label,
            automated,
            reference,
        }
    };
    vec![
        case("no VCA", 0, -7.25, 0.0, &|_| Vec::new()),
        case("one +3 dB VCA", 1, -7.25, 0.0, &|model| {
            vec![flat_case_vca("v-plus", 3.0, &[&model.tracks[1].id])]
        }),
        case(
            "three VCAs, crafted",
            1,
            f32::from_bits(CRAFTED_VALUE),
            0.0,
            &|model| {
                ["va", "vb", "vc"]
                    .into_iter()
                    .zip(CRAFTED_OFFSETS)
                    .map(|(id, offset)| {
                        flat_case_vca(id, f32::from_bits(offset), &[&model.tracks[1].id])
                    })
                    .collect()
            },
        ),
        case("-0.0 dB, two VCAs elsewhere", 2, -0.0, -3.0, &|model| {
            vec![
                flat_case_vca("va", -1.0, &[&model.tracks[0].id]),
                flat_case_vca("vb", 2.0, &[&model.tracks[0].id, &model.tracks[1].id]),
            ]
        }),
    ]
}

/// The crafted case composes to different gain bits in the two orders, so a render that keeps
/// the member-first order cannot equal the static path, which sums the offsets first (#1646 D6).
fn assert_the_crafted_orders_differ() {
    let value = f64::from(f32::from_bits(CRAFTED_VALUE));
    let offsets = CRAFTED_OFFSETS.map(|bits| f64::from(f32::from_bits(bits)));
    let member_first = offsets.iter().fold(value, |sum, offset| sum + offset) as f32;
    let offsets_first = session::vca_compose_db(
        f32::from_bits(CRAFTED_VALUE),
        session::vca_offsets_sum(CRAFTED_OFFSETS.map(f32::from_bits)).expect("three offsets"),
    );
    assert_eq!(member_first.to_bits(), 0x40ad_8ad3);
    assert_eq!(offsets_first.to_bits(), 0x40ad_8ad2);
    let gain = |db: f32| {
        builtins::checked_fader_gain(db)
            .expect("in domain")
            .to_bits()
    };
    assert_ne!(
        gain(member_first),
        gain(offsets_first),
        "the two orders give different gain bits"
    );
}

/// Renders `model` on the C ABI for `blocks` blocks of the continuity signal.
fn render_flat_case_c(model: &session::SessionModel, blocks: u64) -> Vec<Vec<u32>> {
    let document = session::canonical_session_json(model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children_with_limits(
        &document,
        CompileLimits {
            maximum_vcas: 8,
            ..limits()
        },
    );
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, 48_000, &left, &right, false);
    let blocks = feed_and_render_c(c_session, c_plan, 48_000, 0, blocks);
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
    sample_bits(&blocks)
}

/// #1646 gate 1 (C ABI): a flat stored fader automation renders, bit for bit, the static fader at
/// its value, for 64 blocks, with no VCA, under one `+3` dB VCA, under the crafted three-VCA
/// offsets, and at `-0.0` dB on a lane no VCA reaches. Each automated session also renders
/// differently from itself without the automation, so the value is the automation's.
///
/// Test value: red if preparation composes the offsets in another order than the static path, if
/// the `+0.0` of a lane no VCA reaches changes the gain bits, if the prepared value is not the
/// curve's, or if the C ABI converts dB differently from the static fader.
#[test]
fn a_flat_fader_automation_renders_the_static_fader_bits_on_the_c_abi() {
    const BLOCKS: u64 = 64;
    assert_the_crafted_orders_differ();
    for case in flat_fader_cases() {
        let automated = render_flat_case_c(&case.automated, BLOCKS);
        let reference = render_flat_case_c(&case.reference, BLOCKS);
        let mut unautomated = case.automated.clone();
        unautomated.automation.clear();
        let without = render_flat_case_c(&unautomated, BLOCKS);
        assert!(
            reference
                .iter()
                .flatten()
                .any(|bits| f32::from_bits(*bits) != 0.0),
            "{}: the reference plays",
            case.label
        );
        assert_ne!(
            without, reference,
            "{}: the automation is heard",
            case.label
        );
        for (block, (automated, reference)) in automated.iter().zip(&reference).enumerate() {
            assert_eq!(
                automated, reference,
                "{}: block {block} renders the static fader's bits",
                case.label
            );
        }
    }
}

/// #1647 gate 4 (C ABI): the timeline sample of the seek and the fader ride that is flat around
/// it: `0` dB before timeline 20,000 and [`SEEK_RIDE_DB`] from it on.
const SEEK_TIMELINE: u64 = 40_000;
const SEEK_RIDE_DB: f32 = -9.0;

/// `model` with track 1's fader on the ride (automated) or held at [`SEEK_RIDE_DB`] (reference).
fn seek_ride(mut model: session::SessionModel, automated: bool) -> session::SessionModel {
    if automated {
        model.automation = vec![session::Automation {
            id: session::StableId::parse("ride").expect("automation ID"),
            target: session::ControlAddress {
                owner: session::Owner::Strip(model.tracks[1].id.clone()),
                section: session::Section::Builtins,
                control: 5,
                lane: session::ParameterChannel::Both,
            },
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Step,
                start_sample: 0,
                end_sample: 20_000,
                start_value: 0.0,
                end_value: SEEK_RIDE_DB,
                unit: session::ParameterUnit::Db,
            }],
            offset: session::AutomationOffset::default(),
        }];
    } else {
        let fader = &mut model.tracks[1].fader;
        (fader.left_db, fader.right_db) = (SEEK_RIDE_DB, SEEK_RIDE_DB);
    }
    model
}

/// Renders `model` on the C ABI: ten blocks of generation 1 (after the removal transaction of
/// [`limited_session`] at block 6 when `floored`), a plain session seek to [`SEEK_TIMELINE`] with
/// its runway, and eight blocks of generation 2 from there. Returns the output, block by block, and the render
/// sample the seek applied at (the source's seek report).
fn render_seek_ride_c(
    model: &session::SessionModel,
    floored: Option<&[protocol::SessionEdit; 2]>,
) -> (Vec<Vec<f32>>, u64) {
    let document = session::canonical_session_json(model).expect("canonical");
    let (c_session, c_plan) = boxed_c_children(&document);
    let (left, right) = continuity_block(0, 0);
    submit_c(c_session, 1, 0, 48_000, &left, &right, false);
    let mut blocks = feed_and_render_c(c_session, c_plan, 48_000, 0, 6);
    if let Some(edits) = floored {
        let latency = crate::ffi::test_plan_latency(c_plan);
        assert!(latency > 0, "the limiter delays the plan");
        assert_eq!(
            command_c(c_session, &transaction_at(1, 42, edits)).0,
            crate::RESULT_OK
        );
        blocks.extend(feed_and_render_c(c_session, c_plan, 48_000, 6, 4));
        assert_eq!(
            crate::ffi::test_plan_latency(c_plan),
            latency,
            "#1285 floors"
        );
    } else {
        blocks.extend(feed_and_render_c(c_session, c_plan, 48_000, 6, 4));
    }
    assert_eq!(
        crate::ffi::test_session_seek(c_session, 2, SEEK_TIMELINE, None),
        crate::RESULT_OK
    );
    // Generation 2's step `i` carries block `11 + i`'s content; its runway is fed at once
    // (#2205), so the seek lands at the next block, and the feed stays the runway ahead.
    let feed = |step: u64| {
        let (left, right) = continuity_block(11 + step, 0);
        submit_c(
            c_session,
            2,
            SEEK_TIMELINE + step * 128,
            48_000,
            &left,
            &right,
            false,
        );
    };
    let runway = runway_blocks();
    for step in 0..runway - 1 {
        feed(step);
    }
    for step in 0..8_u64 {
        feed(step + runway - 1);
        blocks.push(render_c(c_plan, 10 + step));
    }
    let (generation, first, frame, _, _) = seek_report_c(c_session, b"fixture-source");
    assert_eq!((generation, frame), (2, SEEK_TIMELINE));
    if floored.is_some() {
        assert!(
            crate::ffi::test_plan_latency(c_plan) == 0,
            "the plain seek's successor resets the floors"
        );
    }
    crate::ffi::test_plan_destroy(c_plan);
    crate::ffi::test_session_destroy(c_session);
    (blocks, first)
}

/// #1647 gate 4 on the C ABI: after `miso_engine_v1_session_seek` to `T` on a ride that is flat
/// at `-9` dB around `T` and `0` dB where the session played before, the automated lane equals the
/// static fader at `-9` dB exactly from the render sample `r` the seek applied at (the fader has
/// no latency before it), with no ramp, and differs before it. Twice: on a plan with no raised
/// floor (the seek lands on the running plan as a `Set`), and on a plan whose floors #1285 raised,
/// where the plain seek publishes a discontinuity successor whose lanes start fresh; its timeline
/// history carries the seek, so the seek's step in the successor's first block lands the `Set`
/// (#1647b departure 3), not the D2 restart `Set`.
///
/// Test value: red if a seek ramps the lane or reaches the fader at the wrong sample, on the
/// running plan or across the discontinuity successor, on the C ABI.
#[test]
fn a_session_seek_sets_an_automated_fader_exactly_on_the_c_abi() {
    let (limited, edits) = limited_session();
    for (label, base, floored) in [
        ("running plan", stateless_session(48_000), None),
        ("raised floor", limited, Some(&edits)),
    ] {
        let (automated, r) = render_seek_ride_c(&seek_ride(base.clone(), true), floored);
        let (reference, reference_r) = render_seek_ride_c(&seek_ride(base, false), floored);
        assert_eq!(r, reference_r, "{label}");
        assert_eq!(r % 128, 0, "{label}: the seek applies at a block entry");
        let flat = |blocks: &[Vec<f32>], plane: usize| -> Vec<u32> {
            blocks
                .iter()
                .flat_map(|block| block[plane * 128..(plane + 1) * 128].iter())
                .map(|value| value.to_bits())
                .collect()
        };
        let r = r as usize;
        for plane in 0..2 {
            let (automated, reference) = (flat(&automated, plane), flat(&reference, plane));
            assert_eq!(
                automated[r..],
                reference[r..],
                "{label}, plane {plane}: the static fader's bits from r"
            );
            assert_ne!(
                automated[r - 128..r],
                reference[r - 128..r],
                "{label}, plane {plane}: the lane was elsewhere before r"
            );
        }
    }
}

/// #1670 D2: the existing #1669 transcript after a plain seek, with silent PCM before it.
/// Only the quantum and the seek's arrival change; the fixture and post-seek source frames do not.
/// Test value: red if settled stored automation depends on render time or block partitioning;
/// existing C ABI seek tests use one quantum and a flat fader around the target.
#[test]
fn stored_automation_after_a_seek_is_invariant_to_arrival_and_quantum_on_the_c_abi() {
    const TARGET: u64 = 30_000;
    const FRAMES: usize = 56 * 128; // #1669's blocks 40..96, aligned at the seek.
    const SETTLE: usize = 1_440; // Default mute hold (480) plus fader ramp (960), at 48 kHz.
    const SOURCES: [(&[u8], [u32; 2]); 2] =
        [(b"src-t1", [40_503, 30_011]), (b"src-t2", [52_153, 18_749])];

    fn transcript(quantum: u32, arrival: u64, automated: bool) -> [Vec<u32>; 2] {
        let mut model = parse_session_json(include_str!(
            "../../../../hosts/host-web/tests/browser-v1/automation-session.json"
        ))
        .expect("#1669 automation fixture");
        model.quantum_frames = quantum;
        if !automated {
            model.automation.clear();
        }
        let document = session::canonical_session_json(&model).expect("canonical fixture");
        let (session, plan) = boxed_c_children_with_limits(
            &document,
            CompileLimits {
                source_ring_frames: quantum,
                maximum_submixes: 1,
                maximum_vcas: 1,
                ..limits()
            },
        );
        assert_eq!(crate::ffi::test_plan_latency(plan), 0, "no insert or delay");
        let q = quantum as usize;
        let mut pcm = vec![f32::NAN; q * 2];
        let output = crate::PlanarOutput {
            struct_size: crate::PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: quantum,
            plane_stride_samples: quantum,
            reserved: [0; 2],
        };
        let silence = vec![0.0; q];
        assert_eq!(arrival % u64::from(quantum), 0);
        for start in (0..arrival).step_by(q) {
            for (source, _) in SOURCES {
                submit_c_to(session, source, 1, start, 48_000, &silence, &silence, false);
            }
            assert_eq!(
                crate::ffi::test_render(plan, start, &output),
                crate::RESULT_OK
            );
            assert!(
                pcm.iter().all(|&sample| sample == 0.0),
                "silent pre-seek PCM"
            );
        }
        assert_eq!(
            crate::ffi::test_session_seek(session, 2, TARGET, None),
            crate::RESULT_OK
        );
        let mut bits: [Vec<u32>; 2] = core::array::from_fn(|_| Vec::new());
        for relative in (0..FRAMES).step_by(q) {
            let start = TARGET + relative as u64;
            for (source, multipliers) in SOURCES {
                // #1669's exact integer-formula PCM, repartitioned into the requested quantum.
                let [left, right] = multipliers.map(|multiplier| {
                    (start..start + u64::from(quantum))
                        .map(|frame| {
                            let residue =
                                ((frame as u32).wrapping_mul(multiplier) & 0xffff) as i32 - 32_768;
                            residue as f32 / 65_536.0
                        })
                        .collect::<Vec<_>>()
                });
                submit_c_to(session, source, 2, start, 48_000, &left, &right, false);
            }
            assert_eq!(
                crate::ffi::test_render(plan, arrival + relative as u64, &output),
                crate::RESULT_OK
            );
            if relative == 0 {
                for (source, _) in SOURCES {
                    let (generation, first, frame, underruns, _) = seek_report_c(session, source);
                    assert_eq!((generation, first, frame), (2, arrival, TARGET));
                    assert_eq!(underruns, 0, "the transcript supplied every source frame");
                }
            }
            let take = q.min(FRAMES - relative);
            for plane in 0..2 {
                bits[plane].extend(pcm[plane * q..plane * q + take].iter().map(|v| v.to_bits()));
            }
        }
        crate::ffi::test_plan_destroy(plan);
        crate::ffi::test_session_destroy(session);
        bits
    }

    // Plain seeks land at block entries: the original q128/block40 arrival is 5120. The q100
    // arrivals change the 64-sample phase as well as the partition; each quantum also varies time.
    let configurations = [(128, 5_120), (128, 5_248), (100, 5_100), (100, 5_200)];
    assert_ne!((configurations[0].1 - configurations[2].1) % 64, 0);
    let reference = transcript(configurations[0].0, configurations[0].1, true);
    let plain = transcript(configurations[0].0, configurations[0].1, false);
    for plane in 0..2 {
        assert!(
            reference[plane][SETTLE..]
                .iter()
                .any(|&v| f32::from_bits(v) != 0.0)
        );
        assert_ne!(
            reference[plane][SETTLE..],
            plain[plane][SETTLE..],
            "plane {plane}: automation heard"
        );
    }
    for (quantum, arrival) in configurations.into_iter().skip(1) {
        let candidate = transcript(quantum, arrival, true);
        for plane in 0..2 {
            let first = (SETTLE..FRAMES).find(|&i| candidate[plane][i] != reference[plane][i]);
            if let Some(first) = first {
                panic!(
                    "q{quantum}, arrival {arrival}, plane {plane}: first differing relative sample \
                     {first}, timeline {}, actual bits {:#010x}, reference bits {:#010x}",
                    TARGET + first as u64,
                    candidate[plane][first],
                    reference[plane][first]
                );
            }
        }
    }
}
