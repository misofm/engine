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
fn direct_common_encoder_is_byte_stable_in_caller_storage() {
    let codec = codec();
    let value = response();
    let required = codec.encoded_non_ok_payload_len(&value).expect("length");
    let mut output = vec![0_u8; required];
    for _ in 0..16 {
        output.fill(0);
        assert_eq!(
            codec.encode_non_ok_payload(&value, &mut output),
            Ok(required)
        );
        assert_eq!(codec.decode_non_ok_payload(&output, 3), Ok(value.clone()));
    }
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
            maximum_parameter_page_items: 256,
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
    assert_parity_rejects(&[1, 2, 12], &events);
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
        maximum_parameter_page_items: 256,
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
    };
    let mut applied_bytes = [0_u8; 16];
    codec
        .encode_transaction_applied(applied, &mut applied_bytes)
        .expect("applied");
    assert_eq!(
        codec.decode_transaction_applied(&applied_bytes, 1),
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
    };
    let mut applied_bytes = [0_u8; 16];
    codec
        .encode_transaction_applied(applied, &mut applied_bytes)
        .expect("applied");
    assert_eq!(hex(&applied_bytes), "01000301040000000300000000000000");
    for end in 0..applied_bytes.len() {
        assert!(
            codec
                .decode_transaction_applied(&applied_bytes[..end], 1)
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

#[test]
fn b2a_metadata_and_state_are_typed_bounded_and_strict() {
    let codec = codec();
    let descriptor = ParameterDescriptor {
        handle: 1,
        track_id: "vocal".to_owned(),
        rack: ParameterRack::Dynamic,
        effect_id: "comp".to_owned(),
        parameter_id: 7,
        channel: ParameterChannel::Left,
        value_kind: ParameterValueKind::F32,
        unit: ParameterUnit::Db,
        domain: ParameterDomain::Continuous,
        minimum: Some(-24.0),
        maximum: Some(24.0),
        default: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: ParameterAutomationRate::Sample,
        smoothing_samples: 12,
        flags: 3,
        display_name: Some("Threshold".to_owned()),
        display_unit: Some("dB".to_owned()),
        enum_choices: Vec::new(),
    };
    let page = ParameterMetadataPage {
        last_handle: 1,
        eof: true,
        descriptors: vec![descriptor],
    };
    let mut metadata = vec![
        0;
        codec
            .encoded_parameter_metadata_page_len(&page)
            .expect("len")
    ];
    codec
        .encode_parameter_metadata_page(&page, &mut metadata)
        .expect("encode");
    assert_eq!(metadata[51], 1, "descriptor handle stays mandatory");
    assert_eq!(codec.decode_parameter_metadata_page(&metadata, 3), Ok(page));
    for end in 0..metadata.len() {
        assert!(
            codec
                .decode_parameter_metadata_page(&metadata[..end], 3)
                .is_err()
        );
    }
    let request = ParameterStateRequest {
        handles: vec![1, 2],
    };
    let mut request_bytes = [0_u8; 16];
    codec
        .encode_parameter_state_request(&request, &mut request_bytes)
        .expect("request");
    assert_eq!(
        codec.decode_parameter_state_request(&request_bytes, 1),
        Ok(request)
    );
    let state = ParameterStatePage {
        observed_sample: 44,
        records: vec![
            ParameterStateRecord {
                handle: 1,
                flags: 3,
                value: -2.0,
            },
            ParameterStateRecord {
                handle: 2,
                flags: 0,
                value: 0.0,
            },
        ],
    };
    let mut state_bytes = vec![0; codec.encoded_parameter_state_page_len(&state).expect("len")];
    codec
        .encode_parameter_state_page(&state, &mut state_bytes)
        .expect("encode");
    assert_eq!(
        codec.decode_parameter_state_page(&state_bytes, 4),
        Ok(state)
    );
    let invalid = ParameterStatePage {
        observed_sample: 0,
        records: vec![ParameterStateRecord {
            handle: 1,
            flags: 2,
            value: 0.0,
        }],
    };
    assert_eq!(
        codec.encoded_parameter_state_page_len(&invalid),
        Err(EncodeError::LimitExceeded)
    );
}

fn b2_descriptor(handle: u32, domain: ParameterDomain) -> ParameterDescriptor {
    let (minimum, maximum, default, choices) = match domain {
        ParameterDomain::Continuous => (Some(-1.0), Some(1.0), 0.0, Vec::new()),
        ParameterDomain::Boolean => (None, None, 1.0, Vec::new()),
        ParameterDomain::Enumeration => (
            None,
            None,
            2.0,
            vec![
                EnumChoice {
                    value: 1.0,
                    label: "one".to_owned(),
                },
                EnumChoice {
                    value: 2.0,
                    label: "two".to_owned(),
                },
            ],
        ),
    };
    ParameterDescriptor {
        handle,
        track_id: "vocal".to_owned(),
        rack: ParameterRack::Dynamic,
        effect_id: "comp".to_owned(),
        parameter_id: handle,
        channel: ParameterChannel::Left,
        value_kind: ParameterValueKind::F32,
        unit: ParameterUnit::Db,
        domain,
        minimum,
        maximum,
        default,
        mapping: ParameterMapping::Linear,
        automation_rate: ParameterAutomationRate::Sample,
        smoothing_samples: 12,
        flags: 3,
        display_name: None,
        display_unit: None,
        enum_choices: choices,
    }
}

#[test]
fn descriptor_invariant_rejects_encode_and_decode_identically() {
    let codec = ProtocolCodec::default();
    let descriptor = b2_descriptor(1, ParameterDomain::Continuous);
    assert!(descriptor_is_valid(codec.limits(), &descriptor));
    let page = ParameterMetadataPage {
        last_handle: 1,
        eof: true,
        descriptors: vec![descriptor.clone()],
    };
    let mut encoded = vec![
        0;
        codec
            .encoded_parameter_metadata_page_len(&page)
            .expect("valid descriptor length")
    ];
    codec
        .encode_parameter_metadata_page(&page, &mut encoded)
        .expect("valid descriptor encode");
    assert_eq!(
        codec
            .decode_parameter_metadata_page(&encoded, 3)
            .expect("valid descriptor decode"),
        page
    );

    let mut invalid = descriptor;
    invalid.default = 2.0;
    assert!(!descriptor_is_valid(codec.limits(), &invalid));
    assert_eq!(
        codec.encoded_parameter_metadata_page_len(&ParameterMetadataPage {
            last_handle: 1,
            eof: true,
            descriptors: vec![invalid],
        }),
        Err(EncodeError::LimitExceeded)
    );

    let default_prefix = [12, 0, 6, 1, 4, 0, 0, 0];
    let offset = encoded
        .windows(default_prefix.len())
        .position(|window| window == default_prefix)
        .expect("descriptor default field")
        + default_prefix.len();
    encoded[offset..offset + 4].copy_from_slice(&2.0_f32.to_le_bytes());
    assert_eq!(
        codec.decode_parameter_metadata_page(&encoded, 3),
        Err(DecodeError::InvalidTlv)
    );
}

#[test]
fn b2a_goldens_truncations_malformed_matrix_and_encoder_audit() {
    let codec = ProtocolCodec::default();
    let metadata_request = ParameterMetadataRequest {
        after_handle: 4,
        limit: 3,
    };
    let mut metadata_request_bytes = [0_u8; 32];
    codec
        .encode_parameter_metadata_request(metadata_request, &mut metadata_request_bytes)
        .expect("metadata request");
    assert_eq!(
        hex(&metadata_request_bytes),
        "0100030104000000040000000000000002000201020000000300000000000000"
    );
    for end in 0..metadata_request_bytes.len() {
        assert!(
            codec
                .decode_parameter_metadata_request(&metadata_request_bytes[..end], 2)
                .is_err()
        );
    }
    let page = ParameterMetadataPage {
        last_handle: 3,
        eof: true,
        descriptors: vec![
            b2_descriptor(1, ParameterDomain::Continuous),
            b2_descriptor(2, ParameterDomain::Boolean),
            b2_descriptor(3, ParameterDomain::Enumeration),
        ],
    };
    let mut metadata = vec![
        0;
        codec
            .encoded_parameter_metadata_page_len(&page)
            .expect("metadata len")
    ];
    codec
        .encode_parameter_metadata_page(&page, &mut metadata)
        .expect("metadata encode");
    assert_eq!(
        hex(&metadata),
        concat!(
            "010003010400000003000000000000000200080101000000010000000000000003000b01080100001000000000000000",
            "010003010400000001000000000000000200090105000000766f63616c00000003000101010000000200000000000000",
            "0400090104000000636f6d70000000000500030104000000010000000000000006000101010000000100000000000000",
            "070001010100000001000000000000000800010101000000010000000000000009000101010000000100000000000000",
            "0a00060004000000000080bf000000000b000600040000000000803f000000000c000601040000000000000000000000",
            "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
            "1000030104000000030000000000000003000b01e80000000e0000000000000001000301040000000200000000000000",
            "0200090105000000766f63616c000000030001010100000002000000000000000400090104000000636f6d7000000000",
            "050003010400000002000000000000000600010101000000010000000000000007000101010000000100000000000000",
            "08000101010000000100000000000000090001010100000002000000000000000c000601040000000000803f00000000",
            "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
            "1000030104000000030000000000000003000b0148010000100000000000000001000301040000000300000000000000",
            "0200090105000000766f63616c000000030001010100000002000000000000000400090104000000636f6d7000000000",
            "050003010400000003000000000000000600010101000000010000000000000007000101010000000100000000000000",
            "08000101010000000100000000000000090001010100000003000000000000000c000601040000000000004000000000",
            "0d0001010100000001000000000000000e0001010100000001000000000000000f000301040000000c00000000000000",
            "1000030104000000030000000000000013000b0028000000020000000000000001000601040000000000803f00000000",
            "02000901030000006f6e65000000000013000b0028000000020000000000000001000601040000000000004000000000",
            "020009010300000074776f0000000000"
        )
    );
    for end in 0..metadata.len() {
        assert!(
            codec
                .decode_parameter_metadata_page(&metadata[..end], 5)
                .is_err()
        );
    }
    let state_request = ParameterStateRequest {
        handles: vec![1, 2],
    };
    let mut state_request_bytes = [0_u8; 16];
    codec
        .encode_parameter_state_request(&state_request, &mut state_request_bytes)
        .expect("state request");
    assert_eq!(
        hex(&state_request_bytes),
        "01000d01080000000100000002000000"
    );
    for end in 0..state_request_bytes.len() {
        assert!(
            codec
                .decode_parameter_state_request(&state_request_bytes[..end], 1)
                .is_err()
        );
    }
    let state = ParameterStatePage {
        observed_sample: 8,
        records: vec![
            ParameterStateRecord {
                handle: 1,
                flags: 1,
                value: 0.5,
            },
            ParameterStateRecord {
                handle: 2,
                flags: 0,
                value: 0.0,
            },
        ],
    };
    let mut state_bytes = vec![
        0;
        codec
            .encoded_parameter_state_page_len(&state)
            .expect("state len")
    ];
    codec
        .encode_parameter_state_page(&state, &mut state_bytes)
        .expect("state encode");
    assert_eq!(
        hex(&state_bytes),
        "01000401080000000800000000000000020002010200000002000000000000000300020102000000100000000000000004000a012000000001000000010000000000003f0000000002000000000000000000000000000000"
    );
    for end in 0..state_bytes.len() {
        assert!(
            codec
                .decode_parameter_state_page(&state_bytes[..end], 4)
                .is_err()
        );
    }
    assert!(
        codec
            .decode_parameter_metadata_request(
                &[
                    1, 0, 3, 1, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0
                ],
                2
            )
            .is_err()
    );
    for request in [
        ParameterStateRequest { handles: vec![] },
        ParameterStateRequest { handles: vec![0] },
        ParameterStateRequest {
            handles: vec![2, 1],
        },
        ParameterStateRequest {
            handles: vec![1, 1],
        },
    ] {
        assert!(
            codec
                .encode_parameter_state_request(&request, &mut [0_u8; 16])
                .is_err()
        );
    }
    let mut bad = b2_descriptor(1, ParameterDomain::Continuous);
    bad.default = 2.0;
    assert!(
        codec
            .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                last_handle: 1,
                eof: true,
                descriptors: vec![bad]
            })
            .is_err()
    );
    let mut bad = b2_descriptor(1, ParameterDomain::Boolean);
    bad.default = 0.5;
    assert!(
        codec
            .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                last_handle: 1,
                eof: true,
                descriptors: vec![bad]
            })
            .is_err()
    );
    let mut bad = b2_descriptor(1, ParameterDomain::Enumeration);
    bad.enum_choices.push(EnumChoice {
        value: 2.0,
        label: "duplicate".to_owned(),
    });
    assert!(
        codec
            .encoded_parameter_metadata_page_len(&ParameterMetadataPage {
                last_handle: 1,
                eof: true,
                descriptors: vec![bad]
            })
            .is_err()
    );
    let mut bad_state = state_bytes.clone();
    bad_state[60] = 4;
    assert!(codec.decode_parameter_state_page(&bad_state, 4).is_err());
    bad_state = state_bytes.clone();
    bad_state[68] = 1;
    assert!(codec.decode_parameter_state_page(&bad_state, 4).is_err());
    let mut many = (1..=256)
        .map(|handle| b2_descriptor(handle, ParameterDomain::Continuous))
        .collect::<Vec<_>>();
    let full = ParameterMetadataPage {
        last_handle: 256,
        eof: true,
        descriptors: many.clone(),
    };
    let required = codec
        .encoded_parameter_metadata_page_len(&full)
        .expect("full len");
    let mut output = vec![0; required];
    for _ in 0..4 {
        output.fill(0);
        assert_eq!(
            codec.encode_parameter_metadata_page(&full, &mut output),
            Ok(required)
        );
        assert_eq!(
            codec.decode_parameter_metadata_page(&output, 258),
            Ok(full.clone())
        );
    }
    many.clear();
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
    for _ in 0..64 {
        set_bytes.fill(0);
        snapshot_bytes.fill(0);
        event_bytes.fill(0);
        assert_eq!(
            codec.encode_transport_set_request(set, &mut set_bytes),
            Ok(32)
        );
        assert_eq!(codec.decode_transport_set_request(&set_bytes, 2), Ok(set));
        assert_eq!(
            codec.encode_transport_snapshot(snapshot, &mut snapshot_bytes),
            Ok(48)
        );
        assert_eq!(
            codec.decode_transport_snapshot(&snapshot_bytes, 3),
            Ok(snapshot)
        );
        assert_eq!(
            codec.encode_transport_state_event(event, &mut event_bytes),
            Ok(80)
        );
        assert_eq!(
            codec.decode_transport_state_event(&event_bytes, 5),
            Ok(event)
        );
    }
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
    for _ in 0..32 {
        config_bytes.fill(0);
        request_bytes.fill(0);
        snapshot_bytes.fill(0);
        assert_eq!(
            codec.encode_telemetry_configuration(&configuration, &mut config_bytes),
            Ok(96)
        );
        assert_eq!(
            codec.encode_counters_request(&request, &mut request_bytes),
            Ok(32)
        );
        assert_eq!(
            codec.encode_counter_snapshot(&snapshot, &mut snapshot_bytes),
            Ok(snapshot_bytes.len())
        );
        assert_eq!(
            codec.decode_counter_snapshot(&snapshot_bytes, 3),
            Ok(snapshot.clone())
        );
    }
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

    for _ in 0..64 {
        page_bytes.fill(0);
        assert_eq!(
            codec.encode_diagnostics_page(&page, &mut page_bytes),
            Ok(page_bytes.len())
        );
        assert_eq!(
            codec.decode_diagnostics_page(&page_bytes, 3),
            Ok(page.clone())
        );
        request_bytes.fill(0);
        assert_eq!(
            codec.encode_diagnostics_request(request, &mut request_bytes),
            Ok(48)
        );
        assert_eq!(
            codec.decode_diagnostics_request(&request_bytes, 3),
            Ok(request)
        );
    }
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

    for _ in 0..32 {
        canceled_bytes.fill(0);
        meter_bytes.fill(0);
        diagnostic_bytes.fill(0);
        counter_bytes.fill(0);
        assert_eq!(
            codec.encode_automation_canceled(canceled, &mut canceled_bytes),
            Ok(96)
        );
        assert_eq!(codec.encode_meter_batch(batch, &mut meter_bytes), Ok(72));
        assert_eq!(
            codec.encode_diagnostic_event(&diagnostic, &mut diagnostic_bytes),
            Ok(64)
        );
        assert_eq!(
            codec.encode_counter_snapshot_event(&counters, &mut counter_bytes),
            Ok(counter_bytes.len())
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    engine::hex_lower(bytes)
}
