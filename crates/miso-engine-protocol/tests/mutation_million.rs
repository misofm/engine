//! Deterministic complete-schema mutation coverage for all typed BTLV dispatch paths.

use miso_engine_protocol::{
    ConformanceDecoder, DecodeError, DecodeScratch, ProtocolCodec, ProtocolLimits,
    complete_schema_corpus,
};

const MUTATION_RUNS: usize = 1_000_000;
const MAX_FRAME_BYTES: usize = 65_536;

#[test]
fn one_million_deterministic_mutations_cover_complete_schema_closed_dispatch() {
    let codec = ProtocolCodec::new(ProtocolLimits {
        max_frame_bytes: MAX_FRAME_BYTES,
        max_tlv_count: 1024,
        max_string_bytes: 4096,
        max_nesting: 4,
    });
    let seeds = complete_schema_corpus();
    assert_eq!(
        seeds.len(),
        48,
        "12 commands (including nudge and all-opcode transaction) + 12 success + 18 non-OK + 6 events"
    );
    assert!(seeds.iter().any(|seed| {
        seed.name == "command.session_transaction_apply"
            && seed.decoder == ConformanceDecoder::Command
    }));
    let mut state = 0x4d49_534f_4354_4c05_u64;
    let mut frame = vec![0_u8; MAX_FRAME_BYTES + 32];
    for index in 0..MUTATION_RUNS {
        let seed = &seeds[index % seeds.len()];
        frame[..seed.bytes.len()].copy_from_slice(&seed.bytes);
        state = next(state);
        let length = (state as usize) % (seed.bytes.len() + 33);
        if length > seed.bytes.len() {
            for byte in &mut frame[seed.bytes.len()..length] {
                state = next(state);
                *byte = state as u8;
            }
        }
        for _ in 0..((state as usize & 3) + 1) {
            state = next(state);
            let byte = (state as usize) % length.max(1);
            if byte < length {
                frame[byte] ^= (state >> 24) as u8 | 1;
            }
        }
        let input = &frame[..length];
        let first = classify(&codec, seed.decoder, input);
        let second = classify(&codec, seed.decoder, input);
        assert_eq!(
            first, second,
            "{} mutation {index} changed decode class",
            seed.name
        );
        if input.len() > MAX_FRAME_BYTES {
            assert_eq!(first, Err(DecodeError::LimitExceeded));
        }
    }
}

fn classify(
    codec: &ProtocolCodec,
    decoder: ConformanceDecoder,
    input: &[u8],
) -> Result<(), DecodeError> {
    let mut fields = [0_u16; 1024];
    let scratch = &mut DecodeScratch::new(&mut fields);
    match decoder {
        ConformanceDecoder::Command => codec.decode_typed_command(input, scratch).map(|_| ()),
        ConformanceDecoder::Response => codec.decode_typed_response(input, scratch).map(|_| ()),
        ConformanceDecoder::Event => codec.decode_typed_event(input, scratch).map(|_| ()),
        ConformanceDecoder::Transaction => {
            codec.decode_session_transaction(input, scratch).map(|_| ())
        }
    }
}

const fn next(value: u64) -> u64 {
    value
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}
