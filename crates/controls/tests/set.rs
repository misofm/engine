//! Gate 2: `set` then `get` round-trips every writable row onto the model field it names, and each
//! `ControlError` code is produced at its boundary.

mod common;

use common::{RATE, address, fixture, id, route_own, row_of, strip_builtin, strip_own};
use controls::{
    ControlAddress, ControlClass, ControlError, ControlRow, ControlScope, Lane, Motion, Owner,
    Section, check, get, list, set,
};
use session::{CONTROL_SMOOTHING_DEFAULT, ControlSmoothing, MatrixOrPan, SessionModel};

/// The next `f32` away from zero.
fn away_from_zero(value: f32) -> f32 {
    f32::from_bits(value.to_bits() + 1)
}

/// A value in the row's domain that differs from its committed value.
fn probe(row: &ControlRow) -> f32 {
    let d = &row.descriptor;
    match d.kind {
        controls::ControlKind::Toggle => 1.0 - row.value,
        controls::ControlKind::Enumeration => d
            .choices
            .iter()
            .map(|choice| choice.value)
            .find(|value| *value != row.value)
            .expect("two choices"),
        controls::ControlKind::Continuous => {
            let mut value = d.minimum + (d.maximum - d.minimum) * 0.25;
            if d.unit == effect_contract_unit_samples() {
                value = value.floor();
            }
            if value == row.value {
                value = d.minimum + (d.maximum - d.minimum) * 0.75;
                if d.unit == effect_contract_unit_samples() {
                    value = value.floor();
                }
            }
            value
        }
    }
}

fn effect_contract_unit_samples() -> effect_contract::ParameterUnit {
    effect_contract::ParameterUnit::Samples
}

fn flag(value: bool) -> f32 {
    f32::from(u8::from(value))
}

/// The value of a row read straight from the model field it must be bound to, with no help from
/// the registry. `None` for a row with no model field.
fn model_field(model: &SessionModel, row: &ControlRow) -> Option<f32> {
    let address = &row.address;
    let left = address.lane != Lane::Right;
    let name = row.descriptor.name;
    match &address.owner {
        Owner::Strip(strip_id) => {
            let strip = model.strips().find(|strip| strip.id == strip_id)?;
            let side = if left {
                &strip.builtins.left
            } else {
                &strip.builtins.right
            };
            let fader = strip.fader;
            let matrix = match strip.matrix_or_pan {
                MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => [*ll, *lr, *rl, *rr],
                MatrixOrPan::Pan { .. } => [f32::NAN; 4],
            };
            let pan = match strip.matrix_or_pan {
                MatrixOrPan::Pan { left, right, .. } => [*left, *right],
                MatrixOrPan::Matrix { .. } => [f32::NAN; 2],
            };
            let window = match strip.matrix_or_pan {
                MatrixOrPan::Pan {
                    smoothing_samples, ..
                }
                | MatrixOrPan::Matrix {
                    smoothing_samples, ..
                } => *smoothing_samples,
            };
            Some(match name {
                "polarity_invert" => flag(side.polarity_invert),
                "trim_db" => side.trim_db,
                "hpf_hz" => side.hpf_hz,
                "lpf_hz" => side.lpf_hz,
                "delay_samples" => side.delay_samples as f32,
                "fader_db" => {
                    if left {
                        fader.left_db
                    } else {
                        fader.right_db
                    }
                }
                "mute" => flag(if left {
                    fader.left_mute
                } else {
                    fader.right_mute
                }),
                "matrix_ll" => matrix[0],
                "matrix_lr" => matrix[1],
                "matrix_rl" => matrix[2],
                "matrix_rr" => matrix[3],
                "pan" => pan[usize::from(!left)],
                "matrix_mode" => flag(matches!(strip.matrix_or_pan, MatrixOrPan::Matrix { .. })),
                "pan_window_samples" => window as f32,
                _ => return None,
            })
        }
        Owner::Route(route_id) => {
            let route = model.routes.iter().find(|route| route.id == *route_id)?;
            Some(match name {
                "gain_db" => route.gain_db,
                "mute" => flag(route.mute),
                "matrix_ll" => route.channel_matrix.ll,
                "matrix_lr" => route.channel_matrix.lr,
                "matrix_rl" => route.channel_matrix.rl,
                "matrix_rr" => route.channel_matrix.rr,
                "follows_mute" => flag(route.follows_mute),
                _ => return None,
            })
        }
        Owner::Vca(vca_id) => {
            let vca = model.vcas.iter().find(|vca| vca.id == *vca_id)?;
            Some(match (name, left) {
                ("offset_db", true) => vca.fader.left_db,
                ("offset_db", false) => vca.fader.right_db,
                ("mute", true) => flag(vca.fader.left_mute),
                ("mute", false) => flag(vca.fader.right_mute),
                _ => return None,
            })
        }
        Owner::Session => {
            let lengths = model.control_smoothing.unwrap_or(CONTROL_SMOOTHING_DEFAULT);
            Some(match name {
                "fader_ms" => lengths.fader_ms,
                "mute_ms" => lengths.mute_ms,
                "pan_ms" => lengths.pan_ms,
                _ => return None,
            })
        }
    }
}

fn writable(row: &ControlRow) -> bool {
    row.descriptor.class != ControlClass::Derived
        && row.descriptor.scope == ControlScope::Model
        && row.descriptor.active
}

fn non_derived(model: &SessionModel) -> Vec<ControlRow> {
    list(model, &controls::MonitorOverlay::default(), RATE)
        .filter(|row| row.descriptor.class != ControlClass::Derived)
        .collect()
}

/// Gate 2, round trip. Every writable row of the fixture is set to a different in-domain value:
/// `get` returns it, the model field the row's name says holds it, and no other stored row moves.
///
/// Red if a row's accessor reads or writes another field (a swapped builtin `match` arm, the left
/// lane in place of the right, the wrong matrix coefficient), or a write leaks into a neighbour.
#[test]
fn set_then_get_round_trips_every_writable_row_onto_its_own_field() {
    let model = fixture();
    let rows: Vec<ControlRow> = list(&model, &controls::MonitorOverlay::default(), RATE).collect();
    let mut written = 0;
    for row in rows.iter().filter(|row| writable(row)) {
        let value = probe(row);
        let mut next = model.clone();
        set(&mut next, &row.address, value, None, RATE)
            .unwrap_or_else(|error| panic!("{:?} <- {value}: {error}", row.address));
        assert_eq!(
            get(
                &next,
                &controls::MonitorOverlay::default(),
                &row.address,
                RATE
            ),
            Ok(value),
            "{:?}",
            row.address
        );
        if let Some(field) = model_field(&next, row) {
            assert_eq!(field.to_bits(), value.to_bits(), "{:?}", row.address);
        }
        // `matrix_mode` rewrites the pan and matrix rows by design (tested below).
        if row.descriptor.name != "matrix_mode" {
            for (before, after) in non_derived(&model).iter().zip(non_derived(&next)) {
                if before.address == row.address {
                    continue;
                }
                assert_eq!(
                    *before, after,
                    "{:?} moved by {:?}",
                    before.address, row.address
                );
            }
        }
        written += 1;
    }
    // 18 (the pan strip) + 20 + 20 (two matrix strips) + 7 + 6 + 6 + 6 (routes) + 4 (VCA) + 3
    // (session): the loop must not silently cover less.
    assert_eq!(written, 90);
}

/// A derived row is read-only through `set`, and `check` says so.
#[test]
fn derived_rows_are_read_only() {
    let mut model = fixture();
    for address in [
        strip_own("vocal", 64, Lane::Left),
        strip_own("vocal", 65, Lane::Right),
        ControlAddress {
            lane: Lane::Left,
            ..route_own("send", 64)
        },
    ] {
        assert_eq!(
            set(&mut model, &address, 0.0, None, RATE),
            Err(ControlError::ReadOnly)
        );
    }
}

/// The derived values equal the VCA composition: `vocal` sits under a VCA of -6 dB / -3 dB and a
/// left mute, and the send follows the strip's muted lanes.
#[test]
fn derived_rows_report_the_vca_composition_and_the_follow_gate() {
    let mut model = fixture();
    let value = |model: &SessionModel, address: ControlAddress| {
        get(model, &controls::MonitorOverlay::default(), &address, RATE).unwrap()
    };
    assert_eq!(value(&model, strip_own("vocal", 64, Lane::Left)), -6.0);
    assert_eq!(value(&model, strip_own("vocal", 64, Lane::Right)), -3.0);
    assert_eq!(value(&model, strip_own("vocal", 65, Lane::Left)), 1.0);
    assert_eq!(value(&model, strip_own("vocal", 65, Lane::Right)), 0.0);
    assert_eq!(value(&model, strip_own("guitar", 64, Lane::Left)), 0.0);
    assert_eq!(value(&model, strip_own("guitar", 65, Lane::Left)), 0.0);
    let gate = |lane| ControlAddress {
        lane,
        ..route_own("send", 64)
    };
    assert_eq!(value(&model, gate(Lane::Left)), 1.0);
    assert_eq!(value(&model, gate(Lane::Right)), 0.0);
    // A route that does not follow gates on its own mute only.
    let plain = ControlAddress {
        lane: Lane::Left,
        ..route_own("post-fader-out", 64)
    };
    assert_eq!(value(&model, plain.clone()), 0.0);
    set(
        &mut model,
        &strip_builtin("vocal", 6, Lane::Right),
        1.0,
        None,
        RATE,
    )
    .unwrap();
    assert_eq!(value(&model, gate(Lane::Right)), 1.0);
    assert_eq!(value(&model, strip_own("vocal", 65, Lane::Right)), 1.0);
    set(&mut model, &route_own("post-fader-out", 2), 1.0, None, RATE).unwrap();
    assert_eq!(value(&model, plain), 1.0);
}

/// `both` on a per-lane row sets both lanes in one edit.
#[test]
fn both_on_a_per_lane_row_sets_both_lanes() {
    let mut model = fixture();
    set(
        &mut model,
        &strip_builtin("guitar", 5, Lane::Both),
        -12.5,
        None,
        RATE,
    )
    .unwrap();
    assert_eq!(model.tracks[1].fader.left_db, -12.5);
    assert_eq!(model.tracks[1].fader.right_db, -12.5);
}

/// Switching `matrix_mode` seeds the matrix with the pan's exact coefficients, or resets each
/// lane's pan to its default, keeps the pan window, and a switch to the present mode is a no-op.
#[test]
fn matrix_mode_switch_is_seeded_and_keeps_the_window() {
    let mut model = fixture();
    let vocal_mode = strip_own("vocal", 1, Lane::Both);
    set(&mut model, &vocal_mode, 1.0, None, RATE).unwrap();
    let seed = builtins::pan_matrix(1.0, 1.0).unwrap();
    assert_eq!(
        model.tracks[0].matrix_or_pan,
        MatrixOrPan::Matrix {
            ll: seed.ll,
            lr: seed.lr,
            rl: seed.rl,
            rr: seed.rr,
            smoothing_samples: 16,
        }
    );
    // The rows swap activity.
    assert!(
        row_of(&model, &strip_builtin("vocal", 7, Lane::Both))
            .descriptor
            .active
    );
    assert!(
        !row_of(&model, &strip_builtin("vocal", 12, Lane::Left))
            .descriptor
            .active
    );

    set(&mut model, &vocal_mode, 0.0, None, RATE).unwrap();
    assert_eq!(
        model.tracks[0].matrix_or_pan,
        MatrixOrPan::Pan {
            left: 0.0,
            right: 0.0,
            smoothing_samples: 16,
        }
    );

    let before = model.clone();
    set(&mut model, &vocal_mode, 0.0, None, RATE).unwrap();
    assert_eq!(model, before);
}

/// A value equal to the committed one changes nothing, down to the canonical spelling of a
/// session that says `default` for its ramp lengths.
#[test]
fn a_write_of_the_committed_value_changes_nothing() {
    let mut model = fixture();
    assert!(
        model.control_smoothing.is_none()
            || model.control_smoothing == Some(CONTROL_SMOOTHING_DEFAULT)
    );
    model.control_smoothing = None;
    let before = model.clone();
    let fader_ms = address(Owner::Session, Section::Own, 1, Lane::Both);
    set(
        &mut model,
        &fader_ms,
        CONTROL_SMOOTHING_DEFAULT.fader_ms,
        None,
        RATE,
    )
    .unwrap();
    assert_eq!(model, before);
    set(&mut model, &fader_ms, 5.0, None, RATE).unwrap();
    assert_eq!(
        model.control_smoothing,
        Some(ControlSmoothing {
            fader_ms: 5.0,
            ..CONTROL_SMOOTHING_DEFAULT
        })
    );
}

/// Gate 2, boundaries of `control.domain`.
#[test]
fn domain_boundaries() {
    let mut model = fixture();
    let mut try_set =
        |address: ControlAddress, value: f32| set(&mut model, &address, value, None, RATE);
    let fader = strip_builtin("vocal", 5, Lane::Left);
    assert_eq!(try_set(fader.clone(), 24.0), Ok(()));
    assert_eq!(try_set(fader.clone(), -144.0), Ok(()));
    assert_eq!(
        try_set(fader.clone(), away_from_zero(24.0)),
        Err(ControlError::Domain)
    );
    assert_eq!(
        try_set(fader.clone(), away_from_zero(-144.0)),
        Err(ControlError::Domain)
    );
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(try_set(fader.clone(), bad), Err(ControlError::Domain));
    }
    // A toggle is exactly 0 or 1.
    let mute = strip_builtin("vocal", 6, Lane::Left);
    assert_eq!(try_set(mute.clone(), 1.0), Ok(()));
    assert_eq!(try_set(mute.clone(), 0.5), Err(ControlError::Domain));
    assert_eq!(try_set(mute, 2.0), Err(ControlError::Domain));
    // A filter: 0 is off, below 10 Hz is out, the rate's bound is in and its successor is out.
    let hpf = strip_builtin("vocal", 3, Lane::Left);
    let bound = builtins::builtin_filter_cutoff_maximum_hz(RATE).unwrap();
    assert_eq!(try_set(hpf.clone(), 0.0), Ok(()));
    assert_eq!(try_set(hpf.clone(), 9.99), Err(ControlError::Domain));
    assert_eq!(try_set(hpf.clone(), 10.0), Ok(()));
    assert_eq!(try_set(hpf.clone(), bound), Ok(()));
    assert_eq!(
        try_set(hpf, away_from_zero(bound)),
        Err(ControlError::Domain)
    );
    // A delay is a whole number of samples up to 48,000.
    let delay = strip_builtin("vocal", 11, Lane::Left);
    assert_eq!(try_set(delay.clone(), 48_000.0), Ok(()));
    assert_eq!(try_set(delay.clone(), 48_001.0), Err(ControlError::Domain));
    assert_eq!(try_set(delay.clone(), 1.5), Err(ControlError::Domain));
    assert_eq!(try_set(delay, -1.0), Err(ControlError::Domain));
    // An enumeration takes only its choices.
    let mode = strip_own("vocal", 1, Lane::Both);
    assert_eq!(try_set(mode.clone(), 2.0), Err(ControlError::Domain));
    assert_eq!(try_set(mode, 0.5), Err(ControlError::Domain));
    // The session's ramp lengths end at 1000 ms.
    let ramp = address(Owner::Session, Section::Own, 3, Lane::Both);
    assert_eq!(try_set(ramp.clone(), 1000.0), Ok(()));
    assert_eq!(
        try_set(ramp, away_from_zero(1000.0)),
        Err(ControlError::Domain)
    );
    // A refused write changed nothing.
    assert_eq!(model.tracks[0].fader.left_db, -144.0);
}

/// Gate 2, the other codes at their boundaries.
#[test]
fn lane_inactive_ramp_unknown_and_monitor_refusals() {
    let mut model = fixture();
    let before = model.clone();
    let mut try_set = |address: ControlAddress, value: f32, ramp: Option<u32>| {
        set(&mut model, &address, value, ramp, RATE)
    };

    // `left` or `right` on a shared row.
    assert_eq!(
        try_set(strip_builtin("guitar", 7, Lane::Left), 0.5, None),
        Err(ControlError::Lane)
    );
    assert_eq!(
        try_set(
            ControlAddress {
                lane: Lane::Right,
                ..route_own("send", 1)
            },
            0.0,
            None
        ),
        Err(ControlError::Lane)
    );
    // The matrix rows of a pan strip, the pan rows of a matrix strip, and a follow on an output
    // route do not apply.
    assert_eq!(
        try_set(strip_builtin("vocal", 7, Lane::Both), 0.5, None),
        Err(ControlError::Inactive)
    );
    assert_eq!(
        try_set(strip_builtin("guitar", 12, Lane::Left), 0.5, None),
        Err(ControlError::Inactive)
    );
    assert_eq!(
        try_set(route_own("post-fader-out", 7), 1.0, None),
        Err(ControlError::Inactive)
    );
    // An explicit ramp on a fixed-ramp row, and on a row with a ramp.
    assert_eq!(
        try_set(strip_builtin("vocal", 3, Lane::Left), 100.0, Some(10)),
        Err(ControlError::RampFixed)
    );
    assert_eq!(
        try_set(
            address(Owner::Session, Section::Own, 1, Lane::Both),
            5.0,
            Some(0)
        ),
        Err(ControlError::RampFixed)
    );
    assert_eq!(
        try_set(strip_builtin("vocal", 5, Lane::Left), -3.0, Some(0)),
        Ok(())
    );
    assert_eq!(
        try_set(strip_builtin("vocal", 5, Lane::Right), -3.0, Some(4800)),
        Ok(())
    );
    // Unknown strip, builtin, route, VCA, and a section that has no rows yet.
    let unknown = [
        strip_builtin("nobody", 5, Lane::Left),
        strip_builtin("vocal", 99, Lane::Left),
        strip_builtin("vocal", 0, Lane::Left),
        strip_own("vocal", 99, Lane::Both),
        route_own("nobody", 1),
        route_own("send", 99),
        address(Owner::Vca(id("nobody")), Section::Own, 1, Lane::Left),
        address(Owner::Vca(id("riders")), Section::Builtins, 1, Lane::Left),
        address(Owner::Session, Section::Own, 99, Lane::Both),
        address(Owner::Session, Section::Builtins, 1, Lane::Both),
        address(
            Owner::Strip(id("vocal")),
            Section::Inserts(id("eq")),
            1,
            Lane::Left,
        ),
        address(
            Owner::Strip(id("vocal")),
            Section::Console(id("slot")),
            0,
            Lane::Left,
        ),
    ];
    for address in unknown {
        assert_eq!(
            try_set(address.clone(), 0.0, None),
            Err(ControlError::AddressUnknown),
            "{address:?}"
        );
    }
    // Solo is monitor state: `set` does not write it, but the value check applies first.
    let solo = strip_own("vocal", 2, Lane::Both);
    assert_eq!(try_set(solo.clone(), 1.0, None), Err(ControlError::Monitor));
    assert_eq!(try_set(solo, 0.5, None), Err(ControlError::Domain));
    // A submix is solo-safe: it has no solo row.
    assert_eq!(
        try_set(strip_own("bus", 2, Lane::Both), 1.0, None),
        Err(ControlError::AddressUnknown)
    );
    // Only the two accepted explicit-ramp writes changed the model (the vocal fader, both lanes).
    assert_eq!(model.tracks[0].fader.left_db, -3.0);
    let mut expected = before;
    expected.tracks[0].fader.left_db = -3.0;
    expected.tracks[0].fader.right_db = -3.0;
    assert_eq!(model, expected);
}

/// `get` answers only for listed addresses.
#[test]
fn get_refuses_unlisted_lanes_and_unknown_addresses() {
    let model = fixture();
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_builtin("vocal", 5, Lane::Both),
            RATE
        ),
        Err(ControlError::Lane)
    );
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_builtin("guitar", 7, Lane::Left),
            RATE
        ),
        Err(ControlError::Lane)
    );
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_builtin("nobody", 5, Lane::Left),
            RATE
        ),
        Err(ControlError::AddressUnknown)
    );
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_builtin("vocal", 99, Lane::Left),
            RATE
        ),
        Err(ControlError::AddressUnknown)
    );
    // Solo reads as off until the overlay exists; an inactive row reads as its default.
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_own("vocal", 2, Lane::Both),
            RATE
        ),
        Ok(0.0)
    );
    assert_eq!(
        get(
            &model,
            &controls::MonitorOverlay::default(),
            &strip_builtin("vocal", 10, Lane::Both),
            RATE
        ),
        Ok(1.0)
    );
}

/// `check` is the validator on its own: pure, over a descriptor.
#[test]
fn check_validates_a_descriptor_without_a_model() {
    let model = fixture();
    let fader = row_of(&model, &strip_builtin("vocal", 5, Lane::Left)).descriptor;
    assert_eq!(check(&fader, Lane::Left, 0.0, None), Ok(()));
    assert_eq!(check(&fader, Lane::Both, 0.0, Some(8)), Ok(()));
    assert_eq!(
        check(&fader, Lane::Left, 25.0, None),
        Err(ControlError::Domain)
    );
    let matrix = row_of(&model, &strip_builtin("guitar", 7, Lane::Both)).descriptor;
    assert_eq!(
        check(&matrix, Lane::Left, 0.0, None),
        Err(ControlError::Lane)
    );
    assert_eq!(matrix.motion, Motion::Ramp(controls::LiveRampRow::Matrix));
}
