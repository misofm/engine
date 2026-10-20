//! Issue #1241: VCAs are edited through session transactions, `0700` (`UpsertVca`) and `0701`
//! (`RemoveVca`); a VCA's fader and mute are `SET_CONTROL` rows (decision 17, #1561 retired the
//! `0702` `SetVcaFader`).
//!
//! The base session has tracks `a` and `x`, submix `bus` (fed by `a`, feeding output `main-out`),
//! and one VCA, `grp`, over `a` and `x`. Every transaction travels through the wire codec first,
//! so each assertion is about what a peer's frame does to the committed snapshot.

use protocol::control_core::{Lane, Owner, Section, row_address};
use protocol::{
    DecodeScratch, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    SessionStore, SessionTransactionFrame,
};
use session::{
    CompileCaps, DualMonoFader, SessionModel, StableId, Vca, canonical_session_json,
    parse_session_json,
};

const BUILTINS: &str = r#"{ "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 20.0, "lpf_hz": 20000.0, "delay_samples": 0 }"#;
const FADER: &str =
    r#"{ "left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false }"#;

fn base_document() -> String {
    let strip = format!(
        r#""builtins": {{ "left": {BUILTINS}, "right": {BUILTINS} }}, "console": [],
           "inserts": {{ "effects": [] }}, "fader": {FADER},
           "pan": {{ "left": 1.0, "right": 1.0, "smoothing_samples": 16 }}"#
    );
    let track = |id: &str| {
        format!(
            r#"{{ "id": "{id}", "source_id": "voice", "left_source_channel": 0, "right_source_channel": 1, {strip} }}"#
        )
    };
    format!(
        r#"{{ "schema_version": 1, "session_id": "vca.edits", "revision": "7",
          "sample_rate_hz": 48000, "quantum_frames": 128,
          "render_profile": {{ "id": "native", "mode": "single_thread" }},
          "output_profile": {{ "id": "main", "channels": 2, "sample_format": "f32_planar" }},
          "sources": [ {{ "id": "voice", "content": "blake3:2a97516c354b68848cdbd8f54a226a0a55b21ed138e207ad6c5cbb9c00aa5aea", "channels": 2, "bit_depth": "32f", "frames": "48000" }} ],
          "console": {{ "pre_insert": [], "post_insert": [] }},
          "tracks": [ {}, {} ],
          "submixes": [ {{ "id": "bus", {strip} }} ],
          "vcas": [ {{ "id": "grp", "fader": {FADER}, "members": [ "a", "x" ] }} ],
          "control_smoothing": {{ "kind": "default" }},
          "outputs": [ {{ "id": "main-out" }} ],
          "routes": [
            {{ "id": "a-bus", "source": {{ "kind": "track", "track_id": "a", "tap": "post_pan" }},
              "destination": {{ "kind": "submix_input", "submix_id": "bus" }},
              "channel_matrix": {{ "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 }}, "gain_db": 0.0, "mute": false, "follows_mute": false }},
            {{ "id": "bus-out", "source": {{ "kind": "submix", "submix_id": "bus", "tap": "post_pan" }},
              "destination": {{ "kind": "output_input", "output_id": "main-out" }},
              "channel_matrix": {{ "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 }}, "gain_db": 0.0, "mute": false, "follows_mute": false }} ],
          "automation": [] }}"#,
        track("a"),
        track("x"),
    )
}

fn base() -> SessionModel {
    parse_session_json(&base_document()).expect("the base document parses")
}

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

fn store() -> SessionStore {
    SessionStore::new(base(), caps()).expect("the base document compiles")
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

fn ids(values: &[&str]) -> Vec<StableId> {
    values.iter().copied().map(id).collect()
}

fn fader(left_db: f32, right_db: f32, left_mute: bool, right_mute: bool) -> DualMonoFader {
    DualMonoFader {
        left_db,
        right_db,
        left_mute,
        right_mute,
    }
}

/// A `SET_CONTROL` of the row `row` (`offset_db` or `mute`) of VCA `vca_id`, on `lane`.
fn vca_row(vca_id: &str, row: &str, lane: Lane, value: f32) -> SessionEdit {
    SessionEdit::set_control(
        row_address(Owner::Vca(id(vca_id)), Section::Own, row, lane).expect("a VCA row"),
        value,
        None,
    )
}

/// The four `SET_CONTROL`s that give VCA `vca_id` the fader `fader`.
fn set_fader(vca_id: &str, fader: &DualMonoFader) -> Vec<SessionEdit> {
    vec![
        vca_row(vca_id, "offset_db", Lane::Left, fader.left_db),
        vca_row(vca_id, "offset_db", Lane::Right, fader.right_db),
        vca_row(
            vca_id,
            "mute",
            Lane::Left,
            f32::from(u8::from(fader.left_mute)),
        ),
        vca_row(
            vca_id,
            "mute",
            Lane::Right,
            f32::from(u8::from(fader.right_mute)),
        ),
    ]
}

fn vca(vca_id: &str, fader: DualMonoFader, members: &[&str]) -> Vca {
    Vca {
        id: id(vca_id),
        fader,
        members: ids(members),
    }
}

/// Encode the edits as one wire transaction and decode them back.
fn through_the_wire(edits: &[SessionEdit]) -> Vec<SessionEdit> {
    let transaction = SessionTransactionFrame {
        request_id: RequestId::new(1).expect("request ID"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(7)),
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
    let decoded = codec
        .decode_session_transaction(&bytes, &mut DecodeScratch::new(&mut [0_u16; 64]))
        .expect("decode")
        .edits;
    assert_eq!(decoded, edits, "the wire carries every VCA edit unchanged");
    decoded
}

/// Commit one wire transaction at the store's current revision.
fn commit(store: &mut SessionStore, edits: &[SessionEdit]) {
    let revision = store.revision();
    let commit = store
        .apply_transaction(ExpectedRevision::Exact(revision), &through_the_wire(edits))
        .unwrap_or_else(|error| panic!("{edits:?}: {error:?}"));
    assert_eq!(commit.revision, SessionRevision(revision.0 + 1));
}

/// The committed snapshot is exactly the canonical JSON of `expected` (at the store's revision),
/// and it re-parses and re-canonicalizes to itself.
fn assert_snapshot(store: &SessionStore, mut expected: SessionModel, step: &str) {
    expected.revision = store.revision().0;
    let snapshot = store.canonical_snapshot();
    assert_eq!(
        snapshot,
        canonical_session_json(&expected).expect("canonical"),
        "{step}"
    );
    let reparsed = parse_session_json(snapshot).expect("the snapshot parses");
    assert_eq!(
        canonical_session_json(&reparsed).expect("canonical"),
        snapshot,
        "{step}: the snapshot round-trips"
    );
}

/// The VCAs exactly as the snapshot text spells them, in text order.
fn snapshot_vcas(store: &SessionStore) -> Vec<(String, Vec<String>)> {
    parse_session_json(store.canonical_snapshot())
        .expect("the snapshot parses")
        .vcas
        .iter()
        .map(|vca| {
            (
                vca.id.as_str().to_owned(),
                vca.members
                    .iter()
                    .map(|member| member.as_str().to_owned())
                    .collect(),
            )
        })
        .collect()
}

/// Gate 2, the commits. One transaction upserts a parent VCA that lists a nested VCA before it
/// upserts that nested VCA (`0700` twice; the parent is declared first, so the parent's member
/// resolves only on the final candidate), and the snapshot carries both, sorted by ID with sorted
/// members. Then `SET_CONTROL` sets the nested VCA's fader (the parent and `grp` keep theirs), `0701`
/// removes the parent, and `0701` removes the nested VCA, now a leaf no VCA lists. Each step's
/// snapshot is the canonical JSON of the model written directly, and round-trips.
///
/// Red if an apply arm edits the wrong entity (another VCA, or a strip), if `0700` appends a
/// duplicate instead of replacing, if a VCA edit validated per edit (the parent's forward
/// reference refuses), or if the snapshot drops or misorders a VCA.
#[test]
fn vca_edits_commit_and_snapshot_canonically() {
    let mut store = store();
    let mut expected = base();

    let parent = vca("parent", fader(-3.0, -2.0, false, false), &["x", "nested"]);
    let nested = vca("nested", fader(1.5, 0.5, false, true), &["bus", "a"]);
    commit(
        &mut store,
        &[
            SessionEdit::UpsertVca {
                vca: parent.clone(),
            },
            SessionEdit::UpsertVca {
                vca: nested.clone(),
            },
        ],
    );
    expected.vcas.extend([parent, nested.clone()]);
    assert_snapshot(&store, expected.clone(), "upsert parent and nested");
    assert_eq!(
        snapshot_vcas(&store),
        [
            ("grp".into(), vec!["a".into(), "x".into()]),
            ("nested".into(), vec!["a".into(), "bus".into()]),
            ("parent".into(), vec!["nested".into(), "x".into()]),
        ],
        "the snapshot carries every VCA sorted by ID, members sorted"
    );

    let moved = fader(-12.0, 6.0, true, false);
    commit(&mut store, &set_fader("nested", &moved));
    expected.vcas[2].fader = moved;
    assert_snapshot(&store, expected.clone(), "set the nested fader");

    // Re-upserting `grp` replaces it in place rather than adding a second `grp`.
    let regrouped = vca("grp", fader(-1.0, -1.0, true, true), &["bus"]);
    commit(
        &mut store,
        &[SessionEdit::UpsertVca {
            vca: regrouped.clone(),
        }],
    );
    expected.vcas[0] = regrouped;
    assert_snapshot(&store, expected.clone(), "replace grp");

    commit(
        &mut store,
        &[SessionEdit::RemoveVca {
            vca_id: id("parent"),
        }],
    );
    expected.vcas.remove(1);
    assert_snapshot(&store, expected.clone(), "remove the parent");

    commit(
        &mut store,
        &[SessionEdit::RemoveVca {
            vca_id: id("nested"),
        }],
    );
    expected.vcas.remove(1);
    assert_snapshot(&store, expected, "remove the leaf");
    assert_eq!(snapshot_vcas(&store).len(), 1);
}
