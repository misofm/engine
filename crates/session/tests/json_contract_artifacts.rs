//! The checked normative JSON Schema artifact.

use std::{collections::BTreeSet, fs, path::PathBuf};

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

/// The schema's automation segment sample domain is the validator's: `0..automation::TIME_LIMIT`.
/// Both leaves are probed at the ceiling read from the constant, so a drift in either side turns
/// this red.
#[test]
fn session_schema_automation_sample_domain_is_the_validators_ceiling() {
    let source = fs::read_to_string(workspace("docs/session-v1.schema.json"))
        .expect("read checked JSON Schema");
    let schema: serde_json::Value = serde_json::from_str(&source).expect("valid JSON Schema JSON");
    let validator = jsonschema::validator_for(&schema).expect("Draft 2020-12 schema compiles");
    let fixture = fs::read_to_string(workspace("fixtures/session/v1/canonical.json"))
        .expect("read canonical session artifact");
    let base: serde_json::Value = serde_json::from_str(&fixture).expect("valid session JSON");
    let segment = |leaf: &str, value: String| {
        let mut candidate = base.clone();
        candidate["automation"][0]["segments"][0][leaf] = serde_json::Value::String(value);
        candidate
    };

    let ceiling = automation::TIME_LIMIT - 1;
    for leaf in ["start_sample", "end_sample"] {
        for valid in [0, 1, 999_999_999_999_999, ceiling - 1, ceiling] {
            assert!(
                validator.is_valid(&segment(leaf, valid.to_string())),
                "schema refused {leaf} {valid}"
            );
        }
        for invalid in [ceiling + 1, ceiling + 2, 9_999_999_999_999_999, u64::MAX] {
            assert!(
                !validator.is_valid(&segment(leaf, invalid.to_string())),
                "schema admitted {leaf} {invalid}"
            );
        }
        for invalid in ["", "01", "-1", "18446744073709551616"] {
            assert!(
                !validator.is_valid(&segment(leaf, invalid.to_owned())),
                "schema admitted {leaf} {invalid:?}"
            );
        }
    }
    // The leaf the schema accepts at the ceiling is the leaf the validator accepts.
    let mut model = session::parse_session_json(&fixture).expect("fixture parses");
    model.automation[0].segments[0].start_sample = ceiling - 1;
    model.automation[0].segments[0].end_sample = ceiling;
    session::canonical_session_json(&model).expect("validator accepts the ceiling");
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

mod walked_keys {
    use std::collections::{BTreeMap, BTreeSet};

    use session::{
        FieldKey, ModelVisitor, SourceBitDepth, StableId, Token, VisitModel, WalkOrder, keys,
        parse_session_json,
    };

    /// `(key module, arm)` to the key names its records emit.
    pub(crate) type Walked = BTreeMap<(String, Option<String>), BTreeSet<String>>;

    #[derive(Default)]
    pub(crate) struct KeySets {
        stack: Vec<Option<(String, Option<String>)>>,
        pub(crate) walked: Walked,
    }

    impl KeySets {
        fn key(&mut self, name: &str) {
            if let Some(Some(shape)) = self.stack.last() {
                self.walked
                    .entry(shape.clone())
                    .or_default()
                    .insert(name.to_owned());
            }
        }
    }

    impl ModelVisitor for KeySets {
        type Error = ();
        fn record_begin(&mut self, key: Option<FieldKey>, _: u32) -> Result<(), ()> {
            if let Some(key) = key {
                self.key(key.name);
            }
            self.stack.push(None);
            Ok(())
        }
        fn record_schema(
            &mut self,
            module: &'static str,
            arm: Option<&'static str>,
        ) -> Result<(), ()> {
            *self.stack.last_mut().ok_or(())? = Some((module.to_owned(), arm.map(str::to_owned)));
            self.walked
                .entry((module.to_owned(), arm.map(str::to_owned)))
                .or_default();
            Ok(())
        }
        fn record_end(&mut self) -> Result<(), ()> {
            self.stack.pop().map(|_| ()).ok_or(())
        }
        fn array_begin(&mut self, key: FieldKey, _: usize) -> Result<(), ()> {
            self.key(key.name);
            self.stack.push(None);
            Ok(())
        }
        fn array_end(&mut self) -> Result<(), ()> {
            self.stack.pop().map(|_| ()).ok_or(())
        }
        fn wire_tag(&mut self, _: Token) -> Result<(), ()> {
            Ok(())
        }
        fn bool(&mut self, key: FieldKey, _: bool) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn u8(&mut self, key: FieldKey, _: u8) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn u32(&mut self, key: FieldKey, _: u32) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn u64(&mut self, key: FieldKey, _: u64) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn source_bit_depth(&mut self, key: FieldKey, _: SourceBitDepth) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn f32(&mut self, key: FieldKey, _: f32) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn id(&mut self, key: FieldKey, _: &StableId) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn id_item(&mut self, _: &StableId) -> Result<(), ()> {
            Ok(())
        }
        fn text(&mut self, key: FieldKey, _: &str) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
        fn token(&mut self, key: FieldKey, _: Token) -> Result<(), ()> {
            self.key(key.name);
            Ok(())
        }
    }

    /// The keys every document of the canonical writer corpus emits, by record shape.
    pub(crate) fn corpus_walk() -> Walked {
        let manifest: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(super::workspace(
                "fixtures/session-canonical/v1/canonical-writer-corpus.json",
            ))
            .expect("read the canonical writer corpus"),
        )
        .expect("corpus is JSON");
        let mut visitor = KeySets::default();
        for entry in manifest["documents"].as_array().expect("documents") {
            let source = match entry["path"].as_str() {
                Some(path) => std::fs::read_to_string(super::workspace(path)).expect("document"),
                None => entry["canonical"]
                    .as_str()
                    .expect("canonical text")
                    .to_owned(),
            };
            parse_session_json(&source)
                .expect("corpus document parses")
                .visit(WalkOrder::Canonical, &mut visitor)
                .expect("walk");
        }
        // The walk must reach every key of every key module, or the comparison below proves less.
        for (module, fields) in keys::MODULES {
            let reached: BTreeSet<&str> = visitor
                .walked
                .iter()
                .filter(|((name, _), _)| name == module)
                .flat_map(|(_, names)| names.iter().map(String::as_str))
                .collect();
            for field in *fields {
                assert!(
                    reached.contains(field.name),
                    "the corpus never walks {module}.{}",
                    field.name
                );
            }
        }
        visitor.walked
    }
}

/// The keys one schema definition declares: the union over the definition itself and every arm of
/// its `oneOf`, `anyOf` and `allOf`, as `(required, properties)`.
fn schema_keys(definition: &serde_json::Value) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut required = BTreeSet::new();
    let mut properties = BTreeSet::new();
    let mut pending = vec![definition];
    while let Some(node) = pending.pop() {
        if let Some(names) = node.get("required").and_then(serde_json::Value::as_array) {
            required.extend(names.iter().filter_map(|n| n.as_str().map(str::to_owned)));
        }
        if let Some(map) = node
            .get("properties")
            .and_then(serde_json::Value::as_object)
        {
            properties.extend(map.keys().cloned());
        }
        for combinator in ["oneOf", "anyOf", "allOf"] {
            if let Some(arms) = node.get(combinator).and_then(serde_json::Value::as_array) {
                pending.extend(arms);
            }
        }
    }
    (required, properties)
}

/// Each `$defs` record and the walked `(key module, arm)` shapes it describes. `"#"` is the
/// document root.
#[allow(clippy::type_complexity)]
const SCHEMA_RECORDS: &[(&str, &[(&str, Option<&str>)])] = &[
    ("#", &[("session", None)]),
    ("render_profile", &[("render_profile", None)]),
    ("output_profile", &[("output_profile", None)]),
    ("source", &[("source", None)]),
    ("id_record", &[("output", None)]),
    ("channel_builtins", &[("channel_builtins", None)]),
    ("builtins", &[("builtins", None)]),
    ("console_slot", &[("console_slot", None)]),
    ("console", &[("console", None)]),
    ("console_entry", &[("console_entry", None)]),
    ("rack", &[("rack", None)]),
    (
        "effect_identity",
        &[("identity", Some("native")), ("identity", Some("cid"))],
    ),
    ("effect_param", &[("param", None)]),
    (
        "route_source",
        &[
            ("route_source", Some("track")),
            ("route_source", Some("submix")),
        ],
    ),
    (
        "sidechain",
        &[("sidechain", Some("none")), ("sidechain", Some("routed"))],
    ),
    ("effect", &[("effect", None)]),
    ("fader", &[("fader", None)]),
    ("pan", &[("matrix_or_pan", Some("pan"))]),
    ("matrix", &[("matrix_or_pan", Some("matrix"))]),
    (
        "track",
        &[("track", Some("pan")), ("track", Some("matrix"))],
    ),
    (
        "submix",
        &[("submix", Some("pan")), ("submix", Some("matrix"))],
    ),
    (
        "control_smoothing",
        &[
            ("control_smoothing", Some("default")),
            ("control_smoothing", Some("explicit")),
        ],
    ),
    ("vca", &[("vca", None)]),
    (
        "route_destination",
        &[
            ("route_destination", Some("submix_input")),
            ("route_destination", Some("output_input")),
        ],
    ),
    ("channel_matrix", &[("channel_matrix", None)]),
    ("route", &[("route", None)]),
    (
        "automation_target",
        &[
            ("target", Some("own")),
            ("target", Some("builtins")),
            ("target", Some("console")),
            ("target", Some("inserts")),
        ],
    ),
    ("automation_segment", &[("segment", None)]),
    ("automation_offset", &[("automation_offset", None)]),
    ("automation", &[("automation", None)]),
];

/// `$defs` entries that declare keys but are not a walked record: `track_base` is the shared
/// property list the two `track` arms repeat.
const NOT_A_RECORD: &[&str] = &["track_base"];

#[test]
fn session_schema_lists_exactly_the_walked_keys() {
    let source = fs::read_to_string(workspace("docs/session-v1.schema.json"))
        .expect("read checked JSON Schema");
    let schema: serde_json::Value = serde_json::from_str(&source).expect("valid JSON Schema JSON");
    let walked = walked_keys::corpus_walk();

    // Every walked shape is described by some definition.
    let described: BTreeSet<(String, Option<String>)> = SCHEMA_RECORDS
        .iter()
        .flat_map(|(_, shapes)| shapes.iter())
        .map(|(module, arm)| ((*module).to_owned(), arm.map(str::to_owned)))
        .collect();
    let walked_shapes: BTreeSet<_> = walked.keys().cloned().collect();
    assert_eq!(
        walked_shapes, described,
        "the schema table and the walk disagree on the record shapes; add the new record to SCHEMA_RECORDS"
    );

    // Every definition that declares keys is in the table.
    let table: BTreeSet<&str> = SCHEMA_RECORDS.iter().map(|(name, _)| *name).collect();
    for (name, definition) in schema["$defs"].as_object().expect("$defs") {
        let (required, properties) = schema_keys(definition);
        if required.is_empty() && properties.is_empty() {
            continue;
        }
        assert!(
            table.contains(name.as_str()) || NOT_A_RECORD.contains(&name.as_str()),
            "$defs/{name} declares keys but no walked record maps to it"
        );
    }

    for (name, shapes) in SCHEMA_RECORDS {
        let definition = if *name == "#" {
            &schema
        } else {
            schema["$defs"]
                .get(*name)
                .unwrap_or_else(|| panic!("the schema has no $defs/{name}"))
        };
        let expected: BTreeSet<String> = shapes
            .iter()
            .flat_map(|(module, arm)| {
                walked[&((*module).to_owned(), arm.map(str::to_owned))]
                    .iter()
                    .cloned()
            })
            .collect();
        let (required, properties) = schema_keys(definition);
        assert_eq!(
            required, expected,
            "$defs/{name}: `required` differs from the keys the engine writes"
        );
        assert_eq!(
            properties, expected,
            "$defs/{name}: `properties` differs from the keys the engine writes"
        );
    }
}
