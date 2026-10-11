//! Issue #1394: a live-value edit may carry its own ramp length, recorded as transaction metadata
//! (`EditRamps`) beside the candidate model and bounded at one second of the committed rate.
//!
//! The base is the canonical fixture (track `vocal` with an insert `eq`, route `to-main`, 48 kHz,
//! revision 7) plus a second track `second`.

use protocol::{
    ExpectedRevision, SessionEdit, SessionEditError, SessionRevision, SessionStore,
    SessionStoreError,
};
use session::{CompileCaps, EditRampRow, EditRampTarget, RackName, StableId, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

fn strip(name: &str) -> EditRampTarget {
    EditRampTarget::Strip(id(name))
}

/// The fixture with a second track `second`, committed.
fn store() -> SessionStore {
    let mut model = parse_session_json(FIXTURE).expect("fixture");
    let mut second = model.tracks[0].clone();
    second.id = id("second");
    model.tracks.push(second);
    SessionStore::new(model, caps()).expect("the base compiles")
}

/// The fixture's track; the normalized order puts `second` first.
fn vocal(store: &SessionStore) -> &session::Track {
    let model = store.compiled().normalized_model();
    model
        .tracks
        .iter()
        .find(|t| t.id == id("vocal"))
        .expect("vocal")
}

fn fader(store: &SessionStore, track: &str, ramp_samples: Option<u32>) -> SessionEdit {
    let model = store.compiled().normalized_model();
    let track = model
        .tracks
        .iter()
        .find(|t| t.id == id(track))
        .expect("track");
    SessionEdit::SetTrackFader {
        track_id: track.id.clone(),
        fader: track.fader.clone(),
        ramp_samples,
    }
}

fn recorded(
    store: &SessionStore,
    edits: &[SessionEdit],
) -> Vec<(EditRampTarget, EditRampRow, u32)> {
    store
        .prepare_transaction(ExpectedRevision::Exact(store.revision()), edits)
        .expect("the transaction prepares")
        .edit_ramps()
        .entries()
}

/// #1394 gate 2: the last edit that writes a `(target, row)` decides its length; a fader edit
/// records the fader and the mute rows; `None` removes an earlier length; an upsert removes every
/// entry of its strip.
///
/// Red if a later edit without a length keeps an earlier one, if an edit records the wrong row,
/// target or one row too few, or if an upsert leaves a stale entry.
#[test]
fn the_last_edit_to_write_a_row_decides_its_length() {
    let store = store();
    let matrix = {
        let track = vocal(&store);
        SessionEdit::SetTrackMatrixOrPan {
            track_id: track.id.clone(),
            matrix_or_pan: track.matrix_or_pan.clone(),
            ramp_samples: None,
        }
    };
    let route_gain = SessionEdit::SetRouteGainDb {
        route_id: id("to-main"),
        gain_db: 0.0,
        ramp_samples: Some(0),
    };
    let edits = [
        fader(&store, "vocal", Some(37)),
        matrix.clone(),
        route_gain.clone(),
        fader(&store, "second", Some(5)),
        fader(&store, "second", None),
    ];
    assert_eq!(
        recorded(&store, &edits),
        vec![
            (strip("vocal"), EditRampRow::Fader, 37),
            (strip("vocal"), EditRampRow::Mute, 37),
            (
                EditRampTarget::Route(id("to-main")),
                EditRampRow::RouteGain,
                0
            ),
        ],
        "the vocal fader carries 37 on its fader and mute rows, the route's explicit 0 is an \
         entry, and the second track's later edit without a length removed its 5"
    );

    // The matrix edit with a length writes only the matrix row of its own strip.
    let mut with_matrix = edits.to_vec();
    if let SessionEdit::SetTrackMatrixOrPan { ramp_samples, .. } = &mut with_matrix[1] {
        *ramp_samples = Some(9);
    }
    let entries = recorded(&store, &with_matrix);
    assert!(entries.contains(&(strip("vocal"), EditRampRow::Matrix, 9)));
    assert_eq!(entries.len(), 4);

    // An upsert of the strip after the first edit removes its entries; one before keeps them.
    let upsert = SessionEdit::UpsertTrack {
        track: vocal(&store).clone(),
    };
    let after = [fader(&store, "vocal", Some(37)), upsert.clone()];
    assert_eq!(recorded(&store, &after), vec![]);
    let before = [upsert, fader(&store, "vocal", Some(37))];
    assert_eq!(recorded(&store, &before).len(), 2);
}

/// #1394 D3: every live-value edit records exactly the rows the table names, on the target the
/// table names, and nothing else.
///
/// Red if an edit records the wrong row (a mute edit on the gain row, a follow toggle off the mute
/// row) or the wrong target (an effect's link on the strip).
#[test]
fn every_live_value_edit_records_its_rows_on_its_target() {
    let store = store();
    let model = store.compiled().normalized_model();
    let track = vocal(&store);
    let effect = &track.inserts.effects[0];
    let route = &model.routes[0];
    let edits_and_rows: Vec<(SessionEdit, EditRampTarget, Vec<EditRampRow>)> = vec![
        (
            SessionEdit::SetTrackBuiltins {
                track_id: track.id.clone(),
                builtins: track.builtins.clone(),
                ramp_samples: Some(11),
            },
            strip("vocal"),
            vec![EditRampRow::Input],
        ),
        (
            SessionEdit::SetTrackConsole {
                track_id: track.id.clone(),
                console: track.console.clone(),
                ramp_samples: Some(11),
            },
            strip("vocal"),
            vec![EditRampRow::Link],
        ),
        (
            SessionEdit::SetEffectLinkMode {
                track_id: track.id.clone(),
                rack_name: RackName::Inserts,
                effect_id: effect.id.clone(),
                link_mode: effect.link_mode,
                ramp_samples: Some(11),
            },
            EditRampTarget::Effect {
                strip: track.id.clone(),
                rack: RackName::Inserts,
                effect: effect.id.clone(),
            },
            vec![EditRampRow::Link],
        ),
        (
            SessionEdit::SetConsole {
                console: model.console.clone(),
                ramp_samples: Some(11),
            },
            EditRampTarget::Console,
            vec![EditRampRow::Link],
        ),
        (
            SessionEdit::SetRouteMute {
                route_id: route.id.clone(),
                mute: route.mute,
                ramp_samples: Some(11),
            },
            EditRampTarget::Route(route.id.clone()),
            vec![EditRampRow::RouteMute],
        ),
        (
            SessionEdit::SetRouteFollowsMute {
                route_id: route.id.clone(),
                follows_mute: route.follows_mute,
                ramp_samples: Some(11),
            },
            EditRampTarget::Route(route.id.clone()),
            vec![EditRampRow::RouteMute],
        ),
        (
            SessionEdit::SetRouteChannelMatrix {
                route_id: route.id.clone(),
                channel_matrix: route.channel_matrix.clone(),
                ramp_samples: Some(11),
            },
            EditRampTarget::Route(route.id.clone()),
            vec![EditRampRow::RouteMatrix],
        ),
    ];
    for (edit, target, rows) in edits_and_rows {
        let expected: Vec<_> = rows.iter().map(|row| (target.clone(), *row, 11)).collect();
        assert_eq!(
            recorded(&store, std::slice::from_ref(&edit)),
            expected,
            "{:?}",
            edit.opcode()
        );
        // The same edit without a length records nothing: absence is not an entry.
        let mut bare = edit;
        match &mut bare {
            SessionEdit::SetTrackBuiltins { ramp_samples, .. }
            | SessionEdit::SetTrackConsole { ramp_samples, .. }
            | SessionEdit::SetEffectLinkMode { ramp_samples, .. }
            | SessionEdit::SetConsole { ramp_samples, .. }
            | SessionEdit::SetRouteMute { ramp_samples, .. }
            | SessionEdit::SetRouteFollowsMute { ramp_samples, .. }
            | SessionEdit::SetRouteChannelMatrix { ramp_samples, .. } => *ramp_samples = None,
            other => panic!("unexpected {other:?}"),
        }
        assert_eq!(recorded(&store, &[bare]), vec![]);
    }
}

/// #1394 gate 3: one second at the committed rate commits, one sample more refuses with
/// `RampOutOfRange` at that edit's index, and a refusal changes neither the revision nor the model.
///
/// Red if the bound is off by one, checked against the base rate instead of the rate the
/// transaction commits, applied to the last edit of a row only, or if a refused transaction
/// commits.
#[test]
fn a_length_above_one_second_of_the_committed_rate_refuses_the_whole_transaction() {
    let mut store = store();
    let ok = [fader(&store, "vocal", Some(48_000))];
    store
        .apply_transaction(ExpectedRevision::Exact(store.revision()), &ok)
        .expect("exactly one second commits");

    let before = store.canonical_snapshot().to_owned();
    let revision = store.revision();
    let refuse = |store: &SessionStore, edits: &[SessionEdit]| {
        store
            .prepare_transaction(ExpectedRevision::Exact(store.revision()), edits)
            .err()
            .expect("the transaction refuses")
    };
    let one_over = [
        SessionEdit::SetSessionId {
            session_id: id("renamed"),
        },
        fader(&store, "vocal", Some(48_001)),
    ];
    assert_eq!(
        refuse(&store, &one_over),
        SessionStoreError::Edit {
            operation_index: 1,
            error: SessionEditError::RampOutOfRange,
        }
    );
    assert!(
        store
            .apply_transaction(ExpectedRevision::Exact(revision), &one_over)
            .is_err()
    );
    assert_eq!(store.revision(), revision);
    assert_eq!(store.canonical_snapshot(), before);

    // A later edit of the same row without a length does not excuse an earlier one.
    let overwritten = [
        fader(&store, "vocal", Some(48_001)),
        fader(&store, "vocal", None),
    ];
    assert_eq!(
        refuse(&store, &overwritten),
        SessionStoreError::Edit {
            operation_index: 0,
            error: SessionEditError::RampOutOfRange,
        }
    );

    // The bound is the rate the transaction commits, not the base rate.
    let faster = [
        fader(&store, "vocal", Some(96_000)),
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 96_000,
        },
    ];
    store
        .prepare_transaction(ExpectedRevision::Exact(store.revision()), &faster)
        .expect("one second at the committed 96 kHz");
    let slower = [
        fader(&store, "vocal", Some(48_000)),
        SessionEdit::SetSampleRateHz {
            sample_rate_hz: 44_100,
        },
    ];
    assert_eq!(
        refuse(&store, &slower),
        SessionStoreError::Edit {
            operation_index: 0,
            error: SessionEditError::RampOutOfRange,
        }
    );
    assert_eq!(store.revision(), SessionRevision(revision.0));
}
