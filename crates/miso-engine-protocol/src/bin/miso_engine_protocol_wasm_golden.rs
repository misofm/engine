//! Scalar/SIMD Wasm executable conformance runner; it exports no control or C ABI.
//!
//! `main` is a wasm-interp entry point only, and it reports its verdict as a **returned value**
//! rather than as a panic. `wasm-interp` exits 0 even when the guest traps, so a gate that only
//! watched the process status could not fail (#274); `scripts/check-protocol-wasm-parity.sh`
//! invokes this export by name and reads the returned exit code out of the interpreter's own
//! printed result line.
//!
//! The digest itself is not re-stated here. It is `COMPLETE_SCHEMA_HASH`, the single pin the
//! native `conformance_corpus` test asserts too, so the two arms cannot drift apart by omission.

use miso_engine_protocol::{
    COMPLETE_SCHEMA_HASH, ConformanceDecoder, DecodeScratch, ProtocolCodec, complete_schema_corpus,
};
use std::process::ExitCode;

/// Returned when the Wasm-computed corpus digest is not the pinned one. Distinct from a trap,
/// which is what a decode refusal or a corpus-length change produces.
const DIGEST_MISMATCH: u8 = 1;

fn main() -> ExitCode {
    if complete_schema_hash() == COMPLETE_SCHEMA_HASH {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(DIGEST_MISMATCH)
    }
}

fn complete_schema_hash() -> u64 {
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
    hash
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
