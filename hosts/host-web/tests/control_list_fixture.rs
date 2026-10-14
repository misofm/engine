//! Issue #1564 (decision 17, C4/C5), gate 1: the byte-for-byte reference the SDK's
//! `engine.controls()` is held to.
//!
//! The control core (`controls::list`) walks the #1560 parity session (`common::model_at`) and this
//! test writes each row it yields -- address, class, descriptor facts and committed value, in list
//! order -- beside the canonical session document, as `fixtures/control-list.json`. The SDK eval
//! (`sdk/test/control-list-evals.mjs`) boots the headless engine on that document and compares what
//! `engine.controls()` returns, row by row. The wire is the browser host's own
//! (`SessionControlProvider`), so a field the SDK decodes wrong, a row it drops or a row it reorders
//! turns that eval red.
//!
//! Run with `MISO_ENGINE_BLESS_FIXTURES=1` to rewrite the fixture after a deliberate change to the
//! control tables or the parity session.

use std::fmt::Write as _;
use std::path::PathBuf;

use controls::{
    ControlClass, ControlKind, ControlRow, ControlScope, Family, MonitorOverlay, Owner, Section,
    list,
};
use effect_contract::ParameterUnit;
use session::{ParameterChannel, canonical_session_json, parse_session_json};

mod common;
use common::model_at;

const FIXTURE: &str = "tests/fixtures/control-list.json";
const RATE: u32 = 48_000;

fn owner_json(owner: &Owner) -> String {
    match owner {
        Owner::Session => "{ \"kind\": \"session\" }".to_owned(),
        Owner::Strip(id) => format!("{{ \"kind\": \"strip\", \"id\": \"{}\" }}", id.as_str()),
        Owner::Route(id) => format!("{{ \"kind\": \"route\", \"id\": \"{}\" }}", id.as_str()),
        Owner::Vca(id) => format!("{{ \"kind\": \"vca\", \"id\": \"{}\" }}", id.as_str()),
    }
}

fn section_json(section: &Section) -> String {
    match section {
        Section::Own => "{ \"kind\": \"own\" }".to_owned(),
        Section::Builtins => "{ \"kind\": \"builtins\" }".to_owned(),
        Section::Console(slot) => format!(
            "{{ \"kind\": \"console\", \"slot\": \"{}\" }}",
            slot.as_str()
        ),
        Section::Inserts(effect) => format!(
            "{{ \"kind\": \"inserts\", \"effect\": \"{}\" }}",
            effect.as_str()
        ),
    }
}

const fn lane_name(lane: ParameterChannel) -> &'static str {
    match lane {
        ParameterChannel::Left => "left",
        ParameterChannel::Right => "right",
        ParameterChannel::Both => "both",
    }
}

const fn class_name(class: ControlClass) -> &'static str {
    match class {
        ControlClass::Live => "live",
        ControlClass::RebuildOnly { .. } => "rebuildOnly",
        ControlClass::Derived => "derived",
    }
}

const fn kind_name(kind: ControlKind) -> &'static str {
    match kind {
        ControlKind::Continuous => "continuous",
        ControlKind::Toggle => "toggle",
        ControlKind::Enumeration => "enumeration",
    }
}

const fn family_name(family: Family) -> &'static str {
    match family {
        Family::StripLevel => "stripLevel",
        Family::StripMatrix => "stripMatrix",
        Family::StripInput => "stripInput",
        Family::Route => "route",
        Family::Effect => "effect",
        Family::Setting => "setting",
    }
}

const fn scope_name(scope: ControlScope) -> &'static str {
    match scope {
        ControlScope::Model => "model",
        ControlScope::Monitor => "monitor",
    }
}

const fn unit_name(unit: ParameterUnit) -> &'static str {
    match unit {
        ParameterUnit::Db => "db",
        ParameterUnit::Hz => "hz",
        ParameterUnit::Milliseconds => "milliseconds",
        ParameterUnit::Samples => "samples",
        ParameterUnit::Linear => "linear",
        ParameterUnit::Ratio => "ratio",
    }
}

/// One row as the fixture's JSON object. Floats are written as their IEEE 754 bits, so the SDK's
/// comparison is exact.
fn row_json(row: &ControlRow) -> String {
    let descriptor = &row.descriptor;
    let address = &row.address;
    format!(
        "{{ \"owner\": {}, \"section\": {}, \"control\": {}, \"lane\": \"{}\", \"name\": \"{}\", \
         \"class\": \"{}\", \"kind\": \"{}\", \"unit\": \"{}\", \"scope\": \"{}\", \
         \"family\": \"{}\", \"active\": {}, \"automatable\": {}, \"minimumBits\": \"{:08x}\", \
         \"maximumBits\": \"{:08x}\", \"defaultBits\": \"{:08x}\", \"valueBits\": \"{:08x}\", \
         \"choices\": {} }}",
        owner_json(&address.owner),
        section_json(&address.section),
        address.control,
        lane_name(address.lane),
        descriptor.name,
        class_name(descriptor.class),
        kind_name(descriptor.kind),
        unit_name(descriptor.unit),
        scope_name(descriptor.scope),
        family_name(descriptor.family),
        descriptor.active,
        descriptor.automatable(),
        descriptor.minimum.to_bits(),
        descriptor.maximum.to_bits(),
        descriptor.default.to_bits(),
        row.value.to_bits(),
        descriptor.choices.len(),
    )
}

/// The fixture text: the canonical session document and the native list of it.
fn render() -> String {
    let document = canonical_session_json(&model_at(RATE)).expect("canonical parity session");
    // The model the host boots is the parse of that document, whose routes are in ID order: the
    // order of the fixture's rows is the booted model's, not the builder's.
    let model = parse_session_json(&document).expect("the canonical document parses");
    let rows: Vec<String> = list(&model, &MonitorOverlay::new(), RATE)
        .map(|row| row_json(&row))
        .collect();
    let mut text = String::new();
    writeln!(text, "{{").expect("write to a string");
    writeln!(text, "  \"sampleRate\": {RATE},").expect("write to a string");
    writeln!(
        text,
        "  \"session\": {},",
        serde_json::to_string(&document).expect("a string is JSON")
    )
    .expect("write to a string");
    writeln!(text, "  \"rows\": [").expect("write to a string");
    for (index, row) in rows.iter().enumerate() {
        let comma = if index + 1 == rows.len() { "" } else { "," };
        writeln!(text, "    {row}{comma}").expect("write to a string");
    }
    writeln!(text, "  ]").expect("write to a string");
    writeln!(text, "}}").expect("write to a string");
    text
}

/// Red if the control tables or the parity session change without the SDK's reference changing
/// with them: the SDK eval would then compare the wire to a list the core no longer yields.
#[test]
fn the_control_list_fixture_is_the_native_list_of_the_parity_session() {
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
        "{FIXTURE} differs from controls::list; rerun with MISO_ENGINE_BLESS_FIXTURES=1 after a deliberate change"
    );
}
