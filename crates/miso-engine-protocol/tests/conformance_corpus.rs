//! Frozen complete-schema byte and decoder conformance.

use miso_engine_protocol::{
    ConformanceDecoder, DecodeScratch, ProtocolCodec, complete_schema_corpus,
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
    assert_eq!(hash, 0x88a8_ee6a_6d9e_4acc);
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
