//! Issue #1562b-K: the structural command frames the SDK's structure evals send.
//!
//! The SDK has no encoder for a structural edit (its transaction API is later work), so the evals
//! that follow a rebuild -- a strip added, an insert swapped for another effect, an insert removed --
//! send frames the protocol codec wrote. This test writes them beside the #1560 parity session
//! (`fixtures/control-list.json`, which the evals boot), as `fixtures/structural-frames.json`. A
//! frame is a wire format, so a byte comparison is the right gate: it turns red when the codec or
//! the parity session changes without the frames following.
//!
//! The frames carry request ID 1 and the boot revision; the evals set both words of the header
//! (bytes 24 and 32) before they send a frame, because the SDK's endpoint allocates the IDs.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate change.

use std::fmt::Write as _;
use std::path::PathBuf;

use protocol::{
    CommandPayload, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{RouteSource, SessionModel, StableId, canonical_session_json, parse_session_json};

mod common;
use common::model_at;

const FIXTURE: &str = "tests/fixtures/structural-frames.json";
const RATE: u32 = 48_000;

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

/// The booted model: the parse of the canonical parity document, as the evals boot it.
fn booted() -> SessionModel {
    let document = canonical_session_json(&model_at(RATE)).expect("canonical parity session");
    parse_session_json(&document).expect("the canonical document parses")
}

fn frame(edits: &[SessionEdit], revision: u64) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    bytes
}

/// `t1` with its `comp` insert changed by `change` (`None` removes it).
fn t1_with_comp(model: &SessionModel, change: Option<session::Effect>) -> SessionEdit {
    let mut track = model.tracks[0].clone();
    assert_eq!(track.id.as_str(), "t1");
    let at = track
        .inserts
        .effects
        .iter()
        .position(|effect| effect.id.as_str() == "comp")
        .expect("t1 has a comp insert");
    match change {
        Some(effect) => track.inserts.effects[at] = effect,
        None => {
            track.inserts.effects.remove(at);
        }
    }
    SessionEdit::UpsertTrack { track }
}

/// `t1` without its `eq` insert, so `comp` is the first insert.
fn t1_without_eq(model: &SessionModel) -> SessionEdit {
    let mut track = model.tracks[0].clone();
    assert_eq!(track.id.as_str(), "t1");
    track
        .inserts
        .effects
        .retain(|effect| effect.id.as_str() != "eq");
    assert_eq!(track.inserts.effects.len(), 1);
    SessionEdit::UpsertTrack { track }
}

/// `t3` with its first two inserts, `multi` and `limiter`, swapped. Both declare the same tap
/// (`Gain Reduction`, ID 1), so a record for one effect's index arms the other's tap when the
/// indices belong to another plan than the one the core admits it against (#1562b, batch 12b).
fn t3_with_multi_and_limiter_swapped(model: &SessionModel) -> SessionEdit {
    let mut track = model
        .tracks
        .iter()
        .find(|track| track.id.as_str() == "t3")
        .expect("the parity session has t3")
        .clone();
    assert_eq!(track.inserts.effects[0].id.as_str(), "multi");
    assert_eq!(track.inserts.effects[1].id.as_str(), "limiter");
    track.inserts.effects.swap(0, 1);
    SessionEdit::UpsertTrack { track }
}

fn frames() -> Vec<(&'static str, Vec<u8>)> {
    let model = booted();
    let revision = model.revision;
    // A muted copy of t1 that sorts before it, with its route to the output: every other strip
    // moves one place later in the strip order, and the mixed output keeps its bits.
    let mut added = model.tracks[0].clone();
    added.id = id("a0");
    added.fader.left_mute = true;
    added.fader.right_mute = true;
    let mut route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "t1-main")
        .expect("t1's route to the output")
        .clone();
    route.id = id("a0-main");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = added.id.clone();
    // The soft clip of t2 under the compressor's ID: the same insert now runs another effect,
    // which has no threshold row and no observation tap.
    let mut clip = model.tracks[1]
        .inserts
        .effects
        .iter()
        .find(|effect| effect.id.as_str() == "clip")
        .expect("t2 has a clip insert")
        .clone();
    clip.id = id("comp");
    // t3 replaced by a copy under another ID, with its route: the strip count stays three and the
    // strip order changes, which a count alone cannot show.
    let t3 = model
        .tracks
        .iter()
        .find(|track| track.id.as_str() == "t3")
        .expect("the parity session has t3");
    let mut replacement = t3.clone();
    replacement.id = id("t4");
    let t3_route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "t3-main")
        .expect("t3's route to the output");
    let mut replacement_route = t3_route.clone();
    replacement_route.id = id("t4-main");
    let RouteSource::Track { track_id, .. } = &mut replacement_route.source else {
        panic!("a track route")
    };
    *track_id = replacement.id.clone();
    vec![
        (
            "replace_last_strip",
            frame(
                &[
                    SessionEdit::RemoveRoute {
                        route_id: t3_route.id.clone(),
                    },
                    SessionEdit::RemoveTrack {
                        track_id: t3.id.clone(),
                    },
                    SessionEdit::UpsertTrack { track: replacement },
                    SessionEdit::UpsertRoute {
                        route: replacement_route,
                    },
                ],
                revision,
            ),
        ),
        (
            "add_first_strip",
            frame(
                &[
                    SessionEdit::UpsertTrack { track: added },
                    SessionEdit::UpsertRoute { route },
                ],
                revision,
            ),
        ),
        (
            "swap_comp_for_soft_clip",
            frame(&[t1_with_comp(&model, Some(clip))], revision),
        ),
        (
            "remove_comp",
            frame(&[t1_with_comp(&model, None)], revision),
        ),
        // The compressor stays and moves from the second insert to the first.
        ("remove_eq", frame(&[t1_without_eq(&model)], revision)),
        // `multi` and `limiter` change places: each moves to the other's index.
        (
            "swap_t3_multi_and_limiter",
            frame(&[t3_with_multi_and_limiter_swapped(&model)], revision),
        ),
        // t3's true-peak limiter removed: the natural latency falls, and #1285 keeps the old one
        // as floors, so a declared discontinuity has a successor to publish (B16-R9's SDK eval).
        (
            "remove_t3_limiter",
            frame(
                &[SessionEdit::RemoveTrackEffect {
                    track_id: id("t3"),
                    rack_name: session::RackName::Inserts,
                    effect_id: id("limiter"),
                }],
                revision,
            ),
        ),
    ]
}

fn render() -> String {
    let model = booted();
    let mut text = String::new();
    writeln!(text, "{{").expect("write to a string");
    writeln!(
        text,
        "  \"schema\": \"miso.host-web.structural-frames\",\n  \"revision\": \"{}\",\n  \"frames\": [",
        model.revision
    )
    .expect("write to a string");
    let frames = frames();
    for (index, (name, bytes)) in frames.iter().enumerate() {
        let hex = bytes.iter().fold(String::new(), |mut hex, byte| {
            write!(hex, "{byte:02x}").expect("write to a string");
            hex
        });
        let comma = if index + 1 == frames.len() { "" } else { "," };
        writeln!(
            text,
            "    {{ \"name\": \"{name}\", \"hex\": \"{hex}\" }}{comma}"
        )
        .expect("write to a string");
    }
    writeln!(text, "  ]\n}}").expect("write to a string");
    text
}

/// Red if the codec or the parity session changes without the frames the SDK evals send following.
#[test]
fn the_structural_frames_fixture_is_what_the_codec_writes() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE);
    let expected = render();
    if std::env::var_os("MISO_ENGINE_BLESS_FIXTURES").is_some() {
        std::fs::create_dir_all(path.parent().expect("fixture directory")).expect("mkdir");
        std::fs::write(&path, &expected).expect("write the fixture");
        return;
    }
    let actual = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        actual == expected,
        "{FIXTURE} differs from the codec's frames; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
    );
}
