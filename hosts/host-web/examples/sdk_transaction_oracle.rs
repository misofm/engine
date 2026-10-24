//! Decode, re-encode and apply the SDK's session-transaction frames natively (issue #1383 D4).
//!
//! The SDK's transaction builder (`sdk/src/core/session-transaction.ts`) writes
//! `SESSION_TRANSACTION_APPLY` frames for structural session edits. This oracle is the engine's
//! answer to each of them, so `sdk/test/transaction-evals.mjs` asserts the SDK against the codec
//! and the session store rather than against a committed digest:
//!
//! - the frame is decoded with `ProtocolCodec::decode_session_transaction` and re-encoded with
//!   `encode_session_transaction`: a frame that decodes to other edits re-encodes to other bytes;
//! - the decoded edits are applied with `SessionStore::apply_transaction` to the base session
//!   given as the argument, and the store's canonical snapshot is reported.
//!
//! The oracle holds no case list: every case lives in the eval, so a later slice adds cases in
//! JavaScript only. Frames are read from stdin back to back, each delimited by its own header's
//! payload length, and they apply in order to one store, so a later frame sees an earlier one's
//! commit. A refused frame leaves the store as it was.
//!
//! Usage: `sdk_transaction_oracle BASE_SESSION.json < frames`, printing one JSON line per frame:
//! `{"bytes": hex, "ok": snapshot}`, `{"bytes": hex, "refused": error}` or `{"decode": error}`.

use std::io::{Read as _, Write as _};

use protocol::{DecodeScratch, ProtocolCodec, SessionStore, SessionTransactionFrame};
use session::{CompileCaps, parse_session_json};

/// The outer command header's length and the offset of its `payload_len` word.
const HEADER_BYTES: usize = 48;
const PAYLOAD_LENGTH_OFFSET: usize = 20;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let base_path = arguments
        .next()
        .expect("usage: sdk_transaction_oracle BASE_SESSION.json < frames");
    assert!(arguments.next().is_none(), "too many arguments");

    let base = std::fs::read_to_string(&base_path).expect("read the base session document");
    let model = parse_session_json(&base).expect("the base session document parses");
    // No resource cap: the oracle judges encoding and semantics, not a host's budget.
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    let mut store = SessionStore::new(model, caps).expect("the base session compiles");

    let mut input = Vec::new();
    std::io::stdin()
        .read_to_end(&mut input)
        .expect("read the frames from stdin");

    let codec = ProtocolCodec::default();
    let mut field_ids = vec![0_u16; usize::from(u16::MAX)];
    let mut stdout = std::io::stdout().lock();
    let mut offset = 0;
    while offset < input.len() {
        let header = input
            .get(offset..offset + HEADER_BYTES)
            .expect("a frame ends inside its header");
        let mut word = [0_u8; 4];
        word.copy_from_slice(&header[PAYLOAD_LENGTH_OFFSET..PAYLOAD_LENGTH_OFFSET + 4]);
        let end = offset + HEADER_BYTES + u32::from_le_bytes(word) as usize;
        let frame = input
            .get(offset..end)
            .expect("a frame ends inside its payload");
        offset = end;

        let line = match codec.decode_session_transaction(frame, &mut DecodeScratch::new(&mut field_ids)) {
            Err(error) => serde_json::json!({ "decode": format!("{error:?}") }),
            Ok(decoded) => {
                let command = decoded
                    .frame
                    .header
                    .command()
                    .expect("a decoded transaction is a command");
                let transaction = SessionTransactionFrame {
                    request_id: command.request_id,
                    expected_revision: command.expected_revision,
                    edits: &decoded.edits,
                };
                let length = codec
                    .encoded_session_transaction_len(&transaction)
                    .expect("a decoded transaction re-encodes");
                let mut bytes = vec![0_u8; length];
                codec
                    .encode_session_transaction(&transaction, &mut bytes)
                    .expect("a decoded transaction re-encodes");
                match store.apply_transaction(command.expected_revision, &decoded.edits) {
                    Ok(_) => serde_json::json!({
                        "bytes": hex(&bytes),
                        "ok": store.canonical_snapshot(),
                    }),
                    Err(error) => serde_json::json!({
                        "bytes": hex(&bytes),
                        "refused": format!("{error:?}"),
                    }),
                }
            }
        };
        writeln!(stdout, "{line}").expect("write the oracle's answer");
    }
}
