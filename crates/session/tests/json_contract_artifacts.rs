//! The checked normative JSON Schema artifact.

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
fn session_schema_u64_pattern_executes_the_exact_unsigned_64_bit_domain() {
    let source = fs::read_to_string(workspace("docs/session-v1.schema.json"))
        .expect("read checked JSON Schema");
    let schema: serde_json::Value = serde_json::from_str(&source).expect("valid JSON Schema JSON");
    let u64_schema = &schema["$defs"]["u64"];
    assert_eq!(u64_schema["type"], "string");
    assert!(u64_schema.get("maxLength").is_none());
    let validator = jsonschema::validator_for(&schema).expect("Draft 2020-12 schema compiles");
    let fixture = fs::read_to_string(workspace("fixtures/session/v1/canonical-minimal.json"))
        .expect("read canonical session artifact");
    let base: serde_json::Value = serde_json::from_str(&fixture).expect("valid session JSON");
    assert!(
        validator.is_valid(&base),
        "canonical artifact satisfies schema"
    );

    let mut base = base;
    base["sources"] = serde_json::json!([{
        "id": "voice",
        "content": format!("blake3:{}", "0".repeat(64)),
        "channels": 2,
        "bit_depth": "32f",
        "frames": "1"
    }]);
    assert!(
        validator.is_valid(&base),
        "one-source artifact satisfies schema"
    );

    for valid in [
        "0",
        "1",
        "9999999999999999999",
        "10000000000000000000",
        "18446744073709551614",
        "18446744073709551615",
    ] {
        let mut candidate = base.clone();
        candidate["sources"][0]["frames"] = serde_json::Value::String(valid.to_owned());
        assert!(
            validator.is_valid(&candidate),
            "schema refused valid u64 {valid}"
        );
    }
    for invalid in [
        "",
        "00",
        "01",
        "-1",
        "+1",
        "18446744073709551616",
        "19999999999999999999",
        "184467440737095516150",
    ] {
        let mut candidate = base.clone();
        candidate["sources"][0]["frames"] = serde_json::Value::String(invalid.to_owned());
        assert!(
            !validator.is_valid(&candidate),
            "schema admitted out-of-contract u64 {invalid}"
        );
    }
}

/// The schema's `revision` domain is the engine validator's (#1503): the ceiling is computed from
/// `session::MAX_SESSION_REVISION`, so a drift in either side turns this red.
#[test]
fn session_schema_revision_domain_is_the_validators_ceiling() {
    let source = fs::read_to_string(workspace("docs/session-v1.schema.json"))
        .expect("read checked JSON Schema");
    let schema: serde_json::Value = serde_json::from_str(&source).expect("valid JSON Schema JSON");
    let validator = jsonschema::validator_for(&schema).expect("Draft 2020-12 schema compiles");
    let fixture = fs::read_to_string(workspace("fixtures/session/v1/canonical-minimal.json"))
        .expect("read canonical session artifact");
    let base: serde_json::Value = serde_json::from_str(&fixture).expect("valid session JSON");
    let accepts = |revision: String| {
        let mut candidate = base.clone();
        candidate["revision"] = serde_json::Value::String(revision);
        validator.is_valid(&candidate)
    };

    let ceiling = session::MAX_SESSION_REVISION;
    for valid in [0, 1, 9_999_999_999_999_999_999 / 10, ceiling - 1, ceiling] {
        assert!(
            accepts(valid.to_string()),
            "schema refused revision {valid}"
        );
    }
    for invalid in [
        ceiling + 1,
        ceiling + 2,
        9_999_999_999_999_999_999,
        u64::MAX,
    ] {
        assert!(
            !accepts(invalid.to_string()),
            "schema admitted revision {invalid}"
        );
    }
    for invalid in ["", "01", "-1", "18446744073709551616"] {
        assert!(!accepts(invalid.to_owned()), "schema admitted {invalid:?}");
    }
    // The model the schema accepts at the ceiling is the model the validator accepts.
    let mut model = session::parse_session_json(&fixture).expect("fixture parses");
    model.revision = ceiling;
    session::canonical_session_json(&model).expect("validator accepts the ceiling");
}
