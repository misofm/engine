//! Checked normative JSON Schema and migration-inventory artifacts.

use std::{fs, path::PathBuf};

fn workspace(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn assert_objects_are_closed(value: &serde_json::Value, path: &str) {
    match value {
        serde_json::Value::Object(object) => {
            if object.get("type").and_then(serde_json::Value::as_str) == Some("object") {
                assert_eq!(
                    object.get("additionalProperties"),
                    Some(&serde_json::Value::Bool(false)),
                    "object schema at {path} is not closed"
                );
            }
            for (key, child) in object {
                assert_objects_are_closed(child, &format!("{path}/{key}"));
            }
        }
        serde_json::Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                assert_objects_are_closed(child, &format!("{path}/{index}"));
            }
        }
        _ => {}
    }
}

#[test]
fn session_schema_is_draft_2020_12_and_closes_every_object() {
    let source = fs::read_to_string(workspace("docs/session-v1.schema.json"))
        .expect("read checked JSON Schema");
    let schema: serde_json::Value = serde_json::from_str(&source).expect("valid JSON Schema JSON");
    assert_eq!(
        schema.get("$schema").and_then(serde_json::Value::as_str),
        Some("https://json-schema.org/draft/2020-12/schema")
    );
    assert_objects_are_closed(&schema, "#");
}

#[test]
fn migration_inventory_keeps_all_four_classifications_and_reproducible_audit() {
    let inventory = fs::read_to_string(workspace(
        "docs/rulings/canonical-json-migration-inventory.md",
    ))
    .expect("read checked migration inventory");
    for heading in [
        "## Live contract and implementation names",
        "## Current session fixtures",
        "## Generic configuration",
        "## Immutable historical evidence",
        "## Reproducible audit",
    ] {
        assert!(
            inventory.contains(heading),
            "missing inventory section {heading}"
        );
    }
    assert!(inventory.contains("Baseline contains **25** live session-document TOMLs"));
    assert!(inventory.contains("canonical-minimal.json"));
    assert!(inventory.contains("canonical.json"));
    assert!(inventory.contains("parametric-eq-nine-track.json"));
}
