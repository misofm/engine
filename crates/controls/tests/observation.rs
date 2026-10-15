//! The host's observation subscriptions (#1584 D6a): the table a rebuild is prepared from, and the
//! bindings a transaction ends.

mod common;

use common::{effects_fixture, id, insert_id};
use controls::{ObservationBindings, TapKey, TapSection};
use session::{EffectIdentity, SessionModel};

/// The compressor's gain-reduction tap, effect-local ID 1.
const GAIN_REDUCTION: u32 = 1;

fn insert_key(strip: &str, tap_id: u32) -> TapKey {
    TapKey {
        strip: id(strip),
        section: TapSection::Inserts(insert_id("miso.compressor")),
        tap_id,
    }
}

fn console_key(strip: &str) -> TapKey {
    TapKey {
        strip: id(strip),
        section: TapSection::Console(id("glue")),
        tap_id: GAIN_REDUCTION,
    }
}

fn bound(keys: &[(TapKey, u32)]) -> ObservationBindings {
    let mut bindings = ObservationBindings::new();
    for (key, window) in keys {
        bindings.set(key.clone(), *window);
    }
    bindings
}

fn strip_mut<'a>(model: &'a mut SessionModel, name: &str) -> &'a mut session::Track {
    model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == name)
        .expect("a track")
}

/// The table is sorted, an absolute set replaces the window, and a clear reports whether it held.
///
/// Red if a binding is duplicated, a set keeps the older window, or a clear of an unarmed tap
/// claims it held one.
#[test]
fn the_table_sets_and_clears_by_key() {
    let mut bindings = ObservationBindings::new();
    assert!(bindings.is_empty());
    bindings.set(insert_key("vocal", GAIN_REDUCTION), 4);
    bindings.set(console_key("guitar"), 8);
    bindings.set(insert_key("guitar", GAIN_REDUCTION), 0);
    assert_eq!(bindings.len(), 3);
    assert!(
        bindings
            .bindings()
            .windows(2)
            .all(|pair| pair[0].key < pair[1].key),
        "sorted by key, no duplicate"
    );
    bindings.set(insert_key("vocal", GAIN_REDUCTION), 16);
    assert_eq!(bindings.len(), 3);
    assert_eq!(
        bindings.window_of(&insert_key("vocal", GAIN_REDUCTION)),
        Some(16)
    );
    assert!(bindings.clear(&insert_key("vocal", GAIN_REDUCTION)));
    assert!(!bindings.clear(&insert_key("vocal", GAIN_REDUCTION)));
    assert_eq!(
        bindings.window_of(&insert_key("vocal", GAIN_REDUCTION)),
        None
    );
    assert_eq!(bindings.len(), 2);
}

/// A transaction that leaves every target in place ends nothing; each way a target can go ends
/// exactly its bindings and names them: the strip removed, the insert removed, the insert's ID
/// reused for an effect without the tap, the console slot removed, a tap ID the effect has not.
///
/// Red if a binding outlives its strip, its insert or its slot (a subscription with no tap),
/// if an ID reused for another effect keeps the old binding, or if a surviving binding is dropped.
#[test]
fn a_binding_ends_when_its_target_goes() {
    let model = effects_fixture();
    let bindings = bound(&[
        (insert_key("vocal", GAIN_REDUCTION), 4),
        (insert_key("guitar", GAIN_REDUCTION), 8),
        (console_key("vocal"), 4),
    ]);
    let unchanged = bindings.after(&model).expect("room");
    assert_eq!(unchanged.bindings, bindings);
    assert!(unchanged.ended.is_empty());

    // A strip removed: its bindings end, the others stay.
    let mut without_guitar = model.clone();
    without_guitar
        .tracks
        .retain(|track| track.id.as_str() != "guitar");
    let change = bindings.after(&without_guitar).expect("room");
    assert_eq!(change.ended, vec![insert_key("guitar", GAIN_REDUCTION)]);
    assert_eq!(change.bindings.len(), 2);

    // An insert removed.
    let mut without_insert = model.clone();
    strip_mut(&mut without_insert, "vocal")
        .inserts
        .effects
        .retain(|effect| effect.id != insert_id("miso.compressor"));
    let change = bindings.after(&without_insert).expect("room");
    assert_eq!(change.ended, vec![insert_key("vocal", GAIN_REDUCTION)]);
    assert_eq!(
        change
            .bindings
            .window_of(&insert_key("guitar", GAIN_REDUCTION)),
        Some(8)
    );

    // The insert's ID reused for an effect without that tap.
    let mut reused = model.clone();
    strip_mut(&mut reused, "guitar")
        .inserts
        .effects
        .iter_mut()
        .find(|effect| effect.id == insert_id("miso.compressor"))
        .expect("the compressor insert")
        .identity = EffectIdentity::Native {
        effect_id: id("miso.delay"),
    };
    let change = bindings.after(&reused).expect("room");
    assert_eq!(change.ended, vec![insert_key("guitar", GAIN_REDUCTION)]);

    // A console slot removed from the session (and every strip's entry for it).
    let mut without_slot = model.clone();
    without_slot.console.pre_insert.clear();
    for strip in &mut without_slot.tracks {
        strip.console.retain(|entry| entry.slot != id("glue"));
    }
    for strip in &mut without_slot.submixes {
        strip.console.retain(|entry| entry.slot != id("glue"));
    }
    let change = bindings.after(&without_slot).expect("room");
    assert_eq!(change.ended, vec![console_key("vocal")]);

    // A tap the effect does not declare ends too.
    let unknown = bound(&[(insert_key("vocal", 99), 4)]);
    assert_eq!(
        unknown.after(&model).expect("room").ended,
        vec![insert_key("vocal", 99)]
    );
    assert!(unknown.after(&model).expect("room").bindings.is_empty());
}
