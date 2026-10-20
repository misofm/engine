//! The monitor overlay's writer (decision 17, C8, #1559): the validated write, the overlay it
//! makes, and the ramp its records take.

mod common;

use common::{RATE, fixture, id, strip_own};
use controls::{
    ControlError, Lane, MonitorOverlay, MonitorRow, MonitorWrite, Owner, Section, check_monitor,
    effective,
};

const SOLO: u32 = 2;

fn write(strip: &str, soloed: bool, ramp: Option<u32>) -> MonitorWrite {
    MonitorWrite {
        strip: id(strip),
        row: MonitorRow::Solo,
        on: soloed,
        ramp,
    }
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

const MASTER: u32 = 4;

fn master(strip: &str, on: bool) -> MonitorWrite {
    MonitorWrite {
        strip: id(strip),
        row: MonitorRow::Master,
        on,
        ramp: None,
    }
}

/// #1683: the master designation is a monitor row on every strip, a submix included, checked like
/// a solo by its row; it takes no ramp, a strip the session does not have is refused before any
/// commit, and the overlay holds one master at most, by ID, which `get` and the list read back
/// and which a solo-only reader never sees.
///
/// Red if a submix cannot be designated, a ramp or a strip the session lacks is accepted, a second
/// designation leaves two masters, ending another strip's designation clears the master, a removed
/// master survives, a master write gives the solo's records a ramp, or the row reads the solo.
#[test]
fn the_master_designation_is_one_monitor_row_held_by_strip_id() {
    let model = fixture();
    let check_master = |strip: &str, value, ramp| {
        check_monitor(
            &model,
            &strip_own(strip, MASTER, Lane::Both),
            value,
            ramp,
            RATE,
        )
    };
    assert_eq!(
        check_master("bus", 1.0, None),
        Ok(Some(master("bus", true)))
    );
    assert_eq!(
        check_master("vocal", 0.0, None),
        Ok(Some(master("vocal", false)))
    );
    assert_eq!(
        check_master("vocal", 1.0, Some(48)),
        Err(ControlError::RampFixed),
        "a designation moves no render value, so it takes no ramp"
    );
    assert_eq!(
        check_master("nobody", 1.0, None),
        Err(ControlError::AddressUnknown),
        "a strip the session does not have is refused before commit"
    );
    assert_eq!(check_master("vocal", 0.5, None), Err(ControlError::Domain));

    let none = MonitorOverlay::new();
    let change = none.after(
        &[
            master("vocal", true),
            write("guitar", true, Some(480)),
            master("bus", true),
            master("vocal", false),
        ],
        &model,
    );
    assert_eq!(
        change.overlay.master(),
        Some(&id("bus")),
        "one master, the last named"
    );
    assert!(!change.overlay.is_master(&id("vocal")));
    assert_eq!(change.overlay.soloed(), [id("guitar")]);
    assert_eq!(
        change.ramp,
        Some(480),
        "the master write takes no part in the ramp"
    );
    assert!(
        !none.same_solo(&change.overlay),
        "the solo changed with the master"
    );
    let master_only = none.after(&[master("bus", true)], &model);
    assert_eq!(master_only.ramp, None);
    assert!(
        none.same_solo(&master_only.overlay) && none != master_only.overlay,
        "a master alone changes the overlay and no solo"
    );

    let read = |overlay: &MonitorOverlay, strip: &str, control: u32| {
        controls::get(
            &model,
            overlay,
            &strip_own(strip, control, Lane::Both),
            RATE,
        )
        .expect("a listed row")
    };
    assert_eq!(read(&master_only.overlay, "bus", MASTER), 1.0);
    assert_eq!(read(&master_only.overlay, "vocal", MASTER), 0.0);
    assert_eq!(read(&master_only.overlay, "vocal", SOLO), 0.0);
    let listed = controls::list(&model, &master_only.overlay, RATE)
        .filter(|row| row.address.control == MASTER && row.address.section == Section::Own)
        .filter(|row| row.value == 1.0)
        .map(|row| row.address.owner)
        .collect::<Vec<_>>();
    assert_eq!(listed, [Owner::Strip(id("bus"))]);

    // A removed master is no longer designated; a removed other strip leaves it.
    let mut without_bus = model.clone();
    without_bus.submixes.clear();
    let mut overlay = master_only.overlay.clone();
    overlay.retain_strips_of(&without_bus);
    assert_eq!(overlay.master(), None);
    let mut without_guitar = model.clone();
    without_guitar
        .tracks
        .retain(|track| track.id != id("guitar"));
    let mut overlay = master_only.overlay;
    overlay.retain_strips_of(&without_guitar);
    assert_eq!(overlay.master(), Some(&id("bus")));
}
