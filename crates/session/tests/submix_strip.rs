//! The submix strip's grammar, validation and canonical spelling (#1199).
//!
//! A submix carries the track's strip values -- `builtins`, `console`, `inserts`, `fader`, and
//! exactly one of `pan` or `matrix` -- with the track's rules, codes and canonical spelling, at
//! index paths. Its `console` entries (#1202) follow the track's console-entry rules.

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
        "console": [],
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

/// A console whose slot order (`tone`, then `ceiling`) is not canonical ID order, so an entry
/// array sorted by ID would be refused.
fn console_json() -> Value {
    json!({
        "pre_insert": [{
            "slot": "tone",
            "identity": {"kind": "native", "effect_id": "miso.parametric-eq"},
            "quality": "normal",
            "link_mode": "dual_mono"
        }],
        "post_insert": [{
            "slot": "ceiling",
            "identity": {"kind": "native", "effect_id": "miso.true-peak-limiter"},
            "quality": "normal",
            "link_mode": "maximum"
        }]
    })
}

/// One entry per [`console_json`] slot, in slot order: a live EQ with two parameters (in canonical
/// order) and a bypassed limiter.
fn console_entries() -> Value {
    json!([
        {"slot": "tone", "bypass": false, "params": [
            {"parameter_id": 2, "channel": "left", "unit": "hz", "value": 90.0},
            {"parameter_id": 4, "channel": "both", "unit": "db", "value": 1.5}
        ]},
        {"slot": "ceiling", "bypass": true, "params": []}
    ])
}

/// The fixture with [`console_json`] declared and the track carrying [`console_entries`].
fn console_document(edit: impl FnOnce(&mut Map<String, Value>)) -> String {
    document(|root| {
        root.insert("console".to_owned(), console_json());
        root["tracks"][0]["console"] = console_entries();
        edit(root);
    })
}

/// [`console_document`] with `strip` as its one submix.
fn with_console_strip(strip: Value) -> String {
    console_document(|root| {
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
            "missing console",
            |strip| drop(strip.remove("console")),
            DiagnosticCode::MissingField,
            ".console",
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

type ConsoleEdit = fn(&mut Vec<Value>);

/// #1202 gate 4: a submix's console entries are refused with exactly the diagnostics the same
/// entries earn on a track, at the submix's index path.
#[test]
fn submix_console_entries_are_refused_with_the_tracks_codes_at_index_paths() {
    let cases: &[(&str, ConsoleEdit, DiagnosticCode, &str)] = &[
        (
            "missing entry",
            |entries| drop(entries.pop()),
            DiagnosticCode::ConsoleEntryMissing,
            ".console",
        ),
        (
            "entries out of order",
            |entries| entries.swap(0, 1),
            DiagnosticCode::ConsoleEntryOrder,
            ".console[0].slot",
        ),
        (
            "entries in canonical ID order",
            |entries| entries.sort_by(|a, b| a["slot"].as_str().cmp(&b["slot"].as_str())),
            DiagnosticCode::ConsoleEntryOrder,
            ".console[1].slot",
        ),
        (
            "an undeclared slot",
            |entries| entries[0]["slot"] = json!("ghost"),
            DiagnosticCode::MissingEntityReference,
            ".console[0].slot",
        ),
        (
            "a repeated slot",
            |entries| entries[1] = entries[0].clone(),
            DiagnosticCode::DuplicateId,
            ".console[1].slot",
        ),
        (
            "an effect field on an entry",
            |entries| entries[0]["quality"] = json!("high"),
            DiagnosticCode::UnknownField,
            ".console[0].quality",
        ),
    ];
    let strip = || {
        let mut strip = strip_json();
        strip["console"] = console_entries();
        strip
    };
    parse_session_json(&with_console_strip(strip())).expect("the valid console strip parses");
    for &(name, edit, code, suffix) in cases {
        let mut submix = strip();
        edit(submix["console"].as_array_mut().expect("entries"));
        let submix = parse_session_json(&with_console_strip(submix)).expect_err(name);
        let submix = diagnostics(&submix);
        assert!(
            submix.contains(&(code, format!("$.submixes[0]{suffix}"))),
            "{name}: {submix:?}"
        );

        let track = parse_session_json(&console_document(|root| {
            edit(
                root["tracks"][0]["console"]
                    .as_array_mut()
                    .expect("entries"),
            );
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
/// `id, builtins, console, inserts, fader, pan|matrix` (#1202 D1), that parses back to the same
/// model byte for byte. Its console entries keep their declared (slot) order.
#[test]
fn submix_strip_round_trips_canonically_in_key_order() {
    let console_strip = || {
        let mut strip = strip_json();
        strip["console"] = console_entries();
        strip
    };
    let matrix = {
        let mut strip = console_strip();
        let strip_object = strip.as_object_mut().expect("strip object");
        strip_object.remove("pan");
        strip_object.insert(
            "matrix".to_owned(),
            json!({"ll": 0.5, "lr": -0.25, "rl": 0.125, "rr": 1.5, "smoothing_samples": 9}),
        );
        strip
    };
    for (last, strip) in [("pan", console_strip()), ("matrix", matrix)] {
        let model = parse_session_json(&with_console_strip(strip)).expect("strip session parses");
        assert_eq!(
            model.submixes[0]
                .console
                .iter()
                .map(|entry| entry.slot.as_str())
                .collect::<Vec<_>>(),
            ["tone", "ceiling"]
        );
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
        let positions: Vec<usize> = ["id", "builtins", "console", "inserts", "fader", last]
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

/// `compile_session` canonicalizes a submix's console-entry (#1202) and insert parameters by
/// `(parameter_id, channel)`, as it does a track's, and keeps its entries in slot order.
#[test]
fn compile_session_canonicalizes_submix_parameters() {
    let mut strip = strip_json();
    strip["console"] = console_entries();
    let mut model = parse_session_json(&with_console_strip(strip)).expect("strip session parses");
    model.submixes[0].inserts.effects[0].params.reverse();
    model.submixes[0].console[0].params.reverse();
    let declared = |params: &[session::EffectParam]| {
        params
            .iter()
            .map(|param| (param.parameter_id, param.channel))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        declared(&model.submixes[0].inserts.effects[0].params),
        [(4, ParameterChannel::Right), (2, ParameterChannel::Both)],
        "the model keeps the insert's declared order"
    );
    assert_eq!(
        declared(&model.submixes[0].console[0].params),
        [(4, ParameterChannel::Both), (2, ParameterChannel::Left)],
        "the model keeps the entry's declared order"
    );
    let compiled = compile_session(&model, unlimited_caps()).expect("compiles");
    let submix = &compiled.normalized_model().submixes[0];
    assert_eq!(
        declared(&submix.inserts.effects[0].params),
        [(2, ParameterChannel::Both), (4, ParameterChannel::Right)]
    );
    assert_eq!(
        declared(&submix.console[0].params),
        [(2, ParameterChannel::Left), (4, ParameterChannel::Both)]
    );
    assert_eq!(
        submix
            .console
            .iter()
            .map(|entry| entry.slot.as_str())
            .collect::<Vec<_>>(),
        ["tone", "ceiling"],
        "entries keep slot order, not ID order"
    );
}

/// D6: an automation target may name a submix's insert; a missing insert on it is refused as on a
/// track. A console target on a submix names the submix's entry for the slot (#1199 verdict
/// MINOR-2, #1202): accepted when the entry carries the parameter, refused at the target's
/// `effect_id` when the slot is absent and at its `parameter_id` when the parameter is.
#[test]
fn automation_target_may_name_a_submix() {
    let mut strip = strip_json();
    strip["console"] = console_entries();
    let mut model = parse_session_json(&with_console_strip(strip)).expect("strip session parses");
    let target = &mut model.automation[0].target;
    target.entity_id = id("bus");
    target.effect_id = id("bus-eq");
    target.parameter_id = 2;
    target.channel = ParameterChannel::Both;
    canonical_session_json(&model).expect("a submix insert is a valid target");

    let refused_at = |model: &SessionModel, field: &str| {
        let error = canonical_session_json(model).expect_err(field);
        assert!(
            diagnostics(&error).contains(&(
                DiagnosticCode::MissingEntityReference,
                format!("$.automation[0].target.{field}")
            )),
            "{field}: {error}"
        );
    };
    let mut missing = model.clone();
    missing.automation[0].target.effect_id = id("absent");
    refused_at(&missing, "effect_id");

    let mut console = model;
    let target = &mut console.automation[0].target;
    target.rack = RackName::Console;
    target.effect_id = id("tone");
    target.parameter_id = 2;
    target.channel = ParameterChannel::Left;
    canonical_session_json(&console).expect("a submix console entry is a valid target");

    let mut absent_slot = console.clone();
    absent_slot.automation[0].target.effect_id = id("bus-eq");
    refused_at(&absent_slot, "effect_id");

    let mut absent_param = console;
    absent_param.automation[0].target.channel = ParameterChannel::Right;
    refused_at(&absent_param, "parameter_id");
}

/// D5 (#1199) and D4 (#1202): `Submix::unity` is the transparent strip -- one bypassed entry with
/// no parameters per declared console slot, in slot order -- and it is valid.
#[test]
fn unity_submix_is_transparent_and_valid() {
    let model = parse_session_json(&console_document(|_| ())).expect("console fixture parses");
    let unity = Submix::unity(id("bus"), &model.console);
    for lane in [&unity.builtins.left, &unity.builtins.right] {
        assert!(!lane.polarity_invert);
        assert_eq!(
            (lane.trim_db, lane.hpf_hz, lane.lpf_hz, lane.delay_samples),
            (0.0, 0.0, 0.0, 0)
        );
    }
    assert_eq!(
        unity
            .console
            .iter()
            .map(|entry| (entry.slot.as_str(), entry.bypass, entry.params.len()))
            .collect::<Vec<_>>(),
        [("tone", true, 0), ("ceiling", true, 0)],
        "every slot, in slot order, bypassed, with no parameters"
    );
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
    let mut with_console = model;
    with_console.submixes.push(unity);
    canonical_session_json(&with_console).expect("a unity strip validates under a console");

    let mut bare = parse_session_json(EXAMPLE).expect("fixture parses");
    let unity = Submix::unity(id("bus"), &bare.console);
    assert!(unity.console.is_empty(), "no slot, no entry");
    bare.submixes.push(unity);
    canonical_session_json(&bare).expect("a unity strip validates without a console");
}
