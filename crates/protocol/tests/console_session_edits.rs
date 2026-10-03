//! Issue #1094 (decision 12): the control protocol's session edits carry the session console.
//!
//! - `SetConsole` (`0x0007`) replaces the session's console declaration and `SetTrackConsole`
//!   (`0x0211`) replaces one track's entries.
//! - `SetEffectBypass`, `UpsertEffectParam` and `RemoveEffectParam` addressed at `console` edit one
//!   track's entry for the slot named by `effect_id`.
//! - Every structural or declaration edit addressed at `console` is refused with
//!   `SessionEditError::ConsoleSlotFixed`: a track cannot add, remove or reorder a slot.
//! - A transaction that changes the slot set without rewriting every strip's entries -- every
//!   track's and, since #1202, every submix's -- refuses whole and commits nothing.
//!
//! The base session has two tracks and one `pre_insert` slot, `eq`. Track `a` also carries an
//! insert whose ID is `eq`, so an edit routed to the wrong place changes a value these tests read.

use protocol::{
    DecodeScratch, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionEditError,
    SessionRevision, SessionStore, SessionStoreError, SessionTransactionFrame, apply_session_edit,
};
use session::{
    CompileCaps, Console, ConsoleEntry, ConsoleSlot, EffectIdentity, EffectParam, EffectQuality,
    LinkMode, ParameterChannel, ParameterUnit, RackName, SessionModel, StableId, Submix,
    canonical_session_json, parse_session_json,
};

const EQ_SLOT: &str = r#"{ "slot": "eq", "identity": { "kind": "native", "effect_id": "miso.parametric-eq" }, "quality": "normal", "link_mode": "dual_mono" }"#;
const EQ_SLOT_HIGH: &str = r#"{ "slot": "eq", "identity": { "kind": "native", "effect_id": "miso.parametric-eq" }, "quality": "high", "link_mode": "dual_mono" }"#;
const COMP_SLOT: &str = r#"{ "slot": "comp", "identity": { "kind": "native", "effect_id": "miso.compressor" }, "quality": "normal", "link_mode": "maximum" }"#;
const LIMIT_SLOT: &str = r#"{ "slot": "limiter", "identity": { "kind": "native", "effect_id": "miso.true-peak-limiter" }, "quality": "high", "link_mode": "maximum" }"#;
const EQ_ENTRY: &str = r#"{ "slot": "eq", "bypass": false, "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 } ] }"#;
const EQ_INSERT: &str = r#"{ "id": "eq", "identity": { "kind": "native", "effect_id": "miso.parametric-eq" }, "quality": "normal", "bypass": false, "link_mode": "dual_mono", "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 } ], "sidechain": { "kind": "none" } }"#;

/// One strict Session V1 document: `console` is the root console object; `a` and `b` are the two
/// tracks' console entry arrays. Track `a` also carries the `eq` insert.
fn document(revision: u64, console: &str, a: &str, b: &str) -> String {
    let builtins = r#"{ "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 20.0, "lpf_hz": 20000.0, "delay_samples": 0 }"#;
    let track = |id: &str, console: &str, inserts: &str| {
        format!(
            r#"{{ "id": "{id}", "source_id": "voice", "left_source_channel": 0, "right_source_channel": 1,
              "builtins": {{ "left": {builtins}, "right": {builtins} }},
              "console": {console}, "inserts": {{ "effects": [{inserts}] }},
              "fader": {{ "left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false }},
              "pan": {{ "left": 1.0, "right": 1.0, "smoothing_samples": 16 }} }}"#
        )
    };
    let route = |id: &str| {
        format!(
            r#"{{ "id": "{id}-out", "source": {{ "kind": "track", "track_id": "{id}", "tap": "post_pan" }},
              "destination": {{ "kind": "output_input", "output_id": "main-out" }},
              "channel_matrix": {{ "ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0 }}, "gain_db": 0.0, "mute": false, "follows_mute": false }}"#
        )
    };
    format!(
        r#"{{ "schema_version": 1, "session_id": "console.edits", "revision": "{revision}",
          "sample_rate_hz": 48000, "quantum_frames": 128,
          "render_profile": {{ "id": "native", "mode": "single_thread" }},
          "output_profile": {{ "id": "main", "channels": 2, "sample_format": "f32_planar" }},
          "sources": [ {{ "id": "voice", "content": "blake3:2a97516c354b68848cdbd8f54a226a0a55b21ed138e207ad6c5cbb9c00aa5aea", "channels": 2, "bit_depth": "32f", "frames": "48000" }} ],
          "console": {console},
          "tracks": [ {}, {} ],
          "submixes": [], "vcas": [], "outputs": [ {{ "id": "main-out" }} ],
          "routes": [ {}, {} ], "automation": [] }}"#,
        track("a", a, EQ_INSERT),
        track("b", b, ""),
        route("a"),
        route("b"),
    )
}

fn console(pre: &[&str], post: &[&str]) -> String {
    format!(
        r#"{{ "pre_insert": [{}], "post_insert": [{}] }}"#,
        pre.join(", "),
        post.join(", ")
    )
}

fn entries(entries: &[&str]) -> String {
    format!("[{}]", entries.join(", "))
}

fn base_document() -> String {
    document(
        7,
        &console(&[EQ_SLOT], &[]),
        &entries(&[EQ_ENTRY]),
        &entries(&[EQ_ENTRY]),
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

fn param(
    parameter_id: u32,
    channel: ParameterChannel,
    unit: ParameterUnit,
    value: f32,
) -> EffectParam {
    EffectParam {
        parameter_id,
        channel,
        unit,
        value,
    }
}

fn slot(name: &str, effect_id: &str, quality: EffectQuality, link_mode: LinkMode) -> ConsoleSlot {
    ConsoleSlot {
        slot: id(name),
        identity: EffectIdentity::Native {
            effect_id: id(effect_id),
        },
        quality,
        link_mode,
    }
}

fn eq_slot() -> ConsoleSlot {
    slot(
        "eq",
        "miso.parametric-eq",
        EffectQuality::Normal,
        LinkMode::DualMono,
    )
}

fn entry(name: &str, bypass: bool, params: Vec<EffectParam>) -> ConsoleEntry {
    ConsoleEntry {
        slot: id(name),
        bypass,
        params,
    }
}

fn eq_entry() -> ConsoleEntry {
    entry(
        "eq",
        false,
        vec![param(1, ParameterChannel::Both, ParameterUnit::Db, 0.0)],
    )
}

/// Encode the edits as one wire transaction and decode them back, so every store-level assertion
/// below is about what a peer's frame does, not only about the typed value.
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
    assert_eq!(
        decoded, edits,
        "the wire carries every console edit unchanged"
    );
    decoded
}

/// Gate 3. Each console edit, applied through the wire and committed by the store, snapshots to
/// exactly the canonical JSON that S1a's parser builds from the same change written by hand.
///
/// Red if an edit reaches the wrong place -- the insert that shares the slot's ID, the other
/// track's entry, the declaration instead of the entry -- or if a declaration field is dropped on
/// the wire, because the committed snapshot then differs from the hand-written document.
#[test]
fn every_console_edit_round_trips_to_the_document_the_parser_builds() {
    let edited = r#"{ "slot": "eq", "bypass": true, "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 } ] }"#;
    let two_params = r#"{ "slot": "eq", "bypass": false, "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 }, { "parameter_id": 2, "channel": "left", "unit": "hz", "value": 1000.0 } ] }"#;
    let no_params = r#"{ "slot": "eq", "bypass": false, "params": [] }"#;
    let limiter = r#"{ "slot": "limiter", "bypass": true, "params": [ { "parameter_id": 3, "channel": "right", "unit": "db", "value": -1.0 } ] }"#;
    let limiter_entry = entry(
        "limiter",
        true,
        vec![param(3, ParameterChannel::Right, ParameterUnit::Db, -1.0)],
    );
    let eq = || RackName::Console;
    let cases: Vec<(&str, Vec<SessionEdit>, String)> = vec![
        (
            "SetEffectBypass on one track's entry",
            vec![SessionEdit::SetEffectBypass {
                track_id: id("a"),
                rack_name: eq(),
                effect_id: id("eq"),
                bypass: true,
            }],
            document(
                8,
                &console(&[EQ_SLOT], &[]),
                &entries(&[edited]),
                &entries(&[EQ_ENTRY]),
            ),
        ),
        (
            "UpsertEffectParam on one track's entry",
            vec![SessionEdit::UpsertEffectParam {
                track_id: id("b"),
                rack_name: eq(),
                effect_id: id("eq"),
                param: param(2, ParameterChannel::Left, ParameterUnit::Hz, 1_000.0),
            }],
            document(
                8,
                &console(&[EQ_SLOT], &[]),
                &entries(&[EQ_ENTRY]),
                &entries(&[two_params]),
            ),
        ),
        (
            "RemoveEffectParam on one track's entry",
            vec![SessionEdit::RemoveEffectParam {
                track_id: id("a"),
                rack_name: eq(),
                effect_id: id("eq"),
                parameter_id: 1,
                channel: ParameterChannel::Both,
            }],
            document(
                8,
                &console(&[EQ_SLOT], &[]),
                &entries(&[no_params]),
                &entries(&[EQ_ENTRY]),
            ),
        ),
        (
            "SetTrackConsole on one track",
            vec![SessionEdit::SetTrackConsole {
                track_id: id("b"),
                console: vec![entry(
                    "eq",
                    true,
                    vec![param(1, ParameterChannel::Both, ParameterUnit::Db, 0.0)],
                )],
            }],
            document(
                8,
                &console(&[EQ_SLOT], &[]),
                &entries(&[EQ_ENTRY]),
                &entries(&[edited]),
            ),
        ),
        (
            "SetConsole keeping the slot set (a declaration-only change)",
            vec![SessionEdit::SetConsole {
                console: Console {
                    pre_insert: vec![slot(
                        "eq",
                        "miso.parametric-eq",
                        EffectQuality::High,
                        LinkMode::DualMono,
                    )],
                    post_insert: Vec::new(),
                },
            }],
            document(
                8,
                &console(&[EQ_SLOT_HIGH], &[]),
                &entries(&[EQ_ENTRY]),
                &entries(&[EQ_ENTRY]),
            ),
        ),
        (
            "SetConsole adding a post_insert slot, with every track rewritten",
            vec![
                SessionEdit::SetConsole {
                    console: Console {
                        pre_insert: vec![eq_slot()],
                        post_insert: vec![slot(
                            "limiter",
                            "miso.true-peak-limiter",
                            EffectQuality::High,
                            LinkMode::Maximum,
                        )],
                    },
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("a"),
                    console: vec![eq_entry(), limiter_entry.clone()],
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("b"),
                    console: vec![eq_entry(), limiter_entry.clone()],
                },
            ],
            document(
                8,
                &console(&[EQ_SLOT], &[LIMIT_SLOT]),
                &entries(&[EQ_ENTRY, limiter]),
                &entries(&[EQ_ENTRY, limiter]),
            ),
        ),
        (
            "SetConsole emptying both sections, with every track rewritten",
            vec![
                SessionEdit::SetConsole {
                    console: Console {
                        pre_insert: Vec::new(),
                        post_insert: Vec::new(),
                    },
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("a"),
                    console: Vec::new(),
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("b"),
                    console: Vec::new(),
                },
            ],
            document(8, &console(&[], &[]), &entries(&[]), &entries(&[])),
        ),
        (
            "SetConsole moving the slot to post_insert keeps the entry sequence",
            vec![SessionEdit::SetConsole {
                console: Console {
                    pre_insert: Vec::new(),
                    post_insert: vec![eq_slot()],
                },
            }],
            document(
                8,
                &console(&[], &[EQ_SLOT]),
                &entries(&[EQ_ENTRY]),
                &entries(&[EQ_ENTRY]),
            ),
        ),
    ];
    for (name, edits, expected) in cases {
        let edits = through_the_wire(&edits);
        let mut store = store();
        let commit = store
            .apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &edits)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(commit.revision, SessionRevision(8), "{name}");
        let parsed = parse_session_json(&expected)
            .unwrap_or_else(|error| panic!("{name}: the expected document parses: {error:?}"));
        assert_eq!(
            store.canonical_snapshot(),
            canonical_session_json(&parsed).expect("canonical"),
            "{name}"
        );
    }
}

/// A console knob edit changes one track's entry and nothing else: not the insert that shares the
/// slot's ID, not the other track, not the declaration. A slot the track does not carry is
/// `NotFound`, like an absent insert.
///
/// Red if `console` is routed to the inserts rack (the `eq` insert changes), to every track, or to
/// the slot declaration.
#[test]
fn console_knob_edits_change_only_that_tracks_entry() {
    let before = base();
    for edit in [
        SessionEdit::SetEffectBypass {
            track_id: id("a"),
            rack_name: RackName::Console,
            effect_id: id("eq"),
            bypass: true,
        },
        SessionEdit::UpsertEffectParam {
            track_id: id("a"),
            rack_name: RackName::Console,
            effect_id: id("eq"),
            param: param(1, ParameterChannel::Both, ParameterUnit::Db, -6.0),
        },
        SessionEdit::RemoveEffectParam {
            track_id: id("a"),
            rack_name: RackName::Console,
            effect_id: id("eq"),
            parameter_id: 1,
            channel: ParameterChannel::Both,
        },
    ] {
        let mut model = before.clone();
        assert_eq!(apply_session_edit(&mut model, &edit), Ok(()), "{edit:?}");
        assert_ne!(
            model.tracks[0].console, before.tracks[0].console,
            "{edit:?} edits track a's entry"
        );
        let mut rest = model.clone();
        rest.tracks[0].console = before.tracks[0].console.clone();
        assert_eq!(rest, before, "{edit:?} changes nothing but track a's entry");
    }
    let mut model = before.clone();
    assert_eq!(
        apply_session_edit(
            &mut model,
            &SessionEdit::SetEffectBypass {
                track_id: id("a"),
                rack_name: RackName::Console,
                effect_id: id("limiter"),
                bypass: true,
            },
        ),
        Err(SessionEditError::NotFound)
    );
    assert_eq!(model, before);
}

fn structural_and_declaration_edits(
    track_id: &StableId,
    rack_name: RackName,
) -> Vec<(&'static str, SessionEdit)> {
    let effect = base().tracks[0].inserts.effects[0].clone();
    let track_id = track_id.clone();
    vec![
        (
            "SetTrackRack",
            SessionEdit::SetTrackRack {
                track_id: track_id.clone(),
                rack_name,
                rack: session::Rack {
                    effects: Vec::new(),
                },
            },
        ),
        (
            "PutTrackEffect",
            SessionEdit::PutTrackEffect {
                track_id: track_id.clone(),
                rack_name,
                final_position: 0,
                effect: effect.clone(),
            },
        ),
        (
            "RemoveTrackEffect",
            SessionEdit::RemoveTrackEffect {
                track_id: track_id.clone(),
                rack_name,
                effect_id: id("eq"),
            },
        ),
        (
            "SetTrackEffectOrder",
            SessionEdit::SetTrackEffectOrder {
                track_id: track_id.clone(),
                rack_name,
                effect_ids: vec![id("eq")],
            },
        ),
        (
            "SetEffectIdentity",
            SessionEdit::SetEffectIdentity {
                track_id: track_id.clone(),
                rack_name,
                effect_id: id("eq"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.compressor"),
                },
            },
        ),
        (
            "SetEffectQuality",
            SessionEdit::SetEffectQuality {
                track_id: track_id.clone(),
                rack_name,
                effect_id: id("eq"),
                quality: EffectQuality::High,
            },
        ),
        (
            "SetEffectLinkMode",
            SessionEdit::SetEffectLinkMode {
                track_id: track_id.clone(),
                rack_name,
                effect_id: id("eq"),
                link_mode: LinkMode::Maximum,
            },
        ),
        (
            "SetEffectSidechain",
            SessionEdit::SetEffectSidechain {
                track_id,
                rack_name,
                effect_id: id("eq"),
                sidechain: session::SidechainDeclaration::None,
            },
        ),
    ]
}

/// A track cannot add, remove or reorder a console slot, and a slot's declaration is not a
/// track's: every structural or declaration edit addressed at `console` is refused with the typed
/// `ConsoleSlotFixed`, whatever the model holds -- even naming a track that does not exist.
///
/// Red if `rack_mut` routes `console` anywhere (the inserts, the entries) or answers `NotFound`,
/// which a client cannot tell from a typo. The control applies the same eight edits to track a's
/// `eq` insert, so the refusal is about the token, not the edits.
#[test]
fn console_structural_and_declaration_edits_refuse_with_a_typed_status() {
    let before = base();
    for track in ["a", "missing"] {
        for (name, edit) in structural_and_declaration_edits(&id(track), RackName::Console) {
            let mut model = before.clone();
            assert_eq!(
                apply_session_edit(&mut model, &edit),
                Err(SessionEditError::ConsoleSlotFixed),
                "{name} on track {track}"
            );
            assert_eq!(model, before, "{name}: a refused edit changes nothing");
        }
    }
    for (name, edit) in structural_and_declaration_edits(&id("a"), RackName::Inserts) {
        let mut model = before.clone();
        assert_eq!(apply_session_edit(&mut model, &edit), Ok(()), "{name}");
    }
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

/// Gate 4. A transaction that changes the slot set -- adds, removes or reorders a slot -- but
/// leaves one track's entries as they were refuses as a whole: the prior revision, model and
/// snapshot stay committed, and the refusal names the track that was skipped. The same
/// transaction with every track rewritten commits.
///
/// Red if the store commits a partial model (a slot some track has no entry for), or if the apply
/// path validated per edit and so refused the rewrite that has to follow `SetConsole`.
#[test]
fn a_slot_set_change_that_skips_a_track_refuses_whole() {
    let comp = || {
        slot(
            "comp",
            "miso.compressor",
            EffectQuality::Normal,
            LinkMode::Maximum,
        )
    };
    let comp_entry = || entry("comp", false, Vec::new());
    // A two-slot starting point, so a reorder is expressible.
    let mut two_slots = store();
    two_slots
        .apply_transaction(
            ExpectedRevision::Exact(SessionRevision(7)),
            &[
                SessionEdit::SetConsole {
                    console: Console {
                        pre_insert: vec![eq_slot(), comp()],
                        post_insert: Vec::new(),
                    },
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("a"),
                    console: vec![eq_entry(), comp_entry()],
                },
                SessionEdit::SetTrackConsole {
                    track_id: id("b"),
                    console: vec![eq_entry(), comp_entry()],
                },
            ],
        )
        .expect("a slot set change with every track rewritten commits");
    let two_slot_model = two_slots.compiled().normalized_model().clone();
    assert_eq!(
        two_slots.canonical_snapshot(),
        canonical_session_json(
            &parse_session_json(&document(
                8,
                &console(&[EQ_SLOT, COMP_SLOT], &[]),
                &entries(&[
                    EQ_ENTRY,
                    r#"{ "slot": "comp", "bypass": false, "params": [] }"#
                ]),
                &entries(&[
                    EQ_ENTRY,
                    r#"{ "slot": "comp", "bypass": false, "params": [] }"#
                ]),
            ))
            .expect("parses")
        )
        .expect("canonical")
    );

    struct Case {
        name: &'static str,
        start: SessionModel,
        console: Console,
        rewritten: Vec<ConsoleEntry>,
        code: &'static str,
    }
    let cases = [
        Case {
            name: "add a slot",
            start: base(),
            console: Console {
                pre_insert: vec![eq_slot(), comp()],
                post_insert: Vec::new(),
            },
            rewritten: vec![eq_entry(), comp_entry()],
            code: "console.entry_missing",
        },
        Case {
            name: "remove a slot",
            start: base(),
            console: Console {
                pre_insert: Vec::new(),
                post_insert: Vec::new(),
            },
            rewritten: Vec::new(),
            code: "reference.missing_entity",
        },
        Case {
            name: "reorder the slots",
            start: two_slot_model,
            console: Console {
                pre_insert: vec![comp(), eq_slot()],
                post_insert: Vec::new(),
            },
            rewritten: vec![comp_entry(), eq_entry()],
            code: "console.entry_order",
        },
    ];
    for case in cases {
        let mut store = SessionStore::new(case.start.clone(), caps()).expect("start compiles");
        let revision = store.revision();
        let snapshot = store.canonical_snapshot().to_owned();
        let model = store.compiled().normalized_model().clone();
        let change = SessionEdit::SetConsole {
            console: case.console.clone(),
        };
        let rewrite = |track: &str| SessionEdit::SetTrackConsole {
            track_id: id(track),
            console: case.rewritten.clone(),
        };
        let skipped = through_the_wire(&[change.clone(), rewrite("a")]);
        let error = store
            .apply_transaction(ExpectedRevision::Exact(revision), &skipped)
            .expect_err(case.name);
        let codes = refusal_codes(&error, skipped.len());
        assert!(
            codes
                .iter()
                .any(|(code, path)| code == case.code && path.starts_with("$.tracks[1].console")),
            "{}: the refusal names track b's entries: {codes:?}",
            case.name
        );
        assert!(
            codes
                .iter()
                .all(|(_, path)| path.starts_with("$.tracks[1]")),
            "{}: track a was rewritten, so only track b is refused: {codes:?}",
            case.name
        );
        assert_eq!(store.revision(), revision, "{}", case.name);
        assert_eq!(store.canonical_snapshot(), snapshot, "{}", case.name);
        assert_eq!(store.compiled().normalized_model(), &model, "{}", case.name);

        let whole = through_the_wire(&[change, rewrite("a"), rewrite("b")]);
        let commit = store
            .apply_transaction(ExpectedRevision::Exact(revision), &whole)
            .unwrap_or_else(|error| panic!("{}: {error:?}", case.name));
        assert_eq!(commit.revision.0, revision.0 + 1, "{}", case.name);
        assert_eq!(
            store.compiled().normalized_model().console,
            case.console,
            "{}",
            case.name
        );
    }
}

/// #1202 gate 5 (D5). A submix carries every slot too, so a transaction that changes the slot set
/// must rewrite every **strip**: the tracks through `SetTrackConsole` and the submix through
/// `UpsertSubmix`. Adding a slot while rewriting both tracks but not the submix refuses whole with
/// the submix's `console.entry_missing` -- nothing is committed and the revision does not advance
/// -- and the same transaction that also rewrites the submix commits.
///
/// Red if the store validates only tracks' entries after a slot-set change, committing a submix
/// with no entry for a declared slot.
#[test]
fn a_slot_set_change_that_skips_a_submix_refuses_whole() {
    let comp = || {
        slot(
            "comp",
            "miso.compressor",
            EffectQuality::Normal,
            LinkMode::Maximum,
        )
    };
    let comp_entry = || entry("comp", false, Vec::new());
    let mut start = base();
    start
        .submixes
        .push(Submix::unity(id("bus"), &start.console));
    let mut store = SessionStore::new(start, caps()).expect("a unity bus compiles");
    let revision = store.revision();
    let snapshot = store.canonical_snapshot().to_owned();
    let model = store.compiled().normalized_model().clone();

    let console = Console {
        pre_insert: vec![eq_slot(), comp()],
        post_insert: Vec::new(),
    };
    let mut edits = vec![SessionEdit::SetConsole {
        console: console.clone(),
    }];
    for track in ["a", "b"] {
        edits.push(SessionEdit::SetTrackConsole {
            track_id: id(track),
            console: vec![eq_entry(), comp_entry()],
        });
    }
    let skipped = through_the_wire(&edits);
    let error = store
        .apply_transaction(ExpectedRevision::Exact(revision), &skipped)
        .expect_err("a slot set change that skips the submix");
    let codes = refusal_codes(&error, skipped.len());
    assert_eq!(
        codes,
        [(
            "console.entry_missing".to_owned(),
            "$.submixes[0].console".to_owned()
        )],
        "only the submix is refused, for its missing entry"
    );
    assert_eq!(store.revision(), revision);
    assert_eq!(store.canonical_snapshot(), snapshot);
    assert_eq!(store.compiled().normalized_model(), &model);

    let mut bus = Submix::unity(id("bus"), &console);
    bus.console[1] = comp_entry();
    edits.push(SessionEdit::UpsertSubmix {
        submix: bus.clone(),
    });
    let whole = through_the_wire(&edits);
    let commit = store
        .apply_transaction(ExpectedRevision::Exact(revision), &whole)
        .expect("the transaction that rewrites every strip commits");
    assert_eq!(commit.revision.0, revision.0 + 1);
    let committed = store.compiled().normalized_model();
    assert_eq!(committed.console, console);
    assert_eq!(committed.submixes, [bus]);
}

/// `SetTrackConsole` edits a track's knobs; it cannot change the slot set. An entry array that
/// adds, drops or reorders a slot refuses whole at final validation.
///
/// Red if `SetTrackConsole` were allowed to reshape the slot set for one track, which is exactly
/// what decision 12 forbids (every track has every slot, in slot order).
#[test]
fn a_track_cannot_change_the_slot_set_through_its_entries() {
    let comp = slot(
        "comp",
        "miso.compressor",
        EffectQuality::Normal,
        LinkMode::Maximum,
    );
    let mut start = base();
    start.console.pre_insert.push(comp);
    for track in &mut start.tracks {
        track.console.push(entry("comp", false, Vec::new()));
    }
    for (name, console, code) in [
        (
            "add",
            vec![
                eq_entry(),
                entry("comp", false, Vec::new()),
                entry("limiter", false, Vec::new()),
            ],
            "reference.missing_entity",
        ),
        ("drop", vec![eq_entry()], "console.entry_missing"),
        (
            "reorder",
            vec![entry("comp", false, Vec::new()), eq_entry()],
            "console.entry_order",
        ),
    ] {
        let mut store = SessionStore::new(start.clone(), caps()).expect("start compiles");
        let snapshot = store.canonical_snapshot().to_owned();
        let edits = through_the_wire(&[SessionEdit::SetTrackConsole {
            track_id: id("a"),
            console,
        }]);
        let error = store
            .apply_transaction(ExpectedRevision::Exact(store.revision()), &edits)
            .expect_err(name);
        let codes = refusal_codes(&error, 1);
        assert!(
            codes.iter().any(|(found, _)| found == code),
            "{name}: {codes:?}"
        );
        assert_eq!(store.revision(), SessionRevision(7), "{name}");
        assert_eq!(store.canonical_snapshot(), snapshot, "{name}");
    }
}

/// #1218 gates 6 and 7 (D1). `SetRouteFollowsMute` (`0x0507`), through the wire, is validated on
/// the final candidate like every edit:
///
/// - `true` on route `a-out`, whose destination is the output, refuses the whole transaction with
///   `schema.invalid_enum` at `$.routes[0].follows_mute`; the revision, model and snapshot stay;
/// - the same edit followed by re-pointing `a-out` into submix `bus` commits, and the committed
///   snapshot is the canonical JSON of the hand-edited model;
/// - `false` then commits back.
///
/// Red if an output route can follow (a strip's main route would stay silent after a live
/// unmute, VERIFY-2 N1), if the refusal were a per-edit check that refused the re-pointing
/// transaction, or if `0x0507` writes the wrong route or ignores the value.
#[test]
fn set_route_follows_mute_is_legal_only_on_a_route_into_a_submix() {
    let mut start = base();
    start
        .submixes
        .push(Submix::unity(id("bus"), &start.console));
    let mut store = SessionStore::new(start, caps()).expect("a unity bus compiles");
    let revision = store.revision();
    let snapshot = store.canonical_snapshot().to_owned();
    let model = store.compiled().normalized_model().clone();
    assert_eq!(model.routes[0].id.as_str(), "a-out");
    let follow = |follows_mute| SessionEdit::SetRouteFollowsMute {
        route_id: id("a-out"),
        follows_mute,
    };

    let refused = through_the_wire(&[follow(true)]);
    let error = store
        .apply_transaction(ExpectedRevision::Exact(revision), &refused)
        .expect_err("an output route cannot follow");
    assert_eq!(
        refusal_codes(&error, refused.len()),
        [(
            "schema.invalid_enum".to_owned(),
            "$.routes[0].follows_mute".to_owned()
        )]
    );
    assert_eq!(store.revision(), revision);
    assert_eq!(store.canonical_snapshot(), snapshot);
    assert_eq!(store.compiled().normalized_model(), &model);

    let repointed = through_the_wire(&[
        follow(true),
        SessionEdit::SetRouteDestination {
            route_id: id("a-out"),
            destination: session::RouteDestination::SubmixInput {
                submix_id: id("bus"),
            },
        },
    ]);
    let commit = store
        .apply_transaction(ExpectedRevision::Exact(revision), &repointed)
        .expect("a follow on a route re-pointed into a submix commits");
    assert_eq!(commit.revision.0, revision.0 + 1);
    let mut expected = model.clone();
    expected.revision = revision.0 + 1;
    expected.routes[0].follows_mute = true;
    expected.routes[0].destination = session::RouteDestination::SubmixInput {
        submix_id: id("bus"),
    };
    assert_eq!(
        store.canonical_snapshot(),
        canonical_session_json(&expected).expect("canonical")
    );
    assert!(!store.compiled().normalized_model().routes[1].follows_mute);

    store
        .apply_transaction(
            ExpectedRevision::Exact(SessionRevision(revision.0 + 1)),
            &through_the_wire(&[follow(false)]),
        )
        .expect("a follow switched off commits");
    assert!(!store.compiled().normalized_model().routes[0].follows_mute);
}
