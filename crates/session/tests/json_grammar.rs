//! Strict JSON grammar, duplicate, depth, and unsigned-domain checks.

use session::{DiagnosticCode, canonical_session_json, parse_session_json};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const CANONICAL_MINIMAL: &str = include_str!("../../../fixtures/session/v1/canonical-minimal.json");

fn only<'a>(
    error: &'a session::DiagnosticSet,
    code: DiagnosticCode,
    path: &str,
) -> &'a session::Diagnostic {
    assert_eq!(
        error.diagnostics().len(),
        1,
        "unexpected diagnostics: {error}"
    );
    let diagnostic = &error.diagnostics()[0];
    assert_eq!(diagnostic.code, code);
    assert_eq!(diagnostic.path.to_string(), path);
    diagnostic
}

#[test]
fn insignificant_whitespace_and_shuffled_keys_canonicalize() {
    let mut value: serde_json::Value = serde_json::from_str(CANONICAL).expect("fixture JSON");
    let object = value.as_object_mut().expect("root object");
    let schema = object.remove("schema_version").expect("schema");
    object.insert("schema_version".to_owned(), schema);
    let source = serde_json::to_string(&value).expect("noncanonical JSON");
    let model = parse_session_json(&source).expect("valid noncanonical JSON");
    assert_eq!(
        canonical_session_json(&model).expect("canonical"),
        CANONICAL
    );
}

#[test]
fn decoded_duplicate_path_and_second_key_span_are_exact() {
    let source = r#"{"outer":{"id":"first","\u0069d":"second","ignored":{"x":1}}}"#;
    let error = parse_session_json(source).expect_err("duplicate keys reject before value visit");
    let diagnostic = only(&error, DiagnosticCode::JsonSyntax, "$.outer.id");
    let start = source.find(r#""\u0069d""#).expect("second key");
    let span = diagnostic.span.expect("syntax span");
    assert_eq!(
        (span.byte_start, span.byte_end),
        (start, start + r#""\u0069d""#.len())
    );
}

#[test]
fn duplicates_refuse_at_root_records_and_array_items_before_their_value() {
    let cases = vec![
        (
            r#"{"schema_version":1,"\u0073chema_version": [this value is never parsed"#.to_owned(),
            "$.schema_version",
            r#""\u0073chema_version""#,
        ),
        (
            CANONICAL.replacen(
                r#""mode": "single_thread""#,
                r#""mode": "single_thread", "mode": {invalid"#,
                1,
            ),
            "$.render_profile.mode",
            r#""mode""#,
        ),
        (
            CANONICAL.replacen(
                r#""id": "voice""#,
                r#""id": "voice", "\u0069d": 1e999999999999999999999999999999999999999999999999"#,
                1,
            ),
            "$.sources[0].id",
            r#""\u0069d""#,
        ),
        (
            CANONICAL.replacen(
                r#""shape": "linear""#,
                r#""shape": "linear", "\u0073hape": [invalid"#,
                1,
            ),
            "$.automation[0].segments[0].shape",
            r#""\u0073hape""#,
        ),
    ];
    for (source, path, key) in &cases {
        let error = parse_session_json(source).expect_err("duplicate refuses before its value");
        let diagnostic = only(&error, DiagnosticCode::JsonSyntax, path);
        let start = source.rfind(key).expect("second key");
        let span = diagnostic.span.expect("duplicate span");
        assert_eq!((span.byte_start, span.byte_end), (start, start + key.len()));
        assert_eq!(diagnostic.message, "duplicate object member");
    }
}

#[test]
fn depth_128_accepts_and_opening_depth_129_refuses() {
    let nested = |depth: usize| format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
    let accepted = nested(127); // root object is depth one; nested arrays reach depth 128.
    let source = format!(r#"{{"schema_version":1,"extra":{accepted}}}"#);
    let error =
        parse_session_json(&source).expect_err("shape is invalid but grammar/depth is valid");
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|d| d.code != DiagnosticCode::JsonSyntax),
        "dependency refused contract-valid depth 128: {error}"
    );

    let refused = nested(128);
    let source = format!(r#"{{"schema_version":1,"extra":{refused}}}"#);
    let error = parse_session_json(&source).expect_err("depth 129");
    let expected_path = format!("$.extra{}", "[0]".repeat(127));
    let diagnostic = only(&error, DiagnosticCode::JsonSyntax, &expected_path);
    let opening = source.rfind('[').expect("depth-129 opening");
    let span = diagnostic.span.expect("depth span");
    assert_eq!((span.byte_start, span.byte_end), (opening, opening + 1));
}

#[test]
fn bom_comments_trailing_commas_and_multiple_values_refuse_as_syntax() {
    for source in ["\u{feff}{}", "{/*x*/}", "{\"x\":1,}", "{} {}", "[NaN]"] {
        only(
            &parse_session_json(source).expect_err("invalid JSON"),
            DiagnosticCode::JsonSyntax,
            "$",
        );
    }
    let bom = parse_session_json("\u{feff}{}").expect_err("BOM");
    let span = bom.diagnostics()[0].span.expect("BOM span");
    assert_eq!((span.byte_start, span.byte_end), (0, 3));
}

#[test]
fn u64_decimal_strings_cover_full_domain_and_reject_noncanonical_forms() {
    for value in [
        "0",
        &i64::MAX.to_string(),
        "9223372036854775808",
        "18446744073709551615",
    ] {
        let source = CANONICAL.replacen(
            "\"revision\": \"7\"",
            &format!("\"revision\": \"{value}\""),
            1,
        );
        assert_eq!(
            parse_session_json(&source)
                .expect("valid u64")
                .revision
                .to_string(),
            value
        );
    }
    for replacement in [
        "7",
        "\"-0\"",
        "\"+1\"",
        "\" 1\"",
        "\"01\"",
        "\"18446744073709551616\"",
    ] {
        let source = CANONICAL.replacen(
            "\"revision\": \"7\"",
            &format!("\"revision\": {replacement}"),
            1,
        );
        assert!(
            parse_session_json(&source).is_err(),
            "must reject {replacement}"
        );
    }
}

#[test]
fn every_u64_leaf_accepts_the_full_unsigned_domain() {
    let maximum = "18446744073709551615";
    for (needle, replacement) in [
        (
            "\"revision\": \"7\"",
            format!("\"revision\": \"{maximum}\""),
        ),
        (
            "\"frames\": \"48000\"",
            format!("\"frames\": \"{maximum}\""),
        ),
    ] {
        let source = CANONICAL.replacen(needle, &replacement, 1);
        let model = parse_session_json(&source).expect("full-domain u64 leaf parses");
        let canonical = canonical_session_json(&model).expect("full-domain u64 leaf writes");
        assert!(canonical.contains(&replacement));
    }

    // The semantic interval rule requires start < end, so exercise both leaves at their highest
    // jointly valid values rather than weakening validation merely to test lexical width.
    let source = CANONICAL
        .replacen(
            "\"start_sample\": \"0\"",
            "\"start_sample\": \"18446744073709551614\"",
            1,
        )
        .replacen(
            "\"end_sample\": \"480\"",
            "\"end_sample\": \"18446744073709551615\"",
            1,
        );
    let model = parse_session_json(&source).expect("full-domain automation interval parses");
    assert_eq!(model.automation[0].segments[0].start_sample, u64::MAX - 1);
    assert_eq!(model.automation[0].segments[0].end_sample, u64::MAX);
}

#[test]
fn numeric_lexemes_reach_typed_rules_without_f64_preprocessing() {
    let signed_zero = CANONICAL.replacen("\"trim_db\": 0.0", "\"trim_db\": -0", 1);
    let model = parse_session_json(&signed_zero).expect("negative zero JSON number");
    assert_eq!(
        model.tracks[0].builtins.left.trim_db.to_bits(),
        (-0.0_f32).to_bits()
    );
    assert!(
        canonical_session_json(&model)
            .expect("canonical")
            .contains("\"trim_db\": -0.0")
    );

    let exponent = CANONICAL.replacen("\"trim_db\": 0.0", "\"trim_db\": 1e1", 1);
    assert_eq!(
        parse_session_json(&exponent)
            .expect("finite exponent")
            .tracks[0]
            .builtins
            .left
            .trim_db,
        10.0
    );

    for token in ["1e999999999999999999999999", "3.5e38"] {
        let source = CANONICAL.replacen("\"trim_db\": 0.0", &format!("\"trim_db\": {token}"), 1);
        let error = parse_session_json(&source).expect_err("not finite f32 representable");
        only(
            &error,
            DiagnosticCode::NumericNotF32Representable,
            "$.tracks[0].builtins.left.trim_db",
        );
    }
}

// Issue #387: json-syntax 0.12.5 (and jstrict 0.14.0 before it, on `main`) never calls
// `end_fragment` for an empty JSON object, so the reserved `CodeMap` entry keeps `volume = 0`
// and every later member is read one slot off. `session::parse::Parser::keys` hits
// `Option::unwrap()` on the resulting `None` at `json-syntax-0.12.5/src/object/mod.rs:795:67`.
// No V1 position ever admits `{}` (docs/SESSION_SCHEMA_V1.md: every object rejects unknown keys
// and requires every field explicit), so the preflight in `json_preflight.rs` refuses an empty
// object anywhere in the document, before the typed walk -- and before the dependency's `Value`
// tree is even built -- with `DiagnosticCode::JsonSyntax`, mirroring the existing duplicate-key
// and depth-129 preflight refusals.
#[test]
fn empty_object_refuses_at_every_placement_instead_of_corrupting_the_code_map() {
    let cases = [
        (
            "render_profile",
            r#""id": "native",
    "mode": "single_thread""#,
        ),
        (
            "output_profile",
            r#""id": "main",
    "channels": 2,
    "sample_format": "f32_planar""#,
        ),
    ];
    for (key, inner) in cases {
        let needle = format!("\"{key}\": {{\n    {inner}\n  }}");
        assert!(
            CANONICAL.contains(&needle),
            "fixture shape drifted for {key}"
        );
        let source = CANONICAL.replacen(&needle, &format!("\"{key}\": {{}}"), 1);
        let error = parse_session_json(&source).expect_err("empty object refuses");
        let diagnostic = only(&error, DiagnosticCode::JsonSyntax, &format!("$.{key}"));
        let start = source.find(&format!("\"{key}\": {{}}")).unwrap() + key.len() + 4;
        let span = diagnostic.span.expect("empty-object span");
        assert_eq!((span.byte_start, span.byte_end), (start, start + 2));
        assert_eq!(
            diagnostic.message,
            "empty JSON object {} is never a valid V1 schema value"
        );
    }

    // A track's `builtins` table: the same refusal fires for a nested object below the root.
    let builtins_start = CANONICAL.find("\"builtins\": {").expect("builtins key");
    let mut depth = 0i32;
    let mut cursor = CANONICAL[builtins_start..].find('{').unwrap() + builtins_start;
    let builtins_end = loop {
        match CANONICAL.as_bytes()[cursor] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    break cursor + 1;
                }
            }
            _ => {}
        }
        cursor += 1;
    };
    let source = format!(
        "{}\"builtins\": {{}}{}",
        &CANONICAL[..builtins_start],
        &CANONICAL[builtins_end..]
    );
    let error = parse_session_json(&source).expect_err("empty builtins table refuses");
    let diagnostic = only(&error, DiagnosticCode::JsonSyntax, "$.tracks[0].builtins");
    let start = source.find("\"builtins\": {}").unwrap() + "\"builtins\": ".len();
    let span = diagnostic.span.expect("empty-object span");
    assert_eq!((span.byte_start, span.byte_end), (start, start + 2));
}

#[test]
fn canonical_minimal_empty_render_profile_reports_syntax_not_a_bogus_numeric_range() {
    // Before the fix this fixture reported a spurious `numeric.out_of_schema_range` at
    // `$.output_profile.channels` (the value is `2`, well inside range) plus degenerate
    // `151..151` spans for the real missing-field errors, because the corrupted `CodeMap`
    // misattributed every entry after the empty `render_profile` object.
    let source = CANONICAL_MINIMAL.replacen(
        "\"render_profile\": {\n    \"id\": \"single\",\n    \"mode\": \"single_thread\"\n  }",
        "\"render_profile\": {}",
        1,
    );
    assert!(source != CANONICAL_MINIMAL, "fixture shape drifted");
    let error = parse_session_json(&source).expect_err("empty object refuses");
    let diagnostic = only(&error, DiagnosticCode::JsonSyntax, "$.render_profile");
    let span = diagnostic.span.expect("empty-object span");
    assert_ne!(
        (span.byte_start, span.byte_end),
        (151, 151),
        "span must not degenerate"
    );
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|d| d.code != DiagnosticCode::NumericOutOfSchemaRange),
        "must not report the bogus numeric.out_of_schema_range: {error}"
    );
}

#[test]
fn fuzz_crash_52d9c906_empty_render_profile_diagnoses_instead_of_aborting() {
    // Content-equivalent to fuzz artifact `crash-52d9c906ce5ad7f1d1e67dad91b13ec69e2caab5`
    // (`session_parse`, found by the #385 verifier): `canonical.json` with `render_profile`'s
    // body reduced to whitespace only, which is still an empty object under the JSON grammar.
    // The literal fuzz artifact file was not present on disk in this worktree; this input
    // reproduces the same `Option::unwrap()` panic pre-fix (verified on unmodified `main`/#385).
    let source = CANONICAL.replacen(
        "\"render_profile\": {\n    \"id\": \"native\",\n    \"mode\": \"single_thread\"\n  }",
        "\"render_profile\": {\n    \n  }",
        1,
    );
    let error = parse_session_json(&source).expect_err("empty object refuses");
    only(&error, DiagnosticCode::JsonSyntax, "$.render_profile");
}

#[test]
fn minimal_two_member_object_with_an_empty_first_value_diagnoses_instead_of_aborting() {
    // The raw `CodeMap` illustration from issue #387: `{"a":{},"b":1}` is the smallest input
    // that demonstrates the off-by-one -- an empty object followed by a sibling member whose
    // `CodeMap` slot the corrupted `IterMapped` would misread.
    let error = parse_session_json(r#"{"a":{},"b":1}"#).expect_err("empty object refuses");
    let diagnostic = only(&error, DiagnosticCode::JsonSyntax, "$.a");
    let span = diagnostic.span.expect("empty-object span");
    assert_eq!((span.byte_start, span.byte_end), (5, 7));
}

#[test]
fn syntax_and_typed_spans_are_exact_after_multibyte_text_and_newlines() {
    let source = "{\n  \"🙂\": 0,\n  \"schema_version\": false\n}";
    let error = parse_session_json(source).expect_err("unknown plus wrong type");
    let wrong = error
        .diagnostics()
        .iter()
        .find(|d| d.path.to_string() == "$.schema_version")
        .expect("typed error");
    let start = source.find("false").expect("value");
    let span = wrong.span.expect("typed span");
    assert_eq!(
        (span.byte_start, span.byte_end, span.line, span.column),
        (start, start + 5, 3, 21)
    );

    let malformed = "{\n  \"🙂\": 0,\n  \"x\": ]\n}";
    let error = parse_session_json(malformed).expect_err("syntax");
    let span = error.diagnostics()[0].span.expect("syntax span");
    let start = malformed.find(']').expect("unexpected token");
    assert_eq!(
        (span.byte_start, span.byte_end, span.line),
        (start, start, 3)
    );
}
