//! Issue #1394: a live-value edit may carry its own ramp length, recorded as transaction metadata
//! (`EditRamps`) beside the candidate model and bounded at one second of the committed rate.
//! Since decision 17 (#1561) the value edits are `SET_CONTROL`, `SetConsole` and `SetTrackConsole`.
//!
//! The base is the canonical fixture (track `vocal` with an insert `eq`, route `to-main`, 48 kHz,
//! revision 7) plus a second track `second`, a submix `bus` and an empty VCA `grp`.

use protocol::control_core::{ControlAddress, ControlError, Lane, Owner, Section, row_address};
use protocol::{
    ExpectedRevision, SessionEdit, SessionEditError, SessionRevision, SessionStore,
    SessionStoreError,
};
use session::{
    CompileCaps, ConsoleEntry, ConsoleSlot, EditRampRow, EditRampTarget, EffectIdentity,
    EffectQuality, LinkMode, RackName, StableId, Submix, Vca, parse_session_json,
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
    // One console slot, so that a `SetConsole` can change the console (a `SetTrackConsole` is
    // accepted only after one that does) without rewriting any strip's entries.
    model.console.pre_insert.push(ConsoleSlot {
        slot: id("comp"),
        identity: EffectIdentity::Native {
            effect_id: id("miso.compressor"),
        },
        quality: EffectQuality::Normal,
        link_mode: LinkMode::Maximum,
    });
    for track in &mut model.tracks {
        track.console.push(ConsoleEntry {
            slot: id("comp"),
            bypass: false,
            link_mode: session::ConsoleLinkMode::Slot,
            params: Vec::new(),
        });
    }
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

/// A `SET_CONTROL` of `track`'s fader dB on both lanes to the value it already holds, with
/// `ramp_samples`: the fader row of the strip, and nothing else.
fn fader(store: &SessionStore, track: &str, ramp_samples: Option<u32>) -> SessionEdit {
    let model = store.compiled().normalized_model();
    let track = model
        .tracks
        .iter()
        .find(|t| t.id == id(track))
        .expect("track");
    set_control(
        builtin(track.id.as_str(), FADER_DB, Lane::Both),
        track.fader.left_db,
        ramp_samples,
    )
}

/// The edit that changes the console without touching the slot set (the slot's quality moves), so
/// a `SetTrackConsole` may follow (decision 17, #1561 D3).
fn declare(store: &SessionStore, ramp_samples: Option<u32>) -> SessionEdit {
    let mut console = store.compiled().normalized_model().console.clone();
    console.pre_insert[0].quality = EffectQuality::High;
    SessionEdit::SetConsole {
        console,
        ramp_samples,
    }
}

/// A `SetTrackConsole` of `strip` to the entries it holds, with `ramp_samples`.
fn track_console(store: &SessionStore, strip: &str, ramp_samples: Option<u32>) -> SessionEdit {
    let model = store.compiled().normalized_model();
    let track = model
        .tracks
        .iter()
        .find(|t| t.id == id(strip))
        .expect("track");
    SessionEdit::SetTrackConsole {
        track_id: track.id.clone(),
        console: track.console.clone(),
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

/// #1394 gate 2: the last per-kind edit that writes a `(target, row)` decides its length, and a
/// `SET_CONTROL` records the one row its address names; `None` removes an earlier length of a
/// per-kind edit; an upsert removes every entry of its strip.
///
/// Red if a later edit without a length keeps an earlier one, if an edit records the wrong row,
/// target or one row too few, or if an upsert leaves a stale entry.
#[test]
fn the_last_edit_to_write_a_row_decides_its_length() {
    let store = store();
    let route_gain = set_control(
        row_address(
            Owner::Route(id("to-main")),
            Section::Own,
            "gain_db",
            Lane::Both,
        )
        .expect("the gain row"),
        0.0,
        Some(0),
    );
    let pan = set_control(builtin("vocal", PAN, Lane::Both), 0.5, Some(9));
    let edits = [
        declare(&store, None),
        fader(&store, "vocal", Some(37)),
        pan,
        route_gain,
        track_console(&store, "second", Some(5)),
        track_console(&store, "second", None),
    ];
    assert_eq!(
        recorded(&store, &edits),
        vec![
            (strip("vocal"), EditRampRow::Fader, 37),
            (strip("vocal"), EditRampRow::Matrix, 9),
            (
                EditRampTarget::Route(id("to-main")),
                EditRampRow::RouteGain,
                0
            ),
        ],
        "the vocal fader row carries 37 and only that row, the pan row 9, the route's explicit 0 \
         is an entry, and the second track's later console edit without a length removed its 5"
    );

    // An upsert of the strip after the first edit removes its entries; one before keeps them.
    let upsert = SessionEdit::UpsertTrack {
        track: vocal(&store).clone(),
    };
    let after = [fader(&store, "vocal", Some(37)), upsert.clone()];
    assert_eq!(recorded(&store, &after), vec![]);
    let before = [upsert, fader(&store, "vocal", Some(37))];
    assert_eq!(recorded(&store, &before).len(), 1);
}

/// #1394 D3: every live-value edit records exactly the rows the table names, on the target the
/// table names, and nothing else.
///
/// Red if an edit records the wrong row (a mute edit on the gain row, a follow toggle off the mute
/// row) or the wrong target (a route's row on the strip).
#[test]
fn every_live_value_edit_records_its_rows_on_its_target() {
    let store = store();
    let model = store.compiled().normalized_model();
    let route = |row: &str, value: f32, ramp| {
        set_control(
            row_address(Owner::Route(id("to-main")), Section::Own, row, Lane::Both)
                .expect("a route row"),
            value,
            ramp,
        )
    };
    let vca = |row: &str, value: f32, ramp| {
        set_control(
            row_address(Owner::Vca(id("grp")), Section::Own, row, Lane::Left).expect("a VCA row"),
            value,
            ramp,
        )
    };
    let on = |row: &str, value: f32, ramp| {
        set_control(
            row_address(
                Owner::Strip(id("vocal")),
                Section::Builtins,
                row,
                Lane::Left,
            )
            .expect("a builtin row"),
            value,
            ramp,
        )
    };
    let own = |row: &str, value: f32, ramp| {
        set_control(
            row_address(Owner::Strip(id("vocal")), Section::Own, row, Lane::Both)
                .expect("a strip row"),
            value,
            ramp,
        )
    };
    type Case = (
        Box<dyn Fn(Option<u32>) -> Vec<SessionEdit>>,
        EditRampTarget,
        EditRampRow,
    );
    let cases: Vec<Case> = vec![
        (
            Box::new(move |ramp| vec![on("fader_db", -1.0, ramp)]),
            strip("vocal"),
            EditRampRow::Fader,
        ),
        (
            Box::new(move |ramp| vec![on("mute", 1.0, ramp)]),
            strip("vocal"),
            EditRampRow::Mute,
        ),
        (
            Box::new(move |ramp| vec![on("pan", 0.5, ramp)]),
            strip("vocal"),
            EditRampRow::Matrix,
        ),
        (
            Box::new(move |ramp| vec![own("matrix_mode", 1.0, ramp)]),
            strip("vocal"),
            EditRampRow::Matrix,
        ),
        (
            Box::new(move |ramp| vec![on("trim_db", -1.0, ramp)]),
            strip("vocal"),
            EditRampRow::Input,
        ),
        (
            Box::new(move |ramp| vec![on("polarity_invert", 1.0, ramp)]),
            strip("vocal"),
            EditRampRow::Input,
        ),
        (
            Box::new(move |ramp| vec![route("gain_db", -1.0, ramp)]),
            EditRampTarget::Route(id("to-main")),
            EditRampRow::RouteGain,
        ),
        (
            Box::new(move |ramp| vec![route("mute", 1.0, ramp)]),
            EditRampTarget::Route(id("to-main")),
            EditRampRow::RouteMute,
        ),
        (
            Box::new(move |ramp| vec![route("matrix_ll", 0.5, ramp)]),
            EditRampTarget::Route(id("to-main")),
            EditRampRow::RouteMatrix,
        ),
        (
            Box::new(move |ramp| vec![vca("offset_db", -1.0, ramp)]),
            EditRampTarget::Vca(id("grp")),
            EditRampRow::VcaFader,
        ),
        (
            Box::new(move |ramp| vec![vca("mute", 1.0, ramp)]),
            EditRampTarget::Vca(id("grp")),
            EditRampRow::VcaMute,
        ),
    ];
    for (edits, target, row) in cases {
        assert_eq!(
            recorded(&store, &edits(Some(11))),
            vec![(target.clone(), row, 11)],
            "{target:?} {row:?}"
        );
        // The same edit without a length records nothing: absence is not an entry.
        assert_eq!(recorded(&store, &edits(None)), vec![], "{target:?} {row:?}");
    }
    // The two console edits keep their ramp field and write the link row of their own target.
    let console_edit = |ramp| SessionEdit::SetConsole {
        console: model.console.clone(),
        ramp_samples: ramp,
    };
    assert_eq!(
        recorded(&store, &[console_edit(Some(11))]),
        vec![(EditRampTarget::Console, EditRampRow::Link, 11)]
    );
    assert_eq!(recorded(&store, &[console_edit(None)]), vec![]);
    assert_eq!(
        recorded(
            &store,
            &[
                declare(&store, None),
                track_console(&store, "vocal", Some(11))
            ]
        ),
        vec![(strip("vocal"), EditRampRow::Link, 11)]
    );
    assert_eq!(
        recorded(
            &store,
            &[declare(&store, None), track_console(&store, "vocal", None)]
        ),
        vec![]
    );
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
        declare(&store, None),
        track_console(&store, "vocal", Some(48_001)),
        track_console(&store, "vocal", None),
    ];
    assert_eq!(
        refuse(&store, &overwritten),
        SessionStoreError::Edit {
            operation_index: 1,
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
/// length for them, so it clears the target's earlier entries; an edit after it decides afresh. A
/// rack edit writes no such row, so it leaves its strip's entries as they were.
///
/// Red if an `UpsertTrack`, `RemoveTrack`, `RemoveSubmix`, `RemoveRoute` or `RemoveVca` leaves the
/// stale length of an earlier edit in place (a later reader would ramp a row over a length that its
/// last writer did not give), if a clear takes another target's entries, if a clear also drops a
/// write that comes after it, or if a rack edit clears its strip's rows.
#[test]
fn an_edit_that_rewrites_or_removes_a_target_clears_its_earlier_lengths() {
    let store = store();
    let model = store.compiled().normalized_model();
    let vocal_track = vocal(&store);
    let eq = vocal_track.inserts.effects[0].clone();
    let set_rack = SessionEdit::SetTrackRack {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        rack: vocal_track.inserts.clone(),
    };
    let put = SessionEdit::PutTrackEffect {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        final_position: 0,
        effect: eq.clone(),
    };
    let remove_effect = SessionEdit::RemoveTrackEffect {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        effect_id: eq.id.clone(),
    };
    let strip_fader = |strip: &str, samples: u32| {
        let value = if strip == "bus" {
            model.submixes[0].fader.left_db
        } else {
            vocal_track.fader.left_db
        };
        set_control(builtin(strip, FADER_DB, Lane::Both), value, Some(samples))
    };
    let upsert_vocal = SessionEdit::UpsertTrack {
        track: vocal_track.clone(),
    };
    let route_gain = |samples| {
        set_control(
            row_address(
                Owner::Route(id("to-main")),
                Section::Own,
                "gain_db",
                Lane::Both,
            )
            .expect("the gain row"),
            0.0,
            Some(samples),
        )
    };
    let vca_offset = |samples| {
        set_control(
            row_address(Owner::Vca(id("grp")), Section::Own, "offset_db", Lane::Left)
                .expect("the offset row"),
            0.0,
            Some(samples),
        )
    };

    let cases: Vec<(&str, Vec<SessionEdit>, Recorded)> = vec![
        (
            "UpsertTrack replaces the strip with no length",
            vec![strip_fader("vocal", 9), upsert_vocal.clone()],
            vec![],
        ),
        (
            "an edit after the rewrite decides the row",
            vec![
                strip_fader("vocal", 9),
                upsert_vocal,
                strip_fader("vocal", 5),
            ],
            vec![(strip("vocal"), EditRampRow::Fader, 5)],
        ),
        (
            "a rack edit leaves its strip's rows",
            vec![strip_fader("vocal", 9), set_rack, put, remove_effect],
            vec![(strip("vocal"), EditRampRow::Fader, 9)],
        ),
        (
            "RemoveTrack removes the strip's rows, and only its own",
            vec![
                strip_fader("second", 5),
                strip_fader("vocal", 7),
                SessionEdit::RemoveTrack {
                    track_id: id("second"),
                },
            ],
            vec![(strip("vocal"), EditRampRow::Fader, 7)],
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
                route_gain(5),
                SessionEdit::RemoveRoute {
                    route_id: id("to-main"),
                },
            ],
            vec![],
        ),
        (
            "RemoveVca removes the VCA's rows",
            vec![vca_offset(5), SessionEdit::RemoveVca { vca_id: id("grp") }],
            vec![],
        ),
    ];
    for (name, edits, expected) in cases {
        assert_eq!(recorded(&store, &edits), expected, "{name}");
    }
}

// ---- #1553: SET_CONTROL --------------------------------------------------------------------

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
    SessionEdit::set_control(address, value, ramp_samples)
}

/// #1553 D4: a `SetControl` records the one row its address names, on that address's owner; a
/// fixed-ramp row records nothing.
///
/// Red if the address maps to the wrong row (a fader edit also taking the mute row, a pan on the
/// fader row), or the wrong target.
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

/// Batch 2 review fold item 1: several `SET_CONTROL` rows share one ramp key, and one transaction
/// that gives the key two lengths (an explicit one against another, or against none) is refused
/// at the later edit with `control.ramp.conflict` and nothing is written; equal lengths agree; an
/// upsert of the strip between them clears the key.
///
/// Red if the later control silently decides the length of the earlier one (the left fader at 480
/// samples would then ramp the right at the session default), or if equal lengths are refused.
#[test]
fn two_controls_that_share_a_ramp_row_must_agree_on_its_length() {
    let store = store();
    let revision = store.revision();
    let before = store.canonical_snapshot();
    let prepare =
        |edits: &[SessionEdit]| store.prepare_transaction(ExpectedRevision::Exact(revision), edits);
    let conflict = |index: usize| SessionStoreError::Edit {
        operation_index: index,
        error: SessionEditError::Control(ControlError::RampConflict),
    };
    let left = |ramp| set_control(builtin("vocal", FADER_DB, Lane::Left), -1.0, ramp);
    let right = |ramp| set_control(builtin("vocal", FADER_DB, Lane::Right), -2.0, ramp);
    // An explicit length against none, in either order, and against another explicit length.
    assert_eq!(
        prepare(&[left(Some(480)), right(None)]).err(),
        Some(conflict(1))
    );
    assert_eq!(
        prepare(&[left(None), right(Some(480))]).err(),
        Some(conflict(1))
    );
    assert_eq!(
        prepare(&[left(Some(480)), right(Some(240))]).err(),
        Some(conflict(1))
    );
    // The same control twice with two lengths shares its own key.
    assert_eq!(
        prepare(&[left(Some(480)), left(Some(240))]).err(),
        Some(conflict(1))
    );
    // The pan and the matrix rows share the matrix key; trim and polarity share the input key.
    assert_eq!(
        prepare(&[
            set_control(builtin("vocal", PAN, Lane::Both), 0.25, Some(9)),
            set_control(builtin("vocal", PAN, Lane::Both), 0.5, None),
        ])
        .err(),
        Some(conflict(1))
    );
    // Equal lengths agree, and the recorded length is the shared one.
    let prepared = prepare(&[left(Some(480)), right(Some(480))]).expect("equal lengths agree");
    assert_eq!(
        prepared.edit_ramps().entries(),
        vec![(strip("vocal"), EditRampRow::Fader, 480)]
    );
    assert!(prepare(&[left(None), right(None)]).is_ok());
    // A different key of the same strip is independent.
    assert!(
        prepare(&[
            left(Some(480)),
            set_control(builtin("vocal", MUTE, Lane::Left), 1.0, None)
        ])
        .is_ok()
    );
    // An upsert of the strip clears its lengths, control writes included.
    let upsert = SessionEdit::UpsertTrack {
        track: vocal(&store).clone(),
    };
    assert!(prepare(&[left(Some(480)), upsert, right(None)]).is_ok());
    assert_eq!(store.revision(), revision);
    assert_eq!(store.canonical_snapshot(), before);
}
