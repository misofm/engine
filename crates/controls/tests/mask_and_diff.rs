//! #1554 D1 and D2: the value mask and the row diff cover every row.
//!
//! A row is the only place that binds a control to a model field, so the mask has to copy every
//! field a row reads and the diff has to see every row's change. Both are tested against the rows
//! themselves, not against a list.

mod common;

use common::{RATE, effects_fixture, id};
use controls::{
    ControlClass, ControlKind, ControlRow, ControlScope, Lane, MonitorOverlay, changed_controls,
    list, mask_values, set,
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
