//! TBD30: the empty-boot leg of the browser qualification boots the live-control session with its
//! source, its track and its route taken out, then adds them back by one structural transaction.
//!
//! The SDK has no encoder for a structural edit, so the page sends the frame the protocol codec
//! wrote. This test writes the two files the page fetches beside the live-control session they
//! derive from: `qualification/empty-session.json` (that session, canonical, with no source, no
//! track and no route) and `qualification/empty-boot-frame.json` (the `SESSION_TRANSACTION_APPLY`
//! frame that adds the live-control session's source, track and route, request ID 1, expecting the
//! empty session's revision). Both are wire or document bytes, so a byte comparison is the right
//! gate: it turns red when the codec or the live-control session changes without them following.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the files after a deliberate change.

use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{canonical_session_json, parse_session_json};

const BASE: &str = "qualification/live-control-session.json";
const EMPTY: &str = "qualification/empty-session.json";
const FRAME: &str = "qualification/empty-boot-frame.json";

fn path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(name)
}

/// The two files' expected text.
fn render() -> (String, String) {
    let base = std::fs::read_to_string(path(BASE)).expect("the live-control session");
    let model = parse_session_json(&base).expect("the live-control session parses");
    let mut empty = model.clone();
    empty.sources.clear();
    empty.tracks.clear();
    empty.routes.clear();
    let document = canonical_session_json(&empty).expect("the empty session is canonical");
    // The committed document, parsed again, is what the page boots: its revision is the one the
    // frame expects.
    let revision = parse_session_json(&document).expect("parses").revision;

    let edits = vec![
        SessionEdit::UpsertSource {
            source: model.sources[0].clone(),
        },
        SessionEdit::UpsertTrack {
            track: model.tracks[0].clone(),
        },
        SessionEdit::UpsertRoute {
            route: model.routes[0].clone(),
        },
    ];
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(&edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    let hex = bytes.iter().fold(String::new(), |mut hex, byte| {
        write!(hex, "{byte:02x}").expect("write to a string");
        hex
    });
    let mut frame = String::new();
    writeln!(
        frame,
        "{{\n  \"schema\": \"miso.host-web.empty-boot-frame\",\n  \"revision\": \"{revision}\",\n  \"hex\": \"{hex}\"\n}}"
    )
    .expect("write to a string");
    (document, frame)
}

/// Red if the codec or the live-control session changes without the empty session and the frame
/// the browser qualification sends following.
#[test]
fn the_empty_boot_fixtures_are_what_the_codec_and_the_session_write() {
    let (document, frame) = render();
    for (name, expected) in [(EMPTY, document), (FRAME, frame)] {
        let file = path(name);
        if std::env::var_os("MISO_ENGINE_BLESS_FIXTURES").is_some() {
            std::fs::write(&file, &expected).expect("write the fixture");
            continue;
        }
        let actual = std::fs::read_to_string(&file).unwrap_or_default();
        assert!(
            actual == expected,
            "{name} differs from what is derived; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
        );
    }
}
