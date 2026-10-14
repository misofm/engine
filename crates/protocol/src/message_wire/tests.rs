use super::*;
use crate::ProtocolLimits;

fn codec() -> ProtocolCodec {
    ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: 4096,
        max_tlv_count: 64,
        max_string_bytes: 64,
        max_nesting: 4,
    })
}

fn diagnostic() -> Diagnostic {
    Diagnostic {
        code: "schema.unknown_field".to_owned(),
        severity: DiagnosticSeverity::Error,
        path: vec![
            PathSegment::Field("tracks".to_owned()),
            PathSegment::Index(2),
            PathSegment::StableId("vocal".to_owned()),
        ],
        detail: Some("unknown key".to_owned()),
        operation_index: Some(7),
        sample_time: Some(48),
        provider_sequence: None,
    }
}

fn response() -> NonOkResponse {
    NonOkResponse {
        diagnostics: vec![diagnostic()],
        omitted_diagnostics: 2,
        backpressure: Some(Backpressure {
            queue_kind: BackpressureQueueKind::ReliableEvent,
            capacity: 8,
            occupancy: 8,
            requested_items: 1,
            generation: Some(4),
            retry_boundary: Some(256),
            requested_bytes: Some(32),
            available_bytes: Some(0),
        }),
    }
}

#[test]
fn common_error_payload_is_canonical_and_caller_owned() {
    let codec = codec();
    let value = response();
    let required = codec.encoded_non_ok_payload_len(&value).expect("size");
    let mut output = vec![0_u8; required];
    assert_eq!(
        codec.encode_non_ok_payload(&value, &mut output),
        Ok(required)
    );
    assert_eq!(
        hex(&output),
        concat!(
            "01000b010001000008000000000000000100090114000000",
            "736368656d612e756e6b6e6f776e5f6669656c6400000000",
            "0200010101000000030000000000000003000b0128000000",
            "020000000000000001000101010000000100000000000000",
            "0200090006000000747261636b73000003000b0128000000",
            "020000000000000001000101010000000200000000000000",
            "0300040008000000020000000000000003000b0128000000",
            "020000000000000001000101010000000300000000000000",
            "0400090005000000766f63616c000000040009000b000000",
            "756e6b6e6f776e206b657900000000000500030004000000",
            "070000000000000006000400080000003000000000000000",
            "0200030104000000020000000000000003000b0088000000",
            "080000000000000001000101010000000400000000000000",
            "020004010800000008000000000000000300040108000000",
            "080000000000000004000201020000000100000000000000",
            "050004000800000004000000000000000600040008000000",
            "000100000000000007000400080000002000000000000000",
            "08000400080000000000000000000000"
        )
    );
    assert_eq!(
        codec.decode_non_ok_payload(&output, 3).expect("decode"),
        value
    );
    let mut short = vec![0xaa; required - 1];
    assert_eq!(
        codec.encode_non_ok_payload(&value, &mut short),
        Err(EncodeError::OutputTooSmall { required })
    );
    assert!(short.iter().all(|byte| *byte == 0xaa));
}

#[test]
fn unknown_optional_skips_but_required_and_wrong_variant_reject() {
    let codec = codec();
    let value = response();
    let required = codec.encoded_non_ok_payload_len(&value).expect("size");
    let mut output = vec![0_u8; required];
    codec
        .encode_non_ok_payload(&value, &mut output)
        .expect("encode");
    let mut optional = output.clone();
    optional.extend_from_slice(&[99, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        codec
            .decode_non_ok_payload(&optional, 4)
            .expect("optional skips"),
        value
    );
    optional[required + 3] = 1;
    assert_eq!(
        codec.decode_non_ok_payload(&optional, 4),
        Err(DecodeError::UnknownRequiredField)
    );
    let mut bad_path = output;
    // The first path begins after the diagnostic's code and severity TLVs. Change the
    // `FIELD` variant's inner value field from id 2 to id 3 while its tag remains FIELD.
    bad_path[96..98].copy_from_slice(&3_u16.to_le_bytes());
    assert!(matches!(
        codec.decode_non_ok_payload(&bad_path, 3),
        Err(DecodeError::InvalidTlv | DecodeError::Truncated)
    ));
}

#[test]
fn every_common_golden_truncation_rejects_and_bounded_count_is_exact() {
    let codec = codec();
    let value = response();
    let required = codec.encoded_non_ok_payload_len(&value).expect("size");
    let mut output = vec![0_u8; required];
    codec
        .encode_non_ok_payload(&value, &mut output)
        .expect("encode");
    for end in 0..output.len() {
        assert!(codec.decode_non_ok_payload(&output[..end], 3).is_err());
    }
    let bounded = NonOkResponse::bounded(&[diagnostic(), diagnostic(), diagnostic()], 2);
    assert_eq!(bounded.diagnostics.len(), 2);
    assert_eq!(bounded.omitted_diagnostics, 1);
}

#[test]
fn path_tags_and_registered_mandatory_flags_are_exact() {
    let codec = codec();
    for (segment, expected_tag) in [
        (PathSegment::Field("field".to_owned()), 1),
        (PathSegment::Index(9), 2),
        (PathSegment::StableId("track_1".to_owned()), 3),
    ] {
        let value = NonOkResponse {
            diagnostics: vec![Diagnostic {
                code: "a.b".to_owned(),
                severity: DiagnosticSeverity::Error,
                path: vec![segment],
                detail: None,
                operation_index: None,
                sample_time: None,
                provider_sequence: None,
            }],
            omitted_diagnostics: 0,
            backpressure: None,
        };
        let required = codec.encoded_non_ok_payload_len(&value).expect("length");
        let mut output = vec![0_u8; required];
        codec
            .encode_non_ok_payload(&value, &mut output)
            .expect("encode");
        // Top-level diagnostic value begins at byte 8. Its nested header, code TLV, and
        // severity TLV are fixed at 8, 16, and 16 bytes for this three-byte code.
        assert_eq!(output[72], expected_tag);
        assert_eq!(codec.decode_non_ok_payload(&output, 2), Ok(value));

        let mut missing_repeated_mandatory = output.clone();
        missing_repeated_mandatory[3] = 0;
        assert_eq!(
            codec.decode_non_ok_payload(&missing_repeated_mandatory, 2),
            Err(DecodeError::InvalidTlv)
        );
        let mut missing_omitted_count_mandatory = output.clone();
        let count_offset = output.len() - 16;
        missing_omitted_count_mandatory[count_offset + 3] = 0;
        assert_eq!(
            codec.decode_non_ok_payload(&missing_omitted_count_mandatory, 2),
            Err(DecodeError::InvalidTlv)
        );
        let mut wrong_tagged_variant = output;
        let incompatible_variant_field: u16 = match expected_tag {
            1 => 3,
            2 => 4,
            3 => 2,
            _ => unreachable!("frozen path tag"),
        };
        wrong_tagged_variant[80..82].copy_from_slice(&incompatible_variant_field.to_le_bytes());
        assert_eq!(
            codec.decode_non_ok_payload(&wrong_tagged_variant, 2),
            Err(DecodeError::InvalidTlv)
        );
    }
}

#[test]
fn direct_common_decoder_enforces_counts_strings_and_nesting() {
    let value = response();
    let normal = codec();
    let required = normal.encoded_non_ok_payload_len(&value).expect("length");
    let mut output = vec![0_u8; required];
    normal
        .encode_non_ok_payload(&value, &mut output)
        .expect("encode");

    let field_limited = ProtocolCodec::new(ProtocolLimits {
        max_tlv_count: 2,
        ..codec().limits()
    });
    assert_eq!(
        field_limited.encoded_non_ok_payload_len(&value),
        Err(EncodeError::LimitExceeded)
    );
    assert_eq!(
        field_limited.decode_non_ok_payload(&output, 3),
        Err(DecodeError::LimitExceeded)
    );

    let string_limited = ProtocolCodec::new(ProtocolLimits {
        max_string_bytes: 3,
        ..codec().limits()
    });
    assert_eq!(
        string_limited.encoded_non_ok_payload_len(&value),
        Err(EncodeError::LimitExceeded)
    );
    assert_eq!(
        string_limited.decode_non_ok_payload(&output, 3),
        Err(DecodeError::LimitExceeded)
    );

    let nesting_limited = ProtocolCodec::new(ProtocolLimits {
        max_nesting: 1,
        ..codec().limits()
    });
    assert_eq!(
        nesting_limited.encoded_non_ok_payload_len(&value),
        Err(EncodeError::LimitExceeded)
    );
    assert_eq!(
        nesting_limited.decode_non_ok_payload(&output, 3),
        Err(DecodeError::LimitExceeded)
    );
}

#[test]
fn capability_id_views_reject_order_allocation_and_session_family_identically() {
    fn le(ids: &[u16]) -> Vec<u8> {
        ids.iter().flat_map(|id| id.to_le_bytes()).collect()
    }
    fn view<'a>(commands: IdSource<'a>, events: IdSource<'a>) -> CapabilityInvariantView<'a> {
        CapabilityInvariantView {
            minimum_version: crate::ProtocolVersion::V1,
            maximum_version: crate::ProtocolVersion::V1,
            maximum_frame_bytes: 4096,
            maximum_tlvs: 64,
            maximum_nesting: 4,
            maximum_automation_records: 256,
            maximum_control_page_items: 256,
            maximum_diagnostic_page_items: 256,
            maximum_transaction_edits: 64,
            supported_commands: commands,
            supported_events: events,
            flags: CapabilityFlags(
                CapabilityFlags::B1B_BASE.0 | CapabilityFlags::SESSION_EVENT_STREAM.0,
            ),
        }
    }
    fn assert_parity_rejects(commands: &[u16], events: &[u16]) {
        let command_bytes = le(commands);
        let event_bytes = le(events);
        assert_eq!(
            check_capabilities_invariants(view(
                IdSource::Native(commands),
                IdSource::Native(events),
            )),
            Err(DecodeError::InvalidTlv)
        );
        assert_eq!(
            check_capabilities_invariants(view(
                IdSource::LittleEndian(&command_bytes),
                IdSource::LittleEndian(&event_bytes),
            )),
            Err(DecodeError::InvalidTlv)
        );
    }

    let commands = [1, 2, 3];
    let events = [0x8001, 0x8002];
    let command_bytes = le(&commands);
    let event_bytes = le(&events);
    assert!(
        check_capabilities_invariants(
            view(IdSource::Native(&commands), IdSource::Native(&events),)
        )
        .is_ok()
    );
    assert!(
        check_capabilities_invariants(view(
            IdSource::LittleEndian(&command_bytes),
            IdSource::LittleEndian(&event_bytes),
        ))
        .is_ok()
    );

    assert_parity_rejects(&[1, 3, 2], &events);
    // The pair rule, an unallocated ID and a retired ID are each refused.
    assert_parity_rejects(&[1, 2, 12], &events);
    assert_parity_rejects(&[1, 2, 14], &events);
    assert_parity_rejects(&[1, 2, 4], &events);
    assert_parity_rejects(&commands, &[0x8001]);
    assert!(!IdSource::LittleEndian(&[1]).is_strict_allocated(false));
}

#[test]
fn b1b_success_schemas_round_trip_and_truncate() {
    let codec = codec();
    let commands = [1_u16, 2, 3];
    let events = [0x8001_u16, 0x8002];
    let capabilities = Capabilities {
        minimum_version: crate::ProtocolVersion::V1,
        maximum_version: crate::ProtocolVersion::V1,
        maximum_frame_bytes: 4096,
        maximum_tlvs: 64,
        maximum_string_bytes: 64,
        maximum_nesting: 4,
        maximum_automation_records: 256,
        control_command_slots: 1,
        control_command_bytes: 64,
        automation_batch_slots: 1,
        reliable_response_slots: 1,
        reliable_event_slots: 1,
        telemetry_slots: 1,
        replay_entries: 1,
        replay_bytes: 1024,
        maximum_cached_response_bytes: 512,
        per_block_automation_density: 256,
        admission_quantum_frames: 64,
        maximum_control_page_items: 256,
        maximum_diagnostic_page_items: 256,
        maximum_telemetry_handles: 256,
        maximum_transaction_edits: 64,
        supported_commands: &commands,
        supported_events: &events,
        flags: CapabilityFlags(
            CapabilityFlags::B1B_BASE.0 | CapabilityFlags::SESSION_EVENT_STREAM.0,
        ),
    };
    let mut capability_bytes = vec![
        0;
        codec
            .encoded_capabilities_len(&capabilities)
            .expect("length")
    ];
    codec
        .encode_capabilities(&capabilities, &mut capability_bytes)
        .expect("encode");
    let decoded = codec
        .decode_capabilities(&capability_bytes, 27)
        .expect("decode");
    assert_eq!(decoded.maximum_frame_bytes, 4096);
    assert_eq!(decoded.supported_commands, &[1, 0, 2, 0, 3, 0]);
    for end in 0..capability_bytes.len() {
        assert!(
            codec
                .decode_capabilities(&capability_bytes[..end], 27)
                .is_err()
        );
    }
    let request = SessionSnapshotRequest {
        offset: 2,
        maximum_bytes: 3,
    };
    let mut request_bytes = [0_u8; 32];
    codec
        .encode_snapshot_request(request, &mut request_bytes)
        .expect("request");
    assert_eq!(
        codec.decode_snapshot_request(&request_bytes, 2),
        Ok(request)
    );
    let snapshot = SessionSnapshot {
        total_bytes: 5,
        offset: 2,
        canonical_json_chunk: b"cde",
        eof: true,
    };
    let mut snapshot_bytes = vec![0; codec.encoded_snapshot_len(snapshot).expect("length")];
    codec
        .encode_snapshot(snapshot, &mut snapshot_bytes)
        .expect("snapshot");
    assert_eq!(codec.decode_snapshot(&snapshot_bytes, 4), Ok(snapshot));
    let applied = TransactionApplied {
        applied_operations: 3,
        edit_path: EditPath::Rebuild,
    };
    let mut applied_bytes = [0_u8; 32];
    codec
        .encode_transaction_applied(applied, &mut applied_bytes)
        .expect("applied");
    assert_eq!(
        codec.decode_transaction_applied(&applied_bytes, 2),
        Ok(applied)
    );
    let event = SessionCommitted {
        event_sequence: 1,
        origin_request_id: crate::RequestId::new(2).expect("id"),
        previous_revision: crate::SessionRevision(7),
        applied_operations: 3,
    };
    let mut event_bytes = [0_u8; 64];
    codec
        .encode_session_committed(event, &mut event_bytes)
        .expect("event");
    assert_eq!(codec.decode_session_committed(&event_bytes, 4), Ok(event));
}

#[test]
fn b1b_fixed_goldens_and_every_byte_truncation() {
    let codec = codec();
    let request = SessionSnapshotRequest {
        offset: 2,
        maximum_bytes: 3,
    };
    let mut request_bytes = [0_u8; 32];
    codec
        .encode_snapshot_request(request, &mut request_bytes)
        .expect("request");
    assert_eq!(
        hex(&request_bytes),
        "0100040108000000020000000000000002000301040000000300000000000000"
    );
    for end in 0..request_bytes.len() {
        assert!(
            codec
                .decode_snapshot_request(&request_bytes[..end], 2)
                .is_err()
        );
    }

    // The two-byte chunk is intentionally an incomplete UTF-8 sequence; snapshot pages are
    // canonical byte ranges and may split a code point.
    let snapshot = SessionSnapshot {
        total_bytes: 5,
        offset: 2,
        canonical_json_chunk: &[b'a', 0xc3],
        eof: false,
    };
    let mut snapshot_bytes = [0_u8; 64];
    codec
        .encode_snapshot(snapshot, &mut snapshot_bytes)
        .expect("snapshot");
    assert_eq!(
        hex(&snapshot_bytes),
        "010004010800000005000000000000000200040108000000020000000000000003000a010200000061c300000000000004000801010000000000000000000000"
    );
    for end in 0..snapshot_bytes.len() {
        assert!(codec.decode_snapshot(&snapshot_bytes[..end], 4).is_err());
    }

    let applied = TransactionApplied {
        applied_operations: 3,
        edit_path: EditPath::Live,
    };
    let mut applied_bytes = [0_u8; 32];
    codec
        .encode_transaction_applied(applied, &mut applied_bytes)
        .expect("applied");
    assert_eq!(
        hex(&applied_bytes),
        "0100030104000000030000000000000002000101010000000100000000000000"
    );
    for end in 0..applied_bytes.len() {
        assert!(
            codec
                .decode_transaction_applied(&applied_bytes[..end], 2)
                .is_err()
        );
    }

    let event = SessionCommitted {
        event_sequence: 1,
        origin_request_id: crate::RequestId::new(2).expect("id"),
        previous_revision: crate::SessionRevision(7),
        applied_operations: 3,
    };
    let mut event_payload = [0_u8; 64];
    codec
        .encode_session_committed(event, &mut event_payload)
        .expect("event");
    assert_eq!(
        hex(&event_payload),
        "01000401080000000100000000000000020004010800000002000000000000000300040108000000070000000000000004000301040000000300000000000000"
    );
    for end in 0..event_payload.len() {
        assert!(
            codec
                .decode_session_committed(&event_payload[..end], 4)
                .is_err()
        );
    }
    let mut frame = [0_u8; crate::OUTER_HEADER_BYTES + 64];
    codec
        .write_outer_header(
            &mut frame,
            crate::FrameKind::Event,
            crate::MessageId::SessionCommitted,
            crate::StatusCode::Ok,
            0,
            8,
            0,
            64,
            4,
        )
        .expect("header");
    frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(&event_payload);
    assert_eq!(
        hex(&frame),
        concat!(
            "4d49534f43544c0001000000300003000180000040000000000000000000000008000000000000000400000000000000",
            "01000401080000000100000000000000020004010800000002000000000000000300040108000000070000000000000004000301040000000300000000000000"
        )
    );
    for end in 0..frame.len() {
        assert!(
            codec
                .decode(
                    &frame[..end],
                    &mut crate::DecodeScratch::new(&mut [0_u16; 4])
                )
                .is_err()
        );
    }
}

/// A parsed BTLV field: the control tests edit a valid encoding as a tree and re-serialise it, so
/// a frame that is well formed on the wire but breaks a schema rule can be built from a valid one.
#[derive(Clone, Debug)]
enum Node {
    Leaf(Vec<u8>),
    Message(Vec<Tlv>),
}

#[derive(Clone, Debug)]
struct Tlv {
    id: u16,
    wire: u8,
    mandatory: u8,
    node: Node,
}

fn parse_tlvs(mut bytes: &[u8]) -> Vec<Tlv> {
    let mut fields = Vec::new();
    while !bytes.is_empty() {
        let id = u16::from_le_bytes([bytes[0], bytes[1]]);
        let length = u32::from_le_bytes(bytes[4..8].try_into().expect("length")) as usize;
        let value = &bytes[8..8 + length];
        fields.push(Tlv {
            id,
            wire: bytes[2],
            mandatory: bytes[3],
            node: if bytes[2] == 11 {
                Node::Message(parse_tlvs(&value[8..]))
            } else {
                Node::Leaf(value.to_vec())
            },
        });
        bytes = &bytes[8 + length.next_multiple_of(8)..];
    }
    fields
}

fn write_tlvs(fields: &[Tlv]) -> Vec<u8> {
    let mut out = Vec::new();
    for field in fields {
        let value = match &field.node {
            Node::Leaf(value) => value.clone(),
            Node::Message(inner) => {
                let mut value = (inner.len() as u32).to_le_bytes().to_vec();
                value.extend_from_slice(&0_u32.to_le_bytes());
                value.extend(write_tlvs(inner));
                value
            }
        };
        out.extend_from_slice(&field.id.to_le_bytes());
        out.extend_from_slice(&[field.wire, field.mandatory]);
        out.extend_from_slice(&(value.len() as u32).to_le_bytes());
        out.extend_from_slice(&value);
        out.resize(out.len() + value.len().next_multiple_of(8) - value.len(), 0);
    }
    out
}

/// An optional field's TLV.
fn leaf(id: u16, wire: u8, value: &[u8]) -> Tlv {
    Tlv {
        id,
        wire,
        mandatory: 0,
        node: Node::Leaf(value.to_vec()),
    }
}

/// The `n`th occurrence of field `id` among `fields`, as a message.
fn nth(fields: &mut [Tlv], id: u16, n: usize) -> &mut Vec<Tlv> {
    match &mut fields
        .iter_mut()
        .filter(|field| field.id == id)
        .nth(n)
        .expect("the field is present")
        .node
    {
        Node::Message(inner) => inner,
        Node::Leaf(_) => panic!("field {id} is not a message"),
    }
}

fn entry(page: &mut [Tlv], index: usize) -> &mut Vec<Tlv> {
    nth(page, 5, index)
}

fn address(page: &mut [Tlv], index: usize) -> &mut Vec<Tlv> {
    nth(entry(page, index), 3, 0)
}

fn set(fields: &mut [Tlv], id: u16, value: &[u8]) {
    let field = fields
        .iter_mut()
        .find(|field| field.id == id)
        .expect("the field is present");
    field.node = Node::Leaf(value.to_vec());
}

fn insert_again(fields: &mut Vec<Tlv>, field: Tlv) {
    let at = fields
        .iter()
        .rposition(|other| other.id <= field.id)
        .map_or(0, |at| at + 1);
    fields.insert(at, field);
}

fn remove(fields: &mut Vec<Tlv>, id: u16) {
    assert!(fields.iter().any(|field| field.id == id), "field {id}");
    fields.retain(|field| field.id != id);
}

fn insert(fields: &mut Vec<Tlv>, field: Tlv) {
    assert!(fields.iter().all(|other| other.id != field.id));
    let at = fields
        .iter()
        .position(|other| other.id > field.id)
        .unwrap_or(fields.len());
    fields.insert(at, field);
}

/// Edit the encoding of `bytes` and return the new bytes with their top-level field count.
fn edited(bytes: &[u8], edit: impl FnOnce(&mut Vec<Tlv>)) -> (Vec<u8>, u32) {
    let mut fields = parse_tlvs(bytes);
    edit(&mut fields);
    (write_tlvs(&fields), fields.len() as u32)
}

fn control_row(name: &str) -> ControlRow {
    ControlRow {
        name: name.to_owned(),
        display_name: "Ratio".to_owned(),
        display_unit: ":1".to_owned(),
        unit: ParameterUnit::Ratio,
        kind: ControlKind::Continuous,
        minimum: 1.0,
        maximum: 20.0,
        disabled_value: None,
        default: 4.0,
        taper: ControlTaper::Logarithmic,
        step: 0.5,
        precision: 1,
        ladder: vec![2.0, 4.0, 8.0],
        choices: Vec::new(),
        ramp_row: None,
        default_ramp_samples: 0,
        class: ControlClass::Live,
        reason: None,
        reason_issue: None,
        reopen: None,
        scope: ControlScope::Model,
        automatable: true,
        family: ControlFamily::Effect,
        active: true,
        value: 4.0,
    }
}

fn control_address_of(
    section: ControlSection,
    slot: Option<&str>,
    control: Option<u32>,
) -> ControlAddress {
    ControlAddress {
        owner_kind: ControlOwnerKind::Strip,
        owner_id: Some("vocal".to_owned()),
        section,
        slot_id: slot.map(str::to_owned),
        control,
        lane: control.map(|_| ParameterChannel::Left),
    }
}

/// One page with every kind of row and every optional field once: entry 0 an owner header, 1 a
/// route owner header, 2 and 3 section headers, 4 a live continuous row (with a disabled value
/// and a ramp row), 5 a toggle, 6 a rebuild-only enumeration, 7 a rebuild-only row that a ruling
/// makes live, 8 a derived monitor row that is inactive.
fn control_page() -> ControlListPage {
    let mut live = control_row("ratio");
    live.disabled_value = Some(0.0);
    live.ramp_row = Some(ControlRampRow::Fader);
    live.default_ramp_samples = 480;
    let mut toggle = control_row("bypass");
    toggle.kind = ControlKind::Toggle;
    toggle.minimum = 0.0;
    toggle.maximum = 1.0;
    toggle.default = 0.0;
    toggle.value = 1.0;
    toggle.taper = ControlTaper::Stepped;
    toggle.ladder = Vec::new();
    let mut detector = control_row("detector");
    detector.kind = ControlKind::Enumeration;
    detector.class = ControlClass::RebuildOnly;
    detector.automatable = false;
    detector.reason = Some(ControlReason::NewMemory);
    detector.reopen = Some("a detector that shares one buffer".to_owned());
    detector.choices = vec![
        EnumChoice {
            value: 0.0,
            label: "Peak".to_owned(),
        },
        EnumChoice {
            value: 1.0,
            label: "RMS".to_owned(),
        },
    ];
    detector.minimum = 0.0;
    detector.maximum = 1.0;
    detector.default = 0.0;
    detector.value = 1.0;
    detector.taper = ControlTaper::Stepped;
    let mut ruled = control_row("lookahead");
    ruled.class = ControlClass::RebuildOnly;
    ruled.automatable = false;
    ruled.reason = Some(ControlReason::RuledLiveNotBuilt);
    ruled.reason_issue = Some(1557);
    ruled.reopen = Some("#1557".to_owned());
    ruled.family = ControlFamily::Route;
    let mut derived = control_row("effective_fader");
    derived.class = ControlClass::Derived;
    derived.automatable = false;
    derived.scope = ControlScope::Monitor;
    derived.active = false;
    derived.unit = ParameterUnit::Db;
    let controls = [live, toggle, detector, ruled, derived];
    let mut entries = vec![
        ControlEntry {
            handle: 1,
            address: ControlAddress {
                owner_kind: ControlOwnerKind::Strip,
                owner_id: Some("vocal".to_owned()),
                section: ControlSection::Own,
                slot_id: None,
                control: None,
                lane: None,
            },
            body: ControlBody::OwnerHeader(ControlOwnerHeader {
                kind: ControlOwnerHeaderKind::Track,
                name: "Vocal".to_owned(),
                route: None,
            }),
        },
        ControlEntry {
            handle: 2,
            address: ControlAddress {
                owner_kind: ControlOwnerKind::Route,
                owner_id: Some("vocal-main".to_owned()),
                section: ControlSection::Own,
                slot_id: None,
                control: None,
                lane: None,
            },
            body: ControlBody::OwnerHeader(ControlOwnerHeader {
                kind: ControlOwnerHeaderKind::Route,
                name: "vocal-main".to_owned(),
                route: Some(ControlRouteEnds {
                    source: "vocal".to_owned(),
                    destination: "main".to_owned(),
                    tap: session::SendTap::PostFader,
                }),
            }),
        },
        ControlEntry {
            handle: 3,
            address: control_address_of(ControlSection::Builtins, None, None),
            body: ControlBody::SectionHeader(ControlSectionHeader {
                name: "Builtins".to_owned(),
                effect: None,
            }),
        },
        ControlEntry {
            handle: 4,
            address: control_address_of(ControlSection::Inserts, Some("comp"), None),
            body: ControlBody::SectionHeader(ControlSectionHeader {
                name: "Compressor".to_owned(),
                effect: Some("miso.compressor".to_owned()),
            }),
        },
    ];
    for (index, control) in controls.into_iter().enumerate() {
        entries.push(ControlEntry {
            handle: 5 + index as u32,
            address: control_address_of(
                ControlSection::Inserts,
                Some("comp"),
                Some(index as u32 + 1),
            ),
            body: ControlBody::Control(control),
        });
    }
    ControlListPage {
        revision: 7,
        structure_epoch: 3,
        last_handle: 9,
        eof: true,
        entries,
    }
}

fn encode_control_page(codec: &ProtocolCodec, page: &ControlListPage) -> Vec<u8> {
    let mut bytes = vec![
        0;
        codec
            .encoded_control_list_page_len(page)
            .expect("the page is valid")
    ];
    assert_eq!(
        codec.encode_control_list_page(page, &mut bytes),
        Ok(bytes.len())
    );
    bytes
}

fn decode_control_page(
    codec: &ProtocolCodec,
    bytes: &[u8],
    count: u32,
) -> Result<ControlListPage, DecodeError> {
    codec.decode_control_list_page(bytes, count)
}

fn top_level_count(bytes: &[u8]) -> u32 {
    parse_tlvs(bytes).len() as u32
}

/// Gate 1: a request, a list page of every kind of row and every optional field, and a state page
/// round-trip exactly, and every strict prefix of each is refused.
///
/// Red if a field is dropped, reordered or mis-typed on either side, so a kind or an optional
/// field (a disabled value, a ramp row, a reason issue, a route's ends) does not survive.
#[test]
fn control_messages_round_trip_every_kind_and_optional_field() {
    let codec = ProtocolCodec::default();
    for owner in [
        None,
        Some(ControlOwnerFilter {
            kind: ControlOwnerKind::Session,
            id: None,
        }),
        Some(ControlOwnerFilter {
            kind: ControlOwnerKind::Strip,
            id: Some("vocal".to_owned()),
        }),
    ] {
        let request = ControlListRequest {
            structure_epoch: 9,
            after_handle: 4,
            limit: 256,
            owner,
        };
        let mut bytes = vec![
            0;
            codec
                .encoded_control_list_request_len(&request)
                .expect("len")
        ];
        codec
            .encode_control_list_request(&request, &mut bytes)
            .expect("encode");
        let count = top_level_count(&bytes);
        assert_eq!(
            codec.decode_control_list_request(&bytes, count),
            Ok(request)
        );
        for end in 0..bytes.len() {
            assert!(
                codec
                    .decode_control_list_request(&bytes[..end], count)
                    .is_err()
            );
        }
    }

    let page = control_page();
    let bytes = encode_control_page(&codec, &page);
    let count = top_level_count(&bytes);
    assert_eq!(decode_control_page(&codec, &bytes, count), Ok(page.clone()));
    for end in 0..bytes.len() {
        assert!(decode_control_page(&codec, &bytes[..end], count).is_err());
    }
    let mut short = vec![0xa5; bytes.len() - 1];
    assert_eq!(
        codec.encode_control_list_page(&page, &mut short),
        Err(EncodeError::OutputTooSmall {
            required: bytes.len()
        })
    );
    assert!(short.iter().all(|byte| *byte == 0xa5));

    // The state page of the same rows: the control rows only, and a last handle past them.
    let state = ControlStatePage {
        revision: 7,
        structure_epoch: 3,
        last_handle: 12,
        eof: false,
        records: vec![
            ControlStateRecord {
                handle: 5,
                value: 4.0,
            },
            ControlStateRecord {
                handle: 9,
                value: -0.5,
            },
        ],
    };
    let mut state_bytes = vec![0; codec.encoded_control_state_page_len(&state).expect("len")];
    codec
        .encode_control_state_page(&state, &mut state_bytes)
        .expect("encode");
    let state_count = top_level_count(&state_bytes);
    assert_eq!(
        codec.decode_control_state_page(&state_bytes, state_count),
        Ok(state)
    );
    for end in 0..state_bytes.len() {
        assert!(
            codec
                .decode_control_state_page(&state_bytes[..end], state_count)
                .is_err()
        );
    }
}

/// A page of 256 control rows and an empty page are valid at the bound; one more row is not.
///
/// Red if the page bound is off by one on either side, or an empty end-of-list page is refused.
#[test]
fn control_pages_hold_exactly_two_hundred_fifty_six_rows() {
    let codec = ProtocolCodec::default();
    let template = control_page().entries[4].clone();
    let rows = |count: u32| {
        (1..=count)
            .map(|handle| ControlEntry {
                handle,
                ..template.clone()
            })
            .collect::<Vec<_>>()
    };
    let page = |count: u32| ControlListPage {
        revision: 1,
        structure_epoch: 1,
        last_handle: count,
        eof: true,
        entries: rows(count),
    };
    let full = page(256);
    let bytes = encode_control_page(&codec, &full);
    assert_eq!(
        decode_control_page(&codec, &bytes, top_level_count(&bytes)),
        Ok(full)
    );
    let empty = page(0);
    let bytes = encode_control_page(&codec, &empty);
    assert_eq!(
        decode_control_page(&codec, &bytes, top_level_count(&bytes)),
        Ok(empty)
    );
    assert_eq!(
        codec.encoded_control_list_page_len(&page(257)),
        Err(EncodeError::LimitExceeded)
    );
    // The decoder holds the same bound: a 257th well-formed row appended to a full page's bytes is
    // refused, whatever the frame limits would have admitted.
    let bytes = encode_control_page(&codec, &page(256));
    let (patched, count) = edited(&bytes, |fields| {
        let mut last = nth(fields, 5, 255).clone();
        set(&mut last, 1, &257_u32.to_le_bytes());
        fields.push(Tlv {
            id: 5,
            wire: 11,
            mandatory: 1,
            node: Node::Message(last),
        });
        set(fields, 3, &257_u32.to_le_bytes());
    });
    assert_eq!(
        decode_control_page(&codec, &patched, count),
        Err(DecodeError::InvalidField)
    );
}

/// Gate 1: each request rule is refused on encode and on decode alike, and a decoded refusal is
/// `INVALID_FIELD`.
///
/// Red if `limit` 0 or 257 is accepted, a cursor rides with epoch 0 (which names no structure
/// to continue in), an owner ID rides with the session or is missing from
/// another owner, or the two directions disagree on which request is valid.
#[test]
fn control_request_rules_refuse_on_encode_and_decode_alike() {
    let codec = ProtocolCodec::default();
    let valid = ControlListRequest {
        structure_epoch: 0,
        after_handle: 0,
        limit: 8,
        owner: Some(ControlOwnerFilter {
            kind: ControlOwnerKind::Strip,
            id: Some("vocal".to_owned()),
        }),
    };
    let mut bytes = vec![0; 256];
    let length = codec
        .encode_control_list_request(&valid, &mut bytes)
        .expect("valid request");
    bytes.truncate(length);

    type Typed = fn(&mut ControlListRequest);
    type Wire = fn(&mut Vec<Tlv>);
    let rules: [(&str, Typed, Wire); 8] = [
        (
            "limit 0",
            |r| r.limit = 0,
            |f| set(f, 3, &0_u16.to_le_bytes()),
        ),
        (
            "a cursor at structure epoch 0",
            |r| r.after_handle = 3,
            |f| set(f, 2, &3_u32.to_le_bytes()),
        ),
        (
            "limit 257",
            |r| r.limit = 257,
            |f| set(f, 3, &257_u16.to_le_bytes()),
        ),
        (
            "session owner with an ID",
            |r| {
                r.owner = Some(ControlOwnerFilter {
                    kind: ControlOwnerKind::Session,
                    id: Some("vocal".to_owned()),
                });
            },
            |f| set(f, 4, &[1]),
        ),
        (
            "strip owner without an ID",
            |r| r.owner.as_mut().expect("owner").id = None,
            |f| remove(f, 5),
        ),
        (
            "owner ID that is not a stable ID",
            |r| r.owner.as_mut().expect("owner").id = Some("Not A Stable ID".to_owned()),
            |f| set(f, 5, b"Not A Stable ID"),
        ),
        (
            "owner ID without a kind",
            |r| {
                // Not representable: a typed owner has its kind. The encoder's refusal is the
                // session owner's ID rule.
                r.owner = Some(ControlOwnerFilter {
                    kind: ControlOwnerKind::Session,
                    id: Some("vocal".to_owned()),
                });
            },
            |f| remove(f, 4),
        ),
        (
            "owner kind outside the table",
            |r| {
                r.owner = Some(ControlOwnerFilter {
                    kind: ControlOwnerKind::Vca,
                    id: None,
                });
            },
            |f| set(f, 4, &[5]),
        ),
    ];
    for (name, typed, wire) in rules {
        let mut request = valid.clone();
        typed(&mut request);
        assert_eq!(
            codec.encoded_control_list_request_len(&request),
            Err(EncodeError::InvalidField),
            "{name}: encode"
        );
        let (patched, count) = edited(&bytes, wire);
        assert_eq!(
            codec.decode_control_list_request(&patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
        assert_eq!(
            DecodeError::InvalidField.status(),
            crate::StatusCode::InvalidField
        );
    }
}

/// Gate 1: each presence rule of the address and of a row kind (a header with a value, a
/// rebuild-only row with no reason, a control row without its lane) is refused on encode where the
/// typed value can say it and on decode always, and a decoded refusal is `INVALID_FIELD`.
///
/// Red if the codec accepts a row whose fields contradict its kind, or only one direction checks.
#[test]
fn control_entry_rules_refuse_on_encode_and_decode_alike() {
    type Typed = fn(&mut ControlListPage);
    type Wire = fn(&mut Vec<Tlv>);
    fn row(page: &mut ControlListPage, index: usize) -> &mut ControlRow {
        match &mut page.entries[index].body {
            ControlBody::Control(row) => row,
            _ => panic!("entry {index} is a control row"),
        }
    }
    fn f32_bytes(value: f32) -> [u8; 4] {
        value.to_le_bytes()
    }
    fn choice(fields: &mut [Tlv]) -> Tlv {
        let inner = nth(entry(fields, 6), 23, 0).clone();
        Tlv {
            id: 23,
            wire: 11,
            mandatory: 0,
            node: Node::Message(inner),
        }
    }
    let rules: Vec<(&str, Option<Typed>, Wire)> = vec![
        ("handle zero", Some(|p| p.entries[0].handle = 0), |f| {
            set(entry(f, 0), 1, &0_u32.to_le_bytes())
        }),
        (
            "handles do not ascend",
            Some(|p| p.entries[1].handle = 1),
            |f| set(entry(f, 1), 1, &1_u32.to_le_bytes()),
        ),
        (
            "last handle is not the last entry's",
            Some(|p| p.last_handle = 8),
            |f| set(f, 3, &8_u32.to_le_bytes()),
        ),
        (
            "session owner with an ID",
            Some(|p| p.entries[4].address.owner_kind = ControlOwnerKind::Session),
            |f| set(address(f, 4), 1, &[1]),
        ),
        (
            "strip owner without an ID",
            Some(|p| p.entries[4].address.owner_id = None),
            |f| remove(address(f, 4), 2),
        ),
        (
            "owner ID that is not a stable ID",
            Some(|p| p.entries[4].address.owner_id = Some("Vocal Track".to_owned())),
            |f| set(address(f, 4), 2, b"Vocal Track"),
        ),
        (
            "inserts section without a slot",
            Some(|p| p.entries[3].address.slot_id = None),
            |f| remove(address(f, 3), 4),
        ),
        (
            "builtins section with a slot",
            Some(|p| p.entries[2].address.slot_id = Some("comp".to_owned())),
            |f| insert(address(f, 2), leaf(4, 9, b"comp")),
        ),
        (
            "console section without a slot",
            Some(|p| {
                p.entries[3].address.section = ControlSection::Console;
                p.entries[3].address.slot_id = None;
            }),
            |f| {
                set(address(f, 3), 3, &[ControlSection::Console.code()]);
                remove(address(f, 3), 4);
            },
        ),
        (
            "own section with a slot",
            Some(|p| p.entries[0].address.slot_id = Some("comp".to_owned())),
            |f| insert(address(f, 0), leaf(4, 9, b"comp")),
        ),
        (
            "control row without a control ID",
            Some(|p| p.entries[4].address.control = None),
            |f| remove(address(f, 4), 5),
        ),
        (
            "control row without a lane",
            Some(|p| p.entries[4].address.lane = None),
            |f| remove(address(f, 4), 6),
        ),
        (
            "header row with a control ID",
            Some(|p| p.entries[2].address.control = Some(1)),
            |f| insert(address(f, 2), leaf(5, 3, &1_u32.to_le_bytes())),
        ),
        (
            "header row with a lane",
            Some(|p| p.entries[2].address.lane = Some(ParameterChannel::Left)),
            |f| insert(address(f, 2), leaf(6, 1, &[1])),
        ),
        (
            "owner header outside the own section",
            Some(|p| p.entries[0].address.section = ControlSection::Builtins),
            |f| set(address(f, 0), 3, &[4]),
        ),
        (
            "owner header kind contradicts the owner",
            Some(|p| {
                if let ControlBody::OwnerHeader(header) = &mut p.entries[0].body {
                    header.kind = ControlOwnerHeaderKind::Vca;
                }
            }),
            |f| set(entry(f, 0), 4, &[4]),
        ),
        (
            "route owner without its ends",
            Some(|p| {
                if let ControlBody::OwnerHeader(header) = &mut p.entries[1].body {
                    header.route = None;
                }
            }),
            |f| remove(entry(f, 1), 7),
        ),
        (
            "other owner with route ends",
            Some(|p| {
                if let ControlBody::OwnerHeader(header) = &mut p.entries[0].body {
                    header.route = Some(ControlRouteEnds {
                        source: "a".to_owned(),
                        destination: "b".to_owned(),
                        tap: session::SendTap::Input,
                    });
                }
            }),
            |f| insert(entry(f, 0), leaf(7, 9, b"a")),
        ),
        ("section header kind contradicts the section", None, |f| {
            set(entry(f, 2), 4, &[5])
        }),
        (
            "effect on a builtins section header",
            Some(|p| {
                if let ControlBody::SectionHeader(header) = &mut p.entries[2].body {
                    header.effect = Some("miso.compressor".to_owned());
                }
            }),
            |f| insert(entry(f, 2), leaf(6, 9, b"miso.compressor")),
        ),
        ("effect on an owner header", None, |f| {
            insert(entry(f, 0), leaf(6, 9, b"miso.compressor"))
        }),
        ("owner header with a value", None, |f| {
            insert(entry(f, 0), leaf(34, 6, &f32_bytes(1.0)))
        }),
        ("section header with a name of a control", None, |f| {
            insert(entry(f, 3), leaf(10, 9, b"ratio"))
        }),
        ("control row with a header kind", None, |f| {
            insert(entry(f, 4), leaf(4, 1, &[1]))
        }),
        ("control row with route ends", None, |f| {
            insert(entry(f, 4), leaf(9, 1, &[1]))
        }),
        (
            "rebuild-only row without a reason",
            Some(|p| row(p, 6).reason = None),
            |f| remove(entry(f, 6), 27),
        ),
        (
            "rebuild-only row without a reopen condition",
            Some(|p| row(p, 6).reopen = None),
            |f| remove(entry(f, 6), 29),
        ),
        (
            "live row with a reason",
            Some(|p| row(p, 4).reason = Some(ControlReason::NewGraph)),
            |f| insert(entry(f, 4), leaf(27, 1, &[2])),
        ),
        (
            "live row with a reopen condition",
            Some(|p| row(p, 4).reopen = Some("never".to_owned())),
            |f| insert(entry(f, 4), leaf(29, 9, b"never")),
        ),
        (
            "ruled-live row without its issue",
            Some(|p| row(p, 7).reason_issue = None),
            |f| remove(entry(f, 7), 28),
        ),
        (
            "issue on another reason",
            Some(|p| row(p, 6).reason_issue = Some(1)),
            |f| insert(entry(f, 6), leaf(28, 3, &1_u32.to_le_bytes())),
        ),
        (
            "automatable row that is not live",
            Some(|p| row(p, 6).automatable = true),
            |f| set(entry(f, 6), 31, &[1]),
        ),
        (
            "automatable row in the monitor scope",
            Some(|p| row(p, 4).scope = ControlScope::Monitor),
            |f| set(entry(f, 4), 30, &[2]),
        ),
        (
            "enumeration without choices",
            Some(|p| row(p, 6).choices.clear()),
            |f| entry(f, 6).retain(|field| field.id != 23),
        ),
        (
            "enumeration default outside its choices",
            Some(|p| row(p, 6).default = 5.0),
            |f| set(entry(f, 6), 18, &f32_bytes(5.0)),
        ),
        (
            "enumeration value outside its choices",
            Some(|p| row(p, 6).value = 5.0),
            |f| set(entry(f, 6), 34, &f32_bytes(5.0)),
        ),
        (
            "enumeration with a duplicate choice",
            Some(|p| {
                let duplicate = row(p, 6).choices[0].clone();
                row(p, 6).choices.push(duplicate);
            }),
            |f| {
                let duplicate = choice(f);
                insert_again(entry(f, 6), duplicate);
            },
        ),
        (
            "continuous row with choices",
            Some(|p| {
                row(p, 4).choices.push(EnumChoice {
                    value: 0.0,
                    label: "x".to_owned(),
                });
            }),
            |f| {
                let extra = choice(f);
                insert(entry(f, 4), extra);
            },
        ),
        ("range inverted", Some(|p| row(p, 4).minimum = 100.0), |f| {
            set(entry(f, 4), 15, &f32_bytes(100.0))
        }),
        (
            "value not finite",
            Some(|p| row(p, 4).value = f32::NAN),
            |f| set(entry(f, 4), 34, &f32_bytes(f32::NAN)),
        ),
        (
            "default not finite",
            Some(|p| row(p, 4).default = f32::INFINITY),
            |f| set(entry(f, 4), 18, &f32_bytes(f32::INFINITY)),
        ),
        (
            "disabled value not finite",
            Some(|p| row(p, 4).disabled_value = Some(f32::NAN)),
            |f| set(entry(f, 4), 17, &f32_bytes(f32::NAN)),
        ),
        (
            "empty machine name",
            Some(|p| row(p, 4).name.clear()),
            |f| set(entry(f, 4), 10, b""),
        ),
    ];
    let codec = ProtocolCodec::default();
    let page = control_page();
    let bytes = encode_control_page(&codec, &page);
    for (name, typed, wire) in rules {
        if let Some(typed) = typed {
            let mut broken = page.clone();
            typed(&mut broken);
            assert_eq!(
                codec.encoded_control_list_page_len(&broken),
                Err(EncodeError::InvalidField),
                "{name}: encode"
            );
        }
        let (patched, count) = edited(&bytes, wire);
        assert_eq!(
            decode_control_page(&codec, &patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
    }

    // A control row that lacks any one of its required fields is refused.
    for id in [
        10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 25, 26, 30, 31, 32, 33, 34,
    ] {
        let (patched, count) = edited(&bytes, |f| remove(entry(f, 4), id));
        assert_eq!(
            decode_control_page(&codec, &patched, count),
            Err(DecodeError::InvalidField),
            "control row without field {id}"
        );
    }
    // A header row that lacks its kind or its name is refused.
    for id in [4, 5] {
        let (patched, count) = edited(&bytes, |f| remove(entry(f, 2), id));
        assert_eq!(
            decode_control_page(&codec, &patched, count),
            Err(DecodeError::InvalidField),
            "header row without field {id}"
        );
    }
}

/// Gate 1: a code outside any control code table is `INVALID_FIELD`, wherever it rides.
///
/// Red if a table accepts a code it does not allocate, or maps one to another variant.
#[test]
fn control_codes_outside_their_tables_are_invalid_fields() {
    let codec = ProtocolCodec::default();
    let bytes = encode_control_page(&codec, &control_page());
    // (entry, field path inside the entry, codes outside the table)
    let cases: [(usize, &[u16], &[u8]); 15] = [
        (4, &[2], &[0, 4, 255]),
        (4, &[3, 1], &[0, 5]),
        (4, &[3, 3], &[0, 1, 3, 7]),
        (4, &[3, 6], &[0, 4]),
        (0, &[4], &[0, 6]),
        (1, &[9], &[0, 8]),
        (4, &[13], &[0, 7]),
        (4, &[14], &[0, 4]),
        (4, &[19], &[0, 6]),
        (4, &[24], &[0, 16]),
        (4, &[26], &[0, 4]),
        (6, &[27], &[0, 9]),
        (4, &[30], &[0, 3]),
        (4, &[32], &[0, 7]),
        (2, &[4], &[0, 1, 3, 7]),
    ];
    for (index, path, invalid) in cases {
        for code in invalid {
            let (patched, count) = edited(&bytes, |f| {
                let mut fields = entry(f, index);
                for id in &path[..path.len() - 1] {
                    fields = nth(fields, *id, 0);
                }
                set(fields, path[path.len() - 1], &[*code]);
            });
            assert_eq!(
                decode_control_page(&codec, &patched, count),
                Err(DecodeError::InvalidField),
                "entry {index} field {path:?} code {code}"
            );
        }
    }
}

/// Gate 1: the state page refuses descending or repeated handles, a handle of zero, a wrong
/// record width, a count that disagrees with the bytes, a value that is not finite and a last
/// handle before its last record, on encode and on decode alike.
///
/// Red if the fixed eight-byte record layout drifts, or an unordered page is acknowledged.
#[test]
fn control_state_page_rules_refuse_on_encode_and_decode_alike() {
    let codec = ProtocolCodec::default();
    let page = ControlStatePage {
        revision: 7,
        structure_epoch: 3,
        last_handle: 6,
        eof: true,
        records: vec![
            ControlStateRecord {
                handle: 2,
                value: 1.0,
            },
            ControlStateRecord {
                handle: 5,
                value: 2.0,
            },
        ],
    };
    let mut bytes = vec![0; codec.encoded_control_state_page_len(&page).expect("len")];
    codec
        .encode_control_state_page(&page, &mut bytes)
        .expect("encode");
    // Record bytes: handle 2 = 1.0, handle 5 = 2.0, in eight-byte records.
    let fields = parse_tlvs(&bytes);
    let records = fields.iter().find(|field| field.id == 7).expect("records");
    let Node::Leaf(raw) = &records.node else {
        panic!("records are bytes")
    };
    assert_eq!(raw.len(), 16, "two eight-byte records");
    assert_eq!(&raw[..4], &2_u32.to_le_bytes());
    assert_eq!(&raw[4..8], &1.0_f32.to_le_bytes());

    type Typed = fn(&mut ControlStatePage);
    let encode_rules: [(&str, Typed); 6] = [
        ("descending handles", |p| p.records.reverse()),
        ("repeated handle", |p| p.records[1].handle = 2),
        ("handle zero", |p| p.records[0].handle = 0),
        ("value not finite", |p| p.records[0].value = f32::NAN),
        ("last handle before the last record", |p| p.last_handle = 4),
        ("more than 256 records", |p| {
            p.records = (1..=257)
                .map(|handle| ControlStateRecord { handle, value: 0.0 })
                .collect();
            p.last_handle = 257;
        }),
    ];
    for (name, typed) in encode_rules {
        let mut broken = page.clone();
        typed(&mut broken);
        // Only a row count above a page is an excess a smaller page cures; the rest is the
        // value's own defect.
        let expected = if name == "more than 256 records" {
            EncodeError::LimitExceeded
        } else {
            EncodeError::InvalidField
        };
        assert_eq!(
            codec.encoded_control_state_page_len(&broken),
            Err(expected),
            "{name}: encode"
        );
    }
    type Wire = fn(&mut Vec<Tlv>);
    let swapped = |first: u32, second: u32| {
        let mut raw = Vec::new();
        for (handle, value) in [(first, 1.0_f32), (second, 2.0_f32)] {
            raw.extend_from_slice(&handle.to_le_bytes());
            raw.extend_from_slice(&value.to_le_bytes());
        }
        raw
    };
    let decode_rules: Vec<(&str, Wire)> = vec![
        ("descending handles", |f| {
            set(
                f,
                7,
                &[5, 0, 0, 0, 0, 0, 0, 0x40, 2, 0, 0, 0, 0, 0, 0x80, 0x3f],
            );
        }),
        ("record width 16", |f| set(f, 6, &16_u16.to_le_bytes())),
        ("record width 4", |f| set(f, 6, &4_u16.to_le_bytes())),
        ("count above 256", |f| set(f, 5, &257_u16.to_le_bytes())),
    ];
    for (name, wire) in decode_rules {
        let (patched, count) = edited(&bytes, wire);
        assert_eq!(
            codec.decode_control_state_page(&patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
    }
    // A count that disagrees with the bytes is a length error, not a shorter page.
    let (patched, count) = edited(&bytes, |f| set(f, 5, &1_u16.to_le_bytes()));
    assert_eq!(
        codec.decode_control_state_page(&patched, count),
        Err(DecodeError::InvalidValueLength)
    );
    // A value that is not finite and a handle of zero, built on the wire.
    for (name, raw) in [
        ("NaN value", {
            let mut raw = swapped(2, 5);
            raw[4..8].copy_from_slice(&f32::NAN.to_le_bytes());
            raw
        }),
        ("handle zero", swapped(0, 5)),
        ("repeated handle", swapped(5, 5)),
    ] {
        let (patched, count) = edited(&bytes, |f| set(f, 7, &raw));
        assert_eq!(
            codec.decode_control_state_page(&patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
    }
    let (patched, count) = edited(&bytes, |f| set(f, 3, &4_u32.to_le_bytes()));
    assert_eq!(
        codec.decode_control_state_page(&patched, count),
        Err(DecodeError::InvalidField),
        "last handle before the last record"
    );
}

/// Every control code table allocates exactly its listed codes, in ascending order, and reads
/// each back to its own variant; every other byte is `INVALID_FIELD`. The tables the sprint's
/// rulings pin are checked against their sources: the section codes are `ParameterRack`'s plus
/// `own`, the live-ramp rows run from `fader` 1 to `send matrix` 15.
///
/// Red if a code is renumbered, reused for another variant or left unreachable, which would move
/// a wire code that a frontend or an agent reads.
#[test]
fn control_code_tables_are_exact() {
    fn check<T: Copy + core::fmt::Debug + PartialEq>(
        codes: &[u8],
        from_code: fn(u8) -> Result<T, DecodeError>,
        code: fn(T) -> u8,
    ) {
        assert!(codes.windows(2).all(|pair| pair[0] < pair[1]));
        for raw in 0..=u8::MAX {
            match from_code(raw) {
                Ok(variant) => {
                    assert!(codes.contains(&raw), "code {raw} is allocated");
                    assert_eq!(code(variant), raw);
                }
                Err(error) => {
                    assert!(!codes.contains(&raw), "code {raw} is not allocated");
                    assert_eq!(error, DecodeError::InvalidField);
                }
            }
        }
    }
    check(
        ControlOwnerKind::CODES,
        ControlOwnerKind::from_code,
        ControlOwnerKind::code,
    );
    check(
        ControlSection::CODES,
        ControlSection::from_code,
        ControlSection::code,
    );
    check(
        ControlOwnerHeaderKind::CODES,
        ControlOwnerHeaderKind::from_code,
        ControlOwnerHeaderKind::code,
    );
    check(
        ControlKind::CODES,
        ControlKind::from_code,
        ControlKind::code,
    );
    check(
        ControlTaper::CODES,
        ControlTaper::from_code,
        ControlTaper::code,
    );
    check(
        ControlRampRow::CODES,
        ControlRampRow::from_code,
        ControlRampRow::code,
    );
    check(
        ControlClass::CODES,
        ControlClass::from_code,
        ControlClass::code,
    );
    check(
        ControlReason::CODES,
        ControlReason::from_code,
        ControlReason::code,
    );
    check(
        ControlScope::CODES,
        ControlScope::from_code,
        ControlScope::code,
    );
    check(
        ControlFamily::CODES,
        ControlFamily::from_code,
        ControlFamily::code,
    );

    assert_eq!(ControlOwnerKind::CODES, [1, 2, 3, 4]);
    assert_eq!(ControlSection::Inserts.code(), ParameterRack::Inserts as u8);
    assert_eq!(
        ControlSection::Builtins.code(),
        ParameterRack::Builtins as u8
    );
    assert_eq!(ControlSection::Console.code(), ParameterRack::Console as u8);
    assert_eq!(ControlSection::Own.code(), 6);
    // Every ramp-row code, pinned: the codes are wire contract and a client matches on them, so a
    // swap of two rows must turn this red even though each code still round-trips.
    let ramp_rows = [
        (ControlRampRow::Fader, 1),
        (ControlRampRow::InputTrim, 2),
        (ControlRampRow::SendGain, 3),
        (ControlRampRow::VcaOffset, 4),
        (ControlRampRow::LinkGlide, 5),
        (ControlRampRow::Mute, 6),
        (ControlRampRow::Solo, 7),
        (ControlRampRow::PolarityInvert, 8),
        (ControlRampRow::SendMute, 9),
        (ControlRampRow::SendFollowsMute, 10),
        (ControlRampRow::VcaMute, 11),
        (ControlRampRow::BypassCrossfade, 12),
        (ControlRampRow::Pan, 13),
        (ControlRampRow::Matrix, 14),
        (ControlRampRow::SendMatrix, 15),
    ];
    assert_eq!(ramp_rows.len(), ControlRampRow::CODES.len());
    for (row, code) in ramp_rows {
        assert_eq!(row.code(), code, "{row:?}");
        assert_eq!(ControlRampRow::from_code(code), Ok(row));
    }
    assert_eq!(ControlReason::RuledLiveNotBuilt.code(), 8);
}

/// Gate 2: a retired message ID decodes to `UNSUPPORTED_MESSAGE`, no message variant uses a
/// retired code, and a capability registry that lists one is refused.
///
/// Red if `0x0004` or `0x0005` is reallocated to a new message or still served, or a peer can
/// advertise one.
#[test]
fn retired_message_ids_are_never_reused_or_advertised() {
    assert_eq!(crate::RETIRED_MESSAGE_IDS, [0x0004, 0x0005]);
    for raw in 0..=u16::MAX {
        match crate::MessageId::from_raw(raw) {
            Ok(message) => {
                assert_eq!(message.raw(), raw);
                assert!(
                    !crate::RETIRED_MESSAGE_IDS.contains(&raw),
                    "a message uses the retired code {raw:#06x}"
                );
            }
            Err(error) => {
                if crate::RETIRED_MESSAGE_IDS.contains(&raw) {
                    assert_eq!(error, DecodeError::UnsupportedMessage);
                }
            }
        }
    }
    assert!(!allocated_id(0x0004, false) && !allocated_id(0x0005, false));
    assert!(
        [1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14]
            .into_iter()
            .all(|id| allocated_id(id, false))
    );
    assert!(!allocated_id(0, false) && !allocated_id(15, false));
}

fn automation_record(sample: u64, handle: u32, value: f32) -> crate::AutomationRecord {
    crate::AutomationRecord {
        kind: crate::AutomationKind::Point,
        handle: crate::ParameterHandle(handle),
        start: crate::SampleTime(sample),
        end: crate::SampleTime(sample),
        start_value: value,
        end_value: value,
    }
}

#[test]
fn b2b_automation_goldens_truncation_and_malformed_cases_are_functional() {
    let codec = codec();
    let records = [automation_record(2, 1, 1.0)];
    let request = AutomationEnqueue { records: &records };
    let required = codec
        .encoded_automation_enqueue_len(request)
        .expect("request length");
    assert_eq!(required, 72);
    let mut bytes = [0_u8; 72];
    assert_eq!(
        codec.encode_automation_enqueue(request, &mut bytes),
        Ok(required)
    );
    assert_eq!(
        hex(&bytes),
        concat!(
            "01000201020000000100000000000000",
            "02000201020000002000000000000000",
            "03000a01200000000100000001000000",
            "02000000000000000200000000000000",
            "0000803f0000803f"
        )
    );
    let decoded = codec
        .decode_automation_enqueue(&bytes, 3)
        .expect("strict decode");
    assert_eq!(decoded.count, 1);
    assert_eq!(decoded.record(0), Ok(records[0]));
    assert_eq!(
        decoded
            .into_batch(
                crate::SessionRevision(7),
                crate::RequestId::new(9).expect("id")
            )
            .expect("slot")
            .as_slice(),
        &records
    );
    for end in 0..bytes.len() {
        assert!(codec.decode_automation_enqueue(&bytes[..end], 3).is_err());
    }
    let success = AutomationEnqueued {
        accepted_records: 1,
        occupancy: 1,
        capacity: 2,
        generation: 2,
    };
    let mut success_bytes = [0_u8; 64];
    codec
        .encode_automation_enqueued(success, &mut success_bytes)
        .expect("success encode");
    assert_eq!(
        hex(&success_bytes),
        concat!(
            "01000201020000000100000000000000",
            "02000401080000000100000000000000",
            "03000401080000000200000000000000",
            "04000401080000000200000000000000"
        )
    );
    assert_eq!(
        codec.decode_automation_enqueued(&success_bytes, 4),
        Ok(success)
    );
    for end in 0..success_bytes.len() {
        assert!(
            codec
                .decode_automation_enqueued(&success_bytes[..end], 4)
                .is_err()
        );
    }

    let mut invalid = bytes;
    invalid[8] = 0; // count zero
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    invalid = bytes;
    invalid[24] = 31; // stride must be 32
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    invalid = bytes;
    invalid[36] = 31; // exact count * stride byte length
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    invalid = bytes;
    invalid[44..48].fill(0); // zero public handle
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    invalid = bytes;
    invalid[35] = 0; // known field with optional flag
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    invalid = bytes;
    invalid[34] = WIRE_U32; // known field wire type
    assert!(codec.decode_automation_enqueue(&invalid, 3).is_err());
    let mut extension = bytes.to_vec();
    extension.extend_from_slice(&[4, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
    assert!(codec.decode_automation_enqueue(&extension, 4).is_ok());
    extension[72 + 3] = 1;
    assert_eq!(
        codec.decode_automation_enqueue(&extension, 4),
        Err(DecodeError::UnknownRequiredField)
    );
    let ordered = [automation_record(3, 1, 1.0), automation_record(4, 2, 1.0)];
    let mut ordered_bytes = [0_u8; 104];
    codec
        .encode_automation_enqueue(AutomationEnqueue { records: &ordered }, &mut ordered_bytes)
        .expect("ordered encode");
    ordered_bytes[80..88].copy_from_slice(&2_u64.to_le_bytes());
    ordered_bytes[88..96].copy_from_slice(&2_u64.to_le_bytes());
    assert!(codec.decode_automation_enqueue(&ordered_bytes, 3).is_err());
    let adjacent = [
        crate::AutomationRecord {
            kind: crate::AutomationKind::Linear,
            handle: crate::ParameterHandle(1),
            start: crate::SampleTime(3),
            end: crate::SampleTime(5),
            start_value: 0.0,
            end_value: 1.0,
        },
        crate::AutomationRecord {
            kind: crate::AutomationKind::Linear,
            handle: crate::ParameterHandle(1),
            start: crate::SampleTime(5),
            end: crate::SampleTime(7),
            start_value: 1.0,
            end_value: 0.0,
        },
    ];
    let mut adjacent_bytes = [0_u8; 104];
    codec
        .encode_automation_enqueue(
            AutomationEnqueue { records: &adjacent },
            &mut adjacent_bytes,
        )
        .expect("adjacent encode");
    adjacent_bytes[80..88].copy_from_slice(&4_u64.to_le_bytes());
    assert!(codec.decode_automation_enqueue(&adjacent_bytes, 3).is_err());

    let queue_config = crate::ProtocolQueueConfig {
        control_command_slots: core::num::NonZeroUsize::new(1).expect("one"),
        control_command_bytes: core::num::NonZeroUsize::new(1).expect("one"),
        automation_batch_slots: core::num::NonZeroUsize::new(1).expect("one"),
        reliable_response_slots: core::num::NonZeroUsize::new(1).expect("one"),
        reliable_event_slots: core::num::NonZeroUsize::new(1).expect("one"),
        telemetry_slots: core::num::NonZeroUsize::new(1).expect("one"),
        per_block_automation_density: core::num::NonZeroUsize::new(256).expect("density"),
        quantum_frames: core::num::NonZeroUsize::new(1).expect("quantum"),
    };
    let mut queues = crate::ProtocolQueues::prepare(queue_config).expect("prepared");
    let mut output = vec![0_u8; 16 + 16 + 8 + crate::AUTOMATION_BATCH_RECORDS * 32];
    let audit_codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: 16 * 1024,
        ..codec.limits()
    });
    let mut next = 0_u64;
    for batch_index in 0..40_u64 {
        let count = if batch_index == 39 { 16 } else { 256 };
        let mut batch = [crate::AutomationRecord::EMPTY; crate::AUTOMATION_BATCH_RECORDS];
        for record in &mut batch[..count] {
            *record = automation_record(next, 1, 1.0);
            next += 1;
        }
        let value = AutomationEnqueue {
            records: &batch[..count],
        };
        output.fill(0xa5);
        let len = audit_codec
            .encode_automation_enqueue(value, &mut output)
            .expect("encode");
        assert!(output[len..].iter().all(|byte| *byte == 0xa5));
        let decoded = audit_codec
            .decode_automation_enqueue(&output[..len], 3)
            .expect("decode");
        let slot = decoded
            .into_batch(
                crate::SessionRevision(7),
                crate::RequestId::new(batch_index + 1).expect("id"),
            )
            .expect("slot");
        queues
            .try_enqueue_automation(crate::SampleTime(0), slot)
            .expect("enqueue");
        assert_eq!(
            queues.try_dequeue_automation().expect("consume").len as usize,
            count
        );
    }
    assert_eq!(next, 10_000);
}

#[test]
fn b3a_transport_goldens_truncation_and_direct_codec_are_strict() {
    let codec = codec();
    let mut empty = [0xaa; 1];
    assert_eq!(codec.encode_transport_get_request(&mut empty), Ok(0));
    assert_eq!(empty, [0xaa]);
    assert_eq!(codec.decode_transport_get_request(&[], 0), Ok(()));

    let set = TransportSetRequest {
        state: TransportState::Playing,
        position: Some(crate::SampleTime(2)),
    };
    let mut set_bytes = [0_u8; 32];
    codec
        .encode_transport_set_request(set, &mut set_bytes)
        .expect("set encode");
    assert_eq!(
        hex(&set_bytes),
        concat!(
            "01000101010000000200000000000000",
            "02000400080000000200000000000000"
        )
    );
    assert_eq!(codec.decode_transport_set_request(&set_bytes, 2), Ok(set));
    let mut short = [0xaa; 31];
    assert_eq!(
        codec.encode_transport_set_request(set, &mut short),
        Err(EncodeError::OutputTooSmall { required: 32 })
    );
    assert!(short.iter().all(|byte| *byte == 0xaa));
    for end in 0..set_bytes.len() {
        assert!(
            codec
                .decode_transport_set_request(&set_bytes[..end], 2)
                .is_err()
        );
    }
    let mut bad = set_bytes;
    bad[8] = 3;
    assert!(codec.decode_transport_set_request(&bad, 2).is_err());
    bad = set_bytes;
    bad[3] = 0;
    assert!(codec.decode_transport_set_request(&bad, 2).is_err());
    let mut extension = set_bytes.to_vec();
    extension.extend_from_slice(&[3, 0, WIRE_U8, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        codec.decode_transport_set_request(&extension, 3),
        Ok(set),
        "unknown optional fields are skippable"
    );
    extension[32 + 3] = 1;
    assert_eq!(
        codec.decode_transport_set_request(&extension, 3),
        Err(DecodeError::UnknownRequiredField)
    );

    let snapshot = TransportSnapshot {
        state: TransportState::Stopped,
        position: crate::SampleTime(2),
        effective_sample: crate::SampleTime(3),
    };
    let mut snapshot_bytes = [0_u8; 48];
    codec
        .encode_transport_snapshot(snapshot, &mut snapshot_bytes)
        .expect("snapshot encode");
    assert_eq!(
        hex(&snapshot_bytes),
        concat!(
            "01000101010000000100000000000000",
            "02000401080000000200000000000000",
            "03000401080000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_transport_snapshot(&snapshot_bytes, 3),
        Ok(snapshot)
    );
    for end in 0..snapshot_bytes.len() {
        assert!(
            codec
                .decode_transport_snapshot(&snapshot_bytes[..end], 3)
                .is_err()
        );
    }

    let event = TransportStateEvent {
        event_sequence: 1,
        state: TransportState::Playing,
        position: crate::SampleTime(2),
        effective_sample: crate::SampleTime(3),
        origin_request_id: crate::RequestId::new(9),
    };
    let mut event_bytes = [0_u8; 80];
    codec
        .encode_transport_state_event(event, &mut event_bytes)
        .expect("event encode");
    assert_eq!(
        hex(&event_bytes),
        concat!(
            "01000401080000000100000000000000",
            "02000101010000000200000000000000",
            "03000401080000000200000000000000",
            "04000401080000000300000000000000",
            "05000400080000000900000000000000"
        )
    );
    assert_eq!(
        codec.decode_transport_state_event(&event_bytes, 5),
        Ok(event)
    );
    for end in 0..event_bytes.len() {
        assert!(
            codec
                .decode_transport_state_event(&event_bytes[..end], 5)
                .is_err()
        );
    }
    let no_origin = TransportStateEvent {
        origin_request_id: None,
        ..event
    };
    let mut no_origin_bytes = [0_u8; 64];
    codec
        .encode_transport_state_event(no_origin, &mut no_origin_bytes)
        .expect("no origin encode");
    assert_eq!(
        codec.decode_transport_state_event(&no_origin_bytes, 4),
        Ok(no_origin)
    );
}

/// Issue #1351 gate 4: every registered counter ID, the newest `TRANSITION_REPREPARE_REFUSALS`
/// included, round-trips through the telemetry configuration, and the next ID is refused.
///
/// Test value: red if a counter joins the enum but not `parse_counter_id`, so that a host's
/// configuration of it decodes as malformed.
#[test]
fn every_counter_id_round_trips_and_the_next_is_refused() {
    let codec = codec();
    let all = [
        CounterId::ControlCommandBackpressure,
        CounterId::AutomationBackpressure,
        CounterId::ReliableResponseBackpressure,
        CounterId::ReliableEventBackpressure,
        CounterId::TelemetryCoalesced,
        CounterId::TelemetryDropped,
        CounterId::MalformedFrames,
        CounterId::ReplayHits,
        CounterId::RequestIdReuse,
        CounterId::ReplayExpired,
        CounterId::LateAutomation,
        CounterId::CanceledAutomation,
        CounterId::AutomationTimePast,
        CounterId::AutomationOrderReject,
        CounterId::ValidationFailures,
        CounterId::LiveValuesSuperseded,
        CounterId::TransitionReprepareRefusals,
    ];
    for (index, id) in all.iter().enumerate() {
        assert_eq!(*id as usize, index + 1, "the registry has no gap or reuse");
    }
    let configuration = TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: all.to_vec(),
        counter_period_blocks: 1,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    };
    let mut bytes = [0_u8; 192];
    let len = codec
        .encode_telemetry_configuration(&configuration, &mut bytes)
        .expect("configuration");
    assert_eq!(
        codec.decode_telemetry_configuration(&bytes[..len], 6),
        Ok(configuration)
    );
    let next = u32::try_from(all.len() + 1).expect("small");
    let mut refused = bytes;
    let id_bytes = (all.len() as u32).to_le_bytes();
    let at = refused[..len]
        .windows(4)
        .position(|window| window == id_bytes)
        .expect("last counter ID");
    refused[at..at + 4].copy_from_slice(&next.to_le_bytes());
    assert!(
        codec
            .decode_telemetry_configuration(&refused[..len], 6)
            .is_err()
    );
}

#[test]
fn b3b1_telemetry_and_counters_are_typed_canonical_and_bounded() {
    let codec = codec();
    let configuration = TelemetryConfiguration {
        meter_handles: vec![1, 2],
        meter_period_blocks: 4,
        counter_ids: vec![
            CounterId::ControlCommandBackpressure,
            CounterId::TelemetryCoalesced,
        ],
        counter_period_blocks: 8,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: DiagnosticSeverity::Error,
    };
    let mut config_bytes = [0_u8; 96];
    codec
        .encode_telemetry_configuration(&configuration, &mut config_bytes)
        .expect("config");
    assert_eq!(
        hex(&config_bytes),
        concat!(
            "01000d01080000000100000002000000",
            "02000301040000000400000000000000",
            "03000d01080000000100000005000000",
            "04000301040000000800000000000000",
            "05000801010000000100000000000000",
            "06000101010000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_telemetry_configuration(&config_bytes, 6),
        Ok(configuration.clone())
    );
    for end in 0..config_bytes.len() {
        assert!(
            codec
                .decode_telemetry_configuration(&config_bytes[..end], 6)
                .is_err()
        );
    }
    let bad_config = TelemetryConfiguration {
        meter_handles: vec![2, 1],
        ..configuration.clone()
    };
    assert!(
        codec
            .encoded_telemetry_configuration_len(&bad_config)
            .is_err()
    );
    let bad_coupling = TelemetryConfiguration {
        meter_period_blocks: 0,
        ..configuration.clone()
    };
    assert!(
        codec
            .encoded_telemetry_configuration_len(&bad_coupling)
            .is_err()
    );

    let request = CountersRequest {
        all: false,
        ids: vec![1, 5],
    };
    let mut request_bytes = [0_u8; 32];
    codec
        .encode_counters_request(&request, &mut request_bytes)
        .expect("request");
    assert_eq!(
        hex(&request_bytes),
        concat!(
            "01000801010000000000000000000000",
            "02000d00080000000100000005000000"
        )
    );
    assert_eq!(
        codec.decode_counters_request(&request_bytes, 2),
        Ok(request.clone())
    );
    for end in 0..request_bytes.len() {
        assert!(
            codec
                .decode_counters_request(&request_bytes[..end], 2)
                .is_err()
        );
    }
    assert!(
        codec
            .encoded_counters_request_len(&CountersRequest {
                all: true,
                ids: vec![8]
            })
            .is_err()
    );
    assert!(
        codec
            .encoded_counters_request_len(&CountersRequest {
                all: false,
                ids: vec![]
            })
            .is_err()
    );

    let snapshot = CounterSnapshot {
        observed_sample: crate::SampleTime(9),
        values: vec![
            CounterValue {
                id: CounterId::ControlCommandBackpressure,
                value: 2,
            },
            CounterValue {
                id: CounterId::TelemetryCoalesced,
                value: 3,
            },
        ],
    };
    let mut snapshot_bytes = vec![0; codec.encoded_counter_snapshot_len(&snapshot).expect("size")];
    codec
        .encode_counter_snapshot(&snapshot, &mut snapshot_bytes)
        .expect("snapshot");
    assert_eq!(
        codec.decode_counter_snapshot(&snapshot_bytes, 3),
        Ok(snapshot.clone())
    );
    for end in 0..snapshot_bytes.len() {
        assert!(
            codec
                .decode_counter_snapshot(&snapshot_bytes[..end], 3)
                .is_err()
        );
    }
    let mut unknown = request_bytes;
    unknown[28] = 99;
    assert_eq!(
        codec.decode_counters_request(&unknown, 2),
        Ok(CountersRequest {
            all: false,
            ids: vec![1, 99]
        })
    );
}

#[test]
fn b3b2_diagnostics_pages_are_canonical_bounded_and_strict() {
    let codec = codec();
    let request = DiagnosticsRequest {
        after_sequence: 2,
        limit: 3,
        minimum_severity: DiagnosticSeverity::Warning,
    };
    let mut request_bytes = [0_u8; 48];
    codec
        .encode_diagnostics_request(request, &mut request_bytes)
        .expect("request");
    assert_eq!(
        hex(&request_bytes),
        concat!(
            "01000401080000000200000000000000",
            "02000201020000000300000000000000",
            "03000101010000000200000000000000"
        )
    );
    assert_eq!(
        codec.decode_diagnostics_request(&request_bytes, 3),
        Ok(request)
    );
    for end in 0..request_bytes.len() {
        assert!(
            codec
                .decode_diagnostics_request(&request_bytes[..end], 3)
                .is_err()
        );
    }
    let mut short = [0xaa; 47];
    assert_eq!(
        codec.encode_diagnostics_request(request, &mut short),
        Err(EncodeError::OutputTooSmall { required: 48 })
    );
    assert!(short.iter().all(|byte| *byte == 0xaa));

    let page_diagnostic = Diagnostic {
        code: "a.b".to_owned(),
        severity: DiagnosticSeverity::Error,
        path: Vec::new(),
        detail: None,
        operation_index: None,
        sample_time: None,
        provider_sequence: Some(3),
    };
    let page = DiagnosticsPage {
        last_sequence: 3,
        eof: true,
        diagnostics: vec![page_diagnostic.clone()],
    };
    let mut page_bytes = vec![
        0_u8;
        codec
            .encoded_diagnostics_page_len(&page)
            .expect("page length")
    ];
    codec
        .encode_diagnostics_page(&page, &mut page_bytes)
        .expect("page");
    assert_eq!(
        hex(&page_bytes),
        concat!(
            "01000401080000000300000000000000",
            "02000801010000000100000000000000",
            "03000b013800000003000000000000000100090103000000",
            "612e62000000000002000101010000000300000000000000",
            "07000400080000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_diagnostics_page(&page_bytes, 3),
        Ok(page.clone())
    );
    for end in 0..page_bytes.len() {
        assert!(
            codec
                .decode_diagnostics_page(&page_bytes[..end], 3)
                .is_err()
        );
    }

    let mut bad_request = request_bytes;
    bad_request[24..26].fill(0);
    assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
    bad_request = request_bytes;
    bad_request[24..26].copy_from_slice(&257_u16.to_le_bytes());
    assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
    bad_request = request_bytes;
    bad_request[40] = 4;
    assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
    bad_request = request_bytes;
    bad_request[19] = 0;
    assert!(codec.decode_diagnostics_request(&bad_request, 3).is_err());
    let mut request_extension = request_bytes.to_vec();
    request_extension.extend_from_slice(&[4, 0, WIRE_U8, 0, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        codec.decode_diagnostics_request(&request_extension, 4),
        Ok(request),
        "unknown optional request extension is skippable"
    );
    request_extension[48 + 3] = 1;
    assert_eq!(
        codec.decode_diagnostics_request(&request_extension, 4),
        Err(DecodeError::UnknownRequiredField)
    );

    let mut missing_sequence = page_bytes.clone();
    missing_sequence[80..82].copy_from_slice(&8_u16.to_le_bytes());
    assert!(codec.decode_diagnostics_page(&missing_sequence, 3).is_err());
    let mut optional_diagnostic = page_bytes.clone();
    optional_diagnostic[35] = 0;
    assert!(
        codec
            .decode_diagnostics_page(&optional_diagnostic, 3)
            .is_err()
    );
    let mismatched_last = DiagnosticsPage {
        last_sequence: 4,
        ..page.clone()
    };
    assert!(
        codec
            .encoded_diagnostics_page_len(&mismatched_last)
            .is_err()
    );
    let too_many = DiagnosticsPage {
        last_sequence: 257,
        eof: true,
        diagnostics: (1..=257)
            .map(|sequence| Diagnostic {
                provider_sequence: Some(sequence),
                ..page_diagnostic.clone()
            })
            .collect(),
    };
    assert!(codec.encoded_diagnostics_page_len(&too_many).is_err());
    let reversed = DiagnosticsPage {
        last_sequence: 3,
        eof: false,
        diagnostics: vec![
            Diagnostic {
                provider_sequence: Some(4),
                ..page_diagnostic.clone()
            },
            page_diagnostic.clone(),
        ],
    };
    assert!(codec.encoded_diagnostics_page_len(&reversed).is_err());
}

#[test]
fn b4_event_payloads_are_typed_canonical_and_truncation_safe() {
    let codec = codec();
    let canceled = AutomationCanceled {
        event_sequence: 1,
        origin_request_id: crate::RequestId::new(2).expect("id"),
        canceled_records: 3,
        reason: AutomationCancellationReason::RevisionChanged,
        queue_generation: 2,
        effective_sample: Some(crate::SampleTime(9)),
    };
    let mut canceled_bytes = [0_u8; 96];
    codec
        .encode_automation_canceled(canceled, &mut canceled_bytes)
        .expect("cancel");
    assert_eq!(
        hex(&canceled_bytes),
        concat!(
            "01000401080000000100000000000000",
            "02000401080000000200000000000000",
            "03000201020000000300000000000000",
            "04000101010000000100000000000000",
            "05000401080000000200000000000000",
            "06000400080000000900000000000000"
        )
    );
    assert_eq!(
        codec.decode_automation_canceled(&canceled_bytes, 6),
        Ok(canceled)
    );
    for end in 0..canceled_bytes.len() {
        assert!(
            codec
                .decode_automation_canceled(&canceled_bytes[..end], 6)
                .is_err()
        );
    }
    let mut bad_canceled = canceled_bytes;
    bad_canceled[40..42].fill(0);
    assert!(codec.decode_automation_canceled(&bad_canceled, 6).is_err());
    bad_canceled = canceled_bytes;
    bad_canceled[56] = 9;
    assert!(codec.decode_automation_canceled(&bad_canceled, 6).is_err());

    let records = [MeterRecord {
        handle: 1,
        component: MeterComponent::Right,
        flags: 3,
        value: 1.5,
    }];
    let batch = MeterBatch {
        observed_sample: crate::SampleTime(9),
        records: &records,
    };
    let mut meter_bytes = [0_u8; 72];
    codec
        .encode_meter_batch(batch, &mut meter_bytes)
        .expect("meter");
    assert_eq!(
        hex(&meter_bytes),
        concat!(
            "01000401080000000900000000000000",
            "02000201020000000100000000000000",
            "03000201020000001000000000000000",
            "04000a011000000001000000020003000000c03f00000000"
        )
    );
    let decoded = codec
        .decode_meter_batch(&meter_bytes, 4)
        .expect("decoded meter");
    assert_eq!(decoded.observed_sample, crate::SampleTime(9));
    assert_eq!(decoded.record(0), Ok(records[0]));
    for end in 0..meter_bytes.len() {
        assert!(codec.decode_meter_batch(&meter_bytes[..end], 4).is_err());
    }
    let mut bad_meter = meter_bytes;
    bad_meter[40..42].copy_from_slice(&15_u16.to_le_bytes());
    assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());
    bad_meter = meter_bytes;
    bad_meter[64..68].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());
    bad_meter = meter_bytes;
    bad_meter[68] = 1;
    assert!(codec.decode_meter_batch(&bad_meter, 4).is_err());

    let counters = CounterSnapshot {
        observed_sample: crate::SampleTime(9),
        values: vec![CounterValue {
            id: CounterId::TelemetryCoalesced,
            value: 2,
        }],
    };
    let mut counter_bytes = vec![
        0;
        codec
            .encoded_counter_snapshot_len(&counters)
            .expect("counter len")
    ];
    codec
        .encode_counter_snapshot_event(&counters, &mut counter_bytes)
        .expect("counter event");
    assert_eq!(
        codec.decode_counter_snapshot_event(&counter_bytes, 2),
        Ok(counters.clone())
    );
    for end in 0..counter_bytes.len() {
        assert!(
            codec
                .decode_counter_snapshot_event(&counter_bytes[..end], 2)
                .is_err()
        );
    }

    let diagnostic = DiagnosticEvent {
        diagnostic: Diagnostic {
            code: "a.b".to_owned(),
            severity: DiagnosticSeverity::Error,
            path: Vec::new(),
            detail: None,
            operation_index: None,
            sample_time: None,
            provider_sequence: Some(3),
        },
    };
    let mut diagnostic_bytes = [0_u8; 64];
    codec
        .encode_diagnostic_event(&diagnostic, &mut diagnostic_bytes)
        .expect("diagnostic event");
    assert_eq!(
        hex(&diagnostic_bytes),
        concat!(
            "01000b01380000000300000000000000",
            "0100090103000000612e620000000000",
            "02000101010000000300000000000000",
            "07000400080000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_diagnostic_event(&diagnostic_bytes, 1),
        Ok(diagnostic.clone())
    );
    for end in 0..diagnostic_bytes.len() {
        assert!(
            codec
                .decode_diagnostic_event(&diagnostic_bytes[..end], 1)
                .is_err()
        );
    }
    let no_sequence = DiagnosticEvent {
        diagnostic: Diagnostic {
            provider_sequence: None,
            ..diagnostic.diagnostic.clone()
        },
    };
    assert!(codec.encoded_diagnostic_event_len(&no_sequence).is_err());

    let assert_event_frame =
        |message_id: crate::MessageId, payload: &[u8], tlv_count: u32, expected: &str| {
            let mut frame = vec![0_u8; crate::OUTER_HEADER_BYTES + payload.len()];
            codec
                .write_outer_header(
                    &mut frame,
                    crate::FrameKind::Event,
                    message_id,
                    crate::StatusCode::Ok,
                    0,
                    7,
                    0,
                    payload.len() as u32,
                    tlv_count,
                )
                .expect("event header");
            frame[crate::OUTER_HEADER_BYTES..].copy_from_slice(payload);
            assert_eq!(hex(&frame), expected);
            assert_eq!(&frame[crate::OUTER_HEADER_BYTES..], payload);
            for end in 0..frame.len() {
                assert!(
                    codec
                        .decode(
                            &frame[..end],
                            &mut crate::DecodeScratch::new(&mut [0_u16; 8]),
                        )
                        .is_err()
                );
            }
        };
    assert_event_frame(
        crate::MessageId::AutomationCanceled,
        &canceled_bytes,
        6,
        concat!(
            "4d49534f43544c00010000003000030002800000600000000000000000000000",
            "07000000000000000600000000000000",
            "0100040108000000010000000000000002000401080000000200000000000000",
            "0300020102000000030000000000000004000101010000000100000000000000",
            "0500040108000000020000000000000006000400080000000900000000000000"
        ),
    );
    assert_event_frame(
        crate::MessageId::MeterBatch,
        &meter_bytes,
        4,
        concat!(
            "4d49534f43544c00010000003000030020800000480000000000000000000000",
            "07000000000000000400000000000000",
            "0100040108000000090000000000000002000201020000000100000000000000",
            "0300020102000000100000000000000004000a01100000000100000002000300",
            "0000c03f00000000"
        ),
    );
    assert_event_frame(
        crate::MessageId::CounterSnapshot,
        &counter_bytes,
        2,
        concat!(
            "4d49534f43544c00010000003000030021800000400000000000000000000000",
            "07000000000000000200000000000000",
            "0100040108000000090000000000000002000b01280000000200000000000000",
            "0100030104000000050000000000000002000401080000000200000000000000"
        ),
    );
    assert_event_frame(
        crate::MessageId::Diagnostic,
        &diagnostic_bytes,
        1,
        concat!(
            "4d49534f43544c00010000003000030030800000400000000000000000000000",
            "07000000000000000100000000000000",
            "01000b013800000003000000000000000100090103000000612e620000000000",
            "0200010101000000030000000000000007000400080000000300000000000000"
        ),
    );
    let mut forbidden = [0_u8; crate::OUTER_HEADER_BYTES];
    codec
        .write_outer_header(
            &mut forbidden,
            crate::FrameKind::Event,
            crate::MessageId::MeterBatch,
            crate::StatusCode::Ok,
            0,
            7,
            0,
            0,
            0,
        )
        .expect("only a typed event message ID can be encoded");
    forbidden[16..18].copy_from_slice(&0x6000_u16.to_le_bytes());
    assert!(
        matches!(
            codec.decode(&forbidden, &mut crate::DecodeScratch::new(&mut [0_u16; 0]),),
            Err(DecodeError::PcmForbidden)
        ),
        "a reserved PCM ID cannot be represented by a typed event encoder"
    );
}

fn hex(bytes: &[u8]) -> String {
    engine::hex_lower(bytes)
}

/// Batch 2 review fold item 4: the control core's owner, section and lane codes are the wire
/// tables' codes, variant for variant, and the one conversion between the two address types is a
/// round trip that refuses what a control row cannot be.
///
/// Red if either side gains, renumbers or drops a kind without the other (the list would then
/// carry an address that `SET_CONTROL` reads as another row), or if the conversion invents an
/// owner or slot ID.
#[test]
fn the_control_core_and_the_wire_tables_agree_on_every_code() {
    let mut core_owners: Vec<u8> = controls::OwnerKind::ALL
        .map(controls::OwnerKind::code)
        .to_vec();
    core_owners.sort_unstable();
    assert_eq!(core_owners, ControlOwnerKind::CODES);
    for kind in controls::OwnerKind::ALL {
        let wire = ControlOwnerKind::from_controls(kind);
        assert_eq!(wire.code(), kind.code());
        assert_eq!(wire.to_controls(), kind);
        assert_eq!(ControlOwnerKind::from_code(kind.code()), Ok(wire));
    }
    let mut core_sections: Vec<u8> = controls::SectionKind::ALL
        .map(controls::SectionKind::code)
        .to_vec();
    core_sections.sort_unstable();
    assert_eq!(core_sections, ControlSection::CODES);
    for kind in controls::SectionKind::ALL {
        let wire = ControlSection::from_controls(kind);
        assert_eq!(wire.code(), kind.code());
        assert_eq!(ControlSection::from_code(kind.code()), Ok(wire));
    }
    for (lane, code) in [
        (controls::Lane::Left, 1),
        (controls::Lane::Right, 2),
        (controls::Lane::Both, 3),
    ] {
        assert_eq!(ParameterChannel::from_session(lane) as u8, code);
        assert_eq!(schema::session_parameter_channel_wire(lane), code);
    }

    let id = |text: &str| session::StableId::parse(text).expect("stable id");
    let shapes = [
        (controls::Owner::Session, controls::Section::Own),
        (
            controls::Owner::Strip(id("vocal")),
            controls::Section::Builtins,
        ),
        (
            controls::Owner::Strip(id("vocal")),
            controls::Section::Inserts(id("eq")),
        ),
        (
            controls::Owner::Strip(id("vocal")),
            controls::Section::Console(id("desk")),
        ),
        (controls::Owner::Route(id("send")), controls::Section::Own),
        (controls::Owner::Vca(id("grp")), controls::Section::Own),
    ];
    for (owner, section) in shapes {
        let address = controls::ControlAddress {
            owner,
            section,
            control: 9,
            lane: controls::Lane::Left,
        };
        let wire = ControlAddress::from(&address);
        assert_eq!(controls::ControlAddress::try_from(&wire), Ok(address));
    }
    let wire = ControlAddress {
        owner_kind: ControlOwnerKind::Strip,
        owner_id: Some("vocal".to_owned()),
        section: ControlSection::Builtins,
        slot_id: None,
        control: Some(1),
        lane: Some(ParameterChannel::Both),
    };
    assert!(controls::ControlAddress::try_from(&wire).is_ok());
    for (name, broken) in [
        (
            "no control",
            ControlAddress {
                control: None,
                ..wire.clone()
            },
        ),
        (
            "no lane",
            ControlAddress {
                lane: None,
                ..wire.clone()
            },
        ),
        (
            "strip without an ID",
            ControlAddress {
                owner_id: None,
                ..wire.clone()
            },
        ),
        (
            "builtins with a slot",
            ControlAddress {
                slot_id: Some("eq".to_owned()),
                ..wire.clone()
            },
        ),
        (
            "inserts without a slot",
            ControlAddress {
                section: ControlSection::Inserts,
                ..wire.clone()
            },
        ),
        (
            "an ID that is not a stable ID",
            ControlAddress {
                owner_id: Some("Not Stable".to_owned()),
                ..wire.clone()
            },
        ),
    ] {
        assert_eq!(
            controls::ControlAddress::try_from(&broken),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
}

#[test]
fn transaction_applied_edit_path_is_required_and_refuses_unallocated_codes() {
    let codec = codec();
    for (path, code) in [
        (EditPath::Live, 1_u8),
        (EditPath::ModelOnly, 2),
        (EditPath::Rebuild, 3),
        (EditPath::Monitor, 4),
    ] {
        let value = TransactionApplied {
            applied_operations: 5,
            edit_path: path,
        };
        let mut bytes = [0_u8; 32];
        let written = codec
            .encode_transaction_applied(value, &mut bytes)
            .expect("encode");
        assert_eq!(
            bytes[written - 8],
            code,
            "the code is the field's first byte"
        );
        assert_eq!(
            codec.decode_transaction_applied(&bytes[..written], 2),
            Ok(value)
        );
        // Zero and every value above the last allocated code are refused.
        for bad in [0_u8, 5, 6, 255] {
            let mut corrupt = bytes;
            corrupt[written - 8] = bad;
            assert!(
                codec
                    .decode_transaction_applied(&corrupt[..written], 2)
                    .is_err(),
                "edit path {bad} must be refused"
            );
        }
    }
    // Field 2 is required: the one-field payload of the earlier schema is refused.
    let mut old = [0_u8; 16];
    old[..4].copy_from_slice(&[1, 0, 3, 1]);
    old[4..8].copy_from_slice(&4_u32.to_le_bytes());
    old[8..12].copy_from_slice(&3_u32.to_le_bytes());
    assert!(codec.decode_transaction_applied(&old, 1).is_err());
}

// --- RESPONSE_GET (decision 17, C13) ---

/// A required field's TLV.
fn required(id: u16, wire: u8, value: &[u8]) -> Tlv {
    Tlv {
        mandatory: 1,
        ..leaf(id, wire, value)
    }
}

fn response_request(target: ResponseTarget) -> ResponseGetRequest {
    ResponseGetRequest {
        target,
        grid: ResponseGrid::Logarithmic,
        points: 64,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: ParameterChannel::Both,
        sections: false,
        overrides: Vec::new(),
        configuration_id: None,
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    }
}

fn effect_target() -> ResponseTarget {
    ResponseTarget::Configuration(ResponseConfiguration::Effect {
        effect: "miso.parametric-eq".to_owned(),
        quality: session::EffectQuality::High,
        link_mode: session::LinkMode::Maximum,
    })
}

/// A request with every optional field of the effect target.
fn full_effect_request() -> ResponseGetRequest {
    ResponseGetRequest {
        sections: true,
        overrides: vec![
            ResponseOverride {
                control: RESPONSE_BYPASS_CONTROL,
                lane: ParameterChannel::Both,
                value: 1.0,
            },
            ResponseOverride {
                control: 9,
                lane: ParameterChannel::Left,
                value: -6.5,
            },
            ResponseOverride {
                control: 9,
                lane: ParameterChannel::Right,
                value: 2.0,
            },
        ],
        configuration_id: Some(0x0102_0304_0506_0708),
        sample_rate_hz: Some(48_000),
        quantum_frames: Some(128),
        maximum_prepared_bytes: Some(1 << 20),
        maximum_state_bytes: Some(1 << 30),
        maximum_scratch_bytes: Some(1 << 29),
        maximum_automation_spans: Some(4096),
        maximum_reply_bytes: Some(65_536),
        ..response_request(effect_target())
    }
}

fn encode_response_request(codec: &ProtocolCodec, value: &ResponseGetRequest) -> Vec<u8> {
    let mut bytes = vec![0; codec.encoded_response_request_len(value).expect("length")];
    codec
        .encode_response_request(value, &mut bytes)
        .expect("encode");
    bytes
}

/// A curve reply of the given shape, with a distinct finite value in every position.
fn response_curves(
    points: usize,
    channels: ParameterChannel,
    section_count: u32,
    sections: bool,
) -> ResponseCurves {
    let curve = |seed: f32, length: usize| -> Vec<f32> {
        (0..length)
            .map(|index| seed - index as f32 * 0.25)
            .collect()
    };
    let left = matches!(channels, ParameterChannel::Left | ParameterChannel::Both);
    let right = matches!(channels, ParameterChannel::Right | ParameterChannel::Both);
    let section_points = section_count as usize * points;
    ResponseCurves {
        sample_rate_hz: 48_000,
        configuration_id: 42,
        floor_db: -120.0,
        bypass: false,
        section_count,
        enabled_left: 0b101,
        enabled_right: 0b001,
        retained_bytes: 1024,
        frequencies_hz: (0..points)
            .map(|index| 20.0 + index as f32 * 10.0)
            .collect(),
        left_db: left.then(|| curve(1.0, points)),
        right_db: right.then(|| curve(2.0, points)),
        sections_left_db: (left && sections).then(|| curve(3.0, section_points)),
        sections_right_db: (right && sections).then(|| curve(4.0, section_points)),
    }
}

fn encode_response_result(codec: &ProtocolCodec, value: &ResponseResult) -> Vec<u8> {
    let mut bytes = vec![0; codec.encoded_response_result_len(value).expect("length")];
    codec
        .encode_response_result(value, &mut bytes)
        .expect("encode");
    bytes
}

/// Gate 1: a configuration request of both kinds with every optional field, an address request, a
/// strip request, and a curve reply with and without sections, on one side and on both, and a lone
/// tag, all round-trip exactly, and every strict prefix is refused.
///
/// Red if an optional field is dropped, a code table is renumbered, or a reply shape does not
/// survive one pass through the codec.
#[test]
fn response_get_round_trips_every_target_reply_and_optional_field() {
    let codec = ProtocolCodec::default();
    let address = ControlAddress {
        owner_kind: ControlOwnerKind::Strip,
        owner_id: Some("vocal".to_owned()),
        section: ControlSection::Inserts,
        slot_id: Some("eq".to_owned()),
        control: None,
        lane: None,
    };
    for request in [
        full_effect_request(),
        response_request(effect_target()),
        response_request(ResponseTarget::Configuration(
            ResponseConfiguration::InputFilters,
        )),
        ResponseGetRequest {
            grid: ResponseGrid::Linear,
            channels: ParameterChannel::Right,
            sections: true,
            ..response_request(ResponseTarget::Address(address))
        },
        response_request(ResponseTarget::Strip("vocal".to_owned())),
    ] {
        let bytes = encode_response_request(&codec, &request);
        let count = top_level_count(&bytes);
        assert_eq!(codec.decode_response_request(&bytes, count), Ok(request));
        for end in 0..bytes.len() {
            assert!(codec.decode_response_request(&bytes[..end], count).is_err());
        }
        let mut short = vec![0xa5; bytes.len() - 1];
        assert_eq!(
            codec.encode_response_request(&full_effect_request(), &mut short),
            Err(EncodeError::OutputTooSmall {
                required: encode_response_request(&codec, &full_effect_request()).len()
            })
        );
    }
    for (channels, sections) in [
        (ParameterChannel::Both, true),
        (ParameterChannel::Both, false),
        (ParameterChannel::Left, true),
        (ParameterChannel::Right, false),
    ] {
        let value = ResponseResult::Curves(response_curves(5, channels, 3, sections));
        let bytes = encode_response_result(&codec, &value);
        let count = top_level_count(&bytes);
        assert_eq!(codec.decode_response_result(&bytes, count), Ok(value));
        for end in 0..bytes.len() {
            assert!(codec.decode_response_result(&bytes[..end], count).is_err());
        }
    }
    let tag = ResponseResult::Tag(u64::MAX);
    let bytes = encode_response_result(&codec, &tag);
    assert_eq!(codec.decode_response_result(&bytes, 1), Ok(tag));
}

/// Gate 1: each shape rule of the request is refused as `INVALID_FIELD` on decode, and on encode
/// where the typed value can say it: two targets, no target, a strip with `sections` or an
/// override, a strip ID that is not a stable ID, a bypass override on one lane or with a value
/// that is not 0 or 1, and a code outside its table (grid, channels, lane, kind, quality, link
/// mode).
///
/// Red if the codec admits a request the provider could not read, or only one direction checks.
#[test]
fn response_get_request_shape_rules_are_refused_both_ways() {
    let codec = ProtocolCodec::default();
    let effect = full_effect_request();
    let strip = response_request(ResponseTarget::Strip("vocal".to_owned()));
    let effect_bytes = encode_response_request(&codec, &effect);
    let strip_bytes = encode_response_request(&codec, &strip);

    type Typed = fn(&mut ResponseGetRequest);
    type Wire = fn(&mut Vec<Tlv>);
    // (name, base, typed mutation if representable, wire edit)
    let rules: [(&str, bool, Option<Typed>, Wire); 14] = [
        ("no target", true, None, |f| remove(f, 1)),
        ("two targets", true, None, |f| {
            insert(f, leaf(3, 9, b"vocal"))
        }),
        (
            "a strip with sections",
            false,
            Some(|r| r.sections = true),
            |f| set(f, 9, &[1]),
        ),
        (
            "a strip with an override",
            false,
            Some(|r| {
                r.overrides.push(ResponseOverride {
                    control: 3,
                    lane: ParameterChannel::Both,
                    value: 1.0,
                });
            }),
            |f| {
                insert(
                    f,
                    Tlv {
                        id: 10,
                        wire: 11,
                        mandatory: 0,
                        node: Node::Message(vec![
                            required(1, 3, &3_u32.to_le_bytes()),
                            required(2, 1, &[3]),
                            required(3, 6, &1.0_f32.to_le_bytes()),
                        ]),
                    },
                );
            },
        ),
        (
            "a strip ID that is not a stable ID",
            false,
            Some(|r| r.target = ResponseTarget::Strip("Not A Stable ID".to_owned())),
            |f| set(f, 3, b"Not A Stable ID"),
        ),
        (
            "a bypass override on the left lane",
            true,
            Some(|r| r.overrides[0].lane = ParameterChannel::Left),
            |f| set(nth(f, 10, 0), 2, &[1]),
        ),
        (
            "a bypass value of one half",
            true,
            Some(|r| r.overrides[0].value = 0.5),
            |f| set(nth(f, 10, 0), 3, &0.5_f32.to_le_bytes()),
        ),
        ("a grid code outside the table", true, None, |f| {
            set(f, 4, &[3])
        }),
        ("a channels code outside the table", true, None, |f| {
            set(f, 8, &[4])
        }),
        ("an override lane outside the table", true, None, |f| {
            set(nth(f, 10, 1), 2, &[0])
        }),
        ("a configuration kind outside the table", true, None, |f| {
            set(nth(f, 1, 0), 1, &[3])
        }),
        ("a quality outside the table", true, None, |f| {
            set(nth(f, 1, 0), 3, &[4])
        }),
        ("a link mode outside the table", true, None, |f| {
            set(nth(f, 1, 0), 4, &[0])
        }),
        ("an effect kind without its effect", true, None, |f| {
            remove(nth(f, 1, 0), 2)
        }),
    ];
    for (name, effect_base, typed, wire) in rules {
        let (base_bytes, base) = if effect_base {
            (&effect_bytes, &effect)
        } else {
            (&strip_bytes, &strip)
        };
        if let Some(typed) = typed {
            let mut request = base.clone();
            typed(&mut request);
            assert_eq!(
                codec.encoded_response_request_len(&request),
                Err(EncodeError::InvalidField),
                "{name}: encode"
            );
        }
        let (patched, count) = edited(base_bytes, wire);
        assert_eq!(
            codec.decode_response_request(&patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
    }
    // An input-filters configuration that names an effect is refused too.
    let filters = encode_response_request(
        &codec,
        &response_request(ResponseTarget::Configuration(
            ResponseConfiguration::InputFilters,
        )),
    );
    let (patched, count) = edited(&filters, |f| insert(nth(f, 1, 0), leaf(2, 9, b"x")));
    assert_eq!(
        codec.decode_response_request(&patched, count),
        Err(DecodeError::InvalidField)
    );
}

/// Gate 1: each shape rule of the reply is refused on encode and on decode: a curve reply with
/// neither side, a side of the wrong length, section curves without their side's total, section
/// curves of the wrong length, a lone tag with any curve field, and a curve reply without its
/// fields.
///
/// Red if the codec admits a reply whose arrays do not match its declared shape.
#[test]
fn response_get_reply_shape_rules_are_refused_both_ways() {
    let codec = ProtocolCodec::default();
    let base = response_curves(4, ParameterChannel::Both, 2, true);
    let bytes = encode_response_result(&codec, &ResponseResult::Curves(base.clone()));

    type Typed = fn(&mut ResponseCurves);
    type Wire = fn(&mut Vec<Tlv>);
    let rules: [(&str, Typed, Wire); 8] = [
        (
            "neither side",
            |c| {
                c.left_db = None;
                c.right_db = None;
                c.sections_left_db = None;
                c.sections_right_db = None;
            },
            |f| {
                remove(f, 11);
                remove(f, 12);
                remove(f, 13);
                remove(f, 14);
            },
        ),
        (
            "a total of the wrong length",
            |c| {
                c.left_db
                    .as_mut()
                    .expect("left")
                    .pop()
                    .map(drop)
                    .unwrap_or(())
            },
            |f| set(f, 11, &[0; 12]),
        ),
        (
            "section curves without their total",
            |c| c.left_db = None,
            |f| remove(f, 11),
        ),
        (
            "section curves of the wrong length",
            |c| c.sections_right_db.as_mut().expect("right").truncate(7),
            |f| set(f, 14, &[0; 28]),
        ),
        (
            "section curves with no sections declared",
            |c| c.section_count = 0,
            |f| set(f, 6, &0_u32.to_le_bytes()),
        ),
        (
            "a single point",
            |c| {
                c.frequencies_hz.truncate(1);
                c.left_db.as_mut().expect("left").truncate(1);
                c.right_db.as_mut().expect("right").truncate(1);
                c.sections_left_db.as_mut().expect("left").truncate(2);
                c.sections_right_db.as_mut().expect("right").truncate(2);
            },
            |f| {
                set(f, 10, &[0; 4]);
                set(f, 11, &[0; 4]);
                set(f, 12, &[0; 4]);
                set(f, 13, &[0; 8]);
                set(f, 14, &[0; 8]);
            },
        ),
        (
            "a floor that is not finite",
            |c| c.floor_db = f32::NAN,
            |f| set(f, 4, &f32::NAN.to_le_bytes()),
        ),
        (
            "a total that is not finite",
            |c| c.right_db.as_mut().expect("right")[1] = f32::INFINITY,
            |f| {
                let mut values = vec![0_u8; 16];
                values[4..8].copy_from_slice(&f32::INFINITY.to_le_bytes());
                set(f, 12, &values);
            },
        ),
    ];
    for (name, typed, wire) in rules {
        let mut curves = base.clone();
        typed(&mut curves);
        assert_eq!(
            codec.encoded_response_result_len(&ResponseResult::Curves(curves)),
            Err(EncodeError::InvalidField),
            "{name}: encode"
        );
        let (patched, count) = edited(&bytes, wire);
        assert_eq!(
            codec.decode_response_result(&patched, count),
            Err(DecodeError::InvalidField),
            "{name}: decode"
        );
    }
    // A tag beside curve fields, and a packed array whose length is not a whole number of floats.
    let (patched, count) = edited(&bytes, |f| insert(f, leaf(1, 4, &7_u64.to_le_bytes())));
    assert_eq!(
        codec.decode_response_result(&patched, count),
        Err(DecodeError::InvalidField)
    );
    let (patched, count) = edited(&bytes, |f| set(f, 10, &[0; 15]));
    assert_eq!(
        codec.decode_response_result(&patched, count),
        Err(DecodeError::InvalidValueLength)
    );
    // A curve reply with a required field missing is not a reply.
    let (patched, count) = edited(&bytes, |f| remove(f, 2));
    assert!(codec.decode_response_result(&patched, count).is_err());
}

/// `response_result_encoded_len` is the exact frame length of the reply the codec writes.
///
/// Red if the arithmetic drifts from the encoder (a provider would then admit a reply that does
/// not fit, or refuse one that does).
#[test]
fn response_result_encoded_len_is_the_exact_frame_length() {
    let codec = ProtocolCodec::default();
    for points in [2_usize, 3, 4, 5, 63, 64, 1000] {
        for channels in [
            ParameterChannel::Left,
            ParameterChannel::Right,
            ParameterChannel::Both,
        ] {
            for (section_count, sections) in [(1_u32, false), (1, true), (3, true), (7, true)] {
                let value = ResponseResult::Curves(response_curves(
                    points,
                    channels,
                    section_count,
                    sections,
                ));
                let mut frame = vec![0_u8; 1 << 20];
                let written = codec
                    .encode_success_response_frame_into(
                        &crate::TypedSuccessResponseFrame {
                            request_id: crate::RequestId::new(1).expect("request"),
                            revision: crate::SessionRevision(7),
                            payload: crate::SuccessResponsePayload::ResponseResult(value),
                        },
                        &mut frame,
                    )
                    .expect("frame");
                assert_eq!(
                    response_result_encoded_len(points, channels, section_count as usize, sections),
                    Some(written),
                    "{points} points, {channels:?}, {section_count} sections, {sections}"
                );
            }
        }
    }
    assert_eq!(
        response_result_encoded_len(usize::MAX / 2, ParameterChannel::Both, 4, true),
        None
    );
}
