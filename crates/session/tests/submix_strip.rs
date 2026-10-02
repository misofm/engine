//! The submix strip's grammar, validation and canonical spelling (#1199).
//!
//! A submix carries the track's strip values -- `builtins`, `inserts`, `fader`, and exactly one of
//! `pan` or `matrix` -- with the track's rules, codes and canonical spelling, at index paths.

use serde_json::{Map, Value, json};
use session::{
    CompileCaps, DiagnosticCode, DiagnosticSet, MatrixOrPan, ParameterChannel, RackName,
    SessionModel, StableId, Submix, canonical_session_json, compile_session, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

fn unlimited_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

/// A submix strip with a non-default value in every field.
fn strip_json() -> Value {
    json!({
        "id": "bus",
        "builtins": {
            "left": {"polarity_invert": true, "trim_db": -2.5, "hpf_hz": 30.0, "lpf_hz": 0.0, "delay_samples": 7},
            "right": {"polarity_invert": false, "trim_db": 1.5, "hpf_hz": 0.0, "lpf_hz": 18000.0, "delay_samples": 48000}
        },
        "inserts": {"effects": [{
            "id": "bus-eq",
            "identity": {"kind": "native", "effect_id": "miso.parametric-eq"},
            "quality": "high",
            "bypass": true,
            "link_mode": "maximum",
            "params": [
                {"parameter_id": 2, "channel": "both", "unit": "hz", "value": 120.0},
                {"parameter_id": 4, "channel": "right", "unit": "db", "value": -1.0}
            ],
            "sidechain": {"kind": "none"}
        }]},
        "fader": {"left_db": -7.5, "right_db": 3.0, "left_mute": true, "right_mute": false},
        "pan": {"left": -0.25, "right": 0.75, "smoothing_samples": 48}
    })
}

fn document(edit: impl FnOnce(&mut Map<String, Value>)) -> String {
    let mut root: Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
    edit(root.as_object_mut().expect("root object"));
    serde_json::to_string(&root).expect("serialize")
}

fn with_strip(strip: Value) -> String {
    document(|root| {
        root.insert("submixes".to_owned(), Value::Array(vec![strip]));
    })
}

fn diagnostics(error: &DiagnosticSet) -> Vec<(DiagnosticCode, String)> {
    error
        .diagnostics()
        .iter()
        .map(|item| (item.code, item.path.to_string()))
        .collect()
}

type StripEdit = fn(&mut Map<String, Value>);

/// Gate 1, the parse and validation halves: every case is refused at the submix's index path with
/// exactly the diagnostics the same edit earns on a track.
#[test]
fn submix_strip_is_refused_with_the_tracks_codes_at_index_paths() {
    let cases: &[(&str, StripEdit, DiagnosticCode, &str)] = &[
        (
            "missing builtins",
            |strip| drop(strip.remove("builtins")),
            DiagnosticCode::MissingField,
            ".builtins",
        ),
        (
            "missing inserts",
            |strip| drop(strip.remove("inserts")),
            DiagnosticCode::MissingField,
            ".inserts",
        ),
        (
            "missing fader",
            |strip| drop(strip.remove("fader")),
            DiagnosticCode::MissingField,
            ".fader",
        ),
        (
            "neither pan nor matrix",
            |strip| {
                strip.remove("pan");
                strip.remove("matrix");
            },
            DiagnosticCode::MissingField,
            "",
        ),
        (
            "both pan and matrix",
            |strip| {
                let pan = json!({"left": 0.0, "right": 0.0, "smoothing_samples": 0});
                let matrix =
                    json!({"ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0, "smoothing_samples": 0});
                strip.insert("pan".to_owned(), pan);
                strip.insert("matrix".to_owned(), matrix);
            },
            DiagnosticCode::WrongType,
            "",
        ),
        (
            "unknown key",
            |strip| drop(strip.insert("send".to_owned(), json!(1))),
            DiagnosticCode::UnknownField,
            ".send",
        ),
        (
            // JSON cannot spell a NaN or an infinity; its non-finite `f32` is a number beyond
            // `f32::MAX`. A model NaN is the validation half's case.
            "fader beyond f32",
            |strip| strip["fader"]["left_db"] = json!(1e39),
            DiagnosticCode::NumericNotF32Representable,
            ".fader.left_db",
        ),
        (
            "pan out of range",
            |strip| {
                strip.remove("matrix");
                let pan = json!({"left": 1.5, "right": 0.0, "smoothing_samples": 0});
                strip.insert("pan".to_owned(), pan);
            },
            DiagnosticCode::NumericOutOfSchemaRange,
            ".pan.left",
        ),
        (
            "delay above the schema maximum",
            |strip| strip["builtins"]["left"]["delay_samples"] = json!(48_001),
            DiagnosticCode::NumericOutOfSchemaRange,
            ".builtins.left.delay_samples",
        ),
    ];
    for &(name, edit, code, suffix) in cases {
        let mut strip = strip_json();
        edit(strip.as_object_mut().expect("strip object"));
        let submix = parse_session_json(&with_strip(strip)).expect_err(name);
        let submix = diagnostics(&submix);
        assert!(
            submix.contains(&(code, format!("$.submixes[0]{suffix}"))),
            "{name}: {submix:?}"
        );

        let track = parse_session_json(&document(|root| {
            edit(root["tracks"][0].as_object_mut().expect("track object"));
        }))
        .expect_err(name);
        let as_track: Vec<_> = submix
            .iter()
            .map(|(code, path)| (*code, path.replacen("$.submixes[0]", "$.tracks[0]", 1)))
            .collect();
        assert_eq!(as_track, diagnostics(&track), "{name}: a track's rule");
    }
}

/// Gate 1, the model half: values the JSON grammar cannot spell reach validation, which runs the
/// track's strip rules on a submix at its index path.
#[test]
fn submix_strip_validation_reports_index_paths() {
    let base = parse_session_json(&with_strip(strip_json())).expect("strip session parses");
    type ModelEdit = fn(&mut Submix);
    let cases: &[(ModelEdit, DiagnosticCode, &str)] = &[
        (
            |submix| submix.fader.right_db = f32::NAN,
            DiagnosticCode::NumericNonFinite,
            "$.submixes[0].fader.right_db",
        ),
        (
            |submix| {
                submix.matrix_or_pan = MatrixOrPan::Pan {
                    left: 0.0,
                    right: 1.5,
                    smoothing_samples: 0,
                }
            },
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.submixes[0].pan.right",
        ),
        (
            |submix| {
                submix.matrix_or_pan = MatrixOrPan::Matrix {
                    ll: 1.0,
                    lr: f32::INFINITY,
                    rl: 0.0,
                    rr: 1.0,
                    smoothing_samples: 0,
                }
            },
            DiagnosticCode::NumericNonFinite,
            "$.submixes[0].matrix.lr",
        ),
        (
            |submix| submix.builtins.right.hpf_hz = -1.0,
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.submixes[0].builtins.right.hpf_hz",
        ),
        (
            |submix| {
                let effect = submix.inserts.effects[0].clone();
                submix.inserts.effects.push(effect);
            },
            DiagnosticCode::DuplicateId,
            "$.submixes[0].inserts.effects[1].id",
        ),
    ];
    for &(edit, code, path) in cases {
        let mut session: SessionModel = base.clone();
        edit(&mut session.submixes[0]);
        let error = canonical_session_json(&session).expect_err(path);
        assert!(
            diagnostics(&error).contains(&(code, path.to_owned())),
            "{path}: {error}"
        );
    }
}

/// Gate 2: a strip with a non-default value in every field writes canonical text, in the key order
/// `id, builtins, inserts, fader, pan|matrix`, that parses back to the same model byte for byte.
#[test]
fn submix_strip_round_trips_canonically_in_key_order() {
    let matrix = {
        let mut strip = strip_json();
        let strip_object = strip.as_object_mut().expect("strip object");
        strip_object.remove("pan");
        strip_object.insert(
            "matrix".to_owned(),
            json!({"ll": 0.5, "lr": -0.25, "rl": 0.125, "rr": 1.5, "smoothing_samples": 9}),
        );
        strip
    };
    for (last, strip) in [("pan", strip_json()), ("matrix", matrix)] {
        let model = parse_session_json(&with_strip(strip)).expect("strip session parses");
        let canonical = canonical_session_json(&model).expect("canonical");
        let reparsed = parse_session_json(&canonical).expect("canonical reparses");
        assert_eq!(reparsed, model, "{last}: no field lost");
        assert_eq!(
            canonical_session_json(&reparsed).expect("canonical"),
            canonical,
            "{last}: byte-stable"
        );

        let start = canonical.find("\"submixes\": [").expect("submixes key");
        let end = start + canonical[start..].find("\n  ],").expect("submixes end");
        let section = &canonical[start..end];
        let positions: Vec<usize> = ["id", "builtins", "inserts", "fader", last]
            .iter()
            .map(|key| {
                section
                    .find(&format!("\n      \"{key}\": "))
                    .unwrap_or_else(|| panic!("{last}: submix key {key}"))
            })
            .collect();
        assert!(
            positions.is_sorted_by(|a, b| a < b),
            "{last}: canonical submix key order {positions:?}"
        );
    }
}

/// `compile_session` canonicalizes a submix's insert parameters by `(parameter_id, channel)`, as it
/// does a track's.
#[test]
fn compile_session_canonicalizes_submix_insert_parameters() {
    let mut model = parse_session_json(&with_strip(strip_json())).expect("strip session parses");
    model.submixes[0].inserts.effects[0].params.reverse();
    assert_eq!(
        model.submixes[0].inserts.effects[0]
            .params
            .iter()
            .map(|param| param.parameter_id)
            .collect::<Vec<_>>(),
        [4, 2],
        "the model keeps declared order"
    );
    let compiled = compile_session(&model, unlimited_caps()).expect("compiles");
    let params = &compiled.normalized_model().submixes[0].inserts.effects[0].params;
    assert_eq!(
        params
            .iter()
            .map(|param| (param.parameter_id, param.channel))
            .collect::<Vec<_>>(),
        [(2, ParameterChannel::Both), (4, ParameterChannel::Right)]
    );
}

/// D6: an automation target may name a submix's insert; a missing insert on it is refused as on a
/// track, and a console target on a submix (which has no console entries yet) names no effect.
#[test]
fn automation_target_may_name_a_submix() {
    let mut model = parse_session_json(&with_strip(strip_json())).expect("strip session parses");
    let target = &mut model.automation[0].target;
    target.entity_id = id("bus");
    target.effect_id = id("bus-eq");
    target.parameter_id = 2;
    target.channel = ParameterChannel::Both;
    canonical_session_json(&model).expect("a submix insert is a valid target");

    let mut missing = model.clone();
    missing.automation[0].target.effect_id = id("absent");
    let error = canonical_session_json(&missing).expect_err("absent insert");
    assert!(
        diagnostics(&error).contains(&(
            DiagnosticCode::MissingEntityReference,
            "$.automation[0].target.effect_id".to_owned()
        )),
        "{error}"
    );

    let mut console = model;
    console.automation[0].target.rack = RackName::Console;
    let error = canonical_session_json(&console).expect_err("submix has no console entry");
    assert!(
        diagnostics(&error).contains(&(
            DiagnosticCode::MissingEntityReference,
            "$.automation[0].target.effect_id".to_owned()
        )),
        "{error}"
    );
}

/// D5: `Submix::unity` is the transparent strip, and it is valid.
#[test]
fn unity_submix_is_transparent_and_valid() {
    let unity = Submix::unity(id("bus"));
    for lane in [&unity.builtins.left, &unity.builtins.right] {
        assert!(!lane.polarity_invert);
        assert_eq!(
            (lane.trim_db, lane.hpf_hz, lane.lpf_hz, lane.delay_samples),
            (0.0, 0.0, 0.0, 0)
        );
    }
    assert!(unity.inserts.effects.is_empty());
    assert_eq!(
        (
            unity.fader.left_db,
            unity.fader.right_db,
            unity.fader.left_mute,
            unity.fader.right_mute
        ),
        (0.0, 0.0, false, false)
    );
    assert_eq!(
        unity.matrix_or_pan,
        MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples: 0
        }
    );
    let mut model = parse_session_json(EXAMPLE).expect("fixture parses");
    model.submixes.push(unity);
    canonical_session_json(&model).expect("a unity strip validates");
}
