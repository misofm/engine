//! The monitor overlay's writer (decision 17, C8, #1559): the validated write, the overlay it
//! makes, and the ramp its records take.

mod common;

use common::{RATE, fixture, id, strip_builtin, strip_own};
use controls::{ControlError, Lane, MonitorOverlay, MonitorWrite, check_monitor, effective, set};

const SOLO: u32 = 2;
const FADER_DB: u32 = 5;

fn write(strip: &str, soloed: bool, ramp: Option<u32>) -> MonitorWrite {
    MonitorWrite {
        strip: id(strip),
        soloed,
        ramp,
    }
}

/// A solo is checked like any write, by its row: a track's row takes exactly 0 or 1 on `both`
/// with an optional ramp, a submix has no solo row, and a model row is not a monitor write.
///
/// Red if a submix becomes soloable, a malformed solo is accepted, a model row is taken for a
/// monitor write, or the write drops its ramp.
#[test]
fn a_solo_write_is_checked_by_its_row() {
    let model = fixture();
    let check_solo = |strip: &str, lane, value, ramp| {
        check_monitor(&model, &strip_own(strip, SOLO, lane), value, ramp, RATE)
    };
    assert_eq!(
        check_solo("vocal", Lane::Both, 1.0, Some(480)),
        Ok(Some(write("vocal", true, Some(480))))
    );
    assert_eq!(
        check_solo("guitar", Lane::Both, 0.0, None),
        Ok(Some(write("guitar", false, None)))
    );
    // A step is a length, not an absent one.
    assert_eq!(
        check_solo("vocal", Lane::Both, 1.0, Some(0)),
        Ok(Some(write("vocal", true, Some(0))))
    );
    assert_eq!(
        check_solo("bus", Lane::Both, 1.0, None),
        Err(ControlError::AddressUnknown),
        "a submix is solo-safe: it has no solo row"
    );
    assert_eq!(
        check_solo("nobody", Lane::Both, 1.0, None),
        Err(ControlError::AddressUnknown)
    );
    assert_eq!(
        check_solo("vocal", Lane::Left, 1.0, None),
        Err(ControlError::Lane)
    );
    for value in [0.5, -0.0, 2.0, f32::NAN] {
        assert_eq!(
            check_solo("vocal", Lane::Both, value, None),
            Err(ControlError::Domain),
            "{value}"
        );
    }
    // A model row is for `set`: the writer says so without judging its value.
    assert_eq!(
        check_monitor(
            &model,
            &strip_builtin("vocal", FADER_DB, Lane::Both),
            99.0,
            None,
            RATE
        ),
        Ok(None)
    );
    // And `set` refuses the monitor row, whatever transport reaches it.
    let mut model = model;
    assert_eq!(
        set(
            &mut model,
            &strip_own("vocal", SOLO, Lane::Both),
            1.0,
            None,
            RATE
        ),
        Err(ControlError::Monitor)
    );
}

/// The overlay after a transaction's writes: in order, the last write of a strip wins, a strip the
/// model no longer has is dropped, and the ramp is that of the last write that changed a strip.
///
/// Red if a later write of a strip does not replace an earlier one, if a removed strip keeps its
/// entry, or if a write that changed nothing sets the ramp of the records another write made.
#[test]
fn the_overlay_after_writes_keeps_order_drops_removed_strips_and_picks_the_ramp() {
    let mut model = fixture();
    let none = MonitorOverlay::new();
    let change = none.after(
        &[
            write("vocal", true, Some(10)),
            write("guitar", true, Some(480)),
            write("vocal", false, Some(30)),
        ],
        &model,
    );
    assert_eq!(change.overlay.soloed(), [id("guitar")]);
    // The last write that changed a strip against `none` is guitar's: vocal's net change is none.
    assert_eq!(change.ramp, Some(480));

    // Unsoloing a strip that is not soloed changes nothing, and has no ramp to give.
    let change = none.after(&[write("vocal", false, Some(7))], &model);
    assert_eq!(change.overlay, none);
    assert_eq!(change.ramp, None);

    // No length given: the session's.
    let change = none.after(&[write("vocal", true, None)], &model);
    assert_eq!(change.ramp, None);

    // The later write of one strip that changes it decides, in transaction order.
    let soloed = MonitorOverlay::with_soloed([id("vocal")]);
    let change = soloed.after(
        &[
            write("vocal", false, Some(1)),
            write("guitar", true, Some(2)),
        ],
        &model,
    );
    assert_eq!(change.overlay.soloed(), [id("guitar")]);
    assert_eq!(change.ramp, Some(2));

    // A strip the model no longer has keeps no solo.
    model.tracks.retain(|track| track.id != id("guitar"));
    let change = MonitorOverlay::with_soloed([id("guitar"), id("vocal")]).after(&[], &model);
    assert_eq!(change.overlay.soloed(), [id("vocal")]);
}

/// `set_soloed` keeps the set sorted and free of duplicates, and `effective` under the result is
/// the solo's mutes.
///
/// Red if the overlay lets `is_soloed` miss a strip (an unsorted insert) or counts one twice.
#[test]
fn set_soloed_keeps_the_set_sorted_and_the_effective_mutes_follow() {
    let model = fixture();
    let mut overlay = MonitorOverlay::new();
    overlay.set_soloed(&id("vocal"), true);
    overlay.set_soloed(&id("guitar"), true);
    overlay.set_soloed(&id("vocal"), true);
    assert_eq!(overlay.soloed(), [id("guitar"), id("vocal")]);
    overlay.set_soloed(&id("guitar"), false);
    overlay.set_soloed(&id("guitar"), false);
    assert_eq!(overlay.soloed(), [id("vocal")]);
    let effective = effective(&model, &overlay);
    let index = |name: &str| {
        model
            .strips()
            .position(|strip| *strip.id == id(name))
            .expect("strip")
    };
    // The guitar is muted by the solo on both lanes; the soloed vocal's own mutes are the VCA's.
    assert_eq!(effective.strips[index("guitar")].mute, [true, true]);
    assert_eq!(
        effective.strips[index("bus")].mute,
        [false, false],
        "a submix is solo-safe"
    );
}
