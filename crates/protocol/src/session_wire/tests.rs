use super::*;
use crate::btlv::{WIRE_BOOL, WIRE_F32, WIRE_MESSAGE, WIRE_U8, WIRE_U32, WIRE_UTF8};
use session::{
    ControlAddress, LinkMode, Owner, ParameterChannel, ParameterUnit, RenderMode, Section, SendTap,
    Sidechain, Track, parse_session_json,
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
        content: "blake3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
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
    let effect = track.inserts.effects[0].clone();
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
        SessionEdit::SetTrackRack {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            rack: track.inserts.clone(),
        },
        SessionEdit::PutTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            final_position: 0,
            effect: effect.clone(),
        },
        SessionEdit::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
        },
        SessionEdit::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_ids: vec![effect_id.clone()],
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            identity: effect.identity.clone(),
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            quality: effect.quality,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: effect_id.clone(),
            sidechain: effect.sidechain.clone(),
        },
        SessionEdit::UpsertSubmix {
            submix: Submix::unity(id("drums"), &session.console),
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
        // #1566: the target is the control address `SET_CONTROL` carries, so each owner kind and
        // each section is a round trip of its own: the strip's builtins and own sections, a
        // console slot, a route's gain and a VCA's offset.
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: ControlAddress {
                owner: Owner::Strip(id("vocal")),
                section: Section::Builtins,
                control: 2,
                lane: ParameterChannel::Left,
            },
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: ControlAddress {
                owner: Owner::Strip(id("vocal")),
                section: Section::Console(id("desk-eq")),
                control: 4,
                lane: ParameterChannel::Right,
            },
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: ControlAddress {
                owner: Owner::Route(id("to-main")),
                section: Section::Own,
                control: 1,
                lane: ParameterChannel::Both,
            },
        },
        SessionEdit::SetAutomationTarget {
            automation_id: automation.id.clone(),
            target: ControlAddress {
                owner: Owner::Vca(id("group")),
                section: Section::Own,
                control: 1,
                lane: ParameterChannel::Both,
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
    output.fill(0);
    assert_eq!(
        codec.encode_session_transaction(&transaction, &mut output),
        Ok(required)
    );
    assert_eq!(output, canonical);
}

#[test]
fn transaction_repeated_fields_match_schema_derived_count() {
    let edits = [SessionEdit::SetTrackEffectOrder {
        track_id: id("track.repeated"),
        rack_name: RackName::Inserts,
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

/// Render-profile mode `1` is `single_thread`, the only live code. `2` was the retired
/// `dependency_waves` (#1063): it refuses like `0` and the unallocated tail, never reinterpreted.
#[test]
fn render_profile_mode_decodes_only_single_thread() {
    let profile = |mode: u8| {
        raw_message(vec![
            (1, WIRE_UTF8, true, b"native".to_vec()),
            (2, WIRE_U8, true, vec![mode]),
        ])
    };
    assert_eq!(
        parse_render_profile(Message::nested(&profile(1)).expect("nested")),
        Ok(RenderProfile {
            id: id("native"),
            mode: RenderMode::SingleThread,
        })
    );
    for refused in [0_u8, 2, 3, u8::MAX] {
        assert_eq!(
            parse_render_profile(Message::nested(&profile(refused)).expect("nested")),
            Err(DecodeError::InvalidTlv),
            "render mode code {refused} must refuse"
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
            source: RouteSource::Submix {
                submix_id: id("drums"),
                tap: SendTap::PostPan,
            },
            port_id: id("key"),
        }),
    );
    // Decision 12's track: two console entries (field 11, repeated) and the inserts rack.
    full_track.console = vec![
        session::ConsoleEntry {
            slot: id("desk-eq"),
            bypass: true,
            link_mode: session::ConsoleLinkMode::Slot,
            params: parameters.clone(),
        },
        session::ConsoleEntry {
            slot: id("desk-limit"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Slot,
            params: Vec::new(),
        },
    ];
    full_track.inserts.effects = vec![cid.clone(), high.clone()];
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
    ];
    for (rack_name, rack) in [
        (RackName::Inserts, full_track.inserts.clone()),
        // Codec coverage of the appended rack code; the store refuses it until #1094 (S1b).
        (
            RackName::Console,
            Rack {
                effects: vec![native.clone()],
            },
        ),
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
            rack_name: RackName::Inserts,
            final_position: 1,
            effect: native.clone(),
        },
        SessionEdit::RemoveTrackEffect {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: cid.id.clone(),
        },
        SessionEdit::SetTrackEffectOrder {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_ids: vec![cid.id.clone(), high.id.clone()],
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: native.id.clone(),
            identity: native.identity.clone(),
        },
        SessionEdit::SetEffectIdentity {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: cid.id.clone(),
            identity: cid.identity.clone(),
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Console,
            effect_id: native.id.clone(),
            quality: EffectQuality::Draft,
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: cid.id.clone(),
            quality: EffectQuality::Normal,
        },
        SessionEdit::SetEffectQuality {
            track_id: track_id.clone(),
            rack_name: RackName::Builtins,
            effect_id: high.id.clone(),
            quality: EffectQuality::High,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: native.id.clone(),
            sidechain: SidechainDeclaration::None,
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Inserts,
            effect_id: cid.id.clone(),
            sidechain: cid.sidechain.clone(),
        },
        SessionEdit::SetEffectSidechain {
            track_id: track_id.clone(),
            rack_name: RackName::Builtins,
            effect_id: high.id.clone(),
            sidechain: high.sidechain.clone(),
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
            .inserts
            .effects
            .iter()
            .map(|effect| effect.id.as_str())
            .collect::<Vec<_>>(),
        ["cid-fx", "high-fx"]
    );
    assert_eq!(decoded_track.console, full_track.console);
}

fn nested_bytes(write: &dyn Fn(&mut dyn Sink) -> Result<(), EncodeError>) -> Vec<u8> {
    let limits = ProtocolCodec::default().limits();
    let mut count = CountSink::new(limits);
    write(&mut count).expect("size nested message");
    let mut encoded = vec![0; count.written()];
    let mut writer = SliceSink::new(&mut encoded, limits);
    write(&mut writer).expect("encode nested message");
    encoded
}

/// Decision 12 retired track fields 6 (`simd1`) and 8 (`simd2`) in place. A peer that still sends
/// one is refused in either flag form, never read as a console section or as the inserts rack, and
/// never skipped as an unknown optional field would be (#1094). The same message without it
/// decodes, and a never-allocated optional field (12) is still skipped, so the refusal is about
/// the retired IDs and nothing else.
///
/// Red if `parse_track` reads the track with the plain `schema_spec` (the optional retired field
/// is then skipped and the track decodes).
#[test]
fn retired_track_rack_fields_are_refused() {
    let session = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture");
    let track = &session.tracks[0];
    let builtins = nested_bytes(&|sink| tx_builtins(sink, &track.builtins));
    let rack = nested_bytes(&|sink| tx_rack(sink, &track.inserts));
    let fader = nested_bytes(&|sink| tx_fader(sink, &track.fader));
    let pan = nested_bytes(&|sink| tx_matrix_or_pan(sink, &track.matrix_or_pan));
    let message = |extra: Option<(u16, bool)>| {
        let mut fields = vec![
            (1, WIRE_UTF8, true, track.id.as_str().as_bytes().to_vec()),
            (
                2,
                WIRE_UTF8,
                true,
                track.source_id.as_str().as_bytes().to_vec(),
            ),
            (3, WIRE_U8, true, vec![track.left_source_channel]),
            (4, WIRE_U8, true, vec![track.right_source_channel]),
            (5, WIRE_MESSAGE, true, builtins.clone()),
            (7, WIRE_MESSAGE, true, rack.clone()),
            (9, WIRE_MESSAGE, true, fader.clone()),
            (10, WIRE_MESSAGE, true, pan.clone()),
        ];
        if let Some((field, mandatory)) = extra {
            fields.push((field, WIRE_MESSAGE, mandatory, rack.clone()));
            fields.sort_by_key(|(id, ..)| *id);
        }
        raw_message(fields)
    };
    let parse = |extra| parse_track(Message::nested(&message(extra)).expect("nested track"));
    assert_eq!(parse(None), Ok(track.clone()));
    assert_eq!(
        parse(Some((12, false))),
        Ok(track.clone()),
        "an unallocated optional field is still skipped"
    );
    for retired in [6, 8] {
        assert_eq!(
            parse(Some((retired, true))),
            Err(DecodeError::UnknownRequiredField),
            "mandatory retired track field {retired}"
        );
        assert_eq!(
            parse(Some((retired, false))),
            Err(DecodeError::InvalidTlv),
            "optional retired track field {retired}"
        );
    }
}

/// Decision 12's console declaration and track entries (#1094): `SetConsole` (`0x0007`) and
/// `SetTrackConsole` (`0x0211`) round-trip canonically -- both sections, empty sections, a
/// third-party identity (the codec carries the model; validation refuses it), every quality and
/// link mode, entries with and without params -- and a slot carries no sidechain field.
///
/// Red if a section's field IDs are swapped (a pre-insert slot decodes as post-insert), if a slot
/// field is dropped or mis-typed, or if the entry repeat count is not derived from the schema.
#[test]
fn console_declaration_and_track_entry_opcodes_round_trip_canonically() {
    let native = |slot: &str, quality, link_mode| session::ConsoleSlot {
        slot: id(slot),
        identity: EffectIdentity::Native {
            effect_id: id("miso.parametric-eq"),
        },
        quality,
        link_mode,
    };
    let full = session::Console {
        pre_insert: vec![
            native("desk-eq", EffectQuality::Draft, LinkMode::DualMono),
            native("desk-comp", EffectQuality::Normal, LinkMode::Maximum),
        ],
        post_insert: vec![session::ConsoleSlot {
            slot: id("desk-cid"),
            identity: EffectIdentity::ThirdPartyCid {
                cid: "bafycid-demo".to_owned(),
            },
            quality: EffectQuality::High,
            link_mode: LinkMode::Average,
        }],
    };
    let entries = vec![
        session::ConsoleEntry {
            slot: id("desk-eq"),
            bypass: true,
            link_mode: session::ConsoleLinkMode::Slot,
            params: vec![EffectParam {
                parameter_id: 1,
                channel: ParameterChannel::Right,
                unit: ParameterUnit::Hz,
                value: 440.0,
            }],
        },
        session::ConsoleEntry {
            slot: id("desk-comp"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::DualMono,
            params: Vec::new(),
        },
        session::ConsoleEntry {
            slot: id("desk-max"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Maximum,
            params: Vec::new(),
        },
        session::ConsoleEntry {
            slot: id("desk-avg"),
            bypass: true,
            link_mode: session::ConsoleLinkMode::Average,
            params: Vec::new(),
        },
    ];
    // #1369 D4: an entry's link mode is required field 4, a closed code table 1 (`slot`) to 4. A
    // decoder that defaulted a missing field would silently read the pre-#1369 grammar as `slot`.
    let entry = |link: Option<u8>| {
        let mut fields = vec![
            (1, WIRE_UTF8, true, b"desk-eq".to_vec()),
            (2, WIRE_BOOL, true, vec![1]),
        ];
        if let Some(code) = link {
            fields.push((4, WIRE_U8, true, vec![code]));
        }
        parse_console_entry(Message::nested(&raw_message(fields)).expect("nested entry"))
    };
    for (code, link_mode) in [
        (1, session::ConsoleLinkMode::Slot),
        (2, session::ConsoleLinkMode::DualMono),
        (3, session::ConsoleLinkMode::Maximum),
        (4, session::ConsoleLinkMode::Average),
    ] {
        assert_eq!(
            entry(Some(code)).map(|entry| entry.link_mode),
            Ok(link_mode),
            "link code {code}"
        );
    }
    assert_eq!(
        entry(None),
        Err(DecodeError::InvalidTlv),
        "a missing link mode refuses"
    );
    for code in [0, 5, u8::MAX] {
        assert_eq!(
            entry(Some(code)),
            Err(DecodeError::InvalidTlv),
            "unknown link code {code}"
        );
    }
    let edits = vec![
        SessionEdit::SetConsole {
            console: full.clone(),
            ramp_samples: None,
        },
        SessionEdit::SetConsole {
            console: session::Console {
                pre_insert: Vec::new(),
                post_insert: full.post_insert.clone(),
            },
            ramp_samples: None,
        },
        SessionEdit::SetConsole {
            console: session::Console {
                pre_insert: Vec::new(),
                post_insert: Vec::new(),
            },
            ramp_samples: None,
        },
        SessionEdit::SetTrackConsole {
            track_id: id("vocal"),
            console: entries,
            ramp_samples: None,
        },
        SessionEdit::SetTrackConsole {
            track_id: id("vocal"),
            console: Vec::new(),
            ramp_samples: None,
        },
    ];
    let bytes = encode(&edits);
    let codec = ProtocolCodec::default();
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 8]))
        .expect("console edits decode");
    assert_eq!(decoded.edits, edits);
    assert_eq!(encode(&decoded.edits), bytes);
    assert_eq!(
        schema::session::console_slot::SPEC
            .fields
            .iter()
            .map(|field| field.id)
            .collect::<Vec<_>>(),
        [1, 2, 3, 4],
        "slot, identity, quality, link mode; no sidechain"
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
        SendTap::PostInput,
        SendTap::InsertSend,
        SendTap::InsertReturn,
        SendTap::PreFader,
        SendTap::PostFader,
        SendTap::PostPan,
    ] {
        // #1203 D2: a submix source carries the same seven tap codes as a track source.
        for source in [
            RouteSource::Track {
                track_id: id("vocal"),
                tap,
            },
            RouteSource::Submix {
                submix_id: id("drums"),
                tap,
            },
        ] {
            let encoded = nested_bytes(&|sink| tx_route_source(sink, &source));
            assert_eq!(
                parse_route_source(Message::nested(&encoded).expect("nested route source")),
                Ok(source)
            );
        }
    }
}

/// #1203 D2: tag 2 (a submix source) requires `TAP` (field 3), in each of the three shapes that
/// share the route-source spec: a route's source, a routed sidechain's source and a
/// `SetRouteSource` value. Each container decodes with the tapped source and refuses the same
/// container with the tap left out, so the refusal is the missing tap and nothing else.
///
/// Red if any of the three decoders reads a tapless submix source (for example, defaulting it to
/// the strip's end, as `submix_output` meant), or if only the route's shape enforces the tap.
#[test]
fn a_tapless_submix_source_is_refused_in_every_shape_that_carries_one() {
    let session = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture");
    let route = session.routes[0].clone();
    let source = RouteSource::Submix {
        submix_id: id("drums"),
        tap: SendTap::PreFader,
    };
    let tapped = nested_bytes(&|sink| tx_route_source(sink, &source));
    let tapless = raw_message(vec![
        (1, WIRE_U8, true, vec![2]),
        (2, WIRE_UTF8, true, b"drums".to_vec()),
    ]);
    let destination = nested_bytes(&|sink| tx_route_destination(sink, &route.destination));
    let matrix = nested_bytes(&|sink| tx_channel_matrix(sink, &route.channel_matrix));
    let route_message = |source: &[u8]| {
        raw_message(vec![
            (1, WIRE_UTF8, true, route.id.as_str().as_bytes().to_vec()),
            (2, WIRE_MESSAGE, true, source.to_vec()),
            (3, WIRE_MESSAGE, true, destination.clone()),
            (4, WIRE_MESSAGE, true, matrix.clone()),
            (5, WIRE_F32, true, route.gain_db.to_le_bytes().to_vec()),
            (6, WIRE_BOOL, true, vec![u8::from(route.mute)]),
            (7, WIRE_BOOL, true, vec![u8::from(route.follows_mute)]),
        ])
    };
    let sidechain_message = |source: &[u8]| {
        raw_message(vec![
            (1, WIRE_U8, true, vec![2]),
            (2, WIRE_MESSAGE, true, source.to_vec()),
            (3, WIRE_UTF8, true, b"key".to_vec()),
        ])
    };
    let edit_message = |source: &[u8]| {
        let payload = raw_message(vec![
            (1, WIRE_UTF8, true, route.id.as_str().as_bytes().to_vec()),
            (2, WIRE_MESSAGE, true, source.to_vec()),
        ]);
        raw_message(vec![
            (
                1,
                schema::Wire::U16.raw(),
                true,
                crate::SessionEditOpcode::SetRouteSource
                    .raw()
                    .to_le_bytes()
                    .to_vec(),
            ),
            (2, WIRE_MESSAGE, true, payload),
        ])
    };

    let parse_route_bytes = |bytes: Vec<u8>| parse_route(Message::nested(&bytes).expect("route"));
    let parse_sidechain_bytes =
        |bytes: Vec<u8>| parse_sidechain(Message::nested(&bytes).expect("sidechain"));
    let parse_edit_bytes = |bytes: Vec<u8>| parse_edit(Message::nested(&bytes).expect("edit"));
    assert_eq!(
        parse_route_bytes(route_message(&tapped)),
        Ok(Route {
            source: source.clone(),
            ..route.clone()
        })
    );
    assert_eq!(
        parse_sidechain_bytes(sidechain_message(&tapped)),
        Ok(SidechainDeclaration::Routed(Sidechain {
            source: source.clone(),
            port_id: id("key"),
        }))
    );
    assert_eq!(
        parse_edit_bytes(edit_message(&tapped)),
        Ok(SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: source.clone(),
        })
    );
    assert_eq!(
        parse_route_bytes(route_message(&tapless)),
        Err(DecodeError::InvalidTlv),
        "a tapless route source"
    );
    assert_eq!(
        parse_sidechain_bytes(sidechain_message(&tapless)),
        Err(DecodeError::InvalidTlv),
        "a tapless sidechain source"
    );
    assert_eq!(
        parse_edit_bytes(edit_message(&tapless)),
        Err(DecodeError::InvalidTlv),
        "a tapless SetRouteSource value"
    );
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
        // #1650 D1: a stored offset crosses the codec as field 4. Distinct lanes and a `-0.0`
        // catch a codec that swaps the two fields or drops a zero's sign.
        offset: session::AutomationOffset {
            left_db: 2.5,
            right_db: -0.0,
        },
    };
    let edits = vec![
        SessionEdit::UpsertSubmix {
            submix: Submix::unity(id("drums"), &session.console),
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
        // #1216: a muted route carries field 6 through the codec; #1218: a following route
        // carries field 7. The two routes swap the booleans, so a codec that confuses the fields
        // in either direction is caught (#1218 verdict MINOR-1).
        SessionEdit::UpsertRoute {
            route: Route {
                mute: false,
                follows_mute: true,
                ..route.clone()
            },
        },
        SessionEdit::UpsertRoute {
            route: Route {
                mute: true,
                follows_mute: false,
                ..route.clone()
            },
        },
        SessionEdit::RemoveRoute {
            route_id: route.id.clone(),
        },
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: RouteSource::Track {
                track_id: id("vocal"),
                tap: SendTap::PostPan,
            },
        },
        SessionEdit::SetRouteSource {
            route_id: route.id.clone(),
            source: RouteSource::Submix {
                submix_id: id("drums"),
                tap: SendTap::PostPan,
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
    let SessionEdit::UpsertAutomation { automation } = &decoded.edits[11] else {
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

/// A deterministic splitmix64 stream for the submix-strip draw below.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    fn bit(&mut self) -> bool {
        self.next() & 1 == 1
    }
    /// Any finite `f32`, signed zeros and subnormals included.
    fn f32(&mut self) -> f32 {
        loop {
            let value = f32::from_bits(self.next() as u32);
            if value.is_finite() {
                return value;
            }
        }
    }
    fn u32(&mut self) -> u32 {
        self.next() as u32
    }
}

fn random_submix(draw: &mut Draw, index: usize) -> Submix {
    let lane = |draw: &mut Draw| ChannelBuiltins {
        polarity_invert: draw.bit(),
        trim_db: draw.f32(),
        hpf_hz: draw.f32(),
        lpf_hz: draw.f32(),
        delay_samples: draw.u32(),
    };
    let builtins = DualMonoBuiltins {
        left: lane(draw),
        right: lane(draw),
    };
    let qualities = [
        EffectQuality::Draft,
        EffectQuality::Normal,
        EffectQuality::High,
    ];
    let links = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];
    let channels = [
        ParameterChannel::Left,
        ParameterChannel::Right,
        ParameterChannel::Both,
    ];
    let units = [
        ParameterUnit::Db,
        ParameterUnit::Hz,
        ParameterUnit::Milliseconds,
        ParameterUnit::Samples,
        ParameterUnit::Linear,
        ParameterUnit::Ratio,
    ];
    let effects = (0..draw.below(3))
        .map(|effect| {
            let params = (0..draw.below(3))
                .map(|param| EffectParam {
                    parameter_id: param as u32 * 2 + draw.below(2) as u32,
                    channel: channels[draw.below(3) as usize],
                    unit: units[draw.below(6) as usize],
                    value: draw.f32(),
                })
                .collect();
            Effect {
                id: id(&format!("fx-{effect}")),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.parametric-eq"),
                },
                quality: qualities[draw.below(3) as usize],
                bypass: draw.bit(),
                link_mode: links[draw.below(3) as usize],
                params,
                sidechain: SidechainDeclaration::None,
            }
        })
        .collect();
    let fader = DualMonoFader {
        left_db: draw.f32(),
        right_db: draw.f32(),
        left_mute: draw.bit(),
        right_mute: draw.bit(),
    };
    // Field 3 (#1202): zero to three console entries, so a draw may repeat the field.
    let console = (0..draw.below(4))
        .map(|entry| ConsoleEntry {
            slot: id(&format!("slot-{entry}")),
            bypass: draw.bit(),
            link_mode: session::ConsoleLinkMode::ALL[draw.below(4) as usize].0,
            params: (0..draw.below(3))
                .map(|param| EffectParam {
                    parameter_id: param as u32 * 2 + draw.below(2) as u32,
                    channel: channels[draw.below(3) as usize],
                    unit: units[draw.below(6) as usize],
                    value: draw.f32(),
                })
                .collect(),
        })
        .collect();
    let matrix_or_pan = if draw.bit() {
        MatrixOrPan::Pan {
            left: draw.f32(),
            right: draw.f32(),
            smoothing_samples: draw.u32(),
        }
    } else {
        MatrixOrPan::Matrix {
            ll: draw.f32(),
            lr: draw.f32(),
            rl: draw.f32(),
            rr: draw.f32(),
            smoothing_samples: draw.u32(),
        }
    };
    Submix {
        id: id(&format!("bus-{index}")),
        builtins,
        console,
        inserts: Rack { effects },
        fader,
        matrix_or_pan,
    }
}

/// #1199 gate 3: a random submix strip, pan in some draws and matrix in others, survives
/// `UpsertSubmix` encode and decode with every field intact, and re-encodes to the same bytes.
/// #1202: the strip carries zero to three console entries (field 3).
///
/// Red if `tx_submix`/`parse_submix` drops or mis-encodes a strip field asymmetrically, swaps pan
/// and matrix, misreads the shared tag (the track's `matrix_or_pan` round trips cannot see the
/// submix codec), or drops, reorders or truncates the repeated console entries. A renumbering made
/// on both sides round-trips; field numbers are pinned by `COMPLETE_SCHEMA_HASH`.
#[test]
fn random_submix_strips_round_trip_losslessly() {
    let mut draw = Draw(0x1199_0000_05ab_0001);
    let mut pans = 0;
    let mut matrices = 0;
    let mut repeated_console = 0;
    for index in 0..64 {
        let submix = random_submix(&mut draw, index);
        match submix.matrix_or_pan {
            MatrixOrPan::Pan { .. } => pans += 1,
            MatrixOrPan::Matrix { .. } => matrices += 1,
        }
        if submix.console.len() > 1 {
            repeated_console += 1;
        }
        let edits = vec![SessionEdit::UpsertSubmix {
            submix: submix.clone(),
        }];
        let bytes = encode(&edits);
        let decoded = ProtocolCodec::default()
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("submix strip decodes");
        let [SessionEdit::UpsertSubmix { submix: back }] = decoded.edits.as_slice() else {
            panic!("one upsert submix");
        };
        assert_eq!(back, &submix, "draw {index}");
        assert_eq!(encode(&decoded.edits), bytes, "draw {index} re-encodes");
    }
    assert!(pans > 0 && matrices > 0, "both variants drawn");
    assert!(repeated_console > 0, "a repeated console field drawn");
}

fn random_vca(draw: &mut Draw, index: usize) -> Vca {
    Vca {
        id: id(&format!("vca-{index}")),
        fader: DualMonoFader {
            left_db: draw.f32(),
            right_db: draw.f32(),
            left_mute: draw.bit(),
            right_mute: draw.bit(),
        },
        // Zero to four members, so a draw may omit field 3 or repeat it; declared order is kept.
        members: (0..draw.below(5))
            .map(|member| id(&format!("m-{}-{member}", draw.below(100))))
            .collect(),
    }
}

/// #1241 gate 1: `UpsertVca` with random VCAs (empty and non-empty `members`, both lanes, muted
/// and unmuted) and `RemoveVca` encode and decode to equal edits and re-encode to the same bytes; a hand-built VCA message with field IDs 1, 2 and 3 (D1) decodes to the expected
/// VCA, and one with no field 3 decodes to empty `members`.
///
/// Red if field 3 is lost, mis-tagged or refused when empty, or if the fader is read from the
/// wrong message or field.
#[test]
fn vca_edits_round_trip_and_members_decode_empty() {
    let mut draw = Draw(0x1241_0000_00ca_0001);
    let mut empty = 0;
    let mut repeated = 0;
    let mut muted = [0; 2];
    for index in 0..64 {
        let vca = random_vca(&mut draw, index);
        match vca.members.len() {
            0 => empty += 1,
            2.. => repeated += 1,
            _ => {}
        }
        muted[0] += usize::from(vca.fader.left_mute);
        muted[1] += usize::from(vca.fader.right_mute);
        let edits = vec![
            SessionEdit::UpsertVca { vca: vca.clone() },
            SessionEdit::RemoveVca {
                vca_id: vca.id.clone(),
            },
        ];
        let bytes = encode(&edits);
        let decoded = ProtocolCodec::default()
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("VCA edits decode");
        assert_eq!(decoded.edits, edits, "draw {index}");
        assert_eq!(encode(&decoded.edits), bytes, "draw {index} re-encodes");
    }
    assert!(
        empty > 0 && repeated > 0,
        "empty and repeated members drawn"
    );
    assert!(
        muted.iter().all(|&count| count > 0 && count < 64),
        "mutes vary"
    );

    let fader = DualMonoFader {
        left_db: -6.5,
        right_db: 3.25,
        left_mute: true,
        right_mute: false,
    };
    let fader_bytes = nested_bytes(&|sink| tx_fader(sink, &fader));
    let message = |members: &[&str]| {
        let mut fields = vec![
            (1, WIRE_UTF8, true, b"drums".to_vec()),
            (2, WIRE_MESSAGE, true, fader_bytes.clone()),
        ];
        fields.extend(
            members
                .iter()
                .map(|member| (3, WIRE_UTF8, true, member.as_bytes().to_vec())),
        );
        raw_message(fields)
    };
    let parse = |members: &[&str]| parse_vca(Message::nested(&message(members)).expect("nested"));
    assert_eq!(
        parse(&["snare", "kick"]),
        Ok(Vca {
            id: id("drums"),
            fader: fader.clone(),
            members: vec![id("snare"), id("kick")],
        })
    );
    assert_eq!(
        parse(&[]),
        Ok(Vca {
            id: id("drums"),
            fader,
            members: Vec::new(),
        })
    );
    assert_eq!(parse(&["not a stable id"]), Err(DecodeError::InvalidTlv));
}

/// Every live-value edit, with the ramp length set to `ramp`. Decision 17 (#1561) left three:
/// `SetConsole` and `SetTrackConsole` (structural, they keep their ramp field) and `SetControl`.
fn live_value_edits_with_ramp(ramp: Option<u32>) -> Vec<SessionEdit> {
    let session = parse_session_json(include_str!(
        "../../../../fixtures/session/v1/canonical.json"
    ))
    .expect("fixture");
    let mut edits = vec![
        SessionEdit::SetConsole {
            console: session.console.clone(),
            ramp_samples: None,
        },
        SessionEdit::SetTrackConsole {
            track_id: session.tracks[0].id.clone(),
            console: session.tracks[0].console.clone(),
            ramp_samples: None,
        },
        control_edit(
            controls::Owner::Strip(session.tracks[0].id.clone()),
            controls::Section::Builtins,
            -3.0,
            None,
        ),
    ];
    for edit in &mut edits {
        match edit {
            SessionEdit::SetConsole { ramp_samples, .. }
            | SessionEdit::SetTrackConsole { ramp_samples, .. }
            | SessionEdit::SetControl { ramp_samples, .. } => *ramp_samples = ramp,
            other => panic!("{other:?} has no ramp_samples field"),
        }
    }
    edits
}

/// #1394 gate 1: each of the three live-value edits round-trips with `ramp_samples` absent, an
/// explicit 0 and 96000 (one second at the highest launch rate), and the three encodings differ.
///
/// Red if the encoder and the decoder disagree on the optional field (a wrong field ID or wire
/// type, a dropped field count), or if an absent field decodes as 0: an explicit 0 steps the edit
/// and an absent one means the session default, so they must stay different edits.
#[test]
fn live_value_edits_round_trip_their_optional_ramp_length() {
    let codec = ProtocolCodec::default();
    let mut encodings = Vec::new();
    for ramp in [None, Some(0), Some(96_000)] {
        let edits = live_value_edits_with_ramp(ramp);
        assert_eq!(edits.len(), 3, "every live-value opcode is in the corpus");
        assert!(edits.iter().all(|edit| edit.ramp_samples() == ramp));
        let bytes = encode(&edits);
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("live-value edits decode");
        assert_eq!(decoded.edits, edits, "ramp {ramp:?}");
        assert!(decoded.edits.iter().all(|edit| edit.ramp_samples() == ramp));
        assert_eq!(encode(&decoded.edits), bytes, "ramp {ramp:?} re-encodes");
        encodings.push(bytes);
    }
    assert!(
        encodings[0] != encodings[1] && encodings[1] != encodings[2],
        "absent, 0 and 96000 are three different encodings"
    );
}

fn control_edit(
    owner: controls::Owner,
    section: controls::Section,
    value: f32,
    ramp_samples: Option<u32>,
) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: controls::ControlAddress {
            owner,
            section,
            control: 3,
            lane: ParameterChannel::Right,
        },
        value,
        ramp_samples,
    }
}

/// #1553 gate 5: `SetControl` round-trips for every owner and section shape, with the ramp absent,
/// an explicit 0 and 96000, and with a non-finite value, which the codec carries to the validator.
/// The three ramp encodings differ.
///
/// Red if the encoder and decoder disagree on the address (a dropped owner or slot ID, a wrong
/// code), if an absent ramp decodes as 0, or if the codec refuses a NaN that the validator must
/// refuse by name.
#[test]
fn set_control_round_trips_every_address_shape_and_ramp() {
    use controls::{Owner, Section};
    let codec = ProtocolCodec::default();
    let shapes = [
        (Owner::Session, Section::Own),
        (Owner::Strip(id("vocal")), Section::Builtins),
        (Owner::Strip(id("vocal")), Section::Own),
        (Owner::Strip(id("vocal")), Section::Inserts(id("eq"))),
        (Owner::Strip(id("vocal")), Section::Console(id("desk-eq"))),
        (Owner::Route(id("to-main")), Section::Own),
        (Owner::Vca(id("grp")), Section::Own),
    ];
    let mut encodings = Vec::new();
    for ramp in [None, Some(0), Some(96_000)] {
        let edits: Vec<_> = shapes
            .iter()
            .cloned()
            .map(|(owner, section)| control_edit(owner, section, -1.5, ramp))
            .collect();
        let bytes = encode(&edits);
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 128]))
            .expect("SetControl decodes");
        assert_eq!(decoded.edits, edits, "ramp {ramp:?}");
        assert_eq!(encode(&decoded.edits), bytes, "ramp {ramp:?} re-encodes");
        encodings.push(bytes);
    }
    assert!(encodings[0] != encodings[1] && encodings[1] != encodings[2]);
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let edits = [control_edit(
            controls::Owner::Session,
            controls::Section::Own,
            value,
            None,
        )];
        let bytes = encode(&edits);
        let decoded = codec
            .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
            .expect("a non-finite value decodes");
        let SessionEdit::SetControl { value: got, .. } = &decoded.edits[0] else {
            panic!("a SetControl");
        };
        assert_eq!(got.to_bits(), value.to_bits());
    }
}

/// #1553 D2: the address of a set requires its `control` and `lane`, and breaks the list's rules
/// when the owner ID or the slot ID is present or absent against its kind or section: each is
/// refused at decode, and an unallocated owner, section or lane code is `InvalidField` too: the
/// list's one address decoder answers both messages.
///
/// Red if a set decodes an address that names no control, a lane-less address, a strip without an
/// ID or a builtins section with a slot.
#[test]
fn set_control_refuses_an_address_that_breaks_its_rules() {
    use schema::control_address as a;
    let field =
        |spec: FieldSpec, value: &[u8]| (spec.id, spec.wire.raw(), spec.mandatory, value.to_vec());
    let parse = |kind: u8,
                 owner: Option<&str>,
                 section: u8,
                 slot: Option<&str>,
                 control: Option<u32>,
                 lane: Option<u8>| {
        let mut fields = vec![field(a::OWNER_KIND, &[kind])];
        if let Some(owner) = owner {
            fields.push(field(a::OWNER_ID, owner.as_bytes()));
        }
        fields.push(field(a::SECTION, &[section]));
        if let Some(slot) = slot {
            fields.push(field(a::SLOT_ID, slot.as_bytes()));
        }
        if let Some(control) = control {
            fields.push(field(a::CONTROL, &control.to_le_bytes()));
        }
        if let Some(lane) = lane {
            fields.push(field(a::LANE, &[lane]));
        }
        parse_control_address(Message::nested(&raw_message(fields)).expect("nested"))
    };
    // Strip, builtins, control 5, left: the accepted shape.
    assert!(parse(2, Some("vocal"), 4, None, Some(5), Some(1)).is_ok());
    assert!(parse(1, None, 6, None, Some(1), Some(3)).is_ok());
    assert!(parse(2, Some("vocal"), 2, Some("eq"), Some(0), Some(3)).is_ok());
    for (name, result) in [
        (
            "no control",
            parse(2, Some("vocal"), 4, None, None, Some(1)),
        ),
        ("no lane", parse(2, Some("vocal"), 4, None, Some(5), None)),
        (
            "strip without an ID",
            parse(2, None, 4, None, Some(5), Some(1)),
        ),
        (
            "session with an ID",
            parse(1, Some("vocal"), 6, None, Some(1), Some(3)),
        ),
        (
            "builtins with a slot",
            parse(2, Some("vocal"), 4, Some("eq"), Some(5), Some(1)),
        ),
        (
            "inserts without a slot",
            parse(2, Some("vocal"), 2, None, Some(0), Some(1)),
        ),
    ] {
        assert_eq!(result, Err(DecodeError::InvalidField), "{name}");
    }
    for (name, result) in [
        ("owner kind 0", parse(0, None, 6, None, Some(1), Some(1))),
        ("owner kind 5", parse(5, None, 6, None, Some(1), Some(1))),
        (
            "section 1 (retired)",
            parse(2, Some("vocal"), 1, None, Some(5), Some(1)),
        ),
        (
            "section 7",
            parse(2, Some("vocal"), 7, None, Some(5), Some(1)),
        ),
        ("lane 0", parse(2, Some("vocal"), 4, None, Some(5), Some(0))),
        ("lane 4", parse(2, Some("vocal"), 4, None, Some(5), Some(4))),
    ] {
        assert_eq!(result, Err(DecodeError::InvalidField), "{name}");
    }
}

/// #1584 D1, gate 5: `Observe` round-trips with a console slot and an insert, armed and disarmed,
/// window `0` and the maximum; its address is the control list's address message with a strip
/// owner, a console or inserts section and its slot ID, and each other shape is refused at decode.
///
/// Red if the codec accepts an address the op cannot use (a `control` or `lane` field, a route,
/// VCA or session owner, a builtins or own section, a missing ID), or if a field is dropped or
/// swapped on the way through.
#[test]
fn observe_round_trips_and_refuses_an_address_it_cannot_use() {
    let edits = vec![
        SessionEdit::Observe {
            strip_id: id("vocal"),
            section: controls::TapSection::Console(id("glue")),
            tap_id: 1,
            armed: true,
            window_blocks: 0,
        },
        SessionEdit::Observe {
            strip_id: id("drums"),
            section: controls::TapSection::Inserts(id("comp")),
            tap_id: 7,
            armed: false,
            window_blocks: controls::MAXIMUM_OBSERVATION_WINDOW_BLOCKS,
        },
    ];
    let bytes = encode(&edits);
    let decoded = ProtocolCodec::default()
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 128]))
        .expect("Observe decodes");
    assert_eq!(decoded.edits, edits);

    use schema::control_address as a;
    let field =
        |spec: FieldSpec, value: &[u8]| (spec.id, spec.wire.raw(), spec.mandatory, value.to_vec());
    let parse = |kind: u8,
                 owner: Option<&str>,
                 section: u8,
                 slot: Option<&str>,
                 control: Option<u32>,
                 lane: Option<u8>| {
        let mut fields = vec![field(a::OWNER_KIND, &[kind])];
        if let Some(owner) = owner {
            fields.push(field(a::OWNER_ID, owner.as_bytes()));
        }
        fields.push(field(a::SECTION, &[section]));
        if let Some(slot) = slot {
            fields.push(field(a::SLOT_ID, slot.as_bytes()));
        }
        if let Some(control) = control {
            fields.push(field(a::CONTROL, &control.to_le_bytes()));
        }
        if let Some(lane) = lane {
            fields.push(field(a::LANE, &[lane]));
        }
        parse_observe_address(Message::nested(&raw_message(fields)).expect("nested"))
    };
    assert_eq!(
        parse(2, Some("vocal"), 2, Some("comp"), None, None),
        Ok((id("vocal"), controls::TapSection::Inserts(id("comp"))))
    );
    for (name, result) in [
        (
            "a control",
            parse(2, Some("vocal"), 2, Some("comp"), Some(1), None),
        ),
        (
            "a lane",
            parse(2, Some("vocal"), 2, Some("comp"), None, Some(3)),
        ),
        (
            "a route owner",
            parse(3, Some("send"), 2, Some("comp"), None, None),
        ),
        (
            "a VCA owner",
            parse(4, Some("grp"), 2, Some("comp"), None, None),
        ),
        ("the session", parse(1, None, 2, Some("comp"), None, None)),
        ("builtins", parse(2, Some("vocal"), 4, None, None, None)),
        ("own", parse(2, Some("vocal"), 6, None, None, None)),
        ("no strip ID", parse(2, None, 2, Some("comp"), None, None)),
        ("no slot ID", parse(2, Some("vocal"), 5, None, None, None)),
    ] {
        assert_eq!(result, Err(DecodeError::InvalidField), "{name}");
    }
}

/// #1678: the expected effect of a `SET_CONTROL` edit is the seventh field of its address message,
/// accepted only in an `inserts` or `console` section and only as a stable ID; the encoder refuses
/// the same shapes. (A stored automation target refuses the field in every section:
/// `an_automation_target_carries_no_expected_effect`.)
///
/// Red if an expected effect on a builtin row is skipped at decode (the edit would be acknowledged
/// with a check that never ran), if a malformed ID reaches the core, or if the encoder writes a
/// field the decoder refuses.
#[test]
fn set_control_expected_effect_is_held_to_its_section() {
    use schema::set_control_address as a;
    let field =
        |spec: FieldSpec, value: &[u8]| (spec.id, spec.wire.raw(), spec.mandatory, value.to_vec());
    let parse = |section: u8, slot: Option<&str>, expected: Option<&str>| {
        let mut fields = vec![
            field(a::OWNER_KIND, &[2]),
            field(a::OWNER_ID, b"vocal"),
            field(a::SECTION, &[section]),
        ];
        if let Some(slot) = slot {
            fields.push(field(a::SLOT_ID, slot.as_bytes()));
        }
        fields.push(field(a::CONTROL, &5_u32.to_le_bytes()));
        fields.push(field(a::LANE, &[1]));
        if let Some(expected) = expected {
            fields.push(field(a::EXPECTED_EFFECT, expected.as_bytes()));
        }
        parse_control_address(Message::nested(&raw_message(fields)).expect("nested"))
    };
    for section in [2, 5] {
        let (address, expected) = parse(section, Some("eq"), Some("miso.compressor"))
            .expect("an effect section carries an expected effect");
        assert_eq!(expected, Some(id("miso.compressor")));
        assert!(matches!(
            address.section,
            controls::Section::Inserts(_) | controls::Section::Console(_)
        ));
        assert_eq!(parse(section, Some("eq"), None).map(|(_, e)| e), Ok(None));
        assert_eq!(
            parse(section, Some("eq"), Some("Not A Stable Id")),
            Err(DecodeError::InvalidField)
        );
        assert_eq!(
            parse(section, Some("eq"), Some("")),
            Err(DecodeError::InvalidField)
        );
    }
    assert!(parse(4, None, None).is_ok() && parse(6, None, None).is_ok());
    assert_eq!(
        parse(4, None, Some("miso.compressor")),
        Err(DecodeError::InvalidField),
        "builtins run no effect"
    );
    assert_eq!(
        parse(6, None, Some("miso.compressor")),
        Err(DecodeError::InvalidField),
        "a strip's own rows run no effect"
    );
    // The encoder refuses what the decoder refuses.
    let builtins = controls::ControlAddress {
        owner: controls::Owner::Strip(id("vocal")),
        section: controls::Section::Builtins,
        control: 5,
        lane: ParameterChannel::Left,
    };
    let edits = [SessionEdit::set_control(builtins, 0.0, None).with_expected_effect(id("a"))];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("request ID"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(0)),
        edits: &edits,
    };
    assert_eq!(
        ProtocolCodec::default().encoded_session_transaction_len(&transaction),
        Err(EncodeError::InvalidField)
    );
}

/// #1566 D4: a stored automation target is the control list's address message with `control` and
/// `lane` present, minus the session owner. No session row is automatable, so a session address has
/// no spelling as a target: the decoder refuses it and the encoder refuses to write one, while the
/// same address is valid for a `SET_CONTROL` write.
///
/// Red if the target codec accepts what the control list accepts (a session-owned row).
/// A stored automation target holds no expected effect: the `SET_CONTROL` address's seventh field is
/// refused in a target in every section, as `SET_CONTROL` refuses it outside an effect section
/// (`InvalidField` in the optional flag form, `InvalidTlv` in the mandatory one).
///
/// Red if the target decoder skips the field as an unknown optional field, so a target written
/// with an effect check that never runs would be stored as if it had none.
#[test]
fn an_automation_target_carries_no_expected_effect() {
    use schema::set_control_address as a;
    let field =
        |spec: FieldSpec, value: &[u8]| (spec.id, spec.wire.raw(), spec.mandatory, value.to_vec());
    let target = |section: u8, slot: Option<&str>, expected: Option<bool>| {
        let mut fields = vec![
            field(a::OWNER_KIND, &[2]),
            field(a::OWNER_ID, b"vocal"),
            field(a::SECTION, &[section]),
        ];
        if let Some(slot) = slot {
            fields.push(field(a::SLOT_ID, slot.as_bytes()));
        }
        fields.push(field(a::CONTROL, &5_u32.to_le_bytes()));
        fields.push(field(a::LANE, &[1]));
        if let Some(mandatory) = expected {
            fields.push((
                a::EXPECTED_EFFECT.id,
                a::EXPECTED_EFFECT.wire.raw(),
                mandatory,
                b"miso.compressor".to_vec(),
            ));
        }
        let bytes = raw_message(fields);
        parse_automation_target(Message::nested(&bytes).expect("nested"))
    };
    for (section, slot) in [(2, Some("eq")), (5, Some("eq")), (4, None), (6, None)] {
        assert!(target(section, slot, None).is_ok(), "section {section}");
        assert_eq!(
            target(section, slot, Some(false)),
            Err(DecodeError::InvalidField),
            "section {section}, optional form"
        );
        assert_eq!(
            target(section, slot, Some(true)),
            Err(DecodeError::InvalidTlv),
            "section {section}, mandatory form"
        );
    }
}

#[test]
fn an_automation_target_cannot_be_owned_by_the_session() {
    use schema::control_address as a;
    let field =
        |spec: FieldSpec, value: &[u8]| (spec.id, spec.wire.raw(), spec.mandatory, value.to_vec());
    let session_address = raw_message(vec![
        field(a::OWNER_KIND, &[1]),
        field(a::SECTION, &[6]),
        field(a::CONTROL, &2_u32.to_le_bytes()),
        field(a::LANE, &[3]),
    ]);
    assert!(
        parse_control_address(Message::nested(&session_address).expect("nested")).is_ok(),
        "a SET_CONTROL write may name a session row"
    );
    assert_eq!(
        parse_automation_target(Message::nested(&session_address).expect("nested")),
        Err(DecodeError::InvalidField)
    );
    let strip_address = raw_message(vec![
        field(a::OWNER_KIND, &[2]),
        field(a::OWNER_ID, b"vocal"),
        field(a::SECTION, &[4]),
        field(a::CONTROL, &5_u32.to_le_bytes()),
        field(a::LANE, &[1]),
    ]);
    assert_eq!(
        parse_automation_target(Message::nested(&strip_address).expect("nested")),
        Ok(ControlAddress {
            owner: Owner::Strip(id("vocal")),
            section: Section::Builtins,
            control: 5,
            lane: ParameterChannel::Left,
        })
    );

    let edits = [SessionEdit::SetAutomationTarget {
        automation_id: id("eq-gain"),
        target: ControlAddress {
            owner: Owner::Session,
            section: Section::Own,
            control: 2,
            lane: ParameterChannel::Both,
        },
    }];
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(9).expect("request"),
        expected_revision: ExpectedRevision::Exact(crate::SessionRevision(7)),
        edits: &edits,
    };
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0; 4096];
    assert_eq!(
        codec.encode_session_transaction(&transaction, &mut bytes),
        Err(EncodeError::InvalidField)
    );
}

/// #1650 D1: the automation offset is a required nested message of two required finite `F32`
/// lanes, field 1 left and field 2 right.
///
/// Red if a lane becomes optional, if a non-finite offset decodes, or if the decoder swaps the
/// two lanes.
#[test]
fn automation_offset_is_two_required_finite_lanes() {
    let lane = |value: f32| value.to_le_bytes().to_vec();
    let decode = |fields: Vec<(u16, u8, bool, Vec<u8>)>| {
        let bytes = raw_message(fields);
        parse_automation_offset(Message::nested(&bytes).expect("nested offset"))
    };
    assert_eq!(
        decode(vec![
            (1, WIRE_F32, true, lane(-1.5)),
            (2, WIRE_F32, true, lane(3.0)),
        ]),
        Ok(AutomationOffset {
            left_db: -1.5,
            right_db: 3.0,
        })
    );
    assert!(
        decode(vec![(1, WIRE_F32, true, lane(0.0))]).is_err(),
        "a missing right lane"
    );
    assert!(
        decode(vec![(2, WIRE_F32, true, lane(0.0))]).is_err(),
        "a missing left lane"
    );
    assert!(
        decode(vec![
            (1, WIRE_F32, true, lane(f32::NAN)),
            (2, WIRE_F32, true, lane(0.0)),
        ])
        .is_err(),
        "a non-finite lane"
    );
}
