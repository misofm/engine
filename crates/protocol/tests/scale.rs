//! Issue #1079: the protocol session store holds a session above the former 65,536-track boundary.
//!
//! Release-only nightly row: the store compiles 65,537 tracks twice (the base and the edit's
//! candidate) and writes their canonical snapshot, 88.8 MB each.

use protocol::control_core::{Lane, Owner, Section, row_address};
use protocol::{ExpectedRevision, SessionEdit, SessionRevision, SessionStore};
use session::{CompileCaps, SessionModel, StableId, parse_session_json};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// One past the former 65,536 boundary.
const TRACKS: u32 = 65_537;

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("generated stable ID")
}

/// The fixture's first track [`TRACKS`] times, with the fixture's route and automation moved onto
/// `track-0`.
fn scale_model() -> SessionModel {
    let mut model = parse_session_json(EXAMPLE).expect("fixture parses");
    let template = model.tracks[0].clone();
    model.tracks = (0..TRACKS)
        .map(|index| {
            let mut track = template.clone();
            track.id = id(&format!("track-{index}"));
            track
        })
        .collect();
    model.routes[0].source = session::RouteSource::Track {
        track_id: id("track-0"),
        tap: session::SendTap::PostPan,
    };
    model.automation[0].target.owner = session::Owner::Strip(id("track-0"));
    model
}

/// A fader `SET_CONTROL` addressed to the strip the store holds at index 65,536 (the last one: the
/// compiled model orders tracks by ID, so it is `track-9999`, not `track-65536`) commits, and its
/// value is the one the snapshot carries for that track and no other.
#[test]
#[ignore = "release-mode scale gate; runs nightly"]
fn a_fader_edit_on_the_strip_at_index_65536_commits_and_snapshots() {
    let mut store = SessionStore::new(scale_model(), caps()).expect("adequate caps compile");
    let last = store.compiled().normalized_model().tracks[TRACKS as usize - 1]
        .id
        .clone();
    assert_eq!(last.as_str(), "track-9999", "the compiled order is by ID");
    let revision = store.revision();
    let edit = SessionEdit::set_control(
        row_address(
            Owner::Strip(last.clone()),
            Section::Builtins,
            "fader_db",
            Lane::Left,
        )
        .expect("a strip fader row"),
        -7.5,
        None,
    );
    let commit = store
        .apply_transaction(ExpectedRevision::Exact(revision), &[edit])
        .unwrap_or_else(|error| panic!("the edit commits: {error:?}"));
    assert_eq!(commit.revision, SessionRevision(revision.0 + 1));

    let snapshot = parse_session_json(store.canonical_snapshot()).expect("the snapshot parses");
    assert_eq!(snapshot.tracks.len(), TRACKS as usize);
    let base = scale_model();
    let template = &base.tracks[0];
    let edited: Vec<_> = snapshot
        .tracks
        .iter()
        .filter(|track| track.fader != template.fader)
        .collect();
    assert_eq!(edited.len(), 1, "only the addressed track moved");
    assert_eq!(edited[0].id, last);
    assert_eq!(edited[0].fader.left_db.to_bits(), (-7.5_f32).to_bits());
}
