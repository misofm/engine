//! Issue #1204: the strip edits (`0203`-`0211`) address a **strip ID**.
//!
//! - A track ID resolves to the track's strip and a submix ID to the submix's (D1). Tracks are
//!   searched first, then submixes (D4).
//! - `0202` (source assignment) stays track-only, and an output ID is not a strip (D2).
//! - Rack and knob semantics are unchanged; only the resolution widens (D3).
//!
//! The base session has track `a` routed into submix `bus`, which feeds output `main-out`, and an
//! unrouted track `x`. Track `a` and submix `bus` carry identical strips -- the same console entry
//! and the same two inserts, `eq` and `comp` -- so an edit that lands on the wrong strip changes a
//! value the snapshot comparison reads.

use protocol::{
    DecodeScratch, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionEditError,
    SessionRevision, SessionStore, SessionStoreError, SessionTransactionFrame,
};
use session::{
    ChannelBuiltins, CompileCaps, ConsoleEntry, DualMonoBuiltins, DualMonoFader, Effect,
    EffectIdentity, EffectParam, EffectQuality, LinkMode, MatrixOrPan, ParameterChannel,
    ParameterUnit, Rack, RackName, RouteSource, SendTap, SessionModel, Sidechain,
    SidechainDeclaration, StableId, Submix, canonical_session_json, parse_session_json,
};

const BUILTINS: &str = r#"{ "polarity_invert": false, "trim_db": 0.0, "hpf_hz": 20.0, "lpf_hz": 20000.0, "delay_samples": 0 }"#;
const EQ_ENTRY: &str = r#"{ "slot": "eq", "bypass": false, "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 } ] }"#;
const INSERTS: &str = r#"{ "effects": [
    { "id": "eq", "identity": { "kind": "native", "effect_id": "miso.parametric-eq" }, "quality": "normal", "bypass": false, "link_mode": "dual_mono", "params": [ { "parameter_id": 1, "channel": "both", "unit": "db", "value": 0.0 } ], "sidechain": { "kind": "none" } },
    { "id": "comp", "identity": { "kind": "native", "effect_id": "miso.compressor" }, "quality": "normal", "bypass": false, "link_mode": "maximum", "params": [], "sidechain": { "kind": "none" } } ] }"#;
const FADER: &str =
    r#"{ "left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false }"#;
const PAN: &str = r#"{ "left": 1.0, "right": 1.0, "smoothing_samples": 16 }"#;

fn base_document() -> String {
    let strip = format!(
        r#""builtins": {{ "left": {BUILTINS}, "right": {BUILTINS} }}, "console": [ {EQ_ENTRY} ],
           "inserts": {INSERTS}, "fader": {FADER}, "pan": {PAN}"#
    );
    let track = |id: &str| {
        format!(
            r#"{{ "id": "{id}", "source_id": "voice", "left_source_channel": 0, "right_source_channel": 1, {strip} }}"#
        )
    };
    format!(
        r#"{{ "schema_version": 1, "session_id": "submix.strip.edits", "revision": "7",
          "sample_rate_hz": 48000, "quantum_frames": 128,
          "render_profile": {{ "id": "native", "mode": "single_thread" }},
          "output_profile": {{ "id": "main", "channels": 2, "sample_format": "f32_planar" }},
          "sources": [ {{ "id": "voice", "content": "blake3:2a97516c354b68848cdbd8f54a226a0a55b21ed138e207ad6c5cbb9c00aa5aea", "channels": 2, "bit_depth": "32f", "frames": "48000" }} ],
          "console": {{ "pre_insert": [ {{ "slot": "eq", "identity": {{ "kind": "native", "effect_id": "miso.parametric-eq" }}, "quality": "normal", "link_mode": "dual_mono" }} ], "post_insert": [] }},
          "tracks": [ {}, {} ],
          "submixes": [ {{ "id": "bus", {strip} }} ],
          "vcas": [],
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

fn param(parameter_id: u32, channel: ParameterChannel, value: f32) -> EffectParam {
    EffectParam {
        parameter_id,
        channel,
        unit: ParameterUnit::Db,
        value,
    }
}

fn fader(db: f32) -> DualMonoFader {
    DualMonoFader {
        left_db: db,
        right_db: db,
        left_mute: false,
        right_mute: false,
    }
}

fn limiter() -> Effect {
    Effect {
        id: id("limiter"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: EffectQuality::High,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: Vec::new(),
        sidechain: SidechainDeclaration::None,
    }
}

/// Encode the edits as one wire transaction and decode them back, so every store-level assertion
/// is about what a peer's frame does.
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
        "the wire carries every strip edit unchanged"
    );
    decoded
}

fn submix<'a>(model: &'a mut SessionModel, submix_id: &str) -> &'a mut Submix {
    model
        .submixes
        .iter_mut()
        .find(|submix| submix.id == id(submix_id))
        .expect("the submix exists")
}

/// One strip edit per opcode `0203`-`0211`, each addressed to submix `bus`, with the change it
/// makes to the bus written directly on the model.
type Case = (&'static str, SessionEdit, fn(&mut Submix));

fn cases() -> Vec<Case> {
    let bus = || id("bus");
    vec![
        (
            "0203 SetTrackBuiltins",
            SessionEdit::SetTrackBuiltins {
                track_id: bus(),
                builtins: DualMonoBuiltins {
                    left: ChannelBuiltins {
                        polarity_invert: true,
                        trim_db: -3.0,
                        hpf_hz: 40.0,
                        lpf_hz: 18_000.0,
                        delay_samples: 0,
                    },
                    right: ChannelBuiltins {
                        polarity_invert: false,
                        trim_db: 2.0,
                        hpf_hz: 30.0,
                        lpf_hz: 16_000.0,
                        delay_samples: 0,
                    },
                },
            },
            |bus| {
                bus.builtins.left.polarity_invert = true;
                bus.builtins.left.trim_db = -3.0;
                bus.builtins.left.hpf_hz = 40.0;
                bus.builtins.left.lpf_hz = 18_000.0;
                bus.builtins.right.trim_db = 2.0;
                bus.builtins.right.hpf_hz = 30.0;
                bus.builtins.right.lpf_hz = 16_000.0;
            },
        ),
        (
            "0204 SetTrackRack",
            SessionEdit::SetTrackRack {
                track_id: bus(),
                rack_name: RackName::Inserts,
                rack: Rack {
                    effects: vec![limiter()],
                },
            },
            |bus| bus.inserts.effects = vec![limiter()],
        ),
        (
            "0205 PutTrackEffect",
            SessionEdit::PutTrackEffect {
                track_id: bus(),
                rack_name: RackName::Inserts,
                final_position: 1,
                effect: limiter(),
            },
            |bus| bus.inserts.effects.insert(1, limiter()),
        ),
        (
            "0206 RemoveTrackEffect",
            SessionEdit::RemoveTrackEffect {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("eq"),
            },
            |bus| {
                bus.inserts.effects.remove(0);
            },
        ),
        (
            "0207 SetTrackEffectOrder",
            SessionEdit::SetTrackEffectOrder {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_ids: vec![id("comp"), id("eq")],
            },
            |bus| bus.inserts.effects.swap(0, 1),
        ),
        (
            "0208 SetEffectIdentity",
            SessionEdit::SetEffectIdentity {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("comp"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.gate-expander"),
                },
            },
            |bus| {
                bus.inserts.effects[1].identity = EffectIdentity::Native {
                    effect_id: id("miso.gate-expander"),
                };
            },
        ),
        (
            "0209 SetEffectQuality",
            SessionEdit::SetEffectQuality {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("eq"),
                quality: EffectQuality::High,
            },
            |bus| bus.inserts.effects[0].quality = EffectQuality::High,
        ),
        (
            "020a SetEffectBypass (console)",
            SessionEdit::SetEffectBypass {
                track_id: bus(),
                rack_name: RackName::Console,
                effect_id: id("eq"),
                bypass: true,
            },
            |bus| bus.console[0].bypass = true,
        ),
        (
            "020b SetEffectLinkMode",
            SessionEdit::SetEffectLinkMode {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("comp"),
                link_mode: LinkMode::Average,
            },
            |bus| bus.inserts.effects[1].link_mode = LinkMode::Average,
        ),
        (
            "020c SetEffectSidechain",
            SessionEdit::SetEffectSidechain {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("comp"),
                sidechain: SidechainDeclaration::Routed(Sidechain {
                    source: RouteSource::Track {
                        track_id: id("x"),
                        tap: SendTap::PostFader,
                    },
                    port_id: id("sidechain"),
                }),
            },
            |bus| {
                bus.inserts.effects[1].sidechain = SidechainDeclaration::Routed(Sidechain {
                    source: RouteSource::Track {
                        track_id: id("x"),
                        tap: SendTap::PostFader,
                    },
                    port_id: id("sidechain"),
                });
            },
        ),
        (
            "020d UpsertEffectParam (console)",
            SessionEdit::UpsertEffectParam {
                track_id: bus(),
                rack_name: RackName::Console,
                effect_id: id("eq"),
                param: param(2, ParameterChannel::Left, -4.5),
            },
            |bus| {
                bus.console[0]
                    .params
                    .push(param(2, ParameterChannel::Left, -4.5));
            },
        ),
        (
            "020e RemoveEffectParam",
            SessionEdit::RemoveEffectParam {
                track_id: bus(),
                rack_name: RackName::Inserts,
                effect_id: id("eq"),
                parameter_id: 1,
                channel: ParameterChannel::Both,
            },
            |bus| bus.inserts.effects[0].params.clear(),
        ),
        (
            "020f SetTrackFader",
            SessionEdit::SetTrackFader {
                track_id: bus(),
                fader: fader(-6.0),
            },
            |bus| bus.fader = fader(-6.0),
        ),
        (
            "0210 SetTrackMatrixOrPan",
            SessionEdit::SetTrackMatrixOrPan {
                track_id: bus(),
                matrix_or_pan: MatrixOrPan::Matrix {
                    ll: 0.5,
                    lr: 0.25,
                    rl: 0.25,
                    rr: 0.5,
                    smoothing_samples: 32,
                },
            },
            |bus| {
                bus.matrix_or_pan = MatrixOrPan::Matrix {
                    ll: 0.5,
                    lr: 0.25,
                    rl: 0.25,
                    rr: 0.5,
                    smoothing_samples: 32,
                };
            },
        ),
        (
            "0211 SetTrackConsole",
            SessionEdit::SetTrackConsole {
                track_id: bus(),
                console: vec![ConsoleEntry {
                    slot: id("eq"),
                    bypass: true,
                    params: Vec::new(),
                }],
            },
            |bus| {
                bus.console[0].bypass = true;
                bus.console[0].params.clear();
            },
        ),
    ]
}

/// Gate 1. Every strip opcode, addressed to submix `bus` and committed through the wire, snapshots
/// to exactly the base document with only the bus changed: track `a`, which carries an identical
/// strip, and track `x` are untouched.
///
/// Red if any strip opcode still resolves through the track lookup (it refuses `bus` with
/// `NotFound`) or lands on a track's strip. Every earlier edit test addresses a track.
#[test]
fn every_strip_opcode_edits_a_submix() {
    let cases = cases();
    let opcodes: Vec<u16> = cases.iter().map(|case| case.1.opcode() as u16).collect();
    assert_eq!(
        opcodes,
        (0x0203..=0x0211).collect::<Vec<u16>>(),
        "one case per strip opcode, in opcode order"
    );
    for (name, edit, change) in cases {
        let edits = through_the_wire(&[edit]);
        let mut store = store();
        let commit = store
            .apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &edits)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        assert_eq!(commit.revision, SessionRevision(8), "{name}");
        let mut expected = base();
        expected.revision = 8;
        change(submix(&mut expected, "bus"));
        assert_ne!(
            submix(&mut expected, "bus"),
            submix(&mut base(), "bus"),
            "{name}: the case changes the bus"
        );
        assert_eq!(
            store.canonical_snapshot(),
            canonical_session_json(&expected).expect("canonical"),
            "{name}"
        );
    }
}

/// Gate 2. The refusals are the track's: `0202` with a submix ID and `020f` with an output ID are
/// `NotFound` (`session.edit.not_found`), and `0205` naming a submix's console rack is
/// `ConsoleSlotFixed` (`session.edit.console_slot_fixed`). Each follows an edit that would commit
/// on its own, and nothing is committed: the revision and snapshot stay where they were.
///
/// Red if source assignment reaches a submix, if an output ID resolves to a strip (or the edit
/// naming it is skipped silently), or if a submix's console slots become structurally editable.
#[test]
fn strip_refusals_are_the_tracks() {
    let console_insert = Effect {
        id: id("eq"),
        ..limiter()
    };
    let cases = [
        (
            "0202 on a submix",
            SessionEdit::SetTrackSourceAssignment {
                track_id: id("bus"),
                source_id: id("voice"),
                left_source_channel: 1,
                right_source_channel: 0,
            },
            SessionEditError::NotFound,
        ),
        (
            "020f on an output",
            SessionEdit::SetTrackFader {
                track_id: id("main-out"),
                fader: fader(-6.0),
            },
            SessionEditError::NotFound,
        ),
        (
            "0205 on a submix's console rack",
            SessionEdit::PutTrackEffect {
                track_id: id("bus"),
                rack_name: RackName::Console,
                final_position: 0,
                effect: console_insert,
            },
            SessionEditError::ConsoleSlotFixed,
        ),
    ];
    for (name, refused, expected) in cases {
        let mut store = store();
        let revision = store.revision();
        let snapshot = store.canonical_snapshot().to_owned();
        let edits = through_the_wire(&[
            SessionEdit::SetTrackFader {
                track_id: id("a"),
                fader: fader(-3.0),
            },
            refused,
        ]);
        let error = store
            .apply_transaction(ExpectedRevision::Exact(revision), &edits)
            .expect_err(name);
        assert_eq!(
            error,
            SessionStoreError::Edit {
                operation_index: 1,
                error: expected,
            },
            "{name}"
        );
        assert_eq!(store.revision(), revision, "{name}");
        assert_eq!(store.canonical_snapshot(), snapshot, "{name}");
    }
}

/// Gate 3 (D4). Inside one transaction a track and a submix may briefly share an ID; a strip edit
/// then resolves the track. Upserting submix `x` beside track `x`, setting `x`'s fader and then
/// removing track `x` commits submix `x` at its transparent 0 dB: the fader edit went to the track,
/// which is gone. The mirror -- remove the track first, then upsert the submix and set its fader --
/// commits the submix at -6 dB.
///
/// Red if the strip lookup searches submixes first, or picks whichever match it meets.
#[test]
fn a_strip_id_resolves_tracks_before_submixes() {
    let unity = {
        let base = base();
        Submix::unity(id("x"), &base.console)
    };
    let upsert = || SessionEdit::UpsertSubmix {
        submix: unity.clone(),
    };
    let set_fader = || SessionEdit::SetTrackFader {
        track_id: id("x"),
        fader: fader(-6.0),
    };
    let remove = || SessionEdit::RemoveTrack { track_id: id("x") };
    for (name, edits, expected_db) in [
        ("track first", vec![upsert(), set_fader(), remove()], 0.0),
        (
            "track already removed",
            vec![remove(), upsert(), set_fader()],
            -6.0,
        ),
    ] {
        let mut store = store();
        let edits = through_the_wire(&edits);
        store
            .apply_transaction(ExpectedRevision::Exact(SessionRevision(7)), &edits)
            .unwrap_or_else(|error| panic!("{name}: {error:?}"));
        let model = store.compiled().normalized_model();
        assert!(
            model.tracks.iter().all(|track| track.id != id("x")),
            "{name}: track x is removed"
        );
        let committed = model
            .submixes
            .iter()
            .find(|submix| submix.id == id("x"))
            .unwrap_or_else(|| panic!("{name}: submix x is committed"));
        assert_eq!(committed.fader, fader(expected_db), "{name}");
    }
}
