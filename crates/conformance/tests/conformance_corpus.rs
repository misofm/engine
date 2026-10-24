//! Frozen complete-schema byte and decoder conformance.

use conformance::{COMPLETE_SCHEMA_HASH, ConformanceDecoder, complete_schema_corpus};
use protocol::{DecodeScratch, ProtocolCodec};

#[test]
fn frozen_corpus_bytes_and_typed_decoders_are_unchanged() {
    let codec = ProtocolCodec::default();
    let corpus = complete_schema_corpus();
    assert_eq!(corpus.len(), 54);
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
    // #274: the literal moved into `COMPLETE_SCHEMA_HASH` so the Wasm golden runner asserts the
    // same pin instead of carrying its own copy, which is how it fell two re-pins behind.
    assert_eq!(hash, COMPLETE_SCHEMA_HASH);
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

/// The protocol's control code tables are the control vocabulary's own, one for one and in the
/// ruled order: every variant of the vocabulary enums maps to its protocol variant by an
/// exhaustive `match` (so a new variant fails to compile here until it is given a wire code), and
/// the codes are the ones decision 17 rules.
///
/// Red if a vocabulary enum gains a variant with no wire code, a code is mapped to another
/// variant, or the live-ramp rows stop running in declaration order from 1.
#[test]
fn control_wire_codes_follow_the_control_vocabulary() {
    use effect_contract::control as vocabulary;
    use protocol::{
        ControlClass, ControlFamily, ControlKind, ControlRampRow, ControlReason, ControlTaper,
        ParameterUnit,
    };

    let ramp_rows = [
        vocabulary::LiveRampRow::Fader,
        vocabulary::LiveRampRow::InputTrim,
        vocabulary::LiveRampRow::SendGain,
        vocabulary::LiveRampRow::VcaOffset,
        vocabulary::LiveRampRow::LinkGlide,
        vocabulary::LiveRampRow::Mute,
        vocabulary::LiveRampRow::Solo,
        vocabulary::LiveRampRow::PolarityInvert,
        vocabulary::LiveRampRow::SendMute,
        vocabulary::LiveRampRow::SendFollowsMute,
        vocabulary::LiveRampRow::VcaMute,
        vocabulary::LiveRampRow::BypassCrossfade,
        vocabulary::LiveRampRow::Pan,
        vocabulary::LiveRampRow::Matrix,
        vocabulary::LiveRampRow::SendMatrix,
    ];
    for (index, row) in ramp_rows.into_iter().enumerate() {
        let wire = match row {
            vocabulary::LiveRampRow::Fader => ControlRampRow::Fader,
            vocabulary::LiveRampRow::InputTrim => ControlRampRow::InputTrim,
            vocabulary::LiveRampRow::SendGain => ControlRampRow::SendGain,
            vocabulary::LiveRampRow::VcaOffset => ControlRampRow::VcaOffset,
            vocabulary::LiveRampRow::LinkGlide => ControlRampRow::LinkGlide,
            vocabulary::LiveRampRow::Mute => ControlRampRow::Mute,
            vocabulary::LiveRampRow::Solo => ControlRampRow::Solo,
            vocabulary::LiveRampRow::PolarityInvert => ControlRampRow::PolarityInvert,
            vocabulary::LiveRampRow::SendMute => ControlRampRow::SendMute,
            vocabulary::LiveRampRow::SendFollowsMute => ControlRampRow::SendFollowsMute,
            vocabulary::LiveRampRow::VcaMute => ControlRampRow::VcaMute,
            vocabulary::LiveRampRow::BypassCrossfade => ControlRampRow::BypassCrossfade,
            vocabulary::LiveRampRow::Pan => ControlRampRow::Pan,
            vocabulary::LiveRampRow::Matrix => ControlRampRow::Matrix,
            vocabulary::LiveRampRow::SendMatrix => ControlRampRow::SendMatrix,
        };
        assert_eq!(usize::from(wire.code()), index + 1, "{row:?}");
    }
    assert_eq!(ControlRampRow::CODES.len(), ramp_rows.len());

    for (kind, code) in [
        (vocabulary::ControlKind::Continuous, 1),
        (vocabulary::ControlKind::Toggle, 2),
        (vocabulary::ControlKind::Enumeration, 3),
    ] {
        let wire = match kind {
            vocabulary::ControlKind::Continuous => ControlKind::Continuous,
            vocabulary::ControlKind::Toggle => ControlKind::Toggle,
            vocabulary::ControlKind::Enumeration => ControlKind::Enumeration,
        };
        assert_eq!(wire.code(), code, "{kind:?}");
    }
    for (taper, code) in [
        (vocabulary::Taper::Linear, 1),
        (vocabulary::Taper::Logarithmic, 2),
        (vocabulary::Taper::Exponential, 3),
        (vocabulary::Taper::Decibel, 4),
        (vocabulary::Taper::Stepped, 5),
    ] {
        let wire = match taper {
            vocabulary::Taper::Linear => ControlTaper::Linear,
            vocabulary::Taper::Logarithmic => ControlTaper::Logarithmic,
            vocabulary::Taper::Exponential => ControlTaper::Exponential,
            vocabulary::Taper::Decibel => ControlTaper::Decibel,
            vocabulary::Taper::Stepped => ControlTaper::Stepped,
        };
        assert_eq!(wire.code(), code, "{taper:?}");
    }
    for (family, code) in [
        (vocabulary::Family::StripLevel, 1),
        (vocabulary::Family::StripMatrix, 2),
        (vocabulary::Family::StripInput, 3),
        (vocabulary::Family::Route, 4),
        (vocabulary::Family::Effect, 5),
        (vocabulary::Family::Setting, 6),
    ] {
        let wire = match family {
            vocabulary::Family::StripLevel => ControlFamily::StripLevel,
            vocabulary::Family::StripMatrix => ControlFamily::StripMatrix,
            vocabulary::Family::StripInput => ControlFamily::StripInput,
            vocabulary::Family::Route => ControlFamily::Route,
            vocabulary::Family::Effect => ControlFamily::Effect,
            vocabulary::Family::Setting => ControlFamily::Setting,
        };
        assert_eq!(wire.code(), code, "{family:?}");
    }
    for (reason, code) in [
        (vocabulary::RebuildReason::NewMemory, 1),
        (vocabulary::RebuildReason::NewGraph, 2),
        (vocabulary::RebuildReason::ProcessingOrder, 3),
        (vocabulary::RebuildReason::Latency, 4),
        (vocabulary::RebuildReason::Glitch, 5),
        (vocabulary::RebuildReason::Optimisation, 6),
        (vocabulary::RebuildReason::Correctness, 7),
        (vocabulary::RebuildReason::RuledLiveNotBuilt { issue: 1 }, 8),
    ] {
        let wire = match reason {
            vocabulary::RebuildReason::NewMemory => ControlReason::NewMemory,
            vocabulary::RebuildReason::NewGraph => ControlReason::NewGraph,
            vocabulary::RebuildReason::ProcessingOrder => ControlReason::ProcessingOrder,
            vocabulary::RebuildReason::Latency => ControlReason::Latency,
            vocabulary::RebuildReason::Glitch => ControlReason::Glitch,
            vocabulary::RebuildReason::Optimisation => ControlReason::Optimisation,
            vocabulary::RebuildReason::Correctness => ControlReason::Correctness,
            vocabulary::RebuildReason::RuledLiveNotBuilt { .. } => ControlReason::RuledLiveNotBuilt,
        };
        assert_eq!(wire.code(), code, "{reason:?}");
    }
    for (class, code) in [
        (vocabulary::ControlClass::Live, 1),
        (
            vocabulary::ControlClass::RebuildOnly {
                reason: vocabulary::RebuildReason::Latency,
                reopen: "x",
            },
            2,
        ),
        (vocabulary::ControlClass::Derived, 3),
    ] {
        let wire = match class {
            vocabulary::ControlClass::Live => ControlClass::Live,
            vocabulary::ControlClass::RebuildOnly { .. } => ControlClass::RebuildOnly,
            vocabulary::ControlClass::Derived => ControlClass::Derived,
        };
        assert_eq!(wire.code(), code, "{class:?}");
    }
    for (unit, wire) in [
        (effect_contract::ParameterUnit::Db, ParameterUnit::Db),
        (effect_contract::ParameterUnit::Hz, ParameterUnit::Hz),
        (
            effect_contract::ParameterUnit::Milliseconds,
            ParameterUnit::Milliseconds,
        ),
        (
            effect_contract::ParameterUnit::Samples,
            ParameterUnit::Samples,
        ),
        (
            effect_contract::ParameterUnit::Linear,
            ParameterUnit::Linear,
        ),
        (effect_contract::ParameterUnit::Ratio, ParameterUnit::Ratio),
    ] {
        match unit {
            effect_contract::ParameterUnit::Db
            | effect_contract::ParameterUnit::Hz
            | effect_contract::ParameterUnit::Milliseconds
            | effect_contract::ParameterUnit::Samples
            | effect_contract::ParameterUnit::Linear
            | effect_contract::ParameterUnit::Ratio => {}
        }
        assert_eq!(unit as u64, wire as u64, "{unit:?}");
    }
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
