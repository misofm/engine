//! Frozen complete-schema byte and decoder conformance.

use miso_engine_protocol::{
    ConformanceDecoder, DecodeScratch, ParameterAutomationRate, ParameterChannel,
    ParameterDescriptor, ParameterDomain, ParameterMapping, ParameterMetadataPage, ParameterRack,
    ParameterUnit, ParameterValueKind, ProtocolCodec, complete_schema_corpus,
};

#[test]
fn frozen_corpus_bytes_and_typed_decoders_are_unchanged() {
    let codec = ProtocolCodec::default();
    let corpus = complete_schema_corpus();
    assert_eq!(corpus.len(), 46);
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for frame in &corpus {
        hash = hash_bytes(hash, frame.name.as_bytes());
        hash = hash_bytes(hash, &frame.bytes);
        let mut fields = [0_u16; 1024];
        let decoded = match frame.decoder {
            ConformanceDecoder::Command => codec
                .decode_typed_command(&frame.bytes, &mut DecodeScratch::new(&mut fields))
                .map(|_| ()),
            ConformanceDecoder::Response => codec
                .decode_typed_response(&frame.bytes, &mut DecodeScratch::new(&mut fields))
                .map(|_| ()),
            ConformanceDecoder::Event => codec
                .decode_typed_event(&frame.bytes, &mut DecodeScratch::new(&mut fields))
                .map(|_| ()),
            ConformanceDecoder::Transaction => codec
                .decode_session_transaction(&frame.bytes, &mut DecodeScratch::new(&mut fields))
                .map(|_| ()),
        };
        assert!(decoded.is_ok(), "{} must decode", frame.name);
    }
    // #241 re-pin: the frame count remains 46, while the transaction corpus deletes opcodes
    // 0x0006/0x0102/0x0104 and replaces 0x0103's nested content with its five-field source shape.
    assert_eq!(hash, 0xbdeb_b0f8_1c38_ec42);
}

#[test]
fn frozen_deep_transaction_uses_typed_command_dispatch() {
    let codec = ProtocolCodec::default();
    let corpus = complete_schema_corpus();
    let transaction = corpus
        .iter()
        .find(|frame| frame.name == "command.session_transaction_apply")
        .expect("frozen transaction frame");
    assert_eq!(transaction.decoder, ConformanceDecoder::Command);
    let mut fields = [0_u16; 1024];
    codec
        .decode(&transaction.bytes, &mut DecodeScratch::new(&mut fields))
        .expect("generic decode honors the transaction envelope allowance");
    codec
        .decode_typed_command(&transaction.bytes, &mut DecodeScratch::new(&mut fields))
        .expect("typed command dispatch honors the transaction envelope allowance");
}

#[test]
fn descriptor_handle_required_flag_is_frozen() {
    let codec = ProtocolCodec::default();
    let page = ParameterMetadataPage {
        last_handle: 1,
        eof: true,
        descriptors: vec![ParameterDescriptor {
            handle: 1,
            track_id: "track".to_owned(),
            rack: ParameterRack::Dynamic,
            effect_id: "effect".to_owned(),
            parameter_id: 1,
            channel: ParameterChannel::Left,
            value_kind: ParameterValueKind::F32,
            unit: ParameterUnit::Linear,
            domain: ParameterDomain::Continuous,
            minimum: Some(0.0),
            maximum: Some(1.0),
            default: 0.5,
            mapping: ParameterMapping::Linear,
            automation_rate: ParameterAutomationRate::Sample,
            smoothing_samples: 0,
            flags: 3,
            display_name: None,
            display_unit: None,
            enum_choices: Vec::new(),
        }],
    };
    let mut bytes = vec![
        0;
        codec
            .encoded_parameter_metadata_page_len(&page)
            .expect("metadata length")
    ];
    codec
        .encode_parameter_metadata_page(&page, &mut bytes)
        .expect("metadata encode");
    assert_eq!(bytes[51], 1, "descriptor handle stays mandatory");
    assert_eq!(codec.decode_parameter_metadata_page(&bytes, 3), Ok(page));
}

const fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    hash
}
