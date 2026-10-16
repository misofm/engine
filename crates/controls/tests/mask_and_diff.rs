//! #1554 D1 and D2: the value mask and the row diff cover every row.
//!
//! A row is the only place that binds a control to a model field, so the mask has to copy every
//! field a row reads and the diff has to see every row's change. Both are tested against the rows
//! themselves, not against a list.

mod common;

use common::{RATE, effects_fixture, id};
use controls::{
    ControlClass, ControlKind, ControlRow, ControlScope, Lane, MonitorOverlay, Owner, Section,
    changed_controls, changed_rows, list, mask_values, set,
};
use session::SessionModel;

/// A value in the row's domain that differs from its committed value.
fn probe(row: &ControlRow) -> f32 {
    let d = &row.descriptor;
    let whole = |value: f32| {
        if d.unit == effect_contract::ParameterUnit::Samples {
            value.floor()
        } else {
            value
        }
    };
    match d.kind {
        ControlKind::Toggle => 1.0 - row.value,
        ControlKind::Enumeration => d
            .choices
            .iter()
            .map(|choice| choice.value)
            .find(|value| *value != row.value)
            .expect("two choices"),
        ControlKind::Continuous => {
            let low = whole(d.minimum + (d.maximum - d.minimum) * 0.25);
            if low == row.value {
                whole(d.minimum + (d.maximum - d.minimum) * 0.75)
            } else {
                low
            }
        }
    }
}

/// Every writable row of the model.
fn writable(model: &SessionModel) -> Vec<ControlRow> {
    list(model, &MonitorOverlay::default(), RATE)
        .filter(|row| {
            row.descriptor.active
                && row.descriptor.scope == ControlScope::Model
                && row.descriptor.class != ControlClass::Derived
        })
        .collect()
}

/// Red if a row's model field is missing from the mask (the edited model keeps a difference after
/// `mask_values`), or the diff does not see the row's change.
#[test]
fn the_mask_copies_and_the_diff_sees_every_rows_field() {
    let base = effects_fixture();
    let rows = writable(&base);
    assert!(
        rows.len() > 500,
        "the fixture lists a real model: {}",
        rows.len()
    );
    for row in rows {
        let mut edited = base.clone();
        let value = probe(&row);
        // A per-lane row is written on its own lane; `both` on a shared row.
        set(&mut edited, &row.address, value, None, RATE)
            .unwrap_or_else(|error| panic!("{:?}: {error:?}", row.address));
        assert_ne!(
            edited, base,
            "{:?}: the write changed the model",
            row.address
        );

        let changes = changed_controls(&base, &edited, RATE)
            .unwrap_or_else(|| panic!("{:?}: the address lists differ", row.address));
        assert!(
            changes.iter().any(|change| change.address == row.address),
            "{:?}: the diff sees the written row",
            row.address
        );

        let mut masked = edited.clone();
        mask_values(&mut masked, &base);
        assert_eq!(
            masked, base,
            "{:?}: the mask copies the field the row reads",
            row.address
        );
    }
}

/// #1650 D5 (U3 fold, batch 17). A hand move on an automated fader writes the ride's offset, the
/// value of the lane's `fader_db` row; the mask copies it back from the other model, and keeps the
/// lane's static fader (its fallback) as the masked model holds it.
///
/// Test value: red if the mask leaves a ride's hand-move offset out (the masked model keeps the
/// moved offset), or if it copies the static fader of an automated lane.
#[test]
fn the_mask_copies_a_rides_hand_move_offset() {
    let mut base = effects_fixture();
    let strip = base.tracks[0].id.clone();
    base.automation.push(session::Automation {
        id: id("fader-ride"),
        target: common::address(
            Owner::Strip(strip.clone()),
            Section::Builtins,
            5,
            Lane::Both,
        ),
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Step,
            start_sample: 0,
            end_sample: 480,
            start_value: -6.0,
            end_value: -6.0,
            unit: session::ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    });
    let ride = base.automation.len() - 1;
    for lane in [Lane::Left, Lane::Right] {
        let fader = common::address(Owner::Strip(strip.clone()), Section::Builtins, 5, lane);
        let mut edited = base.clone();
        set(&mut edited, &fader, -3.5, None, RATE).expect("a hand move");
        assert_ne!(
            edited.automation[ride].offset, base.automation[ride].offset,
            "{lane:?}: the move writes the offset"
        );
        let mut masked = edited.clone();
        mask_values(&mut masked, &base);
        assert_eq!(masked, base, "{lane:?}: the mask copies the offset");
    }
    // The static fader of an automated lane is not a control value: the mask keeps it.
    let mut fallback = base.clone();
    fallback.tracks[0].fader.left_db = -9.0;
    let mut masked = fallback.clone();
    mask_values(&mut masked, &base);
    assert_eq!(masked, fallback, "the fallback stays different");
}

/// Red if the mask copies a field that is not a control value, or hides an owner it does not
/// share: a removed strip, a renamed strip and a reordered insert stay different after the mask,
/// and the diff of two models with different owners has no answer.
#[test]
fn what_is_not_a_value_stays_different() {
    let base = effects_fixture();

    let mut removed = base.clone();
    removed.tracks.pop();
    let mut masked = removed.clone();
    mask_values(&mut masked, &base);
    assert_eq!(masked, removed);
    assert_eq!(changed_controls(&base, &removed, RATE), None);

    let mut renamed = base.clone();
    renamed.tracks[0].id = id("renamed");
    let mut masked = renamed.clone();
    mask_values(&mut masked, &base);
    assert_eq!(masked.tracks[0].id, id("renamed"));
    assert_eq!(changed_controls(&base, &renamed, RATE), None);

    let mut reordered = base.clone();
    reordered.tracks[0].inserts.effects.swap(0, 1);
    let mut masked = reordered.clone();
    mask_values(&mut masked, &base);
    assert_ne!(masked, base, "an insert order is structure");
    assert_eq!(changed_controls(&base, &reordered, RATE), None);

    // An effect's quality is not a value.
    let mut quality = base.clone();
    quality.tracks[0].inserts.effects[0].quality = session::EffectQuality::High;
    let mut masked = quality.clone();
    mask_values(&mut masked, &base);
    assert_eq!(masked, quality);
}

/// Red if the diff compares resolved values instead of bits, or reports a row whose value did not
/// move: an unchanged model has no change; a trim from `0.0` to `-0.0` is one, on its own lane.
#[test]
fn the_diff_is_value_bits() {
    let base = effects_fixture();
    assert_eq!(changed_controls(&base, &base, RATE), Some(Vec::new()));
    let mut zero = base.clone();
    zero.tracks[0].builtins.left.trim_db = -0.0;
    let changes = changed_controls(&base, &zero, RATE).expect("same addresses");
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].address.lane, Lane::Left);
}

/// #1558 D1, D5. Red if a VCA added or removed, or a member changed, is structure: the mask copies
/// the whole VCA list, the diff of two models with different VCA sets lists the same other rows
/// and no VCA row, and two models with the same VCAs report a VCA row's change like any other.
#[test]
fn the_vca_set_and_its_members_are_values() {
    let base = effects_fixture();
    assert!(!base.vcas.is_empty(), "the fixture has a VCA");

    let mut added = base.clone();
    let mut extra = added.vcas[0].clone();
    extra.id = id("zz-extra");
    extra.members.clear();
    added.vcas.push(extra);
    let mut regrouped = base.clone();
    regrouped.vcas[0].members.clear();
    let mut removed = base.clone();
    removed.vcas.clear();
    for (name, edited) in [
        ("added", added),
        ("regrouped", regrouped),
        ("removed", removed),
    ] {
        let mut masked = edited.clone();
        mask_values(&mut masked, &base);
        assert_eq!(masked, base, "{name}: the mask makes the models equal");
        let mut masked = base.clone();
        mask_values(&mut masked, &edited);
        assert_eq!(masked, edited, "{name}: and the other way");
    }
    let mut added = base.clone();
    let mut extra = added.vcas[0].clone();
    extra.id = id("zz-extra");
    added.vcas.push(extra);
    let changes = changed_controls(&base, &added, RATE).expect("the same strips and routes");
    assert!(
        changes.is_empty(),
        "a VCA added has no row in the diff: {changes:?}"
    );
    let mut moved = base.clone();
    moved.vcas[0].fader.left_db = -9.0;
    let changes = changed_controls(&base, &moved, RATE).expect("same addresses");
    assert_eq!(changes.len(), 1);
    assert!(matches!(changes[0].address.owner, controls::Owner::Vca(_)));
}

/// #1612 D4. Red if `changed_rows` differs from the row's share of `changed_controls` for any
/// writable row of the model, or answers for a row whose write moves more than the row (the
/// session's ramp lengths, a strip's `matrix_mode`), where the caller needs the full diff.
#[test]
fn a_rows_changes_are_its_share_of_the_diff() {
    let base = effects_fixture();
    let rate = RATE;
    let mut local = 0;
    for row in writable(&base) {
        let mut edited = base.clone();
        set(&mut edited, &row.address, probe(&row), None, rate).expect("set");
        let named = |change: &controls::ChangedControl| {
            let (left, right) = (&change.address, &row.address);
            left.owner == right.owner
                && left.section == right.section
                && left.control == right.control
        };
        let expected: Vec<_> = changed_controls(&base, &edited, rate)
            .expect("same addresses")
            .into_iter()
            .filter(named)
            .collect();
        let moves_more = matches!(row.address.owner, Owner::Session)
            || (matches!(row.address.owner, Owner::Strip(_))
                && row.address.section == Section::Own
                && row.descriptor.name == "matrix_mode");
        let got = changed_rows(&base, &edited, &row.address, rate);
        if moves_more {
            assert_eq!(got, None, "{:?}", row.address);
        } else {
            assert_eq!(got, Some(expected), "{:?}", row.address);
            local += 1;
        }
    }
    assert!(local > 500, "the fixture covers the rows: {local}");

    // Unchanged, derived, host-held and unresolvable rows.
    let strip = |name: &str, section, control, lane| controls::ControlAddress {
        owner: Owner::Strip(id(name)),
        section,
        control,
        lane,
    };
    let fader = strip("vocal", Section::Builtins, 5, Lane::Both);
    assert_eq!(changed_rows(&base, &base, &fader, rate), Some(Vec::new()));
    for address in [
        strip("vocal", Section::Own, 64, Lane::Left),
        strip("vocal", Section::Own, 2, Lane::Both),
        strip("nobody", Section::Builtins, 5, Lane::Both),
        strip("vocal", Section::Builtins, 999, Lane::Both),
    ] {
        assert_eq!(
            changed_rows(&base, &base, &address, rate),
            None,
            "{address:?}"
        );
    }
    // The lane of the address does not matter: a `both` write is judged on both lanes.
    let mut louder = base.clone();
    set(&mut louder, &fader, -9.0, None, rate).expect("set");
    let left = strip("vocal", Section::Builtins, 5, Lane::Left);
    let lanes: Vec<_> = changed_rows(&base, &louder, &left, rate)
        .expect("local")
        .into_iter()
        .map(|change| change.address.lane)
        .collect();
    assert_eq!(lanes, [Lane::Left, Lane::Right]);
}
