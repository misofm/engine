//! Issue #1394: a live-value edit may carry its own ramp length, recorded as transaction metadata
//! (`EditRamps`) beside the candidate model and bounded at one second of the committed rate.
//!
//! The base is the canonical fixture (track `vocal` with an insert `eq`, route `to-main`, 48 kHz,
//! revision 7) plus a second track `second`, a submix `bus` and an empty VCA `grp`.

use protocol::{
    ExpectedRevision, SessionEdit, SessionEditError, SessionRevision, SessionStore,
    SessionStoreError,
};
use session::{
    CompileCaps, EditRampRow, EditRampTarget, RackName, StableId, Submix, Vca, parse_session_json,
};

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

/// The fixture with a second track `second`, a submix `bus` and an empty VCA `grp`, committed.
fn store() -> SessionStore {
    let mut model = parse_session_json(FIXTURE).expect("fixture");
    let mut second = model.tracks[0].clone();
    second.id = id("second");
    let vca = Vca {
        id: id("grp"),
        fader: second.fader.clone(),
        members: vec![],
    };
    model.tracks.push(second);
    model
        .submixes
        .push(Submix::unity(id("bus"), &model.console));
    model.vcas.push(vca);
    // The fixture's automation targets the `eq`; the removal cases below must compile without it.
    model.automation.clear();
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

/// The entries a transaction records: `(target, row, samples)`.
type Recorded = Vec<(EditRampTarget, EditRampRow, u32)>;

fn recorded(store: &SessionStore, edits: &[SessionEdit]) -> Recorded {
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
        (
            SessionEdit::SetVcaFader {
                vca_id: id("grp"),
                fader: model.vcas[0].fader.clone(),
                ramp_samples: Some(11),
            },
            EditRampTarget::Vca(id("grp")),
            vec![EditRampRow::VcaFader, EditRampRow::VcaMute],
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
            | SessionEdit::SetRouteChannelMatrix { ramp_samples, .. }
            | SessionEdit::SetVcaFader { ramp_samples, .. } => *ramp_samples = None,
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

/// #1394 D3: an edit that rewrites or removes a target whose rows `EditRamps` keys carries no
/// length for them, so it clears the target's earlier entries; an edit after it decides afresh.
///
/// Red if a `PutTrackEffect`, `SetTrackRack`, `RemoveTrackEffect`, `RemoveTrack`, `RemoveSubmix`,
/// `RemoveRoute` or `RemoveVca` leaves the stale length of an earlier edit in place (a later reader
/// would ramp a row over a length that its last writer did not give), if `SetTrackRack` takes
/// another strip's effect entries, or if a clear also drops a write that comes after it.
#[test]
fn an_edit_that_rewrites_or_removes_a_target_clears_its_earlier_lengths() {
    let store = store();
    let model = store.compiled().normalized_model();
    let vocal_track = vocal(&store);
    let eq = vocal_track.inserts.effects[0].clone();
    let effect_target = |strip: &str| EditRampTarget::Effect {
        strip: id(strip),
        rack: RackName::Inserts,
        effect: eq.id.clone(),
    };
    let link = |strip: &str, samples: u32| SessionEdit::SetEffectLinkMode {
        track_id: id(strip),
        rack_name: RackName::Inserts,
        effect_id: eq.id.clone(),
        link_mode: eq.link_mode,
        ramp_samples: Some(samples),
    };
    let put = SessionEdit::PutTrackEffect {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        final_position: 0,
        effect: eq.clone(),
    };
    let set_rack = SessionEdit::SetTrackRack {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        rack: vocal_track.inserts.clone(),
    };
    let remove_effect = SessionEdit::RemoveTrackEffect {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        effect_id: eq.id.clone(),
    };
    let strip_fader = |strip: &str, samples: u32| {
        let fader = if strip == "bus" {
            model.submixes[0].fader.clone()
        } else {
            vocal_track.fader.clone()
        };
        SessionEdit::SetTrackFader {
            track_id: id(strip),
            fader,
            ramp_samples: Some(samples),
        }
    };
    let vocal_link = (effect_target("vocal"), EditRampRow::Link, 37);
    let second_link = (effect_target("second"), EditRampRow::Link, 38);
    // The entries of `strip_fader("vocal", 9)`: a rack edit does not touch its strip's own rows.
    let vocal_fader = (
        (strip("vocal"), EditRampRow::Fader, 9),
        (strip("vocal"), EditRampRow::Mute, 9),
    );

    let cases: Vec<(&str, Vec<SessionEdit>, Recorded)> = vec![
        (
            "PutTrackEffect replaces the effect with no length",
            vec![link("vocal", 37), put.clone()],
            vec![],
        ),
        (
            "SetTrackRack replaces every effect of the rack with no length",
            vec![link("vocal", 37), set_rack.clone()],
            vec![],
        ),
        (
            "SetTrackRack clears its own strip's rack only",
            vec![
                link("vocal", 37),
                link("second", 38),
                strip_fader("vocal", 9),
                set_rack.clone(),
            ],
            vec![
                second_link.clone(),
                vocal_fader.0.clone(),
                vocal_fader.1.clone(),
            ],
        ),
        (
            "PutTrackEffect clears its own effect only",
            vec![
                link("vocal", 37),
                link("second", 38),
                strip_fader("vocal", 9),
                put.clone(),
            ],
            vec![second_link, vocal_fader.0, vocal_fader.1],
        ),
        (
            "RemoveTrackEffect removes the effect",
            vec![link("vocal", 37), remove_effect.clone()],
            vec![],
        ),
        (
            "RemoveTrackEffect then PutTrackEffect re-creates it with no length",
            vec![link("vocal", 37), remove_effect, put.clone()],
            vec![],
        ),
        (
            "an edit after the rewrite decides the row",
            vec![put, set_rack.clone(), link("vocal", 37)],
            vec![vocal_link],
        ),
        (
            "RemoveTrack removes the strip's rows and its effects' rows, and only its own",
            vec![
                strip_fader("second", 5),
                link("second", 6),
                strip_fader("vocal", 7),
                SessionEdit::RemoveTrack {
                    track_id: id("second"),
                },
            ],
            vec![
                (strip("vocal"), EditRampRow::Fader, 7),
                (strip("vocal"), EditRampRow::Mute, 7),
            ],
        ),
        (
            "RemoveSubmix removes the strip's rows",
            vec![
                strip_fader("bus", 5),
                SessionEdit::RemoveSubmix {
                    submix_id: id("bus"),
                },
            ],
            vec![],
        ),
        (
            "RemoveRoute removes the route's rows",
            vec![
                SessionEdit::SetRouteGainDb {
                    route_id: id("to-main"),
                    gain_db: 0.0,
                    ramp_samples: Some(5),
                },
                SessionEdit::RemoveRoute {
                    route_id: id("to-main"),
                },
            ],
            vec![],
        ),
        (
            "RemoveVca removes the VCA's rows",
            vec![
                SessionEdit::SetVcaFader {
                    vca_id: id("grp"),
                    fader: model.vcas[0].fader.clone(),
                    ramp_samples: Some(5),
                },
                SessionEdit::RemoveVca { vca_id: id("grp") },
            ],
            vec![],
        ),
    ];
    for (name, edits, expected) in cases {
        assert_eq!(recorded(&store, &edits), expected, "{name}");
    }
}

// ---- #1553: SET_CONTROL --------------------------------------------------------------------

use controls::{ControlAddress, ControlError, Lane, Owner, Section};

/// The builtin IDs the address names (`BUILTIN_PARAMETER_DESCRIPTORS`).
const TRIM_DB: u32 = 2;
const FADER_DB: u32 = 5;
const MUTE: u32 = 6;
const PAN: u32 = 12;

fn builtin(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner: Owner::Strip(id(strip)),
        section: Section::Builtins,
        control,
        lane,
    }
}

fn set_control(address: ControlAddress, value: f32, ramp_samples: Option<u32>) -> SessionEdit {
    SessionEdit::SetControl {
        address,
        value,
        ramp_samples,
    }
}

/// #1553 D4: a `SetControl` records the one row its address names, on that address's owner, and a
/// later one without a length removes it; a fixed-ramp row records nothing.
///
/// Red if the address maps to the wrong row (a fader edit also taking the mute row as the per-kind
/// edit does, a pan on the fader row), the wrong target, or if an absent length keeps an earlier
/// explicit one.
#[test]
fn set_control_records_the_row_its_address_names() {
    let store = store();
    let own = |owner, control| ControlAddress {
        owner,
        section: Section::Own,
        control,
        lane: Lane::Both,
    };
    let edits = [
        set_control(builtin("vocal", FADER_DB, Lane::Left), 0.0, Some(37)),
        set_control(builtin("vocal", PAN, Lane::Both), 0.5, Some(9)),
        set_control(builtin("vocal", TRIM_DB, Lane::Both), 0.0, Some(3)),
        set_control(own(Owner::Route(id("to-main")), 1), 0.0, Some(5)),
        set_control(
            ControlAddress {
                lane: Lane::Left,
                ..own(Owner::Vca(id("grp")), 1)
            },
            0.0,
            Some(7),
        ),
        // The second track's fader is set, then set again without a length.
        set_control(builtin("second", FADER_DB, Lane::Both), 0.0, Some(5)),
        set_control(builtin("second", FADER_DB, Lane::Both), 0.0, None),
    ];
    let entries = recorded(&store, &edits);
    let expected = vec![
        (strip("vocal"), EditRampRow::Fader, 37),
        (strip("vocal"), EditRampRow::Matrix, 9),
        (strip("vocal"), EditRampRow::Input, 3),
        (
            EditRampTarget::Route(id("to-main")),
            EditRampRow::RouteGain,
            5,
        ),
        (EditRampTarget::Vca(id("grp")), EditRampRow::VcaFader, 7),
    ];
    assert_eq!(entries, expected);
}

/// #1553 gate 3 and 4 at the store: a refusal names its edit and its control code and changes
/// nothing, even when an earlier edit of the transaction was valid; an explicit length on a
/// fixed-ramp row is `control.ramp.fixed`; a length above the rate is the existing bound.
///
/// Red if a bad edit lets an earlier good edit commit, if a fixed-ramp row ignores a length, or if
/// the control refusal is reported at the wrong index.
#[test]
fn a_refused_set_control_refuses_the_whole_transaction() {
    let store = store();
    let revision = store.revision();
    let before = store.canonical_snapshot();
    let refuse = |edits: &[SessionEdit]| {
        store
            .prepare_transaction(ExpectedRevision::Exact(revision), edits)
            .err()
            .expect("the transaction refuses")
    };
    let good = set_control(builtin("vocal", FADER_DB, Lane::Both), -3.0, None);
    assert_eq!(
        refuse(&[
            good.clone(),
            set_control(builtin("vocal", PAN, Lane::Both), 5.0, None)
        ]),
        SessionStoreError::Edit {
            operation_index: 1,
            error: SessionEditError::Control(ControlError::Domain),
        }
    );
    // Not finite: the codec carries it, the validator refuses it at its own edit.
    assert_eq!(
        refuse(&[
            good.clone(),
            set_control(builtin("vocal", FADER_DB, Lane::Both), f32::NAN, None)
        ]),
        SessionStoreError::Edit {
            operation_index: 1,
            error: SessionEditError::Control(ControlError::Domain),
        }
    );
    let bypass = ControlAddress {
        owner: Owner::Strip(id("vocal")),
        section: Section::Inserts(id("eq")),
        control: 0,
        lane: Lane::Both,
    };
    // The fixture's effect is not a launch effect (no rows); a launch identity gives it a bypass.
    let launch_eq = SessionEdit::SetEffectIdentity {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        effect_id: id("eq"),
        identity: session::EffectIdentity::Native {
            effect_id: id("miso.parametric-eq"),
        },
    };
    assert_eq!(
        refuse(&[set_control(bypass.clone(), 1.0, None)]),
        SessionStoreError::Edit {
            operation_index: 0,
            error: SessionEditError::Control(ControlError::AddressUnknown),
        }
    );
    assert_eq!(
        refuse(&[
            launch_eq.clone(),
            set_control(bypass.clone(), 1.0, Some(10))
        ]),
        SessionStoreError::Edit {
            operation_index: 1,
            error: SessionEditError::Control(ControlError::RampFixed),
        }
    );
    assert_eq!(
        refuse(&[set_control(
            builtin("vocal", FADER_DB, Lane::Both),
            0.0,
            Some(48_001)
        )]),
        SessionStoreError::Edit {
            operation_index: 0,
            error: SessionEditError::RampOutOfRange,
        }
    );
    // The same bypass without a length commits.
    assert!(
        store
            .prepare_transaction(
                ExpectedRevision::Exact(revision),
                &[launch_eq, set_control(bypass, 1.0, None)]
            )
            .is_ok()
    );
    assert_eq!(store.revision(), revision);
    assert_eq!(store.canonical_snapshot(), before);
}

/// #1553 gate 1 at the model: a `SetControl` writes the same field the per-kind edit writes.
///
/// Red if the address resolves a different field, lane or value than the old op.
#[test]
fn set_control_writes_what_the_per_kind_edit_writes() {
    let base = store();
    let per_kind = {
        let track = vocal(&base);
        let mut fader = track.fader.clone();
        fader.left_db = -4.5;
        SessionEdit::SetTrackFader {
            track_id: track.id.clone(),
            fader,
            ramp_samples: None,
        }
    };
    let by_address = set_control(builtin("vocal", FADER_DB, Lane::Left), -4.5, None);
    let commit = |edit: SessionEdit| {
        let store = self::store();
        store
            .prepare_transaction(ExpectedRevision::Exact(store.revision()), &[edit])
            .expect("prepares")
            .compiled()
            .normalized_model()
            .clone()
    };
    assert_eq!(commit(per_kind), commit(by_address));
    // The mute row has its own address, and the left lane only.
    let model = commit(set_control(builtin("vocal", MUTE, Lane::Left), 1.0, None));
    let track = model.tracks.iter().find(|t| t.id == id("vocal")).unwrap();
    assert!(track.fader.left_mute && !track.fader.right_mute);
}
