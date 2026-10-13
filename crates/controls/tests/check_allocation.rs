//! `check` is allocation-free: it is the validator the live path calls (decision 17, C5).

mod common;

use bench_support::alloc::{assert_installed, current_thread_counters, current_thread_delta_since};
use common::{RATE, fixture, strip_builtin};
use controls::{ControlError, Lane, check, list};

/// Red if `check` allocates or frees, on an accepting path or on any of its five refusals.
#[test]
fn check_allocates_nothing() {
    assert_installed();
    let model = fixture();
    let rows: Vec<_> = list(&model, &controls::MonitorOverlay::default(), RATE).collect();
    let descriptor_of = |lane, control| {
        rows.iter()
            .find(|row| row.address == strip_builtin("guitar", control, lane))
            .unwrap()
            .descriptor
    };
    let fader = descriptor_of(Lane::Left, 5);
    let hpf = descriptor_of(Lane::Left, 3);
    let matrix = descriptor_of(Lane::Both, 7);
    let pan = descriptor_of(Lane::Left, 12);
    let derived = rows
        .iter()
        .find(|row| row.descriptor.class == controls::ControlClass::Derived)
        .unwrap()
        .descriptor;

    let mark = current_thread_counters();
    let results = [
        check(&fader, Lane::Left, -3.0, Some(480)),
        check(&fader, Lane::Both, f32::NAN, None),
        check(&hpf, Lane::Left, 100.0, Some(1)),
        check(&matrix, Lane::Right, 0.5, None),
        check(&pan, Lane::Left, 0.0, None),
        check(&derived, Lane::Left, 0.0, None),
    ];
    let delta = current_thread_delta_since(mark);
    assert_eq!(
        results,
        [
            Ok(()),
            Err(ControlError::Domain),
            Err(ControlError::RampFixed),
            Err(ControlError::Lane),
            Err(ControlError::Inactive),
            Err(ControlError::ReadOnly),
        ]
    );
    assert_eq!((delta.allocations, delta.deallocations), (0, 0));
}
