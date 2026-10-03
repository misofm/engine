//! Issue #1241: VCAs are edited through session transactions, `0700` (`UpsertVca`), `0701`
//! (`RemoveVca`) and `0702` (`SetVcaFader`).
//!
//! The base session has tracks `a` and `x`, submix `bus` (fed by `a`, feeding output `main-out`),
//! and one VCA, `grp`, over `a` and `x`. Every transaction travels through the wire codec first,
//! so each assertion is about what a peer's frame does to the committed snapshot.

use protocol::{
    DecodeScratch, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionEditError,
    SessionRevision, SessionStore, SessionStoreError, SessionTransactionFrame,
};
use session::{
    ChannelBuiltins, CompileCaps, DualMonoBuiltins, DualMonoFader, MatrixOrPan, SessionModel,
    StableId, Vca, canonical_session_json, parse_session_json,
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
/// members. Then `0702` sets the nested VCA's fader (the parent and `grp` keep theirs), `0701`
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
    commit(
        &mut store,
        &[SessionEdit::SetVcaFader {
            vca_id: id("nested"),
            fader: moved.clone(),
        }],
    );
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

fn refusal_codes(error: &SessionStoreError, operations: usize) -> Vec<(String, String)> {
    let SessionStoreError::Validation {
        operation_index,
        diagnostics,
    } = error
    else {
        panic!("expected a final-validation refusal, got {error:?}");
    };
    assert_eq!(*operation_index, operations);
    diagnostics
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code.as_str().to_owned(),
                diagnostic.path.to_string(),
            )
        })
        .collect()
}

enum Refusal {
    /// An edit-resolution refusal at this operation index.
    Edit(usize, SessionEditError),
    /// A final-validation refusal carrying exactly these `(code, path)` diagnostics.
    Validation(&'static [(&'static str, &'static str)]),
}

/// Gate 2, the refusals. Each transaction opens with an edit that commits on its own (a `0702` on
/// `grp`), then:
///
/// - `0701` and `0702` at an unknown ID, and the strip edits `0203`, `020f`, `0210` and `0211` at
///   the VCA ID `grp`, are `session.edit.not_found`;
/// - removing member track `x` or member submix `bus` without rewriting `grp`, or a nested VCA its
///   parent still lists, is `reference.missing_entity` at the member path; a `0700` that closes a
///   cycle is `vca.cycle` on both VCAs; a `0700` reusing a track's ID is `id.duplicate`;
/// - a `0702` past the fader domain (+30 dB) is `numeric.out_of_schema_range` at its lane;
///
/// and the revision and snapshot are unchanged after each. Removing `x` while the same
/// transaction rewrites `grp` without it commits.
///
/// Red if a strip edit reaches a VCA, if `0701`/`0702` at an unknown ID is skipped silently, if
/// `0701`, `RemoveTrack` or `RemoveSubmix` cascades into the member lists that name it, if `0702`
/// clamps an out-of-domain offset instead of refusing it, or if a VCA edit commits without the
/// transaction's final validation (a dangling member, a cycle or a collision committed, or the
/// opening edit committed alone).
#[test]
fn vca_refusals_commit_nothing() {
    let opening = || SessionEdit::SetVcaFader {
        vca_id: id("grp"),
        fader: fader(-6.0, -6.0, false, false),
    };
    let unity = || fader(0.0, 0.0, false, false);
    let cases: Vec<(&str, Vec<SessionEdit>, Refusal)> = vec![
        (
            "0701 at an unknown ID",
            vec![SessionEdit::RemoveVca {
                vca_id: id("ghost"),
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "0702 at an unknown ID",
            vec![SessionEdit::SetVcaFader {
                vca_id: id("ghost"),
                fader: unity(),
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "0203 at a VCA ID",
            vec![SessionEdit::SetTrackBuiltins {
                track_id: id("grp"),
                builtins: DualMonoBuiltins {
                    left: ChannelBuiltins {
                        polarity_invert: true,
                        trim_db: 0.0,
                        hpf_hz: 20.0,
                        lpf_hz: 20_000.0,
                        delay_samples: 0,
                    },
                    right: ChannelBuiltins {
                        polarity_invert: false,
                        trim_db: 0.0,
                        hpf_hz: 20.0,
                        lpf_hz: 20_000.0,
                        delay_samples: 0,
                    },
                },
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "020f at a VCA ID",
            vec![SessionEdit::SetTrackFader {
                track_id: id("grp"),
                fader: fader(-12.0, -12.0, true, true),
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "0210 at a VCA ID",
            vec![SessionEdit::SetTrackMatrixOrPan {
                track_id: id("grp"),
                matrix_or_pan: MatrixOrPan::Pan {
                    left: 0.5,
                    right: 0.5,
                    smoothing_samples: 16,
                },
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "0211 at a VCA ID",
            vec![SessionEdit::SetTrackConsole {
                track_id: id("grp"),
                console: Vec::new(),
            }],
            Refusal::Edit(1, SessionEditError::NotFound),
        ),
        (
            "remove member track x without rewriting grp",
            vec![SessionEdit::RemoveTrack { track_id: id("x") }],
            Refusal::Validation(&[("reference.missing_entity", "$.vcas[0].members[1]")]),
        ),
        (
            "0701 removing a VCA its parent still lists",
            vec![
                SessionEdit::UpsertVca {
                    vca: vca("inner", unity(), &["a"]),
                },
                SessionEdit::UpsertVca {
                    vca: vca("grp", unity(), &["a", "inner", "x"]),
                },
                SessionEdit::RemoveVca {
                    vca_id: id("inner"),
                },
            ],
            Refusal::Validation(&[("reference.missing_entity", "$.vcas[0].members[1]")]),
        ),
        (
            "0700 closing a cycle",
            vec![
                SessionEdit::UpsertVca {
                    vca: vca("inner", unity(), &["grp"]),
                },
                SessionEdit::UpsertVca {
                    vca: vca("grp", unity(), &["a", "inner"]),
                },
            ],
            Refusal::Validation(&[("vca.cycle", "$.vcas[0]"), ("vca.cycle", "$.vcas[1]")]),
        ),
        (
            "0700 reusing a track ID",
            vec![SessionEdit::UpsertVca {
                vca: vca("a", unity(), &["x"]),
            }],
            Refusal::Validation(&[("id.duplicate", "$.vcas[1].id")]),
        ),
        (
            "remove member submix bus without rewriting grp",
            vec![
                SessionEdit::UpsertVca {
                    vca: vca("grp", unity(), &["a", "bus", "x"]),
                },
                SessionEdit::RemoveRoute {
                    route_id: id("a-bus"),
                },
                SessionEdit::RemoveRoute {
                    route_id: id("bus-out"),
                },
                SessionEdit::RemoveSubmix {
                    submix_id: id("bus"),
                },
            ],
            Refusal::Validation(&[("reference.missing_entity", "$.vcas[0].members[1]")]),
        ),
        (
            "0702 past the fader domain",
            vec![SessionEdit::SetVcaFader {
                vca_id: id("grp"),
                fader: fader(30.0, 0.0, false, false),
            }],
            Refusal::Validation(&[("numeric.out_of_schema_range", "$.vcas[0].fader.left_db")]),
        ),
    ];
    for (name, tail, refusal) in cases {
        let mut store = store();
        let revision = store.revision();
        let snapshot = store.canonical_snapshot().to_owned();
        let mut edits = vec![opening()];
        edits.extend(tail);
        let error = store
            .apply_transaction(ExpectedRevision::Exact(revision), &through_the_wire(&edits))
            .expect_err(name);
        match refusal {
            Refusal::Edit(operation_index, expected) => assert_eq!(
                error,
                SessionStoreError::Edit {
                    operation_index,
                    error: expected,
                },
                "{name}"
            ),
            Refusal::Validation(expected) => {
                let codes = refusal_codes(&error, edits.len());
                let expected: Vec<(String, String)> = expected
                    .iter()
                    .map(|(code, path)| ((*code).to_owned(), (*path).to_owned()))
                    .collect();
                assert_eq!(codes, expected, "{name}");
            }
        }
        assert_eq!(store.revision(), revision, "{name}");
        assert_eq!(store.canonical_snapshot(), snapshot, "{name}");
    }

    // The positive control: the same removal commits when the transaction rewrites `grp`.
    let mut store = store();
    let rewritten = vca("grp", fader(-6.0, -6.0, false, false), &["a"]);
    commit(
        &mut store,
        &[
            opening(),
            SessionEdit::RemoveTrack { track_id: id("x") },
            SessionEdit::UpsertVca {
                vca: rewritten.clone(),
            },
        ],
    );
    let mut expected = base();
    expected.tracks.retain(|track| track.id != id("x"));
    expected.vcas = vec![rewritten];
    assert_snapshot(&store, expected, "remove x and rewrite grp");
}
