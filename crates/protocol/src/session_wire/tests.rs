use super::*;
use crate::btlv::{WIRE_BOOL, WIRE_F32, WIRE_U8, WIRE_U32, WIRE_UTF8};
use session::{
    LinkMode, ParameterChannel, ParameterUnit, RenderMode, SampleFormat, SendTap, Sidechain, Track,
    parse_session_json,
};

pub(crate) fn complete_all_opcode_fixture() -> Vec<SessionEdit> {
    let bytes = conformance::complete_all_opcode_fixture_bytes();
    ProtocolCodec::default()
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 1024]))
        .expect("conformance fixture decodes")
        .edits
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

fn raw_message(fields: Vec<(u16, u8, bool, Vec<u8>)>) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(fields.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    for (id, wire, mandatory, value) in fields {
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes.push(wire);
        bytes.push(u8::from(mandatory));
        bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&value);
        bytes.resize(bytes.len() + crate::btlv::padding(value.len()), 0);
    }
    bytes
}

#[test]
fn exact_set_session_id_golden_and_no_partial_output() {
    let codec = ProtocolCodec::default();
    let edits = [SessionEdit::SetSessionId {
        session_id: id("next"),
    }];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let required = codec
        .encoded_session_transaction_len(&transaction)
        .expect("length");
    assert_eq!(required, 112);
    let mut short = vec![0xaa; required - 1];
    assert_eq!(
        codec.encode_session_transaction(&transaction, &mut short),
        Err(EncodeError::OutputTooSmall { required })
    );
    assert_eq!(short, vec![0xaa; required - 1]);
    let mut output = vec![0; required];
    codec
        .encode_session_transaction(&transaction, &mut output)
        .expect("encode");
    assert_eq!(
        hex(&output),
        concat!(
            "4d49534f43544c0001000000300001000300000040000000",
            "010000000000000007000000000000000100000000000000",
            "01000b013800000002000000000000000100020102000000",
            "010000000000000002000b01180000000100000000000000",
            "01000901040000006e65787400000000"
        )
    );
    let mut slots = [0_u16; 1];
    assert!(
        codec
            .decode_session_transaction_outer(&output, &mut DecodeScratch::new(&mut slots))
            .is_ok()
    );
}

#[test]
fn five_session_edit_encoders_are_canonical_and_ordered() {
    let edits = [
        SessionEdit::SetSessionId {
            session_id: id("next"),
        },
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 48_000,
        },
        SessionEdit::SetQuantumFrames {
            quantum_frames: 128,
        },
        SessionEdit::SetRenderProfile {
            render_profile: RenderProfile {
                id: id("render"),
                mode: RenderMode::SingleThread,
            },
        },
        SessionEdit::SetOutputProfile {
            output_profile: OutputProfile {
                id: id("output"),
                channels: 2,
                sample_format: SampleFormat::F32Planar,
            },
        },
    ];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(2).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let required = codec
        .encoded_session_transaction_len(&transaction)
        .expect("length");
    let mut output = vec![0; required];
    assert_eq!(
        codec.encode_session_transaction(&transaction, &mut output),
        Ok(required)
    );
    assert_eq!(
        u32::from_le_bytes(output[40..44].try_into().expect("count")),
        5
    );
}

#[test]
fn transaction_outer_header_uses_sizing_sink_repeated_count() {
    let codec = ProtocolCodec::default();
    let edits = [
        SessionEdit::SetSessionId {
            session_id: id("measured"),
        },
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 48_000,
        },
        SessionEdit::SetQuantumFrames {
            quantum_frames: 128,
        },
    ];
    for count in 1..=edits.len() {
        let transaction = SessionTransactionFrame {
            request_id: RequestId::new(3).expect("request"),
            expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
            edits: &edits[..count],
        };
        let required = codec
            .encoded_session_transaction_len(&transaction)
            .expect("measured length");
        let mut output = vec![0; required];
        assert_eq!(
            codec.encode_session_transaction(&transaction, &mut output),
            Ok(required)
        );
        assert_eq!(
            u32::from_le_bytes(output[40..44].try_into().expect("outer TLV count")),
            u32::try_from(count).expect("small count")
        );
    }
}

fn source() -> Source {
    Source {
        id: id("voice"),
        content: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_owned(),
        channels: 2,
        bit_depth: SourceBitDepth::Float32,
        frames: 48_000,
    }
}

fn all_opcode_edits_64() -> Vec<SessionEdit> {
    let session = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture");
    let source = session.sources[0].clone();
    let track = session.tracks[0].clone();
    let effect = track.dynamic.effects[0].clone();
    let route = session.routes[0].clone();
    let automation = session.automation[0].clone();
    let track_id = track.id.clone();
    let effect_id = effect.id.clone();
    let mut edits = vec![
        SessionEdit::SetSessionId {
            session_id: id("demo.session"),
        },
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 48_000,
        },
        SessionEdit::SetQuantumFrames {
            quantum_frames: 128,
        },
        SessionEdit::SetRenderProfile {
            render_profile: session.render_profile.clone(),
        },
        SessionEdit::SetOutputProfile {
            output_profile: session.output_profile.clone(),
        },
        SessionEdit::UpsertSource {
            source: source.clone(),
        },
        SessionEdit::RemoveSource {
            source_id: source.id.clone(),
        },
        SessionEdit::SetSourceContent {
            source_id: source.id.clone(),
            content: source.content.clone(),
            channels: source.channels,
            bit_depth: source.bit_depth,
            frames: source.frames,
        },
        SessionEdit::UpsertTrack {
            track: track.clone(),
        },
        SessionEdit::RemoveTrack {
            track_id: track_id.clone(),
        },
        SessionEdit::SetTrackSourceAssignment {
            track_id: track_id.clone(),
            source_id: source.id.clone(),
            left_source_channel: 0,
            right_source_channel: 1,
        },
        SessionEdit::SetTrackBuiltins {
            track_id: track_id.clone(),
            builtins: track.builtins.clone(),
        },
        SessionEdit::SetTrackRack {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            rack: track.dynamic.clone(),
        },
        SessionEdit::PutTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            final_position: 0,
            effect: effect.clone(),
        },
        SessionEdit::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
        },
        SessionEdit::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_ids: vec![effect_id.clone()],
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            identity: effect.identity.clone(),
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            quality: effect.quality,
        },
        SessionEdit::SetEffectBypass {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            bypass: effect.bypass,
        },
        SessionEdit::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            link_mode: effect.link_mode,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            sidechain: effect.sidechain.clone(),
        },
        SessionEdit::UpsertEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            param: effect.params[0].clone(),
        },
        SessionEdit::RemoveEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: effect_id.clone(),
            parameter_id: effect.params[0].parameter_id,
            channel: effect.params[0].channel,
        },
        SessionEdit::SetTrackFader {
            track_id: track_id.clone(),
            fader: track.fader.clone(),
        },
        SessionEdit::SetTrackMatrixOrPan {
            track_id: track_id.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
        },
        SessionEdit::UpsertSubmix {
            submix: Submix { id: id("drums") },
        },
        SessionEdit::RemoveSubmix {
            submix_id: id("drums"),
        },
        SessionEdit::UpsertOutput {
            output: Output { id: id("alt-out") },
        },
        SessionEdit::RemoveOutput {
            output_id: id("alt-out"),
        },
        SessionEdit::UpsertRoute {
            route: route.clone(),
        },
        SessionEdit::RemoveRoute {
            route_id: route.id.clone(),
        },
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: route.source.clone(),
        },
        SessionEdit::SetRouteDestination {
            route_id: route.id.clone(),
            destination: route.destination.clone(),
        },
        SessionEdit::SetRouteChannelMatrix {
            route_id: route.id.clone(),
            channel_matrix: route.channel_matrix.clone(),
        },
        SessionEdit::SetRouteGainDb {
            route_id: route.id.clone(),
            gain_db: route.gain_db,
        },
        SessionEdit::UpsertAutomation {
            automation: automation.clone(),
        },
        SessionEdit::RemoveAutomation {
            automation_id: automation.id.clone(),
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: automation.target.clone(),
        },
        SessionEdit::SetAutomationSegments {
            automation_id: automation.id.clone(),
            segments: automation.segments.clone(),
        },
        // Issue #178, ruled by #210's D2: the strip's own automation target. It is in the
        // round-trip corpus rather than in a test of its own because the fourth `RackName`
        // token changes no message shape -- `RACK` was already a `Wire::U8` -- so the thing
        // worth proving is that the byte still survives encode/decode with the new value.
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: AutomationTarget {
                entity_id: automation.target.entity_id.clone(),
                rack: RackName::Builtins,
                effect_id: id("strip"),
                parameter_id: 2,
                channel: ParameterChannel::Left,
            },
        },
    ];
    while edits.len() < 64 {
        edits.push(SessionEdit::SetSessionId {
            session_id: id("demo.session"),
        });
    }
    edits
}

fn track() -> Track {
    parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture")
    .tracks
    .into_iter()
    .next()
    .expect("track")
}

fn effect_with(
    slot: &str,
    identity: EffectIdentity,
    quality: EffectQuality,
    link_mode: LinkMode,
    params: Vec<EffectParam>,
    sidechain: SidechainDeclaration,
) -> Effect {
    Effect {
        id: id(slot),
        identity,
        quality,
        bypass: false,
        link_mode,
        params,
        sidechain,
    }
}

fn encode(edits: &[SessionEdit]) -> Vec<u8> {
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(9).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&transaction)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&transaction, &mut bytes)
        .expect("encode");
    bytes
}

#[test]
fn direct_full_schema_encoder_is_byte_identical_in_caller_storage() {
    let edits = all_opcode_edits_64();
    assert_eq!(edits.len(), 64);
    let codec = ProtocolCodec::default();
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(33).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let required = codec
        .encoded_session_transaction_len(&transaction)
        .expect("sizing");
    let mut output = vec![0_u8; required];
    codec
        .encode_session_transaction(&transaction, &mut output)
        .expect("initial direct encode");
    let canonical = output.clone();
    let mut scratch = [0_u16; 64];
    let decoded = codec
        .decode_session_transaction(&output, &mut DecodeScratch::new(&mut scratch))
        .expect("all opcodes decode");
    assert_eq!(decoded.edits, edits);
    for _ in 0..16 {
        assert_eq!(
            codec.encoded_session_transaction_len(&transaction),
            Ok(required)
        );
        output.fill(0);
        assert_eq!(
            codec.encode_session_transaction(&transaction, &mut output),
            Ok(required)
        );
        assert_eq!(output, canonical);
    }
}

#[test]
fn transaction_repeated_fields_match_schema_derived_count() {
    let edits = [SessionEdit::SetTrackEffectOrder {
        track_id: id("track.repeated"),
        rack_name: RackName::Dynamic,
        effect_ids: vec![id("effect.one"), id("effect.two"), id("effect.three")],
    }];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(37).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let required = codec
        .encoded_session_transaction_len(&transaction)
        .expect("schema count includes every repeated field");
    let mut output = vec![0_u8; required];
    assert_eq!(
        codec.encode_session_transaction(&transaction, &mut output),
        Ok(required)
    );
    let decoded = codec
        .decode_session_transaction(&output, &mut DecodeScratch::new(&mut [0_u16; 8]))
        .expect("repeated transaction roundtrip");
    assert_eq!(decoded.edits, edits);
}

#[test]
fn transaction_requires_exact_nonempty_edits_and_repeated_message_wire_type() {
    let codec = ProtocolCodec::default();
    let empty = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &[],
    };
    assert_eq!(
        codec.encoded_session_transaction_len(&empty),
        Err(EncodeError::MessageKindMismatch)
    );
    let any = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("request"),
        expected_revision: ExpectedRevision::Any,
        edits: &[SessionEdit::SetSessionId {
            session_id: StableId::parse("exact-required").expect("stable ID"),
        }],
    };
    assert_eq!(
        codec.encoded_session_transaction_len(&any),
        Err(EncodeError::MessageKindMismatch)
    );
    let mut wrong_wire = encode(&[SessionEdit::SetSessionId {
        session_id: StableId::parse("wrong-wire").expect("stable ID"),
    }]);
    wrong_wire[crate::OUTER_HEADER_BYTES + 2] = WIRE_UTF8;
    assert_eq!(
        codec.decode_session_transaction(&wrong_wire, &mut DecodeScratch::new(&mut [0_u16; 16])),
        Err(DecodeError::InvalidTlv)
    );
}

#[test]
fn direct_encoder_limits_and_short_buffers_preserve_caller_ownership() {
    let edits = all_opcode_edits_64();
    let frame = SessionTransactionFrame {
        request_id: RequestId::new(34).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let required = codec
        .encoded_session_transaction_len(&frame)
        .expect("length");
    let mut short = vec![0xa5; required - 1];
    assert_eq!(
        codec.encode_session_transaction(&frame, &mut short),
        Err(EncodeError::OutputTooSmall { required })
    );
    assert!(short.iter().all(|byte| *byte == 0xa5));
    let limited = ProtocolCodec::new(crate::ProtocolLimits {
        max_tlv_count: 1,
        ..crate::ProtocolLimits::default()
    });
    let mut untouched = [0x5a_u8; 64];
    assert_eq!(
        limited.encode_session_transaction(&frame, &mut untouched),
        Err(EncodeError::LimitExceeded)
    );
    assert!(untouched.iter().all(|byte| *byte == 0x5a));
    let frame_limited = ProtocolCodec::new(crate::ProtocolLimits {
        max_frame_bytes: crate::OUTER_HEADER_BYTES,
        ..crate::ProtocolLimits::default()
    });
    assert_eq!(
        frame_limited.encoded_session_transaction_len(&frame),
        Err(EncodeError::LimitExceeded)
    );
    assert_eq!(
        schema::session::rack::SPEC.field_count(&[(schema::session::rack::EFFECT, usize::MAX,)]),
        Err(EncodeError::LimitExceeded)
    );
    let mut overflow =
        CountSink::with_length_for_test(usize::MAX, crate::ProtocolLimits::default());
    assert_eq!(overflow.raw(&[0]), Err(EncodeError::LimitExceeded));
}

#[test]
fn transaction_encoder_reserves_envelope_depth_for_frozen_deep_fixture() {
    let flat_edits = [SessionEdit::SetSessionId {
        session_id: id("depth-envelope"),
    }];
    let flat = SessionTransactionFrame {
        request_id: RequestId::new(35).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &flat_edits,
    };
    let zero_logical_depth = ProtocolCodec::new(crate::ProtocolLimits {
        max_nesting: 0,
        ..crate::ProtocolLimits::default()
    });
    let flat_len = zero_logical_depth
        .encoded_session_transaction_len(&flat)
        .expect("three fixed envelopes do not consume logical nesting");
    let mut flat_bytes = vec![0; flat_len];
    zero_logical_depth
        .encode_session_transaction(&flat, &mut flat_bytes)
        .expect("flat transaction encodes at zero logical depth");
    zero_logical_depth
        .decode(&flat_bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
        .expect("generic decode reserves exactly the fixed envelopes");
    zero_logical_depth
        .decode_session_transaction(&flat_bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
        .expect("typed decode reserves exactly the fixed envelopes");

    let deep_edits = complete_all_opcode_fixture();
    let deep = SessionTransactionFrame {
        request_id: RequestId::new(36).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &deep_edits,
    };
    let codec = ProtocolCodec::default();
    let deep_len = codec
        .encoded_session_transaction_len(&deep)
        .expect("canonical deep transaction remains encodable");
    let mut deep_bytes = vec![0; deep_len];
    codec
        .encode_session_transaction(&deep, &mut deep_bytes)
        .expect("encode canonical deep transaction");
    assert_eq!(
        zero_logical_depth.encoded_session_transaction_len(&deep),
        Err(EncodeError::LimitExceeded)
    );
    assert_eq!(
        zero_logical_depth.decode(&deep_bytes, &mut DecodeScratch::new(&mut [0_u16; 128]),),
        Err(DecodeError::LimitExceeded)
    );
    assert_eq!(
        zero_logical_depth
            .decode_session_transaction(&deep_bytes, &mut DecodeScratch::new(&mut [0_u16; 128]),)
            .map(|_| ()),
        Err(DecodeError::LimitExceeded)
    );
}

#[test]
fn transaction_descendants_retain_string_limits() {
    let edits = [SessionEdit::SetRenderProfile {
        render_profile: RenderProfile {
            id: id("long-render-profile"),
            mode: RenderMode::SingleThread,
        },
    }];
    let frame = SessionTransactionFrame {
        request_id: RequestId::new(37).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![
        0;
        codec
            .encoded_session_transaction_len(&frame)
            .expect("length")
    ];
    codec
        .encode_session_transaction(&frame, &mut bytes)
        .expect("encode");
    let string_limited = ProtocolCodec::new(crate::ProtocolLimits {
        max_string_bytes: 4,
        ..crate::ProtocolLimits::default()
    });
    assert_eq!(
        string_limited
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 8]),)
            .map(|_| ()),
        Err(DecodeError::LimitExceeded)
    );
}

#[test]
fn source_variants_decode_and_reencode_canonically() {
    let source = source();
    let edits = [
        SessionEdit::UpsertSource {
            source: source.clone(),
        },
        SessionEdit::RemoveSource {
            source_id: source.id.clone(),
        },
        SessionEdit::SetSourceContent {
            source_id: source.id.clone(),
            content: source.content.clone(),
            channels: source.channels,
            bit_depth: source.bit_depth,
            frames: source.frames,
        },
    ];
    let bytes = encode(&edits);
    let codec = ProtocolCodec::default();
    let mut scratch = [0_u16; 5];
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
        .expect("typed decode");
    assert_eq!(decoded.edits, edits);
    assert_eq!(encode(&decoded.edits), bytes);
}

#[test]
fn deleted_source_and_limits_opcodes_are_typed_refusals() {
    let source = source();
    let edit = SessionEdit::SetSourceContent {
        source_id: source.id,
        content: source.content,
        channels: source.channels,
        bit_depth: source.bit_depth,
        frames: source.frames,
    };
    let canonical = encode(core::slice::from_ref(&edit));
    const OPCODE_TLV_PREFIX: [u8; 8] = [1, 0, 2, 1, 2, 0, 0, 0];
    let opcode_at = canonical
        .windows(OPCODE_TLV_PREFIX.len() + 2)
        .position(|window| {
            window[..OPCODE_TLV_PREFIX.len()] == OPCODE_TLV_PREFIX
                && window[OPCODE_TLV_PREFIX.len()..] == 0x0103_u16.to_le_bytes()
        })
        .expect("canonical source-content opcode TLV")
        + OPCODE_TLV_PREFIX.len();
    let codec = ProtocolCodec::default();
    for deleted in [0x0006_u16, 0x0102, 0x0104] {
        let mut bytes = canonical.clone();
        bytes[opcode_at..opcode_at + 2].copy_from_slice(&deleted.to_le_bytes());
        assert_eq!(
            codec.decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 16]),),
            Err(DecodeError::InvalidTlv),
            "deleted opcode 0x{deleted:04x} must refuse before payload dispatch"
        );
    }
}

#[test]
fn every_track_effect_opcode_and_nested_variant_round_trips_canonically() {
    let mut full_track = track();
    let parameters = vec![
        EffectParam {
            parameter_id: 11,
            channel: ParameterChannel::Left,
            unit: ParameterUnit::Db,
            value: -3.0,
        },
        EffectParam {
            parameter_id: 12,
            channel: ParameterChannel::Right,
            unit: ParameterUnit::Hz,
            value: 2_000.0,
        },
        EffectParam {
            parameter_id: 13,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Milliseconds,
            value: 12.0,
        },
        EffectParam {
            parameter_id: 14,
            channel: ParameterChannel::Left,
            unit: ParameterUnit::Samples,
            value: 4.0,
        },
        EffectParam {
            parameter_id: 15,
            channel: ParameterChannel::Right,
            unit: ParameterUnit::Linear,
            value: 0.5,
        },
        EffectParam {
            parameter_id: 16,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Ratio,
            value: 2.0,
        },
    ];
    let native = effect_with(
        "native-fx",
        EffectIdentity::Native {
            effect_id: id("parametric-eq"),
        },
        EffectQuality::Draft,
        LinkMode::DualMono,
        parameters.clone(),
        SidechainDeclaration::None,
    );
    let cid = effect_with(
        "cid-fx",
        EffectIdentity::ThirdPartyCid {
            cid: "bafycid-demo".to_owned(),
        },
        EffectQuality::Normal,
        LinkMode::Maximum,
        Vec::new(),
        SidechainDeclaration::Routed(Sidechain {
            source: RouteSource::Track {
                track_id: id("vocal"),
                tap: SendTap::PostFader,
            },
            port_id: id("detector"),
        }),
    );
    let high = effect_with(
        "high-fx",
        EffectIdentity::Native {
            effect_id: id("compressor"),
        },
        EffectQuality::High,
        LinkMode::Average,
        Vec::new(),
        SidechainDeclaration::Routed(Sidechain {
            source: RouteSource::SubmixOutput {
                submix_id: id("drums"),
            },
            port_id: id("key"),
        }),
    );
    full_track.simd1.effects = vec![native.clone(), cid.clone()];
    full_track.dynamic.effects = vec![cid.clone(), high.clone()];
    full_track.simd2.effects = vec![high.clone(), native.clone()];
    let track_id = full_track.id.clone();
    let source_id = full_track.source_id.clone();
    let mut edits = vec![
        SessionEdit::UpsertTrack {
            track: full_track.clone(),
        },
        SessionEdit::RemoveTrack {
            track_id: track_id.clone(),
        },
        SessionEdit::SetTrackSourceAssignment {
            track_id: track_id.clone(),
            source_id,
            left_source_channel: 1,
            right_source_channel: 0,
        },
        SessionEdit::SetTrackBuiltins {
            track_id: track_id.clone(),
            builtins: full_track.builtins.clone(),
        },
    ];
    for (rack_name, rack) in [
        (RackName::Simd1, full_track.simd1.clone()),
        (RackName::Dynamic, full_track.dynamic.clone()),
        (RackName::Simd2, full_track.simd2.clone()),
    ] {
        edits.push(SessionEdit::SetTrackRack {
            track_id: track_id.clone(),
            rack_name,
            rack,
        });
    }
    edits.extend([
        SessionEdit::PutTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            final_position: 1,
            effect: native.clone(),
        },
        SessionEdit::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: cid.id.clone(),
        },
        SessionEdit::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_ids: vec![cid.id.clone(), high.id.clone()],
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: native.id.clone(),
            identity: native.identity.clone(),
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: cid.id.clone(),
            identity: cid.identity.clone(),
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Simd1,
            effect_id: native.id.clone(),
            quality: EffectQuality::Draft,
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: cid.id.clone(),
            quality: EffectQuality::Normal,
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Simd2,
            effect_id: high.id.clone(),
            quality: EffectQuality::High,
        },
        SessionEdit::SetEffectBypass {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: native.id.clone(),
            bypass: true,
        },
        SessionEdit::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Simd1,
            effect_id: native.id.clone(),
            link_mode: LinkMode::DualMono,
        },
        SessionEdit::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: cid.id.clone(),
            link_mode: LinkMode::Maximum,
        },
        SessionEdit::SetEffectLinkMode {
            track_id: track_id.clone(),
            rack_name: RackName::Simd2,
            effect_id: high.id.clone(),
            link_mode: LinkMode::Average,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: native.id.clone(),
            sidechain: SidechainDeclaration::None,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: cid.id.clone(),
            sidechain: cid.sidechain.clone(),
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Simd2,
            effect_id: high.id.clone(),
            sidechain: high.sidechain.clone(),
        },
    ]);
    for parameter in parameters {
        edits.push(SessionEdit::UpsertEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: native.id.clone(),
            param: parameter,
        });
    }
    edits.extend([
        SessionEdit::RemoveEffectParam {
            track_id: track_id.clone(),
            rack_name: RackName::Dynamic,
            effect_id: native.id.clone(),
            parameter_id: 11,
            channel: ParameterChannel::Left,
        },
        SessionEdit::SetTrackFader {
            track_id: track_id.clone(),
            fader: full_track.fader.clone(),
        },
        SessionEdit::SetTrackMatrixOrPan {
            track_id: track_id.clone(),
            matrix_or_pan: full_track.matrix_or_pan.clone(),
        },
        SessionEdit::SetTrackMatrixOrPan {
            track_id,
            matrix_or_pan: MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.25,
                rl: 0.5,
                rr: 1.0,
                smoothing_samples: 32,
            },
        },
    ]);
    let bytes = encode(&edits);
    let codec = ProtocolCodec::default();
    let mut scratch = [0_u16; 64];
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
        .expect("typed track/effect decode");
    assert_eq!(decoded.edits, edits);
    assert_eq!(encode(&decoded.edits), bytes);
    let SessionEdit::UpsertTrack {
        track: decoded_track,
    } = &decoded.edits[0]
    else {
        panic!("first edit must be upsert track");
    };
    assert_eq!(
        decoded_track
            .dynamic
            .effects
            .iter()
            .map(|effect| effect.id.as_str())
            .collect::<Vec<_>>(),
        ["cid-fx", "high-fx"]
    );
}

#[test]
fn track_effect_schema_rejects_duplicate_type_order_and_variant_errors() {
    let duplicate = raw_message(vec![
        (1, WIRE_BOOL, true, vec![0]),
        (1, WIRE_BOOL, true, vec![0]),
        (2, WIRE_F32, true, 0.0_f32.to_le_bytes().to_vec()),
        (3, WIRE_F32, true, 20.0_f32.to_le_bytes().to_vec()),
        (4, WIRE_F32, true, 20_000.0_f32.to_le_bytes().to_vec()),
    ]);
    assert!(parse_channel_builtins(Message::nested(&duplicate).expect("nested")).is_err());

    let wrong_type = raw_message(vec![
        (1, WIRE_U8, true, vec![0]),
        (2, WIRE_F32, true, 0.0_f32.to_le_bytes().to_vec()),
        (3, WIRE_F32, true, 20.0_f32.to_le_bytes().to_vec()),
        (4, WIRE_F32, true, 20_000.0_f32.to_le_bytes().to_vec()),
    ]);
    assert!(parse_channel_builtins(Message::nested(&wrong_type).expect("nested")).is_err());

    let unknown = raw_message(vec![
        (1, WIRE_U8, true, vec![1]),
        (2, WIRE_U8, true, vec![7]),
    ]);
    assert_eq!(
        parse_sidechain(Message::nested(&unknown).expect("nested")),
        Err(DecodeError::UnknownRequiredField)
    );

    let reversed = [
        2, 0, WIRE_U8, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, WIRE_U8, 1, 1, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(
        Message::raw(&reversed, 2)
            .schema_spec(&schema::session::sidechain::ROUTED)
            .err(),
        Some(DecodeError::InvalidTlv)
    );
}

#[test]
fn tagged_pan_rejects_optional_field_known_only_to_matrix_variant() {
    let pan_with_matrix_field = raw_message(vec![
        (1, WIRE_U8, true, vec![1]),
        (2, WIRE_F32, true, 0.25_f32.to_le_bytes().to_vec()),
        (3, WIRE_F32, true, 0.75_f32.to_le_bytes().to_vec()),
        (4, WIRE_U32, true, 32_u32.to_le_bytes().to_vec()),
        (5, WIRE_F32, false, 1.0_f32.to_le_bytes().to_vec()),
    ]);
    assert_eq!(
        parse_matrix_or_pan(Message::nested(&pan_with_matrix_field).expect("nested")),
        Err(DecodeError::InvalidTlv)
    );
}

#[test]
fn every_send_tap_tag_is_typed_and_canonical() {
    for tap in [
        SendTap::Input,
        SendTap::PostInputBuiltins,
        SendTap::PostSimd1,
        SendTap::PostDynamic,
        SendTap::PostSimd2PreFader,
        SendTap::PostFader,
        SendTap::PostMatrix,
    ] {
        let source = RouteSource::Track {
            track_id: id("vocal"),
            tap,
        };
        let limits = ProtocolCodec::default().limits();
        let mut count = CountSink::new(limits);
        tx_route_source(&mut count, &source).expect("size route source");
        let mut encoded = vec![0; count.written()];
        let mut writer = SliceSink::new(&mut encoded, limits);
        tx_route_source(&mut writer, &source).expect("encode route source");
        assert_eq!(
            parse_route_source(Message::nested(&encoded).expect("nested route source")),
            Ok(source)
        );
    }
}

#[test]
fn every_route_and_automation_opcode_round_trips_canonically() {
    let session = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture");
    let route = session.routes[0].clone();
    let target = session.automation[0].target.clone();
    let automation = Automation {
        id: id("ride"),
        target: target.clone(),
        segments: vec![
            AutomationSegment {
                shape: AutomationShape::Step,
                start_sample: 0,
                end_sample: 16,
                start_value: -6.0,
                end_value: -6.0,
                unit: ParameterUnit::Db,
            },
            AutomationSegment {
                shape: AutomationShape::Linear,
                start_sample: 16,
                end_sample: 32,
                start_value: 100.0,
                end_value: 1_000.0,
                unit: ParameterUnit::Hz,
            },
            AutomationSegment {
                shape: AutomationShape::Exponential,
                start_sample: 32,
                end_sample: 48,
                start_value: 1.0,
                end_value: 2.0,
                unit: ParameterUnit::Ratio,
            },
        ],
    };
    let edits = vec![
        SessionEdit::UpsertSubmix {
            submix: Submix { id: id("drums") },
        },
        SessionEdit::RemoveSubmix {
            submix_id: id("drums"),
        },
        SessionEdit::UpsertOutput {
            output: Output { id: id("alt-out") },
        },
        SessionEdit::RemoveOutput {
            output_id: id("alt-out"),
        },
        SessionEdit::UpsertRoute {
            route: route.clone(),
        },
        SessionEdit::RemoveRoute {
            route_id: route.id.clone(),
        },
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: RouteSource::Track {
                track_id: id("vocal"),
                tap: SendTap::PostMatrix,
            },
        },
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: RouteSource::SubmixOutput {
                submix_id: id("drums"),
            },
        },
        SessionEdit::SetRouteDestination {
            route_id: route.id.clone(),
            destination: RouteDestination::SubmixInput {
                submix_id: id("drums"),
            },
        },
        SessionEdit::SetRouteDestination {
            route_id: route.id.clone(),
            destination: RouteDestination::OutputInput {
                output_id: id("main-out"),
            },
        },
        SessionEdit::SetRouteChannelMatrix {
            route_id: route.id.clone(),
            channel_matrix: ChannelMatrix {
                ll: 1.0,
                lr: 0.25,
                rl: 0.5,
                rr: 1.0,
            },
        },
        SessionEdit::SetRouteGainDb {
            route_id: route.id.clone(),
            gain_db: -1.5,
        },
        SessionEdit::UpsertAutomation {
            automation: automation.clone(),
        },
        SessionEdit::RemoveAutomation {
            automation_id: automation.id.clone(),
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: target.clone(),
        },
        SessionEdit::SetAutomationSegments {
            automation_id: automation.id.clone(),
            segments: automation.segments.clone(),
        },
    ];
    let bytes = encode(&edits);
    let codec = ProtocolCodec::default();
    let mut scratch = [0_u16; 32];
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut scratch))
        .expect("typed route/automation decode");
    assert_eq!(decoded.edits, edits);
    assert_eq!(encode(&decoded.edits), bytes);
    let SessionEdit::UpsertAutomation { automation } = &decoded.edits[12] else {
        panic!("upsert automation");
    };
    assert_eq!(
        automation
            .segments
            .iter()
            .map(|segment| segment.start_sample)
            .collect::<Vec<_>>(),
        [0, 16, 32]
    );
}

#[test]
fn route_and_automation_unknown_optional_fields_are_canonicalized_away() {
    let bytes = raw_message(vec![
        (1, WIRE_U8, true, vec![2]),
        (2, WIRE_UTF8, true, b"main-out".to_vec()),
        (99, WIRE_U8, false, vec![7]),
    ]);
    assert_eq!(
        parse_route_destination(Message::nested(&bytes).expect("nested destination")),
        Ok(RouteDestination::OutputInput {
            output_id: id("main-out"),
        })
    );

    let mut required = bytes;
    required[8 + 16 + 16 + 3] = 1;
    assert_eq!(
        parse_route_destination(Message::nested(&required).expect("nested destination")),
        Err(DecodeError::UnknownRequiredField)
    );
}

#[test]
fn optional_fields_skip_but_required_and_corrupt_nested_fields_reject() {
    let edits = [SessionEdit::SetSessionId {
        session_id: id("next"),
    }];
    let canonical = encode(&edits);
    let mut optional = canonical.clone();
    let optional_offset = optional.len();
    optional.extend_from_slice(&[99, 0, WIRE_U8, 0, 1, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0]);
    put_u32(&mut optional, 20, 80);
    put_u32(&mut optional, 40, 2);
    let codec = ProtocolCodec::default();
    let mut scratch = [0_u16; 2];
    let decoded = codec
        .decode_session_transaction(&optional, &mut DecodeScratch::new(&mut scratch))
        .expect("optional skips");
    assert_eq!(encode(&decoded.edits), canonical);
    optional[optional_offset + 3] = 1;
    assert_eq!(
        codec.decode_session_transaction(&optional, &mut DecodeScratch::new(&mut scratch)),
        Err(DecodeError::UnknownRequiredField)
    );
    let mut bad_type = canonical;
    bad_type[48 + 8 + 8 + 2] = WIRE_U32;
    assert!(
        codec
            .decode_session_transaction(&bad_type, &mut DecodeScratch::new(&mut [0_u16; 1]))
            .is_err()
    );
}

#[test]
fn every_byte_of_transaction_golden_truncates() {
    let bytes = encode(&[SessionEdit::SetSessionId {
        session_id: id("next"),
    }]);
    let codec = ProtocolCodec::default();
    for end in 0..bytes.len() {
        assert!(
            codec
                .decode_session_transaction(&bytes[..end], &mut DecodeScratch::new(&mut [0_u16; 1]))
                .is_err()
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    engine::hex_lower(bytes)
}
