//! Issue #1678: a `SET_CONTROL` edit of an effect row may name the native effect it was written
//! for, and the core refuses it with `control.effect.changed` when the insert or console slot now
//! runs another effect (a rebuild replaced it), before anything is written. The refusal is the
//! same on the full path and on the path that prepares a transaction of values in place.
//!
//! The base is the canonical fixture (track `vocal` with an insert `eq`) plus a console slot
//! `comp` that runs `miso.compressor`.

use protocol::control_core::{ControlAddress, ControlError, Lane, Owner, Section};
use protocol::{ExpectedRevision, SessionEdit, SessionEditError, SessionStore, SessionStoreError};
use session::{
    CompileCaps, ConsoleEntry, ConsoleSlot, EffectIdentity, EffectQuality, LinkMode, RackName,
    StableId, parse_session_json,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("literal stable ID")
}

fn store() -> SessionStore {
    let mut model = parse_session_json(FIXTURE).expect("fixture");
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
            params: Vec::new(),
        });
    }
    // The fixture's automation targets the `eq`, which the identity swaps below replace.
    model.automation.clear();
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    };
    SessionStore::new(model, caps).expect("the base compiles")
}

fn insert_bypass() -> ControlAddress {
    ControlAddress {
        owner: Owner::Strip(id("vocal")),
        section: Section::Inserts(id("eq")),
        control: 0,
        lane: Lane::Both,
    }
}

fn console_bypass() -> ControlAddress {
    ControlAddress {
        owner: Owner::Strip(id("vocal")),
        section: Section::Console(id("comp")),
        control: 0,
        lane: Lane::Both,
    }
}

/// The insert `eq` of `vocal` now runs `effect`.
fn run(effect: &str) -> SessionEdit {
    SessionEdit::SetEffectIdentity {
        track_id: id("vocal"),
        rack_name: RackName::Inserts,
        effect_id: id("eq"),
        identity: EffectIdentity::Native {
            effect_id: id(effect),
        },
    }
}

fn changed(operation_index: usize) -> SessionStoreError {
    SessionStoreError::Edit {
        operation_index,
        error: SessionEditError::Control(ControlError::EffectChanged),
    }
}

/// Gate 1 at the store, on both preparation paths: after a rebuild that changed an insert's effect,
/// an edit written for the old effect is refused with `control.effect.changed`, nothing changes,
/// and the same edit without the expected effect (or with the effect that runs) commits.
///
/// Red if the check is missing on either path (the stale edit would land on the compressor's row
/// 0, which is its bypass, with no refusal), or if the matching and absent forms are refused.
#[test]
fn a_stale_effect_row_edit_is_refused_after_a_rebuild() {
    let mut store = store();
    store
        .apply_transaction(
            ExpectedRevision::Exact(store.revision()),
            &[run("miso.parametric-eq")],
        )
        .expect("the insert runs a launch equalizer");
    let revision = store.revision();
    let before = store.canonical_snapshot().to_owned();
    let stale = SessionEdit::set_control(insert_bypass(), 1.0, None)
        .with_expected_effect(id("miso.parametric-eq"));
    // The insert still runs the equalizer: the edit commits on both paths.
    for in_place in [false, true] {
        let prepared = if in_place {
            store.prepare_transaction_in_place(
                ExpectedRevision::Exact(revision),
                std::slice::from_ref(&stale),
            )
        } else {
            store.prepare_transaction(
                ExpectedRevision::Exact(revision),
                std::slice::from_ref(&stale),
            )
        };
        assert!(
            prepared.is_ok(),
            "the matching edit commits (in place: {in_place})"
        );
    }
    // A rebuild replaces the equalizer with a compressor.
    store
        .apply_transaction(ExpectedRevision::Exact(revision), &[run("miso.compressor")])
        .expect("the insert runs a compressor");
    let revision = store.revision();
    let before_stale = store.canonical_snapshot().to_owned();
    assert_ne!(before, before_stale);
    for in_place in [false, true] {
        let prepared = if in_place {
            store.prepare_transaction_in_place(
                ExpectedRevision::Exact(revision),
                std::slice::from_ref(&stale),
            )
        } else {
            store.prepare_transaction(
                ExpectedRevision::Exact(revision),
                std::slice::from_ref(&stale),
            )
        };
        assert_eq!(
            prepared.err().expect("the stale edit refuses"),
            changed(0),
            "in place: {in_place}"
        );
    }
    // The refusal names its own edit even when an earlier one was valid, and applies the same
    // check inside a transaction that changes the effect itself.
    let good = SessionEdit::set_control(console_bypass(), 1.0, None)
        .with_expected_effect(id("miso.compressor"));
    assert_eq!(
        store
            .prepare_transaction(
                ExpectedRevision::Exact(revision),
                &[good.clone(), stale.clone()]
            )
            .err()
            .expect("refuses"),
        changed(1)
    );
    assert_eq!(
        store
            .prepare_transaction(
                ExpectedRevision::Exact(revision),
                &[run("miso.parametric-eq"), stale.clone()]
            )
            .map(|_| ()),
        Ok(()),
        "an edit is held to the effect the candidate runs at its place in the transaction"
    );
    // Without an expected effect, or with the one that runs, the edit commits as it always did.
    for edit in [
        SessionEdit::set_control(insert_bypass(), 1.0, None),
        SessionEdit::set_control(insert_bypass(), 1.0, None)
            .with_expected_effect(id("miso.compressor")),
    ] {
        for in_place in [false, true] {
            let prepared = if in_place {
                store.prepare_transaction_in_place(
                    ExpectedRevision::Exact(revision),
                    std::slice::from_ref(&edit),
                )
            } else {
                store.prepare_transaction(
                    ExpectedRevision::Exact(revision),
                    std::slice::from_ref(&edit),
                )
            };
            assert!(prepared.is_ok(), "in place: {in_place}");
        }
    }
}

/// A console slot is held the same way, an unknown insert is still `control.address.unknown`
/// (the expected effect does not hide the address error), and an expected effect on a row that
/// runs no effect is refused rather than ignored.
///
/// Red if the console section is not checked, if the unknown address reports the effect error, or
/// if the expectation on a builtin row is dropped without a word.
#[test]
fn the_expected_effect_follows_the_address() {
    let store = store();
    let revision = store.revision();
    let refuse = |edit: SessionEdit| {
        store
            .prepare_transaction(ExpectedRevision::Exact(revision), &[edit])
            .err()
            .expect("refuses")
    };
    assert_eq!(
        refuse(
            SessionEdit::set_control(console_bypass(), 1.0, None)
                .with_expected_effect(id("miso.gate-expander"))
        ),
        changed(0)
    );
    assert!(
        store
            .prepare_transaction(
                ExpectedRevision::Exact(revision),
                &[SessionEdit::set_control(console_bypass(), 1.0, None)
                    .with_expected_effect(id("miso.compressor"))]
            )
            .is_ok()
    );
    let unknown = ControlAddress {
        section: Section::Inserts(id("nope")),
        ..insert_bypass()
    };
    assert_eq!(
        refuse(
            SessionEdit::set_control(unknown, 1.0, None)
                .with_expected_effect(id("miso.compressor"))
        ),
        SessionStoreError::Edit {
            operation_index: 0,
            error: SessionEditError::Control(ControlError::AddressUnknown),
        }
    );
    let builtin = ControlAddress {
        owner: Owner::Strip(id("vocal")),
        section: Section::Builtins,
        control: 3,
        lane: Lane::Both,
    };
    assert_eq!(
        refuse(
            SessionEdit::set_control(builtin, 0.0, None)
                .with_expected_effect(id("miso.compressor"))
        ),
        changed(0)
    );
}
