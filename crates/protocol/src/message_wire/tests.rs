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

/// #1629 gate 1. Queue kind 2 (the automation queue) is retired: a backpressure payload that
/// names it fails typed decode, while the allocated kinds decode. Never reallocated. Kind 7 (the
/// spectrum arm slot, #1944) and kind 8 (the track-response capture slot, #1578 D3) are allocated
/// without a queue generation, as the replay cache is.
///
/// Test value: red if the retired queue code decodes again or is given to another queue, or if
/// a capacity-1 slot's code stops decoding or is decoded with a generation.
#[test]
fn a_backpressure_payload_naming_the_retired_automation_queue_is_refused() {
    let codec = codec();
    let encode = |queue_kind: BackpressureQueueKind, generation: Option<u64>| {
        let mut value = response();
        let backpressure = value.backpressure.as_mut().expect("backpressure");
        backpressure.queue_kind = queue_kind;
        backpressure.generation = generation;
        let mut output = vec![0_u8; codec.encoded_non_ok_payload_len(&value).expect("size")];
        codec
            .encode_non_ok_payload(&value, &mut output)
            .expect("encode");
        output
    };
    let reliable_response = encode(BackpressureQueueKind::ReliableResponse, Some(4));
    let control = encode(BackpressureQueueKind::ControlCommand, Some(4));
    let at = reliable_response
        .iter()
        .zip(&control)
        .position(|(left, right)| left != right)
        .expect("the queue kind is the one byte that differs");
    assert_eq!((control[at], reliable_response[at]), (1, 3));
    let replay_cache = encode(BackpressureQueueKind::ReplayCache, None);
    let decodes = |base: &[u8], kind: u8| {
        let mut bytes = base.to_vec();
        bytes[at] = kind;
        codec.decode_non_ok_payload(&bytes, 3)
    };
    for allocated in [1_u8, 3, 4, 5] {
        assert!(
            decodes(&reliable_response, allocated).is_ok(),
            "{allocated}"
        );
    }
    for generationless in [6_u8, 7, 8] {
        assert!(
            decodes(&replay_cache, generationless).is_ok(),
            "{generationless}"
        );
        assert_eq!(
            decodes(&reliable_response, generationless),
            Err(DecodeError::InvalidTlv),
            "queue kind {generationless} with a generation"
        );
    }
    for refused in [0_u8, 2, 9] {
        assert_eq!(
            decodes(&reliable_response, refused),
            Err(DecodeError::InvalidTlv),
            "queue kind {refused}"
        );
    }
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
    let events = [0x8001];
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
    // The pair rule, an unallocated ID and a retired ID are each refused: a record that adds the
    // retired event `0x8002` back is refused whatever else it lists (#1628).
    assert_parity_rejects(&commands, &[0x8001, 0x8002]);
    assert_parity_rejects(&[1, 2, 12], &events);
    assert_parity_rejects(&[1, 2, 14], &events);
    assert_parity_rejects(&[1, 2, 4], &events);
    assert_parity_rejects(&[1, 2, 6], &events);
    assert_parity_rejects(&commands, &[]);
    assert!(!IdSource::LittleEndian(&[1]).is_strict_allocated(false));
    // #1386: `SESSION_REPLACE` (`0x0013`) is allocated and rides the session family.
    assert!(
        check_capabilities_invariants(view(
            IdSource::Native(&[1, 2, 3, 0x0013]),
            IdSource::Native(&events),
        ))
        .is_ok()
    );
    // #1586 D6: `OBSERVATION_READ` (`0x000f`) is listed exactly when bit 15 is set, and both ride
    // the session family.
    assert_parity_rejects(&[1, 2, 3, 0x000f], &events);
    let bit_15 = |commands: &'static [u16]| CapabilityInvariantView {
        flags: CapabilityFlags(
            view(IdSource::Native(&[]), IdSource::Native(&[])).flags.0
                | CapabilityFlags::OBSERVATION.0,
        ),
        ..view(IdSource::Native(commands), IdSource::Native(&events))
    };
    assert!(check_capabilities_invariants(bit_15(&[1, 2, 3, 0x000f])).is_ok());
    assert_eq!(
        check_capabilities_invariants(bit_15(&[1, 2, 3])),
        Err(DecodeError::InvalidTlv),
        "bit 15 without the command"
    );
    // #1595 D8a: the three spectrum commands are listed together, exactly when bit 16 is set.
    let spectrum = |flags: u64| CapabilityInvariantView {
        flags: CapabilityFlags(view(IdSource::Native(&[]), IdSource::Native(&[])).flags.0 | flags),
        ..view(
            IdSource::Native(&[1, 2, 3, 0x0010, 0x0011, 0x0012]),
            IdSource::Native(&events),
        )
    };
    assert!(check_capabilities_invariants(spectrum(CapabilityFlags::SPECTRUM.0)).is_ok());
    assert_eq!(
        check_capabilities_invariants(spectrum(0)),
        Err(DecodeError::InvalidTlv),
        "the three commands without bit 16"
    );
    let bit_16 = |commands: &'static [u16]| CapabilityInvariantView {
        flags: CapabilityFlags(
            view(IdSource::Native(&[]), IdSource::Native(&[])).flags.0
                | CapabilityFlags::SPECTRUM.0,
        ),
        ..view(IdSource::Native(commands), IdSource::Native(&events))
    };
    for partial in [
        &[1, 2, 3][..],
        &[1, 2, 3, 0x0010, 0x0011],
        &[1, 2, 3, 0x0010, 0x0012],
        &[1, 2, 3, 0x0011, 0x0012],
    ] {
        assert_eq!(
            check_capabilities_invariants(bit_16(partial)),
            Err(DecodeError::InvalidTlv),
            "bit 16 with {partial:x?}"
        );
    }
    for partial in [
        &[1, 2, 3, 0x0010][..],
        &[1, 2, 3, 0x0011],
        &[1, 2, 3, 0x0012],
    ] {
        assert_parity_rejects(partial, &events);
    }
    // #2115: `LOUDNESS` (`0x0014`) is listed exactly when bit 17 is set.
    let bit_17 = |commands: &'static [u16], flags: u64| CapabilityInvariantView {
        flags: CapabilityFlags(view(IdSource::Native(&[]), IdSource::Native(&[])).flags.0 | flags),
        ..view(IdSource::Native(commands), IdSource::Native(&events))
    };
    assert!(
        check_capabilities_invariants(bit_17(&[1, 2, 3, 0x0014], CapabilityFlags::LOUDNESS.0))
            .is_ok()
    );
    assert_eq!(
        check_capabilities_invariants(bit_17(&[1, 2, 3, 0x0014], 0)),
        Err(DecodeError::InvalidTlv),
        "0x0014 without bit 17"
    );
    assert_eq!(
        check_capabilities_invariants(bit_17(&[1, 2, 3], CapabilityFlags::LOUDNESS.0)),
        Err(DecodeError::InvalidTlv),
        "bit 17 without 0x0014"
    );
    let without_session = |commands: &'static [u16]| CapabilityInvariantView {
        maximum_transaction_edits: 0,
        flags: CapabilityFlags(CapabilityFlags::B1B_BASE.0 & !(1 << 3)),
        ..view(IdSource::Native(commands), IdSource::Native(&[]))
    };
    assert!(check_capabilities_invariants(without_session(&[1, 2])).is_ok());
    assert_eq!(
        check_capabilities_invariants(without_session(&[1, 2, 0x0013])),
        Err(DecodeError::InvalidTlv)
    );
    assert_eq!(
        check_capabilities_invariants(CapabilityInvariantView {
            flags: CapabilityFlags(without_session(&[]).flags.0 | CapabilityFlags::OBSERVATION.0),
            ..without_session(&[1, 2, 0x000f])
        }),
        Err(DecodeError::InvalidTlv),
        "the read and bit 15 without the session family"
    );
}

/// #1386 gate 7: the `SESSION_REPLACE` request round-trips; a missing field, `document_bytes` 0
/// and a digest that is not 32 bytes are refused.
#[test]
fn session_replace_request_round_trips_and_refuses_a_bad_digest() {
    let codec = codec();
    let value = crate::SessionReplaceRequest {
        document_bytes: 70_000,
        document_sha256: core::array::from_fn(|index| index as u8),
    };
    let mut bytes = [0_u8; 64];
    let written = codec
        .encode_session_replace_request(value, &mut bytes)
        .expect("encode");
    assert_eq!(
        codec.decode_session_replace_request(&bytes[..written], 2),
        Ok(value)
    );
    assert_eq!(
        codec.encode_session_replace_request(
            crate::SessionReplaceRequest {
                document_bytes: 0,
                ..value
            },
            &mut bytes
        ),
        Err(EncodeError::LimitExceeded)
    );
    let field = |id: u16, wire: u8, payload: &[u8]| {
        let mut out = vec![];
        out.extend_from_slice(&id.to_le_bytes());
        out.push(wire);
        // Mandatory: both fields are required.
        out.push(1);
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        while out.len() % 8 != 0 {
            out.push(0);
        }
        out
    };
    let length = |bytes: u32| field(1, 3, &bytes.to_le_bytes());
    let digest = |len: usize| field(2, 10, &vec![7_u8; len]);
    let frame = |parts: &[Vec<u8>]| parts.concat();
    assert_eq!(
        codec.decode_session_replace_request(&frame(&[length(9), digest(32)]), 2),
        Ok(crate::SessionReplaceRequest {
            document_bytes: 9,
            document_sha256: [7; 32],
        })
    );
    for (bad, count, name) in [
        (frame(&[length(9), digest(31)]), 2, "a 31-byte digest"),
        (frame(&[length(9), digest(33)]), 2, "a 33-byte digest"),
        (frame(&[length(0), digest(32)]), 2, "document_bytes 0"),
        (frame(&[length(9)]), 1, "no digest"),
        (frame(&[digest(32)]), 1, "no length"),
    ] {
        assert!(
            codec.decode_session_replace_request(&bad, count).is_err(),
            "{name} must be refused"
        );
    }
    assert_eq!(
        codec.decode_session_replace_request(&frame(&[length(9), digest(31)]), 2),
        Err(DecodeError::InvalidField)
    );
}

#[test]
fn b1b_success_schemas_round_trip_and_truncate() {
    let codec = codec();
    let commands = [1_u16, 2, 3];
    let events = [0x8001_u16];
    let capabilities = Capabilities {
        minimum_version: crate::ProtocolVersion::V1,
        maximum_version: crate::ProtocolVersion::V1,
        maximum_frame_bytes: 4096,
        maximum_tlvs: 64,
        maximum_string_bytes: 64,
        maximum_nesting: 4,
        control_command_slots: 1,
        control_command_bytes: 64,
        reliable_response_slots: 1,
        reliable_event_slots: 1,
        telemetry_slots: 1,
        replay_entries: 1,
        replay_bytes: 1024,
        maximum_cached_response_bytes: 512,
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
        .decode_capabilities(&capability_bytes, 23)
        .expect("decode");
    assert_eq!(decoded.maximum_frame_bytes, 4096);
    assert_eq!(decoded.supported_commands, &[1, 0, 2, 0, 3, 0]);
    for end in 0..capability_bytes.len() {
        assert!(
            codec
                .decode_capabilities(&capability_bytes[..end], 23)
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
    let applied = TransactionApplied::new(3, EditPath::Rebuild);
    let mut applied_bytes = [0_u8; 32];
    codec
        .encode_transaction_applied(&applied, &mut applied_bytes)
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

    let applied = TransactionApplied::new(3, EditPath::Live);
    let mut applied_bytes = [0_u8; 32];
    codec
        .encode_transaction_applied(&applied, &mut applied_bytes)
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
        automated: false,
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
            headers_only: false,
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
        headers_only: false,
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
        ("section header with an automated flag", None, |f| {
            insert(entry(f, 2), leaf(35, 8, &[0]))
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
            "automated row that is not automatable",
            Some(|p| row(p, 6).automated = true),
            |f| set(entry(f, 6), 35, &[1]),
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
        10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 25, 26, 30, 31, 32, 33, 34, 35,
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
/// Red if `0x0004`, `0x0005`, `0x0006` or `0x8002` is reallocated to a new message or still served,
/// or a peer can advertise one.
#[test]
fn retired_message_ids_are_never_reused_or_advertised() {
    assert_eq!(crate::RETIRED_MESSAGE_IDS, [0x0004, 0x0005, 0x0006, 0x8002]);
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
    assert!(
        !allocated_id(0x0004, false)
            && !allocated_id(0x0005, false)
            && !allocated_id(0x0006, false)
            && !allocated_id(0x8002, true)
    );
    assert!(
        [0x8001, 0x8010, 0x8020, 0x8021, 0x8030]
            .into_iter()
            .all(|id| allocated_id(id, true))
    );
    assert!(
        [1, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0x0013, 0x0014]
            .into_iter()
            .all(|id| allocated_id(id, false))
    );
    assert!(!allocated_id(0, false) && !allocated_id(0x0015, false));
}

fn session_capabilities<'a>(commands: &'a [u16], events: &'a [u16]) -> Capabilities<'a> {
    Capabilities {
        minimum_version: crate::ProtocolVersion::V1,
        maximum_version: crate::ProtocolVersion::V1,
        maximum_frame_bytes: 4096,
        maximum_tlvs: 64,
        maximum_string_bytes: 64,
        maximum_nesting: 4,
        control_command_slots: 1,
        control_command_bytes: 64,
        reliable_response_slots: 1,
        reliable_event_slots: 1,
        telemetry_slots: 1,
        replay_entries: 1,
        replay_bytes: 1024,
        maximum_cached_response_bytes: 512,
        maximum_control_page_items: 256,
        maximum_diagnostic_page_items: 256,
        maximum_telemetry_handles: 256,
        maximum_transaction_edits: 64,
        supported_commands: commands,
        supported_events: events,
        flags: CapabilityFlags(
            CapabilityFlags::B1B_BASE.0 | CapabilityFlags::SESSION_EVENT_STREAM.0,
        ),
    }
}

/// #1627: capability fields 9 (`maximum_automation_records`), 12 (`automation_batch_slots`), 19
/// (`per_block_automation_density`) and 20 (`admission_quantum_frames`) are retired with
/// `AUTOMATION_ENQUEUE`: a record that carries one is refused in either flag form, never skipped as
/// an unknown optional field. The control splice is a never-allocated optional field (28), which
/// decodes, so a broken splice cannot pass as a refusal.
///
/// Red if a retired field is skipped like an unknown optional field (the record decodes), is read
/// again as a field, or the record has more than its 23 fields.
#[test]
fn retired_capability_fields_are_refused_in_either_flag_form() {
    let codec = codec();
    let commands = [1_u16, 2, 3];
    let events = [0x8001_u16];
    let capabilities = session_capabilities(&commands, &events);
    let mut bytes = vec![
        0;
        codec
            .encoded_capabilities_len(&capabilities)
            .expect("length")
    ];
    codec
        .encode_capabilities(&capabilities, &mut bytes)
        .expect("encode");
    assert_eq!(parse_tlvs(&bytes).len(), 23, "the record has 23 fields");
    assert!(codec.decode_capabilities(&bytes, 23).is_ok());
    let (control, count) = edited(&bytes, |fields| insert(fields, leaf(28, 1, &[0])));
    assert!(codec.decode_capabilities(&control, count).is_ok());
    for (id, wire, width) in [(9_u16, 2_u8, 2_usize), (12, 4, 8), (19, 4, 8), (20, 4, 8)] {
        for mandatory in [0_u8, 1] {
            let (patched, count) = edited(&bytes, |fields| {
                let mut field = leaf(id, wire, &vec![1; width]);
                field.mandatory = mandatory;
                insert(fields, field);
            });
            assert_eq!(
                codec.decode_capabilities(&patched, count).err(),
                Some(if mandatory == 1 {
                    DecodeError::UnknownRequiredField
                } else {
                    DecodeError::InvalidTlv
                }),
                "retired field {id}, mandatory {mandatory}"
            );
        }
    }
}

/// #1627: flag bit 5 was `AUTOMATION_ENQUEUE`'s. It is no longer a known flag, so a record that
/// sets it is refused whatever it lists; the 23-field record needs `maximum_tlvs` of at least 23.
///
/// Red if bit 5 is accepted again (a peer advertises the retired command through its flag), or the
/// field-count floor stays at the old 27.
#[test]
fn capability_flag_bit_5_is_unknown_and_the_floor_is_the_field_count() {
    let codec = codec();
    let commands = [1_u16, 2, 3];
    let events = [0x8001_u16];
    let mut capabilities = session_capabilities(&commands, &events);
    assert_eq!(capabilities.flags.0 & (1 << 5), 0);
    assert!(codec.encoded_capabilities_len(&capabilities).is_ok());
    capabilities.flags = CapabilityFlags(capabilities.flags.0 | (1 << 5));
    assert!(codec.encoded_capabilities_len(&capabilities).is_err());
    capabilities.flags = CapabilityFlags(capabilities.flags.0 & !(1 << 5));
    capabilities.maximum_tlvs = 22;
    assert!(codec.encoded_capabilities_len(&capabilities).is_err());
    capabilities.maximum_tlvs = 23;
    assert!(codec.encoded_capabilities_len(&capabilities).is_ok());
    assert_eq!(
        CapabilityFlags::B4_BASE.0 & (1 << 5),
        0,
        "the full provider set no longer sets bit 5"
    );
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
    };
    let mut set_bytes = [0_u8; 16];
    assert_eq!(codec.encoded_transport_set_request_len(set), 16);
    codec
        .encode_transport_set_request(set, &mut set_bytes)
        .expect("set encode");
    assert_eq!(hex(&set_bytes), "01000101010000000200000000000000");
    assert_eq!(codec.decode_transport_set_request(&set_bytes, 1), Ok(set));
    let mut short = [0xaa; 15];
    assert_eq!(
        codec.encode_transport_set_request(set, &mut short),
        Err(EncodeError::OutputTooSmall { required: 16 })
    );
    assert!(short.iter().all(|byte| *byte == 0xaa));
    for end in 0..set_bytes.len() {
        assert!(
            codec
                .decode_transport_set_request(&set_bytes[..end], 1)
                .is_err()
        );
    }
    let mut bad = set_bytes;
    bad[8] = 3;
    assert!(codec.decode_transport_set_request(&bad, 1).is_err());
    bad = set_bytes;
    bad[3] = 0;
    assert!(codec.decode_transport_set_request(&bad, 1).is_err());
    let mut extension = set_bytes.to_vec();
    extension.extend_from_slice(&[3, 0, WIRE_U8, 0, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        codec.decode_transport_set_request(&extension, 2),
        Ok(set),
        "unknown optional fields are skippable"
    );
    extension[16 + 3] = 1;
    assert_eq!(
        codec.decode_transport_set_request(&extension, 2),
        Err(DecodeError::UnknownRequiredField)
    );

    // The retired position (#1635): field 2 of the set, field 2 of the snapshot and field 3 of the
    // event are refused in either flag form, and an unknown optional field in the same slot is
    // skipped only when it is not retired.
    let retired = |base: &[u8], id: u8, mandatory: bool| {
        let mut bytes = base.to_vec();
        bytes.extend_from_slice(&[id, 0, 4, u8::from(mandatory), 8, 0, 0, 0]);
        bytes.extend_from_slice(&9_u64.to_le_bytes());
        bytes
    };
    for mandatory in [false, true] {
        let expected = if mandatory {
            DecodeError::UnknownRequiredField
        } else {
            DecodeError::InvalidTlv
        };
        assert_eq!(
            codec.decode_transport_set_request(&retired(&set_bytes, 2, mandatory), 2),
            Err(expected)
        );
    }

    let snapshot = TransportSnapshot {
        state: TransportState::Stopped,
        effective_sample: crate::SampleTime(3),
    };
    let mut snapshot_bytes = [0_u8; 32];
    codec
        .encode_transport_snapshot(snapshot, &mut snapshot_bytes)
        .expect("snapshot encode");
    assert_eq!(
        hex(&snapshot_bytes),
        concat!(
            "01000101010000000100000000000000",
            "03000401080000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_transport_snapshot(&snapshot_bytes, 2),
        Ok(snapshot)
    );
    for end in 0..snapshot_bytes.len() {
        assert!(
            codec
                .decode_transport_snapshot(&snapshot_bytes[..end], 2)
                .is_err()
        );
    }
    // The position sits between the two fields, where it was: field order is by ID.
    let mut with_position = snapshot_bytes[..16].to_vec();
    with_position.extend_from_slice(&retired(&[], 2, false));
    with_position.extend_from_slice(&snapshot_bytes[16..]);
    assert_eq!(
        codec.decode_transport_snapshot(&with_position, 3),
        Err(DecodeError::InvalidTlv)
    );
    with_position[16 + 3] = 1;
    assert_eq!(
        codec.decode_transport_snapshot(&with_position, 3),
        Err(DecodeError::UnknownRequiredField)
    );

    let event = TransportStateEvent {
        event_sequence: 1,
        state: TransportState::Playing,
        effective_sample: crate::SampleTime(3),
        origin_request_id: crate::RequestId::new(9),
    };
    let mut event_bytes = [0_u8; 64];
    assert_eq!(codec.encoded_transport_state_event_len(event), 64);
    codec
        .encode_transport_state_event(event, &mut event_bytes)
        .expect("event encode");
    assert_eq!(
        hex(&event_bytes),
        concat!(
            "01000401080000000100000000000000",
            "02000101010000000200000000000000",
            "04000401080000000300000000000000",
            "05000400080000000900000000000000"
        )
    );
    assert_eq!(
        codec.decode_transport_state_event(&event_bytes, 4),
        Ok(event)
    );
    for end in 0..event_bytes.len() {
        assert!(
            codec
                .decode_transport_state_event(&event_bytes[..end], 4)
                .is_err()
        );
    }
    let mut with_position = event_bytes[..32].to_vec();
    with_position.extend_from_slice(&retired(&[], 3, false));
    with_position.extend_from_slice(&event_bytes[32..]);
    assert_eq!(
        codec.decode_transport_state_event(&with_position, 5),
        Err(DecodeError::InvalidTlv)
    );
    with_position[32 + 3] = 1;
    assert_eq!(
        codec.decode_transport_state_event(&with_position, 5),
        Err(DecodeError::UnknownRequiredField)
    );
    let no_origin = TransportStateEvent {
        origin_request_id: None,
        ..event
    };
    let mut no_origin_bytes = [0_u8; 48];
    assert_eq!(codec.encoded_transport_state_event_len(no_origin), 48);
    codec
        .encode_transport_state_event(no_origin, &mut no_origin_bytes)
        .expect("no origin encode");
    assert_eq!(
        codec.decode_transport_state_event(&no_origin_bytes, 3),
        Ok(no_origin)
    );
}

fn ended_key(
    strip: &str,
    section: ObservationSection,
    effect: &str,
    tap: u32,
) -> ObservationTapKey {
    ObservationTapKey {
        strip_id: strip.to_owned(),
        section,
        effect_id: effect.to_owned(),
        tap_id: tap,
    }
}

/// #1584 (root ruling A): `OBSERVATION_ENDED` carries every ended key of one commit and round-trips
/// them exactly, every truncation fails, and the codec refuses on both sides an ID that is not a
/// stable ID, a section that holds no effect, no key, and keys out of strict ascending order
/// (console slots before inserts, so a duplicate is out of order too).
///
/// Red if a key loses a field or its place in the round trip, a builtins or own section is taken
/// for an effect section, an empty event decodes, or a list the decoder cannot read as one ordered
/// set is encodable.
#[test]
fn observation_ended_round_trips_every_key_and_refuses_anything_else() {
    use ObservationSection::{Console, Inserts};
    let codec = codec();
    let event = ObservationEnded {
        event_sequence: 7,
        keys: vec![
            ended_key("bass", Inserts, "comp", 1),
            ended_key("vocal", Console, "glue", 3),
            ended_key("vocal", Inserts, "comp", 1),
            ended_key("vocal", Inserts, "comp", 2),
            ended_key("vocal", Inserts, "eq", 1),
        ],
    };
    let mut bytes = vec![0_u8; 1024];
    let length = codec
        .encode_observation_ended(&event, &mut bytes)
        .expect("encode");
    assert_eq!(
        length,
        codec.encoded_observation_ended_len(&event).expect("length")
    );
    bytes.truncate(length);
    // The field count is the sequence and one key message per key.
    let count = 1 + event.keys.len() as u32;
    assert_eq!(
        codec.decode_observation_ended(&bytes, count),
        Ok(event.clone())
    );
    for end in 0..bytes.len() {
        assert!(
            codec
                .decode_observation_ended(&bytes[..end], count)
                .is_err(),
            "{end}"
        );
    }
    // A single key is the smallest event; the section is on the wire.
    let one = |section| ObservationEnded {
        event_sequence: 7,
        keys: vec![ended_key("vocal", section, "glue", 3)],
    };
    let mut other = vec![0_u8; 256];
    let mut encoded = Vec::new();
    for section in [Console, Inserts] {
        let written = codec
            .encode_observation_ended(&one(section), &mut other)
            .expect("encode");
        assert_eq!(
            codec.decode_observation_ended(&other[..written], 2),
            Ok(one(section))
        );
        encoded.push(other[..written].to_vec());
    }
    assert_ne!(encoded[0], encoded[1]);

    // Not a stable ID, no key, or out of order: encode refuses it.
    for bad in ["", "Vocal", "9lives", "has space"] {
        let mut invalid = one(Console);
        invalid.keys[0].strip_id = bad.to_owned();
        assert_eq!(
            codec.encode_observation_ended(&invalid, &mut other),
            Err(EncodeError::InvalidField),
            "{bad:?}"
        );
    }
    let mut empty = one(Console);
    empty.keys.clear();
    assert_eq!(
        codec.encode_observation_ended(&empty, &mut other),
        Err(EncodeError::InvalidField)
    );
    for keys in [
        // Inserts before a console slot of the same strip.
        vec![
            ended_key("vocal", Inserts, "comp", 1),
            ended_key("vocal", Console, "glue", 3),
        ],
        // A repeated key.
        vec![
            ended_key("vocal", Console, "glue", 3),
            ended_key("vocal", Console, "glue", 3),
        ],
        // A lower tap after a higher one.
        vec![
            ended_key("vocal", Inserts, "comp", 2),
            ended_key("vocal", Inserts, "comp", 1),
        ],
    ] {
        assert_eq!(
            codec.encode_observation_ended(
                &ObservationEnded {
                    event_sequence: 7,
                    keys
                },
                &mut other
            ),
            Err(EncodeError::InvalidField)
        );
    }

    // Decode refuses what encode would: the section byte of the first key is 2 or 5, anything else
    // names no effect. (A key message's section field is its second field, id 2, type U8.)
    let written = codec
        .encode_observation_ended(&one(Console), &mut other)
        .expect("encode");
    let mut section_values = Vec::new();
    for (index, window) in other[..written].windows(2).enumerate() {
        // Field header: id (u16 LE) then the wire type byte; a U8 section is `2, 0, 1`.
        if window == [2, 0] && other[..written].get(index + 2) == Some(&1) {
            section_values.push(index + 8);
        }
    }
    let value = *section_values.last().expect("the section field header");
    assert_eq!(other[value], 5, "console");
    for code in [0_u8, 1, 3, 4, 6] {
        let mut forged = other[..written].to_vec();
        forged[value] = code;
        assert_eq!(
            codec.decode_observation_ended(&forged, 2),
            Err(DecodeError::InvalidField),
            "section {code}"
        );
    }
}

/// Issue #1351 gate 4: every registered counter ID, the newest `METER_WINDOWS_LOST` (#1583)
/// included, round-trips through the telemetry configuration; the next ID and the retired IDs
/// 2 and 11 to 14 (#1629) are refused.
///
/// Test value: red if a counter joins the enum but not `parse_counter_id`, so that a host's
/// configuration of it decodes as malformed.
#[test]
fn every_counter_id_round_trips_and_the_next_is_refused() {
    let codec = codec();
    let all = [
        CounterId::ControlCommandBackpressure,
        CounterId::ReliableResponseBackpressure,
        CounterId::ReliableEventBackpressure,
        CounterId::TelemetryCoalesced,
        CounterId::TelemetryDropped,
        CounterId::MalformedFrames,
        CounterId::ReplayHits,
        CounterId::RequestIdReuse,
        CounterId::ReplayExpired,
        CounterId::ValidationFailures,
        CounterId::LiveValuesSuperseded,
        CounterId::TransitionReprepareRefusals,
        CounterId::StripsRestarted,
        CounterId::LiveBlockErrors,
        CounterId::SourceUnderrunBlocks,
        CounterId::SourceFramesDiscarded,
        CounterId::MeterWindowsLost,
    ];
    let allocated = [
        1_u32, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 17, 18, 19, 20, 21, 22,
    ];
    assert_eq!(
        all.iter().map(|id| *id as u32).collect::<Vec<_>>(),
        allocated,
        "IDs 2 and 11 to 14 are retired, never reallocated"
    );
    let configuration = TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 0,
        meter_window_blocks: None,
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
        codec.decode_telemetry_configuration(&bytes[..len], 5),
        Ok(configuration.clone())
    );
    // One ID per request, so a refusal cannot come from the sorted-unique rule.
    let single = TelemetryConfiguration {
        counter_ids: vec![CounterId::MeterWindowsLost],
        counter_period_blocks: 3,
        ..configuration
    };
    let mut bytes = [0_u8; 192];
    let len = codec
        .encode_telemetry_configuration(&single, &mut bytes)
        .expect("single");
    let id_bytes = 22_u32.to_le_bytes();
    let at = bytes[..len]
        .windows(4)
        .position(|window| window == id_bytes)
        .expect("counter ID");
    for refused_id in [0_u32, 2, 11, 12, 13, 14, 23] {
        let mut refused = bytes;
        refused[at..at + 4].copy_from_slice(&refused_id.to_le_bytes());
        assert!(
            codec
                .decode_telemetry_configuration(&refused[..len], 5)
                .is_err(),
            "counter ID {refused_id} must be refused"
        );
    }
}

/// Issue #1583 gate 1: a meter table with each target and metric round-trips as a command and,
/// with its window, as an echo; a batch round-trips with its span; and every D2 rule refuses with
/// `INVALID_FIELD` on encode and on decode alike, with the retired field 1 refused in either flag
/// form and field 8 refused in a command.
///
/// Test value: red if the encoder, the decoder and the registry disagree on the table, the span or
/// the retired field: a rule one side forgets (a duplicate, an output gain reduction, a master peak
/// or a master with a strip ID (#1690 D3), a 257th entry, a zero period, an empty span, a zero
/// sequence, a command window) is a frame one side writes and the other refuses.
#[test]
fn the_meter_table_and_the_window_span_follow_one_rule_on_encode_and_decode() {
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: 1 << 16,
        max_tlv_count: 4096,
        max_string_bytes: 256,
        max_nesting: 4,
    });
    let entry = |target: MeterTarget, metric| MeterEntry { target, metric };
    let strip = |id: &str| MeterTarget::Strip(id.into());
    let base = TelemetryConfiguration {
        meters: vec![
            entry(MeterTarget::Output, MeterMetric::Peak),
            entry(strip("vocal"), MeterMetric::Peak),
            entry(strip("vocal"), MeterMetric::GainReduction),
            entry(MeterTarget::Master, MeterMetric::GainReduction),
        ],
        meter_period_blocks: 24,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: DiagnosticSeverity::Info,
    };
    let encode = |value: &TelemetryConfiguration| {
        let mut bytes = vec![0; codec.encoded_telemetry_configuration_len(value)?];
        codec.encode_telemetry_configuration(value, &mut bytes)?;
        Ok::<_, EncodeError>(bytes)
    };
    // A command is written without a window; the echo form carries one.
    let command = |value: &TelemetryConfiguration| {
        let mut sink = crate::btlv::CountSink::new(codec.limits());
        write_telemetry_configuration(&mut sink, value, false)
    };
    let count = |value: &TelemetryConfiguration| {
        u32::try_from(5 + value.meters.len() + usize::from(value.meter_window_blocks.is_some()))
            .expect("count")
    };
    let bytes = encode(&base).expect("command bytes");
    assert_eq!(command(&base), Ok(()));
    assert_eq!(
        codec.decode_telemetry_configuration(&bytes, count(&base)),
        Ok(base.clone())
    );
    let echo = TelemetryConfiguration {
        meter_window_blocks: core::num::NonZeroU32::new(12),
        ..base.clone()
    };
    let echo_bytes = encode(&echo).expect("echo bytes");
    assert_eq!(
        codec.decode_telemetry_configuration_echo(&echo_bytes, count(&echo)),
        Ok(echo.clone())
    );
    // Field 8 in a command.
    assert_eq!(command(&echo), Err(EncodeError::InvalidField));
    assert_eq!(
        codec.decode_telemetry_configuration(&echo_bytes, count(&echo)),
        Err(DecodeError::InvalidField)
    );

    // Each typed refusal, on encode, and on decode as the same table's bytes with one code moved
    // (the encoder writes none of them).
    let replace = |bytes: &[u8], from: &[u8], to: &[u8], nth: usize| {
        let at = bytes
            .windows(from.len())
            .enumerate()
            .filter(|(_, window)| *window == from)
            .nth(nth)
            .map(|(at, _)| at)
            .expect("pattern");
        let mut patched = bytes.to_vec();
        patched[at..at + to.len()].copy_from_slice(to);
        patched
    };
    // A required one-byte field: ID, `U8`, required, length 1, the value.
    let u8_field = |id: u8, value: u8| [id, 0, 1, 1, 1, 0, 0, 0, value];
    let refused_tables = [
        // A duplicate entry.
        TelemetryConfiguration {
            meters: vec![
                entry(strip("vocal"), MeterMetric::Peak),
                entry(strip("vocal"), MeterMetric::Peak),
            ],
            ..base.clone()
        },
        // The output's gain reduction.
        TelemetryConfiguration {
            meters: vec![entry(MeterTarget::Output, MeterMetric::GainReduction)],
            ..base.clone()
        },
        // The master's peak (#1690 D3).
        TelemetryConfiguration {
            meters: vec![entry(MeterTarget::Master, MeterMetric::Peak)],
            ..base.clone()
        },
        // A table with no period.
        TelemetryConfiguration {
            meter_period_blocks: 0,
            ..base.clone()
        },
        // A strip ID that is no stable ID.
        TelemetryConfiguration {
            meters: vec![entry(strip("Vocal"), MeterMetric::Peak)],
            ..base.clone()
        },
    ];
    for refused in &refused_tables {
        assert_eq!(
            encode(refused),
            Err(EncodeError::InvalidField),
            "{refused:?}"
        );
    }
    let one_strip = TelemetryConfiguration {
        meters: vec![
            entry(strip("vocal"), MeterMetric::Peak),
            entry(strip("vocal"), MeterMetric::GainReduction),
        ],
        ..base.clone()
    };
    let one_strip_bytes = encode(&one_strip).expect("two entries");
    let output_bytes = encode(&TelemetryConfiguration {
        meters: vec![entry(MeterTarget::Output, MeterMetric::Peak)],
        ..base.clone()
    })
    .expect("one entry");
    let period = [2, 0, 3, 1, 4, 0, 0, 0, 24, 0, 0, 0];
    let refused_frames = [
        // The duplicate: the second entry's metric moved to the first's.
        (
            replace(&one_strip_bytes, &u8_field(3, 2), &u8_field(3, 1), 0),
            7,
        ),
        // The output's gain reduction.
        (
            replace(&output_bytes, &u8_field(3, 1), &u8_field(3, 2), 0),
            6,
        ),
        // No period.
        (
            replace(&bytes, &period, &[2, 0, 3, 1, 4, 0, 0, 0, 0, 0, 0, 0], 0),
            count(&base),
        ),
        // The master's peak: the output's peak entry with the master's target (#1690 D3).
        (
            replace(&output_bytes, &u8_field(1, 1), &u8_field(1, 3), 0),
            6,
        ),
        // A master target with a strip ID.
        (
            replace(&one_strip_bytes, &u8_field(1, 2), &u8_field(1, 3), 1),
            7,
        ),
        // An unknown target and an unknown metric.
        (
            replace(&output_bytes, &u8_field(1, 1), &u8_field(1, 4), 0),
            6,
        ),
        (
            replace(&output_bytes, &u8_field(3, 1), &u8_field(3, 3), 0),
            6,
        ),
        // A strip target with no strip ID, and the output with one.
        (
            replace(&output_bytes, &u8_field(1, 1), &u8_field(1, 2), 0),
            6,
        ),
        (
            replace(&one_strip_bytes, &u8_field(1, 2), &u8_field(1, 1), 0),
            7,
        ),
    ];
    for (index, (frame, fields)) in refused_frames.iter().enumerate() {
        assert_eq!(
            codec.decode_telemetry_configuration(frame, *fields),
            Err(DecodeError::InvalidField),
            "refused frame {index}"
        );
    }
    // 256 entries are the most; a 257th is refused both ways.
    let wide = |range: core::ops::Range<u32>| TelemetryConfiguration {
        meters: range
            .map(|index| entry(strip(&format!("s{index:03}")), MeterMetric::Peak))
            .collect(),
        ..base.clone()
    };
    let full = wide(0..256);
    let full_bytes = encode(&full).expect("256 entries");
    assert_eq!(
        codec.decode_telemetry_configuration(&full_bytes, count(&full)),
        Ok(full.clone())
    );
    assert_eq!(encode(&wide(0..257)), Err(EncodeError::InvalidField));
    // Every entry encodes to the same length, so the last entry of `1..257` is the 257th.
    let shifted = encode(&wide(1..257)).expect("256 entries");
    let entry_bytes = shifted.len() - encode(&wide(1..256)).expect("255").len();
    let mut overfull = full_bytes.clone();
    overfull.extend_from_slice(&shifted[shifted.len() - entry_bytes..]);
    assert_eq!(
        codec.decode_telemetry_configuration(&overfull, count(&full) + 1),
        Err(DecodeError::InvalidField)
    );
    // Field 1, the retired positional handles, in either flag form.
    for flags in [0_u8, 1] {
        let mut retired = vec![1, 0, 0x0d, flags, 4, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
        retired.extend_from_slice(&bytes);
        assert!(
            codec
                .decode_telemetry_configuration(&retired, count(&base) + 1)
                .is_err(),
            "field 1 with flags {flags}"
        );
    }

    // The batch's span: a window `[first, end)` with `first < end` and a sequence from 1.
    let records = [MeterRecord {
        handle: 1,
        component: MeterComponent::Left,
        flags: 1,
        value: 0.5,
    }];
    let batch = |first: u64, end: u64, sequence: u64| MeterBatch {
        observed_sample: crate::SampleTime(end),
        first_sample: crate::SampleTime(first),
        window_sequence: sequence,
        records: &records,
    };
    let mut batch_bytes = vec![
        0;
        codec
            .encoded_meter_batch_len(batch(256, 512, 3))
            .expect("len")
    ];
    codec
        .encode_meter_batch(batch(256, 512, 3), &mut batch_bytes)
        .expect("batch");
    let decoded = codec.decode_meter_batch(&batch_bytes, 6).expect("decoded");
    assert_eq!(
        (
            decoded.first_sample,
            decoded.observed_sample,
            decoded.window_sequence
        ),
        (crate::SampleTime(256), crate::SampleTime(512), 3)
    );
    let u64_field = |id: u8, value: u64| {
        let mut field = vec![id, 0, 4, 1, 8, 0, 0, 0];
        field.extend_from_slice(&value.to_le_bytes());
        field
    };
    for (first, end, sequence) in [(512, 512, 3), (600, 512, 3), (256, 512, 0)] {
        assert_eq!(
            codec.encoded_meter_batch_len(batch(first, end, sequence)),
            Err(EncodeError::InvalidField)
        );
        let frame = replace(&batch_bytes, &u64_field(5, 256), &u64_field(5, first), 0);
        let frame = replace(&frame, &u64_field(6, 3), &u64_field(6, sequence), 0);
        assert_eq!(
            codec.decode_meter_batch(&frame, 6),
            Err(DecodeError::InvalidField),
            "[{first}, {end}) sequence {sequence}"
        );
    }
}

#[test]
fn b3b1_telemetry_and_counters_are_typed_canonical_and_bounded() {
    let codec = codec();
    let configuration = TelemetryConfiguration {
        meters: Vec::new(),
        meter_period_blocks: 4,
        meter_window_blocks: None,
        counter_ids: vec![
            CounterId::ControlCommandBackpressure,
            CounterId::TelemetryCoalesced,
        ],
        counter_period_blocks: 8,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: DiagnosticSeverity::Error,
    };
    let mut config_bytes = [0_u8; 80];
    codec
        .encode_telemetry_configuration(&configuration, &mut config_bytes)
        .expect("config");
    assert_eq!(
        hex(&config_bytes),
        concat!(
            "02000301040000000400000000000000",
            "03000d01080000000100000005000000",
            "04000301040000000800000000000000",
            "05000801010000000100000000000000",
            "06000101010000000300000000000000"
        )
    );
    assert_eq!(
        codec.decode_telemetry_configuration(&config_bytes, 5),
        Ok(configuration.clone())
    );
    for end in 0..config_bytes.len() {
        assert!(
            codec
                .decode_telemetry_configuration(&config_bytes[..end], 5)
                .is_err()
        );
    }
    let bad_config = TelemetryConfiguration {
        counter_ids: vec![
            CounterId::TelemetryCoalesced,
            CounterId::ControlCommandBackpressure,
        ],
        ..configuration.clone()
    };
    assert!(
        codec
            .encoded_telemetry_configuration_len(&bad_config)
            .is_err()
    );
    let bad_coupling = TelemetryConfiguration {
        counter_period_blocks: 0,
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
    let records = [MeterRecord {
        handle: 1,
        component: MeterComponent::Right,
        flags: 3,
        value: 1.5,
    }];
    let batch = MeterBatch {
        observed_sample: crate::SampleTime(9),
        first_sample: crate::SampleTime(3),
        window_sequence: 2,
        records: &records,
    };
    let mut meter_bytes = [0_u8; 104];
    codec
        .encode_meter_batch(batch, &mut meter_bytes)
        .expect("meter");
    assert_eq!(
        hex(&meter_bytes),
        concat!(
            "01000401080000000900000000000000",
            "02000201020000000100000000000000",
            "03000201020000001000000000000000",
            "04000a011000000001000000020003000000c03f00000000",
            "05000401080000000300000000000000",
            "06000401080000000200000000000000"
        )
    );
    let decoded = codec
        .decode_meter_batch(&meter_bytes, 6)
        .expect("decoded meter");
    assert_eq!(decoded.observed_sample, crate::SampleTime(9));
    assert_eq!(decoded.record(0), Ok(records[0]));
    for end in 0..meter_bytes.len() {
        assert!(codec.decode_meter_batch(&meter_bytes[..end], 6).is_err());
    }
    let mut bad_meter = meter_bytes;
    bad_meter[40..42].copy_from_slice(&15_u16.to_le_bytes());
    assert!(codec.decode_meter_batch(&bad_meter, 6).is_err());
    bad_meter = meter_bytes;
    bad_meter[64..68].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(codec.decode_meter_batch(&bad_meter, 6).is_err());
    bad_meter = meter_bytes;
    bad_meter[68] = 1;
    assert!(codec.decode_meter_batch(&bad_meter, 6).is_err());

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
        crate::MessageId::MeterBatch,
        &meter_bytes,
        6,
        concat!(
            "4d49534f43544c00010000003000030020800000680000000000000000000000",
            "07000000000000000600000000000000",
            "0100040108000000090000000000000002000201020000000100000000000000",
            "0300020102000000100000000000000004000a01100000000100000002000300",
            "0000c03f00000000",
            "0500040108000000030000000000000006000401080000000200000000000000"
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
        // #1386: a `SESSION_REPLACE` of an identical document.
        (EditPath::Unchanged, 5),
    ] {
        let value = TransactionApplied::new(5, path);
        let mut bytes = [0_u8; 32];
        let written = codec
            .encode_transaction_applied(&value, &mut bytes)
            .expect("encode");
        assert_eq!(
            bytes[written - 8],
            code,
            "the code is the field's first byte"
        );
        assert_eq!(
            codec.decode_transaction_applied(&bytes[..written], 2),
            Ok(value.clone())
        );
        // Zero and every value above the last allocated code are refused.
        for bad in [0_u8, 6, 7, 255] {
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

/// #1585 D5. The two arm-tag lists of `TransactionApplied` are optional, each written exactly when
/// it holds a tag, and round-trip in order; a present list that is empty or not whole words is
/// refused.
///
/// Red if an empty list is written (a client could not tell "no `Observe` edit" from "an edit
/// with no tag"), if a decoder accepts an empty or ragged list, or if the two lists swap fields.
#[test]
fn transaction_applied_arm_tags_are_present_exactly_when_they_hold_a_tag() {
    let codec = codec();
    let value = TransactionApplied {
        observe_arm_tags: vec![1, 1 << 40],
        removed_arm_tags: vec![7],
        ..TransactionApplied::new(2, EditPath::Rebuild)
    };
    let mut bytes = [0_u8; 96];
    let written = codec
        .encode_transaction_applied(&value, &mut bytes)
        .expect("encode");
    assert_eq!(
        hex(&bytes[written - 40..written]),
        "03000e001000000001000000000000000000000000010000\
         04000e00080000000700000000000000"
    );
    assert_eq!(
        codec.decode_transaction_applied(&bytes[..written], 4),
        Ok(value.clone())
    );
    // Only the removed list: field 3 is absent, not empty.
    let removed_only = TransactionApplied {
        observe_arm_tags: Vec::new(),
        ..value.clone()
    };
    let short = codec
        .encode_transaction_applied(&removed_only, &mut bytes)
        .expect("encode");
    assert_eq!(short, written - 24);
    assert_eq!(
        codec.decode_transaction_applied(&bytes[..short], 3),
        Ok(removed_only)
    );
    // An empty field 4, and a field 4 of four bytes, are refused.
    let mut empty = bytes[..short].to_vec();
    empty[short - 12..short - 8].copy_from_slice(&0_u32.to_le_bytes());
    empty.truncate(short - 8);
    assert!(codec.decode_transaction_applied(&empty, 3).is_err());
    let mut ragged = bytes[..short].to_vec();
    ragged[short - 12..short - 8].copy_from_slice(&4_u32.to_le_bytes());
    assert!(codec.decode_transaction_applied(&ragged, 3).is_err());
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
/// override, a strip ID that is not a stable ID, and a code outside its table (grid, channels,
/// lane, kind, quality, link mode). The value rules are
/// `response_get_request_value_rules_are_refused_on_encode_only`.
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
    let rules: [(&str, bool, Option<Typed>, Wire); 12] = [
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

/// #1623: the value rules of a request -- an empty effect ID, a NaN or infinite axis endpoint or
/// override value, and a bypass override on one lane or with a value that is not 0 or 1 -- are
/// refused on encode, and decoded as sent: the core refuses each with its own typed code
/// (host-core's `response_frame` and host-web's `control_parity` tests), which a decode refusal
/// would replace with the generic `protocol.failure`.
///
/// Red if the decoder refuses one again (the typed refusal becomes unreachable), or the encoder
/// stops refusing one.
#[test]
fn response_get_request_value_rules_are_refused_on_encode_only() {
    let codec = ProtocolCodec::default();
    let effect = full_effect_request();
    let effect_bytes = encode_response_request(&codec, &effect);

    type Typed = fn(&mut ResponseGetRequest);
    type Wire = fn(&mut Vec<Tlv>);
    let rules: [(&str, Typed, Wire); 6] = [
        (
            "an empty effect ID",
            |r| {
                if let ResponseTarget::Configuration(ResponseConfiguration::Effect {
                    effect, ..
                }) = &mut r.target
                {
                    effect.clear();
                }
            },
            |f| set(nth(f, 1, 0), 2, b""),
        ),
        (
            "a bypass override on the left lane",
            |r| r.overrides[0].lane = ParameterChannel::Left,
            |f| set(nth(f, 10, 0), 2, &[1]),
        ),
        (
            "a bypass value of one half",
            |r| r.overrides[0].value = 0.5,
            |f| set(nth(f, 10, 0), 3, &0.5_f32.to_le_bytes()),
        ),
        (
            "a NaN minimum frequency",
            |r| r.minimum_hz = f32::NAN,
            |f| set(f, 6, &f32::NAN.to_le_bytes()),
        ),
        (
            "an infinite maximum frequency",
            |r| r.maximum_hz = f32::INFINITY,
            |f| set(f, 7, &f32::INFINITY.to_le_bytes()),
        ),
        (
            "a NaN override value",
            |r| r.overrides[1].value = f32::NAN,
            |f| set(nth(f, 10, 1), 3, &f32::NAN.to_le_bytes()),
        ),
    ];
    for (name, typed, wire) in rules {
        let mut request = effect.clone();
        typed(&mut request);
        assert_eq!(
            codec.encoded_response_request_len(&request),
            Err(EncodeError::InvalidField),
            "{name}: encode"
        );
        let (patched, count) = edited(&effect_bytes, wire);
        let decoded = codec
            .decode_response_request(&patched, count)
            .unwrap_or_else(|error| panic!("{name}: decode refused it: {error:?}"));
        // `Debug` compares a NaN with itself, where `PartialEq` would not.
        assert_eq!(
            format!("{decoded:?}"),
            format!("{request:?}"),
            "{name}: decoded as sent"
        );
    }
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

fn ready_member(section: ResponseMemberSection, kind: ResponseMemberKind) -> ResponseMember {
    ResponseMember {
        section,
        effect: "miso.parametric-eq".to_owned(),
        kind,
        bypassed: false,
        enabled_left: 0b101,
        enabled_right: 0b011,
        section_count: 3,
    }
}

/// A ready `RESPONSE_READY` with both sides and one member of each section.
fn ready_event(points: usize) -> ResponseReady {
    let curve = (0..points).map(|index| -(index as f32)).collect::<Vec<_>>();
    ResponseReady {
        sequence: 9,
        request_tag: 4,
        strip_id: "vocal".to_owned(),
        outcome: ResponseReadyOutcome::Ready(TrackResponseCapture {
            captured_sample: 4_096,
            sample_rate_hz: 48_000,
            floor_db: -120.0,
            frequencies_hz: (0..points).map(|index| 20.0 + index as f32).collect(),
            left_db: Some(curve.clone()),
            right_db: Some(curve),
            excluded_count: 1,
            members: vec![
                ready_member(
                    ResponseMemberSection::Builtins,
                    ResponseMemberKind::InputFilters,
                ),
                ready_member(
                    ResponseMemberSection::Console("bus_eq".to_owned()),
                    ResponseMemberKind::ParametricEq,
                ),
                ResponseMember {
                    bypassed: true,
                    ..ready_member(
                        ResponseMemberSection::Inserts("comp".to_owned()),
                        ResponseMemberKind::Unavailable,
                    )
                },
            ],
        }),
    }
}

fn refused_event() -> ResponseReady {
    ResponseReady {
        outcome: ResponseReadyOutcome::Refused {
            status: crate::StatusCode::NotFound,
            diagnostic: "response.strip.removed".to_owned(),
        },
        ..ready_event(2)
    }
}

fn encode_ready(codec: &ProtocolCodec, value: &ResponseReady) -> Vec<u8> {
    let mut bytes = vec![0_u8; codec.encoded_response_ready_len(value).expect("length")];
    let written = codec
        .encode_response_ready(value, &mut bytes)
        .expect("encode");
    assert_eq!(written, bytes.len());
    bytes
}

fn ready_capture(value: &mut ResponseReady) -> &mut TrackResponseCapture {
    match &mut value.outcome {
        ResponseReadyOutcome::Ready(capture) => capture,
        ResponseReadyOutcome::Refused { .. } => panic!("a ready event"),
    }
}

/// #1578 gate 4, the codec half: both `RESPONSE_READY` forms round-trip exactly, and the codec
/// refuses every other shape: on decode, a ready event without its axis, sides or capture fields
/// or with a diagnostic, a refusal without its diagnostic or with any capture field, an outcome
/// outside the four refused statuses, a member whose slot ID does not match its section, and masks
/// past the section count; on encode, the same rules where the typed value can break them.
///
/// Red if a field or member is lost or reordered in the round trip, or one presence rule of D2 is
/// not enforced in either direction (a client could then read a refusal as a capture, or a host
/// write an event no client decodes).
#[test]
fn response_ready_round_trips_both_forms_and_refuses_every_other_shape() {
    let codec = ProtocolCodec::default();
    let ready = ready_event(4);
    let refused = refused_event();
    let ready_bytes = encode_ready(&codec, &ready);
    let refused_bytes = encode_ready(&codec, &refused);
    // Fields 1-4, 6-12 and three members; fields 1-5.
    assert_eq!(
        codec.decode_response_ready(&ready_bytes, 14),
        Ok(ready.clone())
    );
    assert_eq!(
        codec.decode_response_ready(&refused_bytes, 5),
        Ok(refused.clone())
    );

    type Wire = fn(&mut Vec<Tlv>);
    let ready_rules: [(&str, Wire); 9] = [
        ("no axis", |f| remove(f, 9)),
        ("no capture sample", |f| remove(f, 6)),
        ("no excluded count", |f| remove(f, 12)),
        ("neither side", |f| {
            remove(f, 10);
            remove(f, 11);
        }),
        ("a side of the wrong length", |f| set(f, 10, &[0; 12])),
        ("a diagnostic", |f| insert(f, leaf(5, 9, b"response.limit"))),
        ("a builtins member with a slot ID", |f| {
            set(nth(f, 13, 1), 1, &[4]);
        }),
        ("an insert member without a slot ID", |f| {
            remove(nth(f, 13, 2), 2)
        }),
        ("a mask past the section count", |f| {
            set(nth(f, 13, 0), 6, &0b1000_u32.to_le_bytes());
        }),
    ];
    for (name, wire) in ready_rules {
        let (patched, count) = edited(&ready_bytes, wire);
        assert_eq!(
            codec.decode_response_ready(&patched, count),
            Err(DecodeError::InvalidField),
            "ready: {name}"
        );
    }
    let refused_rules: [(&str, Wire); 5] = [
        ("no diagnostic", |f| remove(f, 5)),
        ("a curve", |f| insert(f, leaf(10, 15, &[0; 8]))),
        ("a capture sample", |f| insert(f, leaf(6, 4, &[0; 8]))),
        ("backpressure", |f| set(f, 4, &11_u16.to_le_bytes())),
        ("invalid field", |f| set(f, 4, &5_u16.to_le_bytes())),
    ];
    for (name, wire) in refused_rules {
        let (patched, count) = edited(&refused_bytes, wire);
        assert_eq!(
            codec.decode_response_ready(&patched, count),
            Err(DecodeError::InvalidField),
            "refused: {name}"
        );
    }

    // The encoder refuses what the typed value can break.
    type Typed = fn(&mut ResponseReady);
    let typed_rules: [(&str, Typed); 7] = [
        ("one axis point", |v| {
            let capture = ready_capture(v);
            capture.frequencies_hz.truncate(1);
            capture.left_db.as_mut().expect("left").truncate(1);
            capture.right_db.as_mut().expect("right").truncate(1);
        }),
        ("neither side", |v| {
            let capture = ready_capture(v);
            capture.left_db = None;
            capture.right_db = None;
        }),
        ("a side of the wrong length", |v| {
            ready_capture(v).right_db.as_mut().expect("right").pop();
        }),
        ("a mask past the section count", |v| {
            ready_capture(v).members[0].enabled_right = 0b1000;
        }),
        ("a strip ID that is not a stable ID", |v| {
            v.strip_id = "Vocal".to_owned();
        }),
        ("an empty diagnostic", |v| {
            *v = refused_event();
            if let ResponseReadyOutcome::Refused { diagnostic, .. } = &mut v.outcome {
                diagnostic.clear();
            }
        }),
        ("a backpressure outcome", |v| {
            *v = refused_event();
            if let ResponseReadyOutcome::Refused { status, .. } = &mut v.outcome {
                *status = crate::StatusCode::Backpressure;
            }
        }),
    ];
    for (name, typed) in typed_rules {
        let mut value = ready.clone();
        typed(&mut value);
        assert_eq!(
            codec.encoded_response_ready_len(&value),
            Err(EncodeError::InvalidField),
            "{name}"
        );
    }
}

/// `response_ready_encoded_bound` is the exact frame length of the largest event it bounds: a
/// ready event whose strip, slot and effect IDs are all 127 bytes, or, when that is smaller, a
/// refusal with a 127-byte diagnostic.
///
/// Red if the arithmetic drifts from the encoder (#1578b's provider would then arm a capture
/// whose event cannot be sent, or refuse one that fits).
#[test]
fn response_ready_encoded_bound_is_the_largest_frame_exactly() {
    let codec = ProtocolCodec::default();
    let long = |prefix: char| {
        let mut id = String::from(prefix);
        id.push_str(&"x".repeat(126));
        id
    };
    let frame_len = |value: &ResponseReady| {
        let mut frame = vec![0_u8; 1 << 20];
        codec
            .encode_event_frame_into(
                &crate::TypedEventFrame {
                    revision: crate::SessionRevision(3),
                    payload: crate::EventPayload::ResponseReady(value),
                },
                &mut frame,
            )
            .expect("frame")
    };
    let refused = ResponseReady {
        strip_id: long('s'),
        outcome: ResponseReadyOutcome::Refused {
            status: crate::StatusCode::Internal,
            diagnostic: long('d'),
        },
        ..ready_event(2)
    };
    for points in [2_usize, 3, 64, 1000] {
        for channels in [ParameterChannel::Left, ParameterChannel::Both] {
            for owners in [0_usize, 1, 5] {
                let mut value = ready_event(points);
                value.strip_id = long('s');
                let capture = ready_capture(&mut value);
                if channels == ParameterChannel::Left {
                    capture.right_db = None;
                }
                capture.members = (0..owners)
                    .map(|_| ResponseMember {
                        effect: long('e'),
                        ..ready_member(
                            ResponseMemberSection::Console(long('c')),
                            ResponseMemberKind::ParametricEq,
                        )
                    })
                    .collect();
                assert_eq!(
                    response_ready_encoded_bound(points, channels, owners),
                    Some(frame_len(&value).max(frame_len(&refused))),
                    "{points} points, {channels:?}, {owners} owners"
                );
            }
        }
    }
    assert_eq!(
        response_ready_encoded_bound(usize::MAX / 2, ParameterChannel::Both, 1),
        None
    );
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

fn spectrum_entry(kind: SpectrumTargetKind, id: &str, channels: ParameterChannel) -> SpectrumEntry {
    SpectrumEntry {
        target_kind: kind,
        target_id: id.to_owned(),
        channels,
    }
}

fn encode_spectrum_request(codec: &ProtocolCodec, value: &SpectrumConfigureRequest) -> Vec<u8> {
    let mut bytes = vec![
        0;
        codec
            .encoded_spectrum_configure_request_len(value)
            .expect("length")
    ];
    codec
        .encode_spectrum_configure_request(value, &mut bytes)
        .expect("encode");
    bytes
}

/// #1594 gate 1: a `SPECTRUM_CONFIGURE` request of each form and its reply with and without a plan
/// epoch round-trip exactly; a form, kind or channel code outside its table, a 128-byte identity, a
/// hop of 300, a single form with two entries and a reply whose `rebuild` disagrees with its plan
/// epoch are each refused as `INVALID_FIELD`, on encode where the typed value can say it and always
/// on decode.
///
/// Red if the codec admits a malformed or contradictory request or reply in either direction.
#[test]
fn spectrum_configure_round_trips_and_refuses_malformed_requests_both_ways() {
    let codec = ProtocolCodec::default();
    let single = SpectrumConfigureRequest {
        form: SpectrumForm::Single,
        entries: vec![spectrum_entry(
            SpectrumTargetKind::TrackPostPan,
            "t1",
            ParameterChannel::Both,
        )],
        maximum_capture_bytes: 1 << 16,
        hop_frames: None,
    };
    let collection = SpectrumConfigureRequest {
        form: SpectrumForm::Collection,
        entries: vec![
            spectrum_entry(
                SpectrumTargetKind::TrackPostInput,
                "t1",
                ParameterChannel::Left,
            ),
            spectrum_entry(SpectrumTargetKind::Output, "main", ParameterChannel::Right),
        ],
        maximum_capture_bytes: 1 << 20,
        hop_frames: Some(2048),
    };
    let empty = SpectrumConfigureRequest {
        form: SpectrumForm::Collection,
        entries: Vec::new(),
        maximum_capture_bytes: 0,
        hop_frames: None,
    };
    for request in [&single, &collection, &empty] {
        let bytes = encode_spectrum_request(&codec, request);
        let count = top_level_count(&bytes);
        assert_eq!(
            codec.decode_spectrum_configure_request(&bytes, count),
            Ok(request.clone())
        );
    }
    for reply in [
        SpectrumConfigured {
            revision: 5,
            configured_epoch: 3,
            adopted_epoch: 2,
            rebuild: true,
            plan_epoch: Some(3),
        },
        SpectrumConfigured {
            revision: 0,
            configured_epoch: 1,
            adopted_epoch: 1,
            rebuild: false,
            plan_epoch: None,
        },
    ] {
        let mut bytes = vec![
            0;
            codec
                .encoded_spectrum_configured_len(&reply)
                .expect("length")
        ];
        codec
            .encode_spectrum_configured(&reply, &mut bytes)
            .expect("encode");
        let count = top_level_count(&bytes);
        assert_eq!(codec.decode_spectrum_configured(&bytes, count), Ok(reply));
        // `rebuild` that disagrees with the plan epoch, both ways.
        let flipped = SpectrumConfigured {
            rebuild: !reply.rebuild,
            ..reply
        };
        assert_eq!(
            codec.encode_spectrum_configured(&flipped, &mut [0; 256]),
            Err(EncodeError::InvalidField)
        );
        let (wire, count) = edited(&bytes, |f| set(f, 4, &[u8::from(!reply.rebuild)]));
        assert_eq!(
            codec.decode_spectrum_configured(&wire, count),
            Err(DecodeError::InvalidField)
        );
    }

    let long = "t".repeat(SPECTRUM_TARGET_ID_MAXIMUM_BYTES + 1);
    // Typed refusals: what the typed value can say.
    let mut long_id = single.clone();
    long_id.entries[0].target_id.clone_from(&long);
    let mut empty_id = single.clone();
    empty_id.entries[0].target_id.clear();
    let mut two = collection.clone();
    two.form = SpectrumForm::Single;
    let mut odd_hop = single.clone();
    odd_hop.hop_frames = Some(300);
    for refused in [&long_id, &empty_id, &two, &odd_hop] {
        assert_eq!(
            codec.encoded_spectrum_configure_request_len(refused),
            Err(EncodeError::InvalidField)
        );
    }

    // Wire refusals: each rule on decode.
    let single_bytes = encode_spectrum_request(&codec, &single);
    let collection_bytes = encode_spectrum_request(&codec, &collection);
    type Edit = Box<dyn Fn(&mut Vec<Tlv>)>;
    let rules: [(&str, &Vec<u8>, Edit); 8] = [
        ("form 3", &single_bytes, Box::new(|f| set(f, 1, &[3]))),
        ("form 0", &single_bytes, Box::new(|f| set(f, 1, &[0]))),
        (
            "kind 4",
            &single_bytes,
            Box::new(|f| set(nth(f, 2, 0), 1, &[4])),
        ),
        (
            "channels 0",
            &single_bytes,
            Box::new(|f| set(nth(f, 2, 0), 3, &[0])),
        ),
        (
            "a 128-byte identity",
            &single_bytes,
            Box::new(move |f| set(nth(f, 2, 0), 2, long.as_bytes())),
        ),
        (
            "an empty identity",
            &single_bytes,
            Box::new(|f| set(nth(f, 2, 0), 2, b"")),
        ),
        (
            "hop 300",
            &collection_bytes,
            Box::new(|f| set(f, 4, &300_u32.to_le_bytes())),
        ),
        (
            "a single form with two entries",
            &collection_bytes,
            Box::new(|f| set(f, 1, &[SpectrumForm::Single.code()])),
        ),
    ];
    for (name, base, edit) in rules {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_spectrum_configure_request(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
        assert_eq!(
            DecodeError::InvalidField.status(),
            crate::StatusCode::InvalidField
        );
    }
}

fn capture_ready(
    channels: ParameterChannel,
    include_window: bool,
) -> SpectrumCaptureReply<'static> {
    use std::borrow::Cow;
    let bins = |seed: f32| -> Option<Cow<'static, [f32]>> {
        Some(Cow::Owned(
            (0..SPECTRUM_CAPTURE_BIN_COUNT)
                .map(|bin| seed - bin as f32 / 16.0)
                .collect(),
        ))
    };
    let window = |seed: f32| -> Option<Cow<'static, [f32]>> {
        include_window.then(|| {
            Cow::Owned(
                (0..SPECTRUM_CAPTURE_WINDOW_FRAMES)
                    .map(|frame| seed * (frame as f32 / 2048.0 - 0.5))
                    .collect(),
            )
        })
    };
    let left = matches!(channels, ParameterChannel::Left | ParameterChannel::Both);
    let right = matches!(channels, ParameterChannel::Right | ParameterChannel::Both);
    SpectrumCaptureReply {
        adopted_epoch: 3,
        selection_epoch: 2,
        status: SpectrumCaptureStatus::Ready,
        window: Some(SpectrumCaptureWindow {
            target_kind: SpectrumTargetKind::TrackPostPan,
            target_id: Cow::Borrowed("eq0"),
            channels,
            sample_rate_hz: 48_000,
            window_frames: SPECTRUM_CAPTURE_WINDOW_FRAMES,
            bin_count: SPECTRUM_CAPTURE_BIN_COUNT,
            floor_db: -120.0,
            captured_sample: 4_096,
            end_sample: 6_144,
            snapshot_token: 2,
            source_underrun: true,
            frequencies_hz: Cow::Owned(
                (0..SPECTRUM_CAPTURE_BIN_COUNT)
                    .map(|bin| bin as f32 * 23.4375)
                    .collect(),
            ),
            left_db: if left { bins(-3.0) } else { None },
            right_db: if right { bins(-9.0) } else { None },
            left_window: if left { window(0.5) } else { None },
            right_window: if right { window(0.25) } else { None },
        }),
    }
}

fn encode_capture_reply(codec: &ProtocolCodec, value: &SpectrumCaptureReply<'_>) -> Vec<u8> {
    let mut bytes = vec![
        0;
        codec
            .encoded_spectrum_capture_reply_len(value)
            .expect("length")
    ];
    codec
        .encode_spectrum_capture_reply(value, &mut bytes)
        .expect("encode");
    bytes
}

/// #1595 gate 1 (the capture half): each `SPECTRUM_CAPTURE` operation and the done, pending and
/// ready replies (each channel mask, with and without the raw window) round-trip exactly, and
/// the frame-limit measure equals the encoded frame of each ready shape. Every presence rule of
/// D2 (a field outside its operation, a missing one), an operation or status outside its table, a
/// plane that disagrees with `channels`, a 1,024-bin plane, a window plane without its pair, a
/// pending reply with a window field and an `end_sample` that is not `captured_sample + 2048`
/// are refused as `INVALID_FIELD`, on encode where the typed value can say it and on decode.
///
/// Red if the codec admits a contradictory request or a reply of the wrong shape in either
/// direction, or the measure the session refuses reads by drifts from the encoder.
#[test]
fn spectrum_capture_round_trips_and_refuses_malformed_frames_both_ways() {
    let codec = ProtocolCodec::default();
    let read = SpectrumCaptureRequest::Read {
        channels: ParameterChannel::Right,
        include_window: true,
    };
    let select = SpectrumCaptureRequest::Select {
        target_kind: SpectrumTargetKind::Output,
        target_id: "main".to_owned(),
        channels: ParameterChannel::Both,
    };
    let encode_request = |value: &SpectrumCaptureRequest| {
        let mut bytes = vec![
            0;
            codec
                .encoded_spectrum_capture_request_len(value)
                .expect("length")
        ];
        codec
            .encode_spectrum_capture_request(value, &mut bytes)
            .expect("encode");
        bytes
    };
    for request in [
        SpectrumCaptureRequest::Arm,
        SpectrumCaptureRequest::Read {
            channels: ParameterChannel::Left,
            include_window: false,
        },
        read.clone(),
        SpectrumCaptureRequest::Cancel,
        select.clone(),
    ] {
        let bytes = encode_request(&request);
        assert_eq!(
            codec.decode_spectrum_capture_request(&bytes, top_level_count(&bytes)),
            Ok(request)
        );
    }
    let read_bytes = encode_request(&read);
    let select_bytes = encode_request(&select);
    type Edit = Box<dyn Fn(&mut Vec<Tlv>)>;
    let requests: [(&str, &Vec<u8>, Edit); 7] = [
        ("op 0", &read_bytes, Box::new(|f| set(f, 1, &[0]))),
        ("op 5", &read_bytes, Box::new(|f| set(f, 1, &[5]))),
        (
            "arm with channels",
            &read_bytes,
            Box::new(|f| set(f, 1, &[1])),
        ),
        (
            "cancel with include_window",
            &read_bytes,
            Box::new(|f| {
                remove(f, 2);
                set(f, 1, &[3]);
            }),
        ),
        (
            "read with a target",
            &select_bytes,
            Box::new(|f| set(f, 1, &[2])),
        ),
        (
            "read without channels",
            &read_bytes,
            Box::new(|f| remove(f, 2)),
        ),
        (
            "select without its target",
            &read_bytes,
            Box::new(|f| {
                remove(f, 5);
                set(f, 1, &[4]);
            }),
        ),
    ];
    for (name, base, edit) in requests {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_spectrum_capture_request(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
    let long = SpectrumCaptureRequest::Select {
        target_kind: SpectrumTargetKind::Output,
        target_id: "m".repeat(SPECTRUM_TARGET_ID_MAXIMUM_BYTES + 1),
        channels: ParameterChannel::Both,
    };
    assert_eq!(
        codec.encoded_spectrum_capture_request_len(&long),
        Err(EncodeError::InvalidField)
    );

    // Replies: each shape round-trips, and the measure is the encoded frame.
    let mut replies = vec![
        SpectrumCaptureReply {
            adopted_epoch: 1,
            selection_epoch: 0,
            status: SpectrumCaptureStatus::Done,
            window: None,
        },
        SpectrumCaptureReply {
            adopted_epoch: 1,
            selection_epoch: 4,
            status: SpectrumCaptureStatus::Pending,
            window: None,
        },
    ];
    for channels in [
        ParameterChannel::Left,
        ParameterChannel::Right,
        ParameterChannel::Both,
    ] {
        for include_window in [false, true] {
            let reply = capture_ready(channels, include_window);
            let frame = crate::TypedSuccessResponseFrame {
                request_id: crate::RequestId::new(9).expect("nonzero"),
                revision: crate::SessionRevision(3),
                payload: crate::SuccessResponsePayload::SpectrumCaptured(reply.clone()),
            };
            let mut output = vec![0; 1 << 16];
            let written = codec
                .encode_success_response_frame_into(&frame, &mut output)
                .expect("frame");
            assert_eq!(
                codec.spectrum_capture_ready_frame_len(channels, include_window, 3),
                Ok(written),
                "{channels:?} {include_window}"
            );
            replies.push(reply);
        }
    }
    for reply in &replies {
        let bytes = encode_capture_reply(&codec, reply);
        assert_eq!(
            codec.decode_spectrum_capture_reply(&bytes, top_level_count(&bytes)),
            Ok(reply.clone())
        );
    }

    // Typed refusals.
    let mut wrong_plane = capture_ready(ParameterChannel::Both, false);
    wrong_plane.window.as_mut().expect("ready").right_db = None;
    let mut short_plane = capture_ready(ParameterChannel::Left, false);
    short_plane.window.as_mut().expect("ready").left_db =
        Some(std::borrow::Cow::Owned(vec![0.0; 1024]));
    let mut ready_without = capture_ready(ParameterChannel::Left, false);
    ready_without.window = None;
    let mut pending_with = capture_ready(ParameterChannel::Left, false);
    pending_with.status = SpectrumCaptureStatus::Pending;
    for refused in [&wrong_plane, &short_plane, &ready_without, &pending_with] {
        assert_eq!(
            codec.encoded_spectrum_capture_reply_len(refused),
            Err(EncodeError::InvalidField)
        );
    }

    // Wire refusals.
    let both = encode_capture_reply(&codec, &capture_ready(ParameterChannel::Both, true));
    let left = encode_capture_reply(&codec, &capture_ready(ParameterChannel::Left, false));
    let rules: [(&str, &Vec<u8>, Edit); 9] = [
        ("status 0", &left, Box::new(|f| set(f, 3, &[0]))),
        ("status 4", &left, Box::new(|f| set(f, 3, &[4]))),
        (
            "a pending reply with a window",
            &left,
            Box::new(|f| set(f, 3, &[2])),
        ),
        (
            "planes that disagree with channels",
            &both,
            Box::new(|f| set(f, 6, &[1])),
        ),
        (
            "a 1,024-bin plane",
            &left,
            Box::new(|f| set(f, 16, &[0; 4 * 1024])),
        ),
        (
            "bin count 1,024",
            &left,
            Box::new(|f| set(f, 9, &1024_u32.to_le_bytes())),
        ),
        (
            "window frames 1,024",
            &left,
            Box::new(|f| set(f, 8, &1024_u32.to_le_bytes())),
        ),
        (
            "a window plane without its pair",
            &both,
            Box::new(|f| remove(f, 19)),
        ),
        (
            "end_sample one past",
            &left,
            Box::new(|f| set(f, 12, &6_145_u64.to_le_bytes())),
        ),
    ];
    for (name, base, edit) in rules {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_spectrum_capture_reply(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
}

/// A ready `SPECTRUM_STREAM` reply on `eq0` with `channels`' planes.
fn stream_ready(channels: ParameterChannel) -> SpectrumStreamReply<'static> {
    use std::borrow::Cow;
    let bins = |level: f32| -> Option<Cow<'static, [f32]>> {
        Some(Cow::Owned(
            (0..SPECTRUM_CAPTURE_BIN_COUNT)
                .map(|bin| level - bin as f32 / 100.0)
                .collect(),
        ))
    };
    let left = matches!(channels, ParameterChannel::Left | ParameterChannel::Both);
    let right = matches!(channels, ParameterChannel::Right | ParameterChannel::Both);
    SpectrumStreamReply {
        adopted_epoch: 2,
        selection_epoch: 3,
        status: SpectrumStreamStatus::Ready,
        target: Some(SpectrumStreamTarget {
            target_kind: SpectrumTargetKind::TrackPostPan,
            target_id: Cow::Borrowed("eq0"),
            channels,
        }),
        sample_rate_hz: 48_000,
        quantum_frames: 128,
        hop_frames: 2_048,
        capture_epoch: 1,
        sequence: 2,
        dropped_captures: 0,
        windows: 3,
        captured_sample: 4_096,
        end_sample: 6_144,
        analysis_epoch: 1,
        history_start_sample: 0,
        smoothing_ms: 100.0,
        source_underrun: false,
        bins: Some(SpectrumStreamBins {
            frequencies_hz: Cow::Owned(
                (0..SPECTRUM_CAPTURE_BIN_COUNT)
                    .map(|bin| bin as f32 * 23.4375)
                    .collect(),
            ),
            left_db: if left { bins(-3.0) } else { None },
            right_db: if right { bins(-9.0) } else { None },
        }),
    }
}

/// #1595 gate 1 (the stream half): each `SPECTRUM_STREAM` operation and a reply of every status
/// (with and without a selection; ready with each channel mask) round-trip exactly, and the
/// frame-limit measure equals the encoded frame of each ready shape. Every presence rule of D3, an
/// operation or a status outside its table, a smoothing of 10,001 ms or NaN, planes that disagree
/// with `channels`, a 1,024-bin plane, a ready reply without a selection, a selection without its
/// identity and a ready window count that is not `sequence + 1` are refused as `INVALID_FIELD`, on
/// encode where the typed value can say it and on decode.
///
/// Red if the codec admits a contradictory stream request or a reply of the wrong shape in either
/// direction, or the measure the session refuses stream reads by drifts from the encoder.
#[test]
fn spectrum_stream_round_trips_and_refuses_malformed_frames_both_ways() {
    use std::borrow::Cow;
    let codec = ProtocolCodec::default();
    let select = SpectrumStreamRequest::Select {
        target_kind: SpectrumTargetKind::TrackPostInput,
        target_id: "eq1".to_owned(),
        channels: ParameterChannel::Left,
        smoothing_ms: Some(50.0),
    };
    let start = SpectrumStreamRequest::Start {
        smoothing_ms: 100.0,
    };
    let encode_request = |value: &SpectrumStreamRequest| {
        let mut bytes = vec![
            0;
            codec
                .encoded_spectrum_stream_request_len(value)
                .expect("length")
        ];
        codec
            .encode_spectrum_stream_request(value, &mut bytes)
            .expect("encode");
        bytes
    };
    for request in [
        start.clone(),
        select.clone(),
        SpectrumStreamRequest::Select {
            target_kind: SpectrumTargetKind::Output,
            target_id: "main".to_owned(),
            channels: ParameterChannel::Both,
            smoothing_ms: None,
        },
        SpectrumStreamRequest::Read,
        SpectrumStreamRequest::Reset,
        SpectrumStreamRequest::Stop,
    ] {
        let bytes = encode_request(&request);
        assert_eq!(
            codec.decode_spectrum_stream_request(&bytes, top_level_count(&bytes)),
            Ok(request)
        );
    }
    let start_bytes = encode_request(&start);
    let select_bytes = encode_request(&select);
    type Edit = Box<dyn Fn(&mut Vec<Tlv>)>;
    let requests: [(&str, &Vec<u8>, Edit); 8] = [
        ("op 0", &start_bytes, Box::new(|f| set(f, 1, &[0]))),
        ("op 6", &start_bytes, Box::new(|f| set(f, 1, &[6]))),
        (
            "start without smoothing",
            &start_bytes,
            Box::new(|f| remove(f, 2)),
        ),
        (
            "read with smoothing",
            &start_bytes,
            Box::new(|f| set(f, 1, &[3])),
        ),
        (
            "start with a target",
            &select_bytes,
            Box::new(|f| set(f, 1, &[1])),
        ),
        (
            "select without channels",
            &select_bytes,
            Box::new(|f| remove(f, 5)),
        ),
        (
            "smoothing 10,001",
            &start_bytes,
            Box::new(|f| set(f, 2, &10_001.0_f64.to_le_bytes())),
        ),
        (
            "smoothing NaN",
            &select_bytes,
            Box::new(|f| set(f, 2, &f64::NAN.to_le_bytes())),
        ),
    ];
    for (name, base, edit) in requests {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_spectrum_stream_request(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
    for refused in [
        SpectrumStreamRequest::Start {
            smoothing_ms: 10_001.0,
        },
        SpectrumStreamRequest::Start { smoothing_ms: -1.0 },
        SpectrumStreamRequest::Select {
            target_kind: SpectrumTargetKind::Output,
            target_id: String::new(),
            channels: ParameterChannel::Both,
            smoothing_ms: None,
        },
    ] {
        assert_eq!(
            codec.encoded_spectrum_stream_request_len(&refused),
            Err(EncodeError::InvalidField),
            "{refused:?}"
        );
    }

    // Replies: a reply of every status round-trips, and the measure is the encoded frame.
    let inactive = SpectrumStreamReply {
        status: SpectrumStreamStatus::Inactive,
        target: None,
        bins: None,
        ..stream_ready(ParameterChannel::Left)
    };
    let mut replies = vec![inactive.clone()];
    for status in [
        SpectrumStreamStatus::Warming,
        SpectrumStreamStatus::Pending,
        SpectrumStreamStatus::Gap,
        SpectrumStreamStatus::Failed,
        SpectrumStreamStatus::Stopped,
    ] {
        replies.push(SpectrumStreamReply {
            status,
            dropped_captures: 2,
            bins: None,
            ..stream_ready(ParameterChannel::Both)
        });
    }
    for channels in [
        ParameterChannel::Left,
        ParameterChannel::Right,
        ParameterChannel::Both,
    ] {
        let reply = stream_ready(channels);
        let frame = crate::TypedSuccessResponseFrame {
            request_id: crate::RequestId::new(9).expect("nonzero"),
            revision: crate::SessionRevision(3),
            payload: crate::SuccessResponsePayload::SpectrumStreamed(reply.clone()),
        };
        let mut output = vec![0; 1 << 16];
        let written = codec
            .encode_success_response_frame_into(&frame, &mut output)
            .expect("frame");
        assert_eq!(
            codec.spectrum_stream_ready_frame_len(channels, 3),
            Ok(written),
            "{channels:?}"
        );
        replies.push(reply);
    }
    let encode_reply = |value: &SpectrumStreamReply<'_>| {
        let mut bytes = vec![
            0;
            codec
                .encoded_spectrum_stream_reply_len(value)
                .expect("length")
        ];
        codec
            .encode_spectrum_stream_reply(value, &mut bytes)
            .expect("encode");
        bytes
    };
    for reply in &replies {
        let bytes = encode_reply(reply);
        assert_eq!(
            codec.decode_spectrum_stream_reply(&bytes, top_level_count(&bytes)),
            Ok(reply.clone())
        );
    }

    // Typed refusals.
    let mut wrong_plane = stream_ready(ParameterChannel::Both);
    wrong_plane.bins.as_mut().expect("ready").right_db = None;
    let mut short_plane = stream_ready(ParameterChannel::Left);
    short_plane.bins.as_mut().expect("ready").left_db = Some(Cow::Owned(vec![0.0; 1024]));
    let ready_without_bins = SpectrumStreamReply {
        bins: None,
        ..stream_ready(ParameterChannel::Left)
    };
    let pending_with_bins = SpectrumStreamReply {
        status: SpectrumStreamStatus::Pending,
        ..stream_ready(ParameterChannel::Left)
    };
    let ready_without_target = SpectrumStreamReply {
        target: None,
        ..stream_ready(ParameterChannel::Left)
    };
    let rough = SpectrumStreamReply {
        smoothing_ms: 10_001.0,
        ..inactive
    };
    for refused in [
        &wrong_plane,
        &short_plane,
        &ready_without_bins,
        &pending_with_bins,
        &ready_without_target,
        &rough,
    ] {
        assert_eq!(
            codec.encoded_spectrum_stream_reply_len(refused),
            Err(EncodeError::InvalidField)
        );
    }

    // Wire refusals.
    let both = encode_reply(&stream_ready(ParameterChannel::Both));
    let left = encode_reply(&stream_ready(ParameterChannel::Left));
    let rules: [(&str, &Vec<u8>, Edit); 10] = [
        ("status 7", &left, Box::new(|f| set(f, 3, &[7]))),
        (
            "windows not sequence + 1",
            &left,
            Box::new(|f| set(f, 13, &4_u64.to_le_bytes())),
        ),
        (
            "a pending reply with bins",
            &left,
            Box::new(|f| set(f, 3, &[2])),
        ),
        (
            "planes that disagree with channels",
            &both,
            Box::new(|f| set(f, 6, &[1])),
        ),
        (
            "a 1,024-bin plane",
            &left,
            Box::new(|f| set(f, 21, &[0; 4 * 1024])),
        ),
        (
            "a plane without the axis",
            &left,
            Box::new(|f| remove(f, 20)),
        ),
        (
            "a selection without its identity",
            &left,
            Box::new(|f| remove(f, 5)),
        ),
        (
            "an identity without a selection",
            &left,
            Box::new(|f| set(f, 4, &[0])),
        ),
        (
            "smoothing 10,001",
            &left,
            Box::new(|f| set(f, 18, &10_001.0_f64.to_le_bytes())),
        ),
        (
            "end_sample one past",
            &left,
            Box::new(|f| set(f, 15, &6_145_u64.to_le_bytes())),
        ),
    ];
    for (name, base, edit) in rules {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_spectrum_stream_reply(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
}

// --- OBSERVATION_READ (#1586 D1) ---

fn observation_selection(strip: &str, section: controls::TapSection) -> ObservationReadSelection {
    ObservationReadSelection {
        strip_id: session::StableId::parse(strip).expect("strip"),
        section,
        tap_id: 1,
        channels: ParameterChannel::Both,
    }
}

fn observation_selections(count: usize) -> ObservationReadRequest {
    let comp = session::StableId::parse("comp").expect("insert");
    ObservationReadRequest {
        selections: (0..count)
            .map(|index| ObservationReadSelection {
                tap_id: u32::try_from(index).expect("small"),
                ..observation_selection("vocal", controls::TapSection::Inserts(comp.clone()))
            })
            .collect(),
    }
}

fn ready_row(left: Option<f32>, right: Option<f32>) -> ObservationReadRow {
    ObservationReadRow {
        status: ObservationRowStatus::Ready,
        armed: true,
        arm_sample: 1_024,
        arm_tag: 3,
        window: Some(ObservationRowWindow {
            first_sample: 1_024,
            end_sample: 1_536,
            sequence: 2,
            blocks: 4,
        }),
        left,
        right,
        unit: ParameterUnit::Db,
    }
}

fn encode_observation_page(codec: &ProtocolCodec, page: &ObservationReadPage<'_>) -> Vec<u8> {
    let mut bytes = vec![
        0;
        codec
            .encoded_observation_read_page_len(page)
            .expect("length")
    ];
    codec
        .encode_observation_read_page(page, &mut bytes)
        .expect("encode");
    bytes
}

/// #1586 D1 and gate 3 (the codec half): a request of console and insert selections with each
/// lane set, a request of 256 selections and pages of every status (ready with each lane set, an
/// armed pending row, an unarmed row that render still runs armed) round-trip exactly, and the
/// largest-frame measure equals the encoded frame of 1 and of 256 widest rows. An empty request,
/// one of 257 selections, a selection address with a `control` field, a status or unit outside its
/// table, a window field without the other three, a window or a lane on a row that is not ready, a
/// ready row with no lane, unarmed, untagged or with a window that opened before the arm, and an
/// empty or 257-row page are refused as `INVALID_FIELD`, on encode where the typed value can say
/// it and on decode.
///
/// Red if the codec admits a read that is not 1 to 256 selections or a row whose window, lanes and
/// arm state disagree with its status in either direction, or if the measure the session bounds a
/// read's reply by drifts from the encoder.
#[test]
fn observation_read_round_trips_and_refuses_malformed_frames_both_ways() {
    use std::borrow::Cow;
    let codec = ProtocolCodec::default();
    let encode_request = |value: &ObservationReadRequest| {
        let mut bytes = vec![
            0;
            codec
                .encoded_observation_read_request_len(value)
                .expect("length")
        ];
        codec
            .encode_observation_read_request(value, &mut bytes)
            .expect("encode");
        bytes
    };
    let two = ObservationReadRequest {
        selections: vec![
            ObservationReadSelection {
                channels: ParameterChannel::Left,
                ..observation_selection(
                    "vocal",
                    controls::TapSection::Console(
                        session::StableId::parse("bus_comp").expect("slot"),
                    ),
                )
            },
            ObservationReadSelection {
                tap_id: 2,
                channels: ParameterChannel::Right,
                ..observation_selection(
                    "drums",
                    controls::TapSection::Inserts(session::StableId::parse("gate").expect("id")),
                )
            },
        ],
    };
    for request in [two.clone(), observation_selections(256)] {
        let bytes = encode_request(&request);
        assert_eq!(
            codec.decode_observation_read_request(&bytes, top_level_count(&bytes)),
            Ok(request)
        );
    }
    for count in [0, 257] {
        assert_eq!(
            codec.encoded_observation_read_request_len(&observation_selections(count)),
            Err(EncodeError::InvalidField),
            "{count} selections"
        );
    }
    let full = encode_request(&observation_selections(256));
    let two_bytes = encode_request(&two);
    type Edit = Box<dyn Fn(&mut Vec<Tlv>)>;
    let requests: [(&str, &Vec<u8>, Edit); 3] = [
        ("no selection", &two_bytes, Box::new(|f| remove(f, 1))),
        (
            "257 selections",
            &full,
            Box::new(|f| {
                let again = f[0].clone();
                insert_again(f, again);
            }),
        ),
        (
            "an address with a control",
            &two_bytes,
            Box::new(|f| insert(nth(nth(f, 1, 0), 1, 0), leaf(5, 3, &7_u32.to_le_bytes()))),
        ),
    ];
    for (name, base, edit) in requests {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_observation_read_request(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }

    // Replies.
    let pending = ObservationReadRow {
        status: ObservationRowStatus::Pending,
        window: None,
        left: None,
        right: None,
        ..ready_row(None, None)
    };
    let unarmed = ObservationReadRow {
        status: ObservationRowStatus::Unarmed,
        unit: ParameterUnit::Linear,
        ..pending
    };
    let page = ObservationReadPage {
        observed_sample: 2_048,
        rows: Cow::Owned(vec![
            ready_row(Some(-3.0), None),
            ready_row(None, Some(-6.0)),
            ready_row(Some(-1.5), Some(-2.5)),
            pending,
            unarmed,
        ]),
    };
    let bytes = encode_observation_page(&codec, &page);
    assert_eq!(
        codec.decode_observation_read_page(&bytes, top_level_count(&bytes)),
        Ok(page.clone())
    );
    for rows in [1, 256] {
        let widest = ObservationReadPage {
            observed_sample: 0,
            rows: Cow::Owned(vec![ready_row(Some(0.0), Some(0.0)); rows]),
        };
        let frame = crate::TypedSuccessResponseFrame {
            request_id: crate::RequestId::new(9).expect("nonzero"),
            revision: crate::SessionRevision(3),
            payload: crate::SuccessResponsePayload::ObservationRead(widest),
        };
        let mut output = vec![0; 1 << 17];
        let written = codec
            .encode_success_response_frame_into(&frame, &mut output)
            .expect("frame");
        assert_eq!(
            codec.observation_read_largest_frame_len(rows),
            Ok(written),
            "{rows} rows"
        );
    }
    assert_eq!(
        codec.observation_read_largest_frame_len(257),
        Err(EncodeError::InvalidField)
    );
    let refused_rows = [
        ObservationReadRow {
            window: None,
            ..ready_row(Some(0.0), None)
        },
        ObservationReadRow {
            window: ready_row(None, None).window,
            ..pending
        },
        ObservationReadRow {
            left: Some(0.0),
            ..unarmed
        },
        ready_row(None, None),
        ObservationReadRow {
            armed: false,
            ..ready_row(Some(0.0), None)
        },
        ObservationReadRow {
            arm_tag: 0,
            ..ready_row(Some(0.0), None)
        },
        ObservationReadRow {
            arm_sample: 1_025,
            ..ready_row(Some(0.0), None)
        },
    ];
    for row in refused_rows {
        assert_eq!(
            codec.encoded_observation_read_page_len(&ObservationReadPage {
                observed_sample: 0,
                rows: Cow::Owned(vec![row]),
            }),
            Err(EncodeError::InvalidField),
            "{row:?}"
        );
    }
    for rows in [0, 257] {
        assert_eq!(
            codec.encoded_observation_read_page_len(&ObservationReadPage {
                observed_sample: 0,
                rows: Cow::Owned(vec![pending; rows]),
            }),
            Err(EncodeError::InvalidField),
            "{rows} rows"
        );
    }
    let many = encode_observation_page(
        &codec,
        &ObservationReadPage {
            observed_sample: 0,
            rows: Cow::Owned(vec![pending; 256]),
        },
    );
    let rules: [(&str, &Vec<u8>, Edit); 9] = [
        ("status 4", &bytes, Box::new(|f| set(nth(f, 2, 0), 1, &[4]))),
        ("unit 7", &bytes, Box::new(|f| set(nth(f, 2, 0), 11, &[7]))),
        (
            "a window without its end",
            &bytes,
            Box::new(|f| remove(nth(f, 2, 0), 6)),
        ),
        (
            "a pending row with a window",
            &bytes,
            Box::new(|f| set(nth(f, 2, 0), 1, &[1])),
        ),
        (
            "an unarmed row with a lane",
            &bytes,
            Box::new(|f| {
                let lane = leaf(9, 6, &0.0_f32.to_le_bytes());
                insert(nth(f, 2, 4), lane);
            }),
        ),
        (
            "a ready row with no lane",
            &bytes,
            Box::new(|f| remove(nth(f, 2, 0), 9)),
        ),
        (
            "a window before the arm",
            &bytes,
            Box::new(|f| set(nth(f, 2, 0), 3, &2_000_u64.to_le_bytes())),
        ),
        (
            "no row",
            &bytes,
            Box::new(|f| f.retain(|field| field.id != 2)),
        ),
        (
            "257 rows",
            &many,
            Box::new(|f| {
                let again = f[1].clone();
                insert_again(f, again);
            }),
        ),
    ];
    for (name, base, edit) in rules {
        let (wire, count) = edited(base, |fields| edit(fields));
        assert_eq!(
            codec.decode_observation_read_page(&wire, count),
            Err(DecodeError::InvalidField),
            "{name}"
        );
    }
}

/// #2115: the `LOUDNESS` reply's value rules hold on encode and on decode, and an operation
/// outside the table is refused. Every level is finite and at or above the -120 floor, a loudness
/// range is at least 0, and the true peak since reset comes exactly with `integrating_since`.
///
/// Test value: red if either direction lets a non-finite or below-floor level, a negative range,
/// a true peak without its integration start or an unknown operation cross the wire.
#[test]
fn loudness_reply_rules_hold_both_ways_and_an_unknown_op_is_refused() {
    let codec = codec();
    let valid = LoudnessReply {
        observed_sample: 4_800,
        armed: true,
        integrating_since: Some(480),
        hops_dropped: 0,
        sanitized_samples: 0,
        gated_block_count: 0,
        momentary_lufs: Some(LOUDNESS_LEVEL_FLOOR),
        short_term_lufs: None,
        integrated_lufs: None,
        loudness_range_lu: Some(0.0),
        momentary_max_lufs: None,
        short_term_max_lufs: None,
        true_peak_dbtp: Some([-6.0, -7.0]),
        hop_true_peak_dbtp: Some([-6.0, -7.0]),
    };
    let encode = |value: &LoudnessReply| {
        let mut bytes = vec![0; 512];
        codec
            .encode_loudness_reply(value, &mut bytes)
            .map(|written| bytes[..written].to_vec())
    };
    let bytes = encode(&valid).expect("a valid reply");
    assert_eq!(
        codec.decode_loudness_reply(&bytes, top_level_count(&bytes)),
        Ok(valid)
    );
    for (label, broken) in [
        (
            "NaN level",
            LoudnessReply {
                momentary_lufs: Some(f32::NAN),
                ..valid
            },
        ),
        (
            "below the floor",
            LoudnessReply {
                integrated_lufs: Some(-120.5),
                ..valid
            },
        ),
        (
            "infinite peak",
            LoudnessReply {
                hop_true_peak_dbtp: Some([f32::INFINITY, 0.0]),
                ..valid
            },
        ),
        (
            "negative range",
            LoudnessReply {
                loudness_range_lu: Some(-0.1),
                ..valid
            },
        ),
        (
            "peak without its start",
            LoudnessReply {
                integrating_since: None,
                ..valid
            },
        ),
    ] {
        assert_eq!(encode(&broken), Err(EncodeError::InvalidField), "{label}");
    }
    // The decoder applies the same rule: the momentary floor patched to -121 is refused.
    let floor = LOUDNESS_LEVEL_FLOOR.to_le_bytes();
    let at = bytes
        .windows(4)
        .position(|word| word == floor)
        .expect("the floor word");
    let mut below = bytes.clone();
    below[at..at + 4].copy_from_slice(&(-121.0_f32).to_le_bytes());
    assert_eq!(
        codec.decode_loudness_reply(&below, top_level_count(&below)),
        Err(DecodeError::InvalidField)
    );
    // Operations 1 to 4 decode; 0 and 5 are `INVALID_FIELD`.
    for op in 0_u8..=5 {
        let mut request = vec![0_u8; 16];
        let mut writer = crate::btlv::SliceSink::new(&mut request, codec.limits());
        write_loudness_request(&mut writer, LoudnessRequest::Start).expect("request");
        let at = request.len() - 8;
        request[at] = op;
        let decoded = codec.decode_loudness_request(&request, 1);
        match op {
            1..=4 => assert!(decoded.is_ok(), "op {op}"),
            _ => assert_eq!(decoded, Err(DecodeError::InvalidField), "op {op}"),
        }
    }
}
