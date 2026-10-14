//! A stored automation's target is a control address (#1566): the grammar, the canonical form and
//! the structural checks. Whether the row at the address is automatable is `controls`' rule.

use serde_json::{Value, json};
use session::{
    DiagnosticCode, DiagnosticSet, Owner, ParameterChannel, Section, canonical_session_json,
    parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

/// The fixture with its one automation's `target` replaced and a VCA `group` over `vocal`.
fn document(target: Value) -> String {
    let mut root: Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
    root["automation"][0]["target"] = target;
    root["vcas"] = json!([{
        "id": "group",
        "fader": {"left_db": 0.0, "right_db": 0.0, "left_mute": false, "right_mute": false},
        "members": ["vocal"]
    }]);
    serde_json::to_string(&root).expect("serialize")
}

fn codes(error: &DiagnosticSet) -> Vec<(DiagnosticCode, String)> {
    error
        .diagnostics()
        .iter()
        .map(|item| (item.code, item.path.to_string()))
        .collect()
}

#[track_caller]
fn refused(target: Value, code: DiagnosticCode, path: &str) {
    // A parse refusal and a validation refusal both come out of the parser.
    let error = parse_session_json(&document(target)).expect_err("the target must refuse");
    assert_eq!(codes(&error), [(code, path.to_owned())]);
}

/// Gate 3: the structure of a target. An unknown owner, a route or VCA owner with a section it
/// does not have, an undeclared console slot, an insert the strip lacks, and the session as an
/// owner are each refused at the key that is wrong.
#[test]
fn a_target_must_name_things_that_exist() {
    let missing = DiagnosticCode::MissingEntityReference;
    let strip = |section: Value, slot: Option<&str>| {
        let mut target = json!({
            "owner": "strip", "owner_id": "vocal", "section": section,
            "control": 1, "lane": "both"
        });
        if let Some(slot) = slot {
            target["slot_id"] = json!(slot);
        }
        target
    };
    refused(
        json!({"owner": "strip", "owner_id": "nobody", "section": "builtins", "control": 5, "lane": "both"}),
        missing,
        "$.automation[0].target.owner_id",
    );
    // A VCA's ID is not a strip.
    refused(
        json!({"owner": "strip", "owner_id": "group", "section": "builtins", "control": 5, "lane": "both"}),
        missing,
        "$.automation[0].target.owner_id",
    );
    refused(
        json!({"owner": "route", "owner_id": "nobody", "section": "own", "control": 1, "lane": "both"}),
        missing,
        "$.automation[0].target.owner_id",
    );
    refused(
        json!({"owner": "vca", "owner_id": "vocal", "section": "own", "control": 1, "lane": "both"}),
        missing,
        "$.automation[0].target.owner_id",
    );
    // A route and a VCA have an own section and nothing else.
    for owner in [("route", "to-main"), ("vca", "group")] {
        refused(
            json!({"owner": owner.0, "owner_id": owner.1, "section": "builtins", "control": 1, "lane": "both"}),
            DiagnosticCode::InvalidEnum,
            "$.automation[0].target.section",
        );
    }
    refused(
        strip(json!("console"), Some("not-a-slot")),
        missing,
        "$.automation[0].target.slot_id",
    );
    refused(
        strip(json!("inserts"), Some("not-an-insert")),
        missing,
        "$.automation[0].target.slot_id",
    );
    // No session row is automatable, so `session` is not an owner token.
    refused(
        json!({"owner": "session", "owner_id": "x", "section": "own", "control": 2, "lane": "both"}),
        DiagnosticCode::InvalidEnum,
        "$.automation[0].target.owner",
    );
}

/// The `slot_id` rule is in the grammar: required for `console` and `inserts`, refused for `own`
/// and `builtins`, as a `sidechain` refuses the keys of its other kind.
#[test]
fn slot_id_exists_exactly_in_a_console_or_inserts_section() {
    refused(
        json!({"owner": "strip", "owner_id": "vocal", "section": "own", "slot_id": "eq", "control": 1, "lane": "both"}),
        DiagnosticCode::UnknownField,
        "$.automation[0].target.slot_id",
    );
    refused(
        json!({"owner": "strip", "owner_id": "vocal", "section": "builtins", "slot_id": "eq", "control": 5, "lane": "both"}),
        DiagnosticCode::UnknownField,
        "$.automation[0].target.slot_id",
    );
    refused(
        json!({"owner": "strip", "owner_id": "vocal", "section": "inserts", "control": 1, "lane": "both"}),
        DiagnosticCode::MissingField,
        "$.automation[0].target.slot_id",
    );
}

/// Gate 4: the retired target keys are unknown keys, never aliases.
#[test]
fn the_retired_target_keys_are_unknown() {
    for key in ["entity_id", "rack", "effect_id", "parameter_id", "channel"] {
        let mut target = json!({
            "owner": "strip", "owner_id": "vocal", "section": "builtins", "control": 5, "lane": "both"
        });
        target[key] = json!("vocal");
        let error = parse_session_json(&document(target)).expect_err("an old key must refuse");
        assert_eq!(
            codes(&error),
            [(
                DiagnosticCode::UnknownField,
                format!("$.automation[0].target.{key}")
            )],
            "{key}"
        );
    }
}

/// Gate 4: every owner kind and every section parses to the address it names and writes back as
/// the same canonical bytes.
#[test]
fn every_owner_and_section_round_trips() {
    let strip = |section: &str, slot: Option<&str>, control: u32, lane: &str| {
        let mut target = json!({
            "owner": "strip", "owner_id": "vocal", "section": section,
            "control": control, "lane": lane
        });
        if let Some(slot) = slot {
            target["slot_id"] = json!(slot);
        }
        target
    };
    let targets = [
        strip("builtins", None, 5, "left"),
        strip("own", None, 1, "both"),
        strip("inserts", Some("eq"), 1, "both"),
        json!({"owner": "route", "owner_id": "to-main", "section": "own", "control": 1, "lane": "both"}),
        json!({"owner": "vca", "owner_id": "group", "section": "own", "control": 1, "lane": "both"}),
    ];
    for target in targets {
        let model = parse_session_json(&document(target.clone())).expect("a valid target parses");
        let address = &model.automation[0].target;
        assert_eq!(
            address.owner.kind().name(),
            target["owner"].as_str().expect("owner"),
        );
        assert_eq!(
            address.section.kind().name(),
            target["section"].as_str().expect("section"),
        );
        let canonical = canonical_session_json(&model).expect("canonicalizes");
        let reparsed = parse_session_json(&canonical).expect("canonical text reparses");
        assert_eq!(reparsed.automation, model.automation);
        assert_eq!(
            canonical_session_json(&reparsed).expect("canonicalizes again"),
            canonical
        );
    }
    // The fixture's own automation is the inserts address of the fixture's EQ.
    let model = parse_session_json(EXAMPLE).expect("fixture parses");
    let target = &model.automation[0].target;
    assert!(matches!(&target.owner, Owner::Strip(id) if id.as_str() == "vocal"));
    assert!(matches!(&target.section, Section::Inserts(id) if id.as_str() == "eq"));
    assert_eq!((target.control, target.lane), (1, ParameterChannel::Both));
}
