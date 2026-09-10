//! The validator's contract: launch fixtures pass preparation; schema examples retain their intended refusals, and a
//! defect is attributed to the stage that actually rejects it.
//!
//! Stage attribution is the whole point of the tool -- an author repairs a grammar typo, a schema
//! violation, a resource cap and a builtin domain error in four different ways -- so the mutation
//! table below is written as `(mutation, stage, code)` triples and fails loudly if a defect ever
//! starts being reported one stage early or late.

use std::path::{Path, PathBuf};

use host_web::{AudioWorkletEngineHost, RESULT_REFUSED_DOCUMENT, WebBootOptions};
use session_validator::{StageStatus, validate_session_document};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root is two levels above tools/<crate>")
        .to_path_buf()
}

fn fixture(name: &str) -> String {
    let path = repository_root().join("fixtures/session/v1").join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn session_fixture_names() -> Vec<String> {
    let directory = repository_root().join("fixtures/session/v1");
    let mut names: Vec<String> = std::fs::read_dir(&directory)
        .expect("fixtures/session/v1 exists")
        .map(|entry| entry.expect("dir entry").file_name())
        .filter_map(|name| name.to_str().map(str::to_owned))
        .filter(|name| name.ends_with(".json"))
        .collect();
    names.sort();
    names
}

#[test]
fn source_identity_format_diagnostics_are_byte_identical_at_validator_and_web_boot() {
    let base = fixture("canonical.json");
    let identity = base
        .lines()
        .find(|line| line.contains("\"content\": \"sha256:"))
        .and_then(|line| line.split("\"content\": \"").nth(1))
        .and_then(|tail| tail.split('"').next())
        .expect("canonical source identity");
    let mut non_hex = identity.to_owned();
    non_hex.replace_range(7..8, "g");
    let uppercase = format!("sha256:{}", identity[7..].to_ascii_uppercase());
    for (label, replacement) in [
        ("wrong-prefix", identity.replacen("sha256:", "sha512:", 1)),
        ("wrong-length", "sha256:abc".to_owned()),
        ("uppercase", uppercase),
        ("non-hex", non_hex),
    ] {
        let document = base.replacen(identity, &replacement, 1);
        let report = validate_session_document(&document);
        assert_eq!(report.failed_stage(), Some(1), "{label}");
        let diagnostics = &report.stages()[1].diagnostics;
        assert_eq!(diagnostics.len(), 1, "{label}");
        let diagnostic = &diagnostics[0];
        assert_eq!(diagnostic.code, "source.content.identity_format", "{label}");
        assert_eq!(diagnostic.path, "$.sources[0].content", "{label}");

        let boot =
            AudioWorkletEngineHost::boot(document.as_bytes(), WebBootOptions::explicit_defaults())
                .err()
                .unwrap_or_else(|| panic!("{label}: web boot accepted invalid identity"));
        assert_eq!(boot.result(), RESULT_REFUSED_DOCUMENT, "{label}");
        assert_eq!(
            boot.diagnostic(),
            format!("{}\t{}\n", diagnostic.code, diagnostic.path).as_bytes(),
            "{label}"
        );
    }
}

#[test]
fn fixtures_distinguish_schema_examples_from_launch_effects() {
    let names = session_fixture_names();
    // A glob that silently matched nothing would make this test vacuous.
    for required in [
        "canonical.json",
        "canonical-minimal.json",
        "console-sixty-four-track.json",
        // The standing 64-track qualification fixture (#175). It is generated rather than
        // authored, and the generator takes its canonical spelling from this very validator, so
        // requiring it here closes the loop: the tool that produced the fixture is the tool that
        // has to keep accepting it.
        "console-sixty-four-track-intended.json",
    ] {
        assert!(
            names.iter().any(|name| name == required),
            "fixture set is missing {required}: {names:?}"
        );
    }
    let mut checked = 0;
    for name in &names {
        if name.contains("invalid") {
            continue;
        }
        let report = validate_session_document(&fixture(name));
        if name == "canonical.json" {
            assert_eq!(report.failed_stage(), Some(4));
            assert_eq!(report.stages()[4].diagnostics.len(), 1);
            assert_eq!(
                report.stages()[4].diagnostics[0].code,
                "effect.native.unavailable"
            );
            assert!(report.canonical().is_none());
            continue;
        }
        assert!(
            report.passed(),
            "{name} must pass every stage:\n{}",
            report.render(name)
        );
        assert!(
            report
                .stages()
                .iter()
                .all(|stage| stage.status == StageStatus::Pass),
            "{name} must run all five stages:\n{}",
            report.render(name)
        );
        assert!(report.canonical().is_some(), "{name} must canonicalize");
        checked += 1;
    }
    assert!(
        checked >= 10,
        "expected the full fixture set, ran {checked}"
    );
}

#[test]
fn the_duplicate_key_fixture_fails_the_grammar_stage() {
    let name = "json-invalid-duplicate-key.json";
    let report = validate_session_document(&fixture(name));
    assert!(!report.passed());
    assert_eq!(report.failed_stage(), Some(0));
    let stage = &report.stages()[0];
    assert_eq!(stage.name, "json-grammar");
    assert_eq!(stage.diagnostics.len(), 1);
    assert_eq!(stage.diagnostics[0].code, "json.syntax");
    for later in &report.stages()[1..] {
        assert_eq!(later.status, StageStatus::Skipped, "{}", later.name);
    }
    assert!(report.canonical().is_none());
}

/// `(fixture, needle, replacement, expected stage index, expected diagnostic code)`.
///
/// Every row is a single textual substitution on a checked-in fixture, so a row that stops
/// applying (because the fixture changed) fails on the substitution rather than passing vacuously.
const MUTATIONS: &[(&str, &str, &str, usize, &str)] = &[
    // Stage 1: grammar.
    (
        "canonical-minimal.json",
        "\"revision\": \"0\"",
        "\"revision\": ",
        0,
        "json.syntax",
    ),
    // Stage 2: strict schema decode and issue-004 validation.
    (
        "canonical-minimal.json",
        "\"schema_version\": 1",
        "\"schema_version\": 0",
        1,
        "schema.version_unsupported",
    ),
    (
        "canonical-minimal.json",
        "\"sample_rate_hz\": 48000",
        "\"sample_rate_hz\": 22050",
        1,
        "sample_rate.unsupported_at_launch",
    ),
    (
        "canonical-minimal.json",
        "\"session_id\": \"minimal.session\"",
        "\"session_id\": \"Minimal.Session\"",
        1,
        "id.invalid",
    ),
    (
        "canonical-minimal.json",
        "\"quantum_frames\": 128",
        "\"quantum_frames\": 0",
        1,
        "capacity.zero",
    ),
    (
        "canonical-minimal.json",
        "\"mode\": \"single_thread\"",
        "\"mode\": \"turbo\"",
        1,
        "schema.invalid_enum",
    ),
    (
        "canonical-minimal.json",
        "\"channels\": 2",
        "\"channels\": 6",
        1,
        "numeric.out_of_schema_range",
    ),
    (
        "canonical.json",
        "\"left_db\": 0.0",
        "\"left_gain\": 0.0",
        1,
        "schema.unknown_field",
    ),
    // Issue #210 phase 2. `delay_samples` is a **required** key like every other V1 key -- the
    // schema has no optional fields -- so deleting it is a missing field, not a default, and
    // exceeding its flat 0..=48000 domain is stage-2 schema work rather than stage-4 DSP work.
    // Both are on the *left* lane so the row is unambiguous about which one it removed.
    (
        "canonical.json",
        "          \"lpf_hz\": 20000.0,\n          \"delay_samples\": 0",
        "          \"lpf_hz\": 20000.0",
        1,
        "schema.missing_field",
    ),
    (
        "canonical.json",
        "\"delay_samples\": 0",
        "\"delay_samples\": 48001",
        1,
        "numeric.out_of_schema_range",
    ),
    (
        "canonical.json",
        "\"delay_samples\": 0",
        "\"delay_samples\": -1",
        1,
        "numeric.out_of_schema_range",
    ),
    (
        "canonical.json",
        "\"output_id\": \"main-out\"",
        "\"output_id\": \"absent-out\"",
        1,
        "reference.missing_entity",
    ),
    (
        "canonical.json",
        "\"unit\": \"db\",\n                \"value\": 0.0",
        "\"unit\": \"hz\",\n                \"value\": -1.0",
        1,
        "numeric.out_of_schema_range",
    ),
    (
        "canonical.json",
        "\"sidechain\": {\n              \"kind\": \"none\"",
        "\"sidechain\": {\n              \"kind\": \"ducking\"",
        1,
        "schema.invalid_enum",
    ),
    (
        "canonical.json",
        "\"shape\": \"linear\"",
        "\"shape\": \"exponential\"",
        1,
        "automation.invalid_range",
    ),
    // Issue #178, ruled by #210's D2: the `rack = "builtins"` automation-target arm. All four are
    // stage-2 refusals, and each names one clause of the arm.
    //
    // `effect_id` carries a fixed validated literal because Session V1 has no optional keys, so
    // the wrong literal has to be a typed diagnostic rather than an ignored field.
    (
        "builtins-automation.json",
        "\"effect_id\": \"strip\",\n        \"parameter_id\": 5",
        "\"effect_id\": \"channel-strip\",\n        \"parameter_id\": 5",
        1,
        "reference.missing_entity",
    ),
    // A `MatrixShared` parameter is one 2x2 for the track; addressing one of its lanes is a
    // category error, not a narrower request.
    (
        "builtins-automation.json",
        "\"parameter_id\": 7,\n        \"channel\": \"both\"",
        "\"parameter_id\": 7,\n        \"channel\": \"left\"",
        1,
        "schema.invalid_enum",
    ),
    // `hpf_hz` (id 3) is `PreparedOnly`: there is no post-preparation write path, so an automation
    // span addressed at it could only ever be inert. The deferred filter tier is reopened by
    // changing the ABI, not by writing a session that quietly does nothing.
    (
        "builtins-automation.json",
        "\"parameter_id\": 2,\n        \"channel\": \"left\"",
        "\"parameter_id\": 3,\n        \"channel\": \"left\"",
        1,
        "reference.missing_entity",
    ),
    // `delay_samples` (id 11) is the same case, and is named separately because its ruling is its
    // own: a delay change re-times the ring.
    (
        "builtins-automation.json",
        "\"parameter_id\": 1,\n        \"channel\": \"right\"",
        "\"parameter_id\": 11,\n        \"channel\": \"right\"",
        1,
        "reference.missing_entity",
    ),
    // The removed document limits are strict unknowns rather than silently ignored host policy.
    (
        "canonical.json",
        "\"sources\": [",
        "\"limits\": {\"memory_bytes\": 1},\n  \"sources\": [",
        1,
        "schema.unknown_field",
    ),
    // Stage 4: off-render builtins preparation, past everything the schema owns.
    (
        "canonical.json",
        "\"hpf_hz\": 20.0",
        "\"hpf_hz\": 900000.0",
        3,
        "builtin.filter.cutoff",
    ),
];

#[test]
fn each_mutation_is_attributed_to_the_stage_that_rejects_it() {
    for (name, needle, replacement, expected_stage, expected_code) in MUTATIONS {
        let source = fixture(name);
        assert!(
            source.contains(needle),
            "{name} no longer contains {needle:?}; the mutation row is stale"
        );
        let mutated = source.replacen(needle, replacement, 1);
        let label = format!("{name} [{needle} -> {replacement}]");
        let report = validate_session_document(&mutated);
        assert!(!report.passed(), "{label} must be rejected");
        assert_eq!(
            report.failed_stage(),
            Some(*expected_stage),
            "{label} was attributed to the wrong stage:\n{}",
            report.render(&label)
        );
        let stage = &report.stages()[*expected_stage];
        assert!(
            stage
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == *expected_code),
            "{label} did not produce {expected_code}:\n{}",
            report.render(&label)
        );
        for earlier in &report.stages()[..*expected_stage] {
            assert_eq!(
                earlier.status,
                StageStatus::Pass,
                "{label}: {}",
                earlier.name
            );
        }
        for later in &report.stages()[expected_stage + 1..] {
            assert_eq!(
                later.status,
                StageStatus::Skipped,
                "{label}: {}",
                later.name
            );
        }
        assert!(
            report.canonical().is_none(),
            "{label} must not canonicalize"
        );
    }
}

#[test]
fn parse_stage_diagnostics_carry_a_source_location_and_preparation_diagnostics_do_not() {
    let schema = validate_session_document(&fixture("canonical.json").replacen(
        "\"left_db\": 0.0",
        "\"left_gain\": 0.0",
        1,
    ));
    for diagnostic in &schema.stages()[1].diagnostics {
        assert!(
            diagnostic.line.is_some() && diagnostic.column.is_some(),
            "parse diagnostics carry a span: {diagnostic:?}"
        );
    }
    let preparation = validate_session_document(&fixture("canonical.json").replacen(
        "\"hpf_hz\": 20.0",
        "\"hpf_hz\": 900000.0",
        1,
    ));
    for diagnostic in &preparation.stages()[3].diagnostics {
        assert!(
            diagnostic.line.is_none() && diagnostic.column.is_none(),
            "typed preparation has no source text: {diagnostic:?}"
        );
    }
}

#[test]
fn canonical_output_reproduces_the_checked_in_canonical_fixtures() {
    for name in ["canonical-minimal.json"] {
        let source = fixture(name);
        let report = validate_session_document(&source);
        assert_eq!(
            report.canonical(),
            Some(source.as_str()),
            "{name} is checked in as canonical"
        );
    }
}

#[test]
fn canonicalization_normalizes_and_is_a_fixed_point() {
    let canonical = fixture("observation-frame-shape.json");
    let value: serde_json::Value = serde_json::from_str(&canonical).expect("fixture JSON");
    let source = serde_json::to_string(&value).expect("noncanonical JSON");
    let first = validate_session_document(&source)
        .canonical()
        .expect("fixture validates")
        .to_owned();
    assert_ne!(first, source, "the fixture is not already canonical");
    let second = validate_session_document(&first)
        .canonical()
        .expect("canonical text revalidates")
        .to_owned();
    assert_eq!(first, second, "canonicalization must be idempotent");
    assert!(first.ends_with('\n'));
    assert!(!first.contains('\r'));
}

#[test]
fn the_report_is_deterministic() {
    let source = fixture("canonical.json").replacen("\"left_db\": 0.0", "\"left_gain\": 0.0", 1);
    let first = validate_session_document(&source).render("session.json");
    let second = validate_session_document(&source).render("session.json");
    assert_eq!(first, second);
}

#[test]
fn effect_preparation_matches_engine_and_cli_refuses_without_canonical_output() {
    use effect_compiler::{
        EffectCompileCaps, launch_native_effect_registry, prepare_native_session_effects,
    };
    use session::{CompileCaps, compile_session, parse_session_json};
    let base: serde_json::Value =
        serde_json::from_str(&fixture("compressor-dynamic-observation.json")).unwrap();
    for (label, expected) in [
        ("ratio-max", None),
        ("ratio-next", Some("effect.parameter.domain")),
        ("unknown-effect", Some("effect.native.unavailable")),
        ("unknown-parameter", Some("effect.parameter.unknown")),
    ] {
        let mut value = base.clone();
        let effect = &mut value["tracks"][0]["dynamic"]["effects"][0];
        match label {
            "ratio-max" => effect["params"][1]["value"] = serde_json::json!(20.0),
            "ratio-next" => {
                effect["params"][1]["value"] =
                    serde_json::json!(f32::from_bits(20.0_f32.to_bits() + 1))
            }
            "unknown-effect" => {
                effect["identity"]["effect_id"] = serde_json::json!("missing.effect")
            }
            "unknown-parameter" => effect["params"][1]["parameter_id"] = serde_json::json!(999),
            _ => unreachable!(),
        }
        let source = serde_json::to_string(&value).unwrap();
        let model = parse_session_json(&source).unwrap();
        let compiled = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .unwrap();
        let engine = prepare_native_session_effects(
            &compiled,
            &launch_native_effect_registry().unwrap(),
            EffectCompileCaps {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: u32::MAX,
            },
        );
        let report = validate_session_document(&source);
        assert_eq!(report.stages().len(), 5);
        assert!(
            report.stages()[..4]
                .iter()
                .all(|stage| stage.status == StageStatus::Pass),
            "{label}"
        );
        assert_eq!(report.passed(), engine.is_ok(), "{label}");
        if let Some(code) = expected {
            assert_eq!(report.failed_stage(), Some(4), "{label}");
            let diagnostics = engine.err().unwrap().0;
            let actual = &report.stages()[4].diagnostics;
            assert!(
                actual.iter().any(|item| item.code == code),
                "{label}: {actual:?}"
            );
            assert_eq!(actual.len(), diagnostics.len());
            for (actual, expected) in actual.iter().zip(diagnostics) {
                assert_eq!(actual.code, expected.code);
                assert_eq!(actual.path, expected.path);
                assert!(actual.line.is_none() && actual.column.is_none());
            }
            assert!(report.canonical().is_none());
        } else {
            assert!(report.canonical().is_some());
        }
        let path = std::env::temp_dir().join(format!(
            "session-validator-211-{}-{label}.json",
            std::process::id()
        ));
        std::fs::write(&path, &source).unwrap();
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_session_validator"))
            .args(["validate", "--canonical"])
            .arg(&path)
            .output()
            .unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(
            output.status.code(),
            Some(i32::from(expected.is_some())),
            "{label}"
        );
        if let Some(code) = expected {
            assert!(output.stdout.is_empty(), "{label}");
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(stderr.contains("FAIL at stage 5 (prepare-effects)"));
            assert!(stderr.contains(code));
            assert!(stderr.contains("$.tracks[id="));
        } else {
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                report.canonical().unwrap()
            );
        }
    }
}
