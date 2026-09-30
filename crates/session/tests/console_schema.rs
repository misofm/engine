//! Decision 12's session console and per-track inserts: the grammar's refusals, each with its code
//! and JSON path, through the text entry point that a hand-written document meets.

use serde_json::{Value, json};
use session::{DiagnosticCode, RackName, SendTap, parse_session_json};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// The representative fixture with one console slot in each section and the track's entries.
fn console_document() -> Value {
    let mut value: Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
    value["console"] = json!({
        "pre_insert": [{
            "slot": "desk-eq",
            "identity": { "kind": "native", "effect_id": "miso.parametric-eq" },
            "quality": "normal",
            "link_mode": "dual_mono"
        }],
        "post_insert": [{
            "slot": "desk-limit",
            "identity": { "kind": "native", "effect_id": "miso.true-peak-limiter" },
            "quality": "normal",
            "link_mode": "maximum"
        }]
    });
    value["tracks"][0]["console"] = json!([
        { "slot": "desk-eq", "bypass": false, "params": [] },
        { "slot": "desk-limit", "bypass": true, "params": [] }
    ]);
    value
}

fn refused(value: &Value, code: DiagnosticCode, path: &str) {
    let source = serde_json::to_string_pretty(value).expect("JSON");
    let error = parse_session_json(&source).expect_err(path);
    let diagnostic = error
        .diagnostics()
        .iter()
        .find(|item| item.code == code && item.path.to_string() == path)
        .unwrap_or_else(|| panic!("missing {code} at {path}: {error}"));
    assert!(
        diagnostic.span.is_some(),
        "{path}: a text refusal carries a span"
    );
}

#[test]
fn the_console_document_parses_and_lowers_in_slot_order() {
    let source = serde_json::to_string(&console_document()).expect("JSON");
    let model = parse_session_json(&source).expect("console document parses");
    let lowered = model.lower_track(&model.tracks[0]);
    let ids = |effects: &[session::Effect]| {
        effects
            .iter()
            .map(|effect| effect.id.as_str().to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(ids(&lowered.pre_insert), ["desk-eq"]);
    assert_eq!(ids(lowered.inserts), ["eq"]);
    assert_eq!(ids(&lowered.post_insert), ["desk-limit"]);
    // The lowered effect takes identity, quality and link mode from the slot and bypass and params
    // from the track: a lowering that read the wrong side would pass the ID check above.
    let limiter = &lowered.post_insert[0];
    assert!(limiter.bypass);
    assert_eq!(limiter.link_mode, session::LinkMode::Maximum);
    assert!(matches!(
        &limiter.identity,
        session::EffectIdentity::Native { effect_id } if effect_id.as_str() == "miso.true-peak-limiter"
    ));
    assert_eq!(limiter.sidechain, session::SidechainDeclaration::None);
}

/// A per-track console entry carries only `slot`, `bypass` and `params`. Every effect field is
/// refused at its key, so a document cannot give one track its own identity, quality, link mode or
/// sidechain for a slot the session declares once.
#[test]
fn console_entries_refuse_every_effect_field() {
    for (key, value) in [
        ("id", json!("desk-eq")),
        (
            "identity",
            json!({ "kind": "native", "effect_id": "miso.compressor" }),
        ),
        ("quality", json!("high")),
        ("link_mode", json!("maximum")),
        ("sidechain", json!({ "kind": "none" })),
    ] {
        let mut document = console_document();
        document["tracks"][0]["console"][0][key] = value;
        refused(
            &document,
            DiagnosticCode::UnknownField,
            &format!("$.tracks[0].console[0].{key}"),
        );
    }
}

/// A console slot takes no sidechain (a keyed effect is an insert), and its per-track knobs are
/// not declared on the slot.
#[test]
fn console_slots_refuse_a_sidechain_and_per_track_fields() {
    for (section, key, value) in [
        ("pre_insert", "sidechain", json!({ "kind": "none" })),
        (
            "post_insert",
            "sidechain",
            json!({ "kind": "routed", "source": { "kind": "track", "track_id": "vocal", "tap": "input" }, "port_id": "sidechain-in" }),
        ),
        ("pre_insert", "bypass", json!(false)),
        ("post_insert", "params", json!([])),
    ] {
        let mut document = console_document();
        document["console"][section][0][key] = value;
        refused(
            &document,
            DiagnosticCode::UnknownField,
            &format!("$.console.{section}[0].{key}"),
        );
    }
}

/// A track's console entries are exactly the session's slots, in slot order. A validator that
/// matched entries by name alone would accept a misordered track (and then lower it by position
/// onto the wrong slots); one that only checked the count would accept an unknown, a repeated or a
/// missing slot.
#[test]
fn unknown_missing_duplicate_and_misordered_entries_refuse() {
    let mut unknown = console_document();
    unknown["tracks"][0]["console"][1]["slot"] = json!("desk-undeclared");
    refused(
        &unknown,
        DiagnosticCode::MissingEntityReference,
        "$.tracks[0].console[1].slot",
    );

    let mut missing = console_document();
    missing["tracks"][0]["console"]
        .as_array_mut()
        .expect("entries")
        .pop();
    refused(
        &missing,
        DiagnosticCode::ConsoleEntryMissing,
        "$.tracks[0].console",
    );

    let mut duplicate = console_document();
    duplicate["tracks"][0]["console"][1]["slot"] = json!("desk-eq");
    refused(
        &duplicate,
        DiagnosticCode::DuplicateId,
        "$.tracks[0].console[1].slot",
    );

    let mut misordered = console_document();
    misordered["tracks"][0]["console"]
        .as_array_mut()
        .expect("entries")
        .swap(0, 1);
    refused(
        &misordered,
        DiagnosticCode::ConsoleEntryOrder,
        "$.tracks[0].console[0].slot",
    );
    refused(
        &misordered,
        DiagnosticCode::ConsoleEntryOrder,
        "$.tracks[0].console[1].slot",
    );
}

/// A console address names the slot and not its section, so a slot ID repeated across
/// `pre_insert` and `post_insert` would make `(track, console, slot)` ambiguous; and third-party
/// code is never a console slot. A uniqueness check scoped per section, or a slot parser that took
/// any effect identity, passes the populated fixtures and fails here.
#[test]
fn slot_ids_are_unique_across_sections_and_slots_are_native() {
    let mut repeated = console_document();
    repeated["console"]["post_insert"][0]["slot"] = json!("desk-eq");
    refused(
        &repeated,
        DiagnosticCode::DuplicateId,
        "$.console.post_insert[0].slot",
    );

    let mut third_party = console_document();
    third_party["console"]["pre_insert"][0]["identity"] =
        json!({ "kind": "cid", "cid": "bafyopaque" });
    refused(
        &third_party,
        DiagnosticCode::ConsoleSlotNotNative,
        "$.console.pre_insert[0].identity",
    );
}

/// `console` at the root and `console`/`inserts` on a track are required like every V1 key: a
/// reader that defaulted a missing one would silently run a track without its console.
#[test]
fn the_console_and_inserts_keys_are_required() {
    let mut root = console_document();
    root.as_object_mut().expect("root").remove("console");
    refused(&root, DiagnosticCode::MissingField, "$.console");
    for key in ["console", "inserts"] {
        let mut track = console_document();
        track["tracks"][0]
            .as_object_mut()
            .expect("track")
            .remove(key);
        refused(
            &track,
            DiagnosticCode::MissingField,
            &format!("$.tracks[0].{key}"),
        );
    }
}

/// The retired per-track racks are refused, never read as their successors.
#[test]
fn retired_track_rack_keys_refuse() {
    for key in ["simd1", "dynamic", "simd2"] {
        let mut document = console_document();
        document["tracks"][0][key] = json!({ "effects": [] });
        refused(
            &document,
            DiagnosticCode::UnknownField,
            &format!("$.tracks[0].{key}"),
        );
    }
}

/// Every retired tap spelling is an unknown token, never an alias of its renamed successor.
#[test]
fn retired_tap_tokens_refuse() {
    for tap in [
        "post_input_builtins",
        "post_simd1",
        "post_dynamic",
        "post_simd2_pre_fader",
        "post_matrix",
    ] {
        let mut document = console_document();
        document["routes"][0]["source"]["tap"] = json!(tap);
        refused(
            &document,
            DiagnosticCode::InvalidEnum,
            "$.routes[0].source.tap",
        );
        assert_eq!(SendTap::from_token(tap), None, "{tap}");
    }
}

/// The retired rack spellings are unknown automation-target tokens, never aliases: a table that
/// kept `dynamic` as a synonym for `inserts` would silently accept a pre-decision-12 target.
#[test]
fn retired_rack_tokens_refuse() {
    for rack in ["simd1", "dynamic", "simd2"] {
        let mut document = console_document();
        document["automation"][0]["target"]["rack"] = json!(rack);
        refused(
            &document,
            DiagnosticCode::InvalidEnum,
            "$.automation[0].target.rack",
        );
        assert_eq!(RackName::from_token(rack), None, "{rack}");
    }
}

/// `RackName`'s wire codes are explicit: `inserts` kept `dynamic`'s `2`, `builtins` kept `4`,
/// `console` is appended as `5`, and the retired `1` and `3` decode to nothing. An index-derived
/// code would make `inserts` `1` and `builtins` `2`; a widened decoder would reinterpret a retired
/// code as a live rack.
#[test]
fn rack_wire_codes_are_explicit_and_retired_codes_refuse() {
    let expected = [
        (RackName::Inserts, 2, "inserts"),
        (RackName::Builtins, 4, "builtins"),
        (RackName::Console, 5, "console"),
    ];
    assert_eq!(RackName::ALL.len(), expected.len());
    for (value, wire, token) in expected {
        assert_eq!(value.wire(), wire, "{token}");
        assert_eq!(RackName::from_wire(wire), Some(value), "{token}");
        assert_eq!(value.token(), token);
        assert_eq!(RackName::from_token(token), Some(value));
    }
    for wire in (0_u8..=u8::MAX).filter(|wire| ![2, 4, 5].contains(wire)) {
        assert_eq!(RackName::from_wire(wire), None, "code {wire} is not a rack");
    }
}

/// The renamed taps keep their positions and wire codes `1..=7`.
#[test]
fn tap_tokens_keep_their_wire_codes() {
    let expected = [
        (SendTap::Input, "input"),
        (SendTap::PostInput, "post_input"),
        (SendTap::InsertSend, "insert_send"),
        (SendTap::InsertReturn, "insert_return"),
        (SendTap::PreFader, "pre_fader"),
        (SendTap::PostFader, "post_fader"),
        (SendTap::PostPan, "post_pan"),
    ];
    for (code, (value, token)) in (1_u8..).zip(expected) {
        assert_eq!((value.wire(), value.token()), (code, token));
    }
}
