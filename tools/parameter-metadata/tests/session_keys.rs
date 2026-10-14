//! Issue #1569: the session key tables the SDK writer reads are the walk's own, and the generator
//! refuses a table it cannot prove complete.

use std::path::Path;

use parameter_metadata::session_keys::{self, Recorder};
use session::{FieldKey, MatrixOrPan, ModelVisitor, SessionModel, keys};

fn repository_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn corpus() -> Vec<SessionModel> {
    session_keys::load_corpus(&repository_root()).expect("load the canonical writer corpus")
}

/// `sdk/assets/miso-engine-v1-session-keys.json` is what `--print-session-keys` writes. The SDK
/// checks of `check-sdk-generated.sh` compare the generated TypeScript with this asset; this holds
/// the asset to the walk even where the check runs against an engine artifact directory, which
/// does not carry it.
#[test]
fn session_keys_asset_is_current() {
    let path = repository_root()
        .join("sdk/assets")
        .join(session_keys::OUTPUT_NAME);
    let checked = std::fs::read_to_string(&path).expect("read the checked session key tables");
    assert_eq!(
        checked,
        session_keys::render(),
        "{} is stale; run `node codegen/assets.mjs` in sdk/",
        path.display()
    );
}

/// A corpus that never walks a `matrix` strip arm leaves the SDK unable to write one; the
/// generator must say which arm instead of emitting a table without it.
#[test]
fn a_corpus_without_the_matrix_arm_is_refused_by_name() {
    let mut models = corpus();
    for model in &mut models {
        for strip in model
            .tracks
            .iter_mut()
            .map(|t| &mut t.matrix_or_pan)
            .chain(model.submixes.iter_mut().map(|s| &mut s.matrix_or_pan))
        {
            *strip = MatrixOrPan::Pan {
                left: 1.0,
                right: 1.0,
                smoothing_samples: 0,
            };
        }
    }
    let error = session_keys::build(&models).expect_err("a corpus with no matrix arm is refused");
    assert!(
        error.contains("matrix_or_pan/matrix"),
        "the refusal names the missing arm: {error}"
    );
}

/// `0` and `0.0` spell differently, so a leaf name walked as both an integer and a float has no
/// single spelling for the SDK to choose.
#[test]
fn a_numeric_key_walked_as_two_kinds_is_refused_by_name() {
    let trim_db: FieldKey = keys::channel_builtins::TRIM_DB;
    let mut recorder = Recorder::default();
    recorder.record_begin(None, 1).expect("begin");
    recorder
        .record_schema("channel_builtins", None)
        .expect("schema");
    recorder.f32(trim_db, 0.0).expect("trim_db as f32");
    recorder.record_end().expect("end");
    recorder.record_begin(None, 1).expect("begin");
    recorder
        .record_schema("channel_builtins", None)
        .expect("schema");
    let error = recorder
        .u32(trim_db, 0)
        .expect_err("trim_db as u32 conflicts");
    assert!(
        error.contains("trim_db"),
        "the refusal names the key: {error}"
    );
}

/// One shape walked with two key lists is a record whose output depends on the document.
#[test]
fn one_shape_walked_with_two_key_lists_is_refused() {
    let mut recorder = Recorder::default();
    recorder.record_begin(None, 1).expect("begin");
    recorder.record_schema("fader", None).expect("schema");
    recorder.bool(keys::fader::LEFT_MUTE, false).expect("key");
    recorder.record_end().expect("end");
    recorder.record_begin(None, 1).expect("begin");
    recorder.record_schema("fader", None).expect("schema");
    recorder.bool(keys::fader::RIGHT_MUTE, false).expect("key");
    let error = recorder
        .record_end()
        .expect_err("a second key list conflicts");
    assert!(
        error.contains("fader"),
        "the refusal names the shape: {error}"
    );
}

fn walk_arm(recorder: &mut Recorder, arm: &'static str, order: [FieldKey; 2]) {
    recorder.record_begin(None, 2).expect("begin");
    recorder
        .record_schema("route_source", Some(arm))
        .expect("schema");
    for key in order {
        recorder.bool(key, false).expect("key");
    }
    recorder.record_end().expect("end");
}

/// Two arms under one parent with one key set are a tie. The SDK takes the first, so the
/// generator must prove the others write the same bytes.
#[test]
fn a_tie_must_agree_on_key_order_and_kinds() {
    let (a, b) = (keys::fader::LEFT_MUTE, keys::fader::RIGHT_MUTE);
    let record_under_parent = |first: [FieldKey; 2], second: [FieldKey; 2]| {
        let mut recorder = Recorder::default();
        recorder.record_begin(None, 1).expect("root");
        recorder.record_schema("session", None).expect("schema");
        recorder
            .array_begin(keys::session::TRACKS, 2)
            .expect("array");
        walk_arm(&mut recorder, "track", first);
        walk_arm(&mut recorder, "submix", second);
        recorder.array_end().expect("end");
        recorder.check_ties()
    };
    record_under_parent([a, b], [a, b]).expect("a tie with the same order is allowed");
    let error = record_under_parent([a, b], [b, a]).expect_err("a tie with another order fails");
    assert!(
        error.contains("route_source/track") && error.contains("route_source/submix"),
        "the refusal names both candidates: {error}"
    );
}
