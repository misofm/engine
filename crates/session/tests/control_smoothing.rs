//! `control_smoothing` (#1054, decision 15 D15-1): the grammar, the canonical form, the default
//! table and the one milliseconds-to-samples conversion every host calls.

use serde_json::{Map, Value, json};
use session::{
    CONTROL_SMOOTHING_DEFAULT, ControlSmoothing, ControlSmoothingSamples, DiagnosticCode,
    DiagnosticSet, canonical_session_json, parse_session_json,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");
const DEFAULT_BLOCK: &str = "  \"control_smoothing\": {\n    \"kind\": \"default\"\n  },\n";

fn document(edit: impl FnOnce(&mut Map<String, Value>)) -> String {
    let mut root: Value = serde_json::from_str(EXAMPLE).expect("fixture JSON");
    edit(root.as_object_mut().expect("root object"));
    serde_json::to_string(&root).expect("serialize")
}

fn with(control_smoothing: Value) -> String {
    document(|root| {
        root.insert("control_smoothing".to_owned(), control_smoothing);
    })
}

fn explicit(mute_ms: f64, fader_ms: f64, pan_ms: f64) -> Value {
    json!({"kind": "explicit", "mute_ms": mute_ms, "fader_ms": fader_ms, "pan_ms": pan_ms})
}

fn diagnostics(error: &DiagnosticSet) -> Vec<(DiagnosticCode, String)> {
    error
        .diagnostics()
        .iter()
        .map(|item| (item.code, item.path.to_string()))
        .collect()
}

#[track_caller]
fn refused(source: &str, expected: &[(DiagnosticCode, &str)]) {
    let error = parse_session_json(source).expect_err("document must refuse");
    let expected: Vec<(DiagnosticCode, String)> = expected
        .iter()
        .map(|(code, path)| (*code, (*path).to_owned()))
        .collect();
    assert_eq!(diagnostics(&error), expected);
}

/// Gate 1, accepted shapes: `default` is `None`; `explicit` keeps 0, `-0.0` and 1000, and `-0.0`
/// keeps its sign. Red if the parser confuses the two kinds or normalizes the lengths.
#[test]
fn default_and_explicit_parse_to_the_model_field() {
    let model = parse_session_json(&with(json!({"kind": "default"}))).expect("default");
    assert_eq!(model.control_smoothing, None);
    let model = parse_session_json(&with(explicit(0.0, 1000.0, 12.5))).expect("explicit");
    assert_eq!(
        model.control_smoothing,
        Some(ControlSmoothing {
            mute_ms: 0.0,
            fader_ms: 1000.0,
            pan_ms: 12.5
        })
    );
    let model = parse_session_json(&with(explicit(-0.0, -0.0, -0.0))).expect("negative zero");
    let lengths = model.control_smoothing.expect("explicit");
    assert!(
        [lengths.mute_ms, lengths.fader_ms, lengths.pan_ms]
            .iter()
            .all(|ms| ms.to_bits() == (-0.0_f32).to_bits()),
        "-0.0 keeps its sign"
    );
}

/// Gate 1, refusals, each at its path. Red if the parser defaults a missing key, accepts a value
/// over 1000 or below 0, or lets a kind carry the other kind's keys.
#[test]
fn every_malformed_control_smoothing_is_refused_at_its_path() {
    let missing = document(|root| drop(root.remove("control_smoothing")));
    refused(
        &missing,
        &[(DiagnosticCode::MissingField, "$.control_smoothing")],
    );
    refused(
        &with(explicit(5.0, -0.001, 5.0)),
        &[(
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.control_smoothing.fader_ms",
        )],
    );
    refused(
        &with(explicit(1000.001, 5.0, 5.0)),
        &[(
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.control_smoothing.mute_ms",
        )],
    );
    refused(
        &with(explicit(5.0, 5.0, 1.0e9)),
        &[(
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.control_smoothing.pan_ms",
        )],
    );
    refused(
        &with(json!({"kind": "explicit", "mute_ms": 5.0, "pan_ms": 5.0})),
        &[(DiagnosticCode::MissingField, "$.control_smoothing.fader_ms")],
    );
    refused(
        &with(json!({"kind": "default", "pan_ms": 20.0})),
        &[(DiagnosticCode::UnknownField, "$.control_smoothing.pan_ms")],
    );
    refused(
        &with(json!({"kind": "linear"})),
        &[(DiagnosticCode::InvalidEnum, "$.control_smoothing.kind")],
    );
    refused(
        &with(json!({"kind": "default", "extra": 1})),
        &[(DiagnosticCode::UnknownField, "$.control_smoothing.extra")],
    );
    refused(
        &with(json!({"mute_ms": 5.0, "fader_ms": 5.0, "pan_ms": 5.0})),
        &[(DiagnosticCode::MissingField, "$.control_smoothing.kind")],
    );
    refused(
        &with(json!("default")),
        &[(DiagnosticCode::WrongType, "$.control_smoothing")],
    );
}

/// Gate 1, round trip: both kinds write back to the same bytes, `-0.0` included, in the canonical
/// place (after `vcas`, before `outputs`). Red if the writer drops, reorders or invents the object.
#[test]
fn both_kinds_round_trip_through_canonical_json_to_identical_bytes() {
    assert!(
        EXAMPLE.contains(DEFAULT_BLOCK),
        "the fixture carries the default key"
    );
    let model = parse_session_json(EXAMPLE).expect("fixture");
    assert_eq!(canonical_session_json(&model).expect("write"), EXAMPLE);

    let block = "  \"control_smoothing\": {\n    \"kind\": \"explicit\",\n    \"mute_ms\": -0.0,\n    \"fader_ms\": 15.0,\n    \"pan_ms\": 1000.0\n  },\n";
    let source = EXAMPLE.replacen(DEFAULT_BLOCK, block, 1);
    assert_ne!(source, EXAMPLE);
    let model = parse_session_json(&source).expect("explicit parses");
    assert_eq!(canonical_session_json(&model).expect("write"), source);
    let at = |needle: &str| source.find(needle).expect("key present");
    assert!(
        at("\"vcas\"") < at("\"control_smoothing\"")
            && at("\"control_smoothing\"") < at("\"outputs\"")
    );
}

fn samples_at(rate: u32, control_smoothing: Option<ControlSmoothing>) -> ControlSmoothingSamples {
    let mut model = parse_session_json(EXAMPLE).expect("fixture");
    model.sample_rate_hz = rate;
    model.control_smoothing = control_smoothing;
    model.control_smoothing_samples()
}

fn all_three(ms: f32) -> Option<ControlSmoothing> {
    Some(ControlSmoothing {
        mute_ms: ms,
        fader_ms: ms,
        pan_ms: ms,
    })
}

/// Gate 2, the rounding: `floor(f64(ms) * rate / 1000 + 1/2)` from the `f32`. Red if the
/// conversion truncates (220 at 5 ms and 44.1 kHz, a tie), rounds half to even (the same 220), or
/// converts in `f32` (45 ms at 44.1 kHz gives 1984, and 263.39 ms gives 11616).
#[test]
fn control_smoothing_samples_round_half_up_in_f64() {
    let rates = [44_100, 48_000, 88_200, 96_000];
    for (ms, expected) in [
        (10.0, [441, 480, 882, 960]),
        (20.0, [882, 960, 1764, 1920]),
        (1000.0, [44_100, 48_000, 88_200, 96_000]),
    ] {
        for (rate, samples) in rates.into_iter().zip(expected) {
            assert_eq!(
                samples_at(rate, all_three(ms)),
                ControlSmoothingSamples {
                    mute: samples,
                    fader: samples,
                    pan: samples
                },
                "{ms} ms at {rate} Hz"
            );
        }
    }
    assert_eq!(
        samples_at(44_100, all_three(5.0)).mute,
        221,
        "a tie rounds up"
    );
    // Two values where a conversion in `f32` gives one sample too many or too few; the expected
    // integers are exact rational arithmetic on the `f32` value, not this crate's formula.
    assert_eq!(
        samples_at(44_100, all_three(45.0)).pan,
        1985,
        "1984.5 is a tie"
    );
    assert_eq!(
        samples_at(44_100, all_three(263.39)).fader,
        11_615,
        "11615.4996 is just under a tie"
    );
    assert_eq!(samples_at(44_100, all_three(-0.0)).fader, 0);
    assert_eq!(samples_at(96_000, all_three(0.0)).pan, 0);
    // The three keys convert independently.
    assert_eq!(
        samples_at(
            48_000,
            Some(ControlSmoothing {
                mute_ms: 5.0,
                fader_ms: 15.0,
                pan_ms: 25.0
            })
        ),
        ControlSmoothingSamples {
            mute: 240,
            fader: 720,
            pan: 1200
        }
    );
}

/// Gate 2, the default: `None` gives the researched table (#1055, `FINDINGS.md` sections 1 and 9) at
/// every launch rate. Red if the default stops being that table.
#[test]
fn the_default_table_is_the_researched_one() {
    assert_eq!(
        CONTROL_SMOOTHING_DEFAULT,
        ControlSmoothing {
            mute_ms: 10.0,
            fader_ms: 20.0,
            pan_ms: 20.0
        }
    );
    for (rate, mute, other) in [
        (44_100, 441, 882),
        (48_000, 480, 960),
        (88_200, 882, 1764),
        (96_000, 960, 1920),
    ] {
        assert_eq!(
            samples_at(rate, None),
            ControlSmoothingSamples {
                mute,
                fader: other,
                pan: other
            },
            "{rate} Hz"
        );
    }
}

/// Gate 2, the matrix property: the default keeps `pan_ms >= 2 x mute_ms`, so a matrix coefficient
/// through zero (a polarity flip of that path on the pan ramp) clicks no more than a mute (#1055,
/// `FINDINGS.md` 9.1). Red if a later default change, such as the #1388 listening result, breaks
/// it, even when the table test above is updated with it.
#[test]
fn the_default_pan_ramp_is_at_least_twice_the_mute_ramp() {
    let defaults = std::hint::black_box(CONTROL_SMOOTHING_DEFAULT);
    assert!(defaults.pan_ms >= 2.0 * defaults.mute_ms);
}
