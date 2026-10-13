//! Gates 1 and 3: the control list has exactly the expected addresses in the ruled order, each
//! row's class matches decision 14, and the tables are self-consistent.

mod common;

use std::collections::BTreeSet;

use common::{RATE, fixture, id, route_own, row_of, strip_builtin, strip_own};
use controls::{
    ControlAddress, ControlClass, ControlRow, ControlScope, EFFECTIVE_SLICE, Lane, Owner,
    ROUTE_SLICE, RebuildReason, Section, list,
};

/// How a row is listed: once as `both`, or as `left` and `right`.
#[derive(Clone, Copy)]
enum Listed {
    Shared,
    PerLane,
}
use Listed::{PerLane, Shared};

/// A submix has no solo row (it is solo-safe).
const SUBMIX_OWN: [(u32, Listed); 4] = [(1, Shared), (3, Shared), (64, PerLane), (65, PerLane)];
const STRIP_OWN: [(u32, Listed); 5] = [
    (1, Shared),
    (2, Shared),
    (3, Shared),
    (64, PerLane),
    (65, PerLane),
];
const BUILTINS: [(u32, Listed); 12] = [
    (1, PerLane),
    (2, PerLane),
    (3, PerLane),
    (4, PerLane),
    (5, PerLane),
    (6, PerLane),
    (7, Shared),
    (8, Shared),
    (9, Shared),
    (10, Shared),
    (11, PerLane),
    (12, PerLane),
];
const ROUTE_OWN: [(u32, Listed); 8] = [
    (1, Shared),
    (2, Shared),
    (3, Shared),
    (4, Shared),
    (5, Shared),
    (6, Shared),
    (7, Shared),
    (64, PerLane),
];
const VCA_OWN: [(u32, Listed); 2] = [(1, PerLane), (2, PerLane)];
const SESSION_OWN: [(u32, Listed); 3] = [(1, Shared), (2, Shared), (3, Shared)];

fn expand(owner: &Owner, section: &Section, rows: &[(u32, Listed)], out: &mut Vec<ControlAddress>) {
    for &(control, listed) in rows {
        let lanes: &[Lane] = match listed {
            Shared => &[Lane::Both],
            PerLane => &[Lane::Left, Lane::Right],
        };
        for &lane in lanes {
            out.push(ControlAddress {
                owner: owner.clone(),
                section: section.clone(),
                control,
                lane,
            });
        }
    }
}

/// The ruled order, written out without the registry: strips (tracks, then submixes), routes,
/// VCAs, the session.
fn expected_addresses() -> Vec<ControlAddress> {
    let mut out = Vec::new();
    for strip in ["vocal", "guitar", "bus"] {
        let owner = Owner::Strip(id(strip));
        let own: &[(u32, Listed)] = if strip == "bus" {
            &SUBMIX_OWN
        } else {
            &STRIP_OWN
        };
        expand(&owner, &Section::Own, own, &mut out);
        expand(&owner, &Section::Builtins, &BUILTINS, &mut out);
    }
    for route in ["post-fader-out", "pre-fader-out", "send", "bus-out"] {
        expand(
            &Owner::Route(id(route)),
            &Section::Own,
            &ROUTE_OWN,
            &mut out,
        );
    }
    expand(&Owner::Vca(id("riders")), &Section::Own, &VCA_OWN, &mut out);
    expand(&Owner::Session, &Section::Own, &SESSION_OWN, &mut out);
    out
}

/// Gate 1, shape. Red if a row is missing, duplicated, in the wrong order or listed on the wrong
/// lanes.
#[test]
fn list_yields_every_expected_address_once_in_ruled_order() {
    let model = fixture();
    let actual: Vec<ControlAddress> = list(&model, &controls::MonitorOverlay::default(), RATE)
        .map(|row| row.address)
        .collect();
    assert_eq!(actual, expected_addresses());
}

fn class(model: &session::SessionModel, address: &ControlAddress) -> ControlClass {
    row_of(model, address).descriptor.class
}

fn reason(class: ControlClass) -> RebuildReason {
    match class {
        ControlClass::RebuildOnly { reason, .. } => reason,
        other => panic!("expected rebuild-only, got {other:?}"),
    }
}

/// Gate 1, classes. Red if a row's class contradicts decision 14 (or decision 17's ruled-live
/// rule): builtins by descriptor, routes by destination.
#[test]
fn classes_match_decision_14() {
    let model = fixture();
    // Live builtins.
    for control in [5, 6, 7, 8, 9, 10, 12] {
        assert_eq!(
            class(
                &model,
                &strip_builtin("guitar", control, listed_lane(control))
            ),
            ControlClass::Live,
            "builtin {control}"
        );
    }
    // Rule 1, new memory.
    assert_eq!(
        reason(class(&model, &strip_builtin("vocal", 11, Lane::Left))),
        RebuildReason::NewMemory
    );
    // The input section is ruled live and not built.
    for control in [1, 2, 3, 4] {
        assert_eq!(
            reason(class(&model, &strip_builtin("vocal", control, Lane::Left))),
            RebuildReason::RuledLiveNotBuilt {
                issue: builtins::STRIP_INPUT_SLICE
            }
        );
    }
    // A send into the submix: ruled live, not built; `follows_mute` waits for the effective slice.
    for control in 1..=6 {
        assert_eq!(
            reason(class(&model, &route_own("send", control))),
            RebuildReason::RuledLiveNotBuilt { issue: ROUTE_SLICE }
        );
    }
    assert_eq!(
        reason(class(&model, &route_own("send", 7))),
        RebuildReason::RuledLiveNotBuilt {
            issue: EFFECTIVE_SLICE
        }
    );
    // A route into the output keeps the fold, whatever its tap.
    for route in ["post-fader-out", "pre-fader-out", "bus-out"] {
        for control in 1..=6 {
            assert_eq!(
                reason(class(&model, &route_own(route, control))),
                RebuildReason::Optimisation,
                "{route} {control}"
            );
        }
        // `follows_mute` is always false there: the row does not apply.
        assert!(!row_of(&model, &route_own(route, 7)).descriptor.active);
    }
    assert!(row_of(&model, &route_own("send", 7)).descriptor.active);
    // VCA rows wait for the effective slice.
    for control in [1, 2] {
        for lane in [Lane::Left, Lane::Right] {
            let address = ControlAddress {
                owner: Owner::Vca(id("riders")),
                section: Section::Own,
                control,
                lane,
            };
            assert_eq!(
                reason(class(&model, &address)),
                RebuildReason::RuledLiveNotBuilt {
                    issue: EFFECTIVE_SLICE
                }
            );
        }
    }
    // Strip own rows: mode and solo are live, the effective values are derived.
    assert_eq!(
        class(&model, &strip_own("vocal", 1, Lane::Both)),
        ControlClass::Live
    );
    assert_eq!(
        class(&model, &strip_own("vocal", 2, Lane::Both)),
        ControlClass::Live
    );
    assert_eq!(
        row_of(&model, &strip_own("vocal", 2, Lane::Both))
            .descriptor
            .scope,
        ControlScope::Monitor
    );
    for control in [64, 65] {
        assert_eq!(
            class(&model, &strip_own("vocal", control, Lane::Left)),
            ControlClass::Derived
        );
    }
    assert_eq!(
        class(
            &model,
            &ControlAddress {
                lane: Lane::Left,
                ..route_own("send", 64)
            }
        ),
        ControlClass::Derived
    );
}

fn listed_lane(builtin: u32) -> Lane {
    if (7..=10).contains(&builtin) {
        Lane::Both
    } else {
        Lane::Left
    }
}

/// Gate 3. Red if `automatable` is true outside live model value rows, or false on one: the
/// automatable controls of the fixture are exactly these names.
#[test]
fn automatable_is_exactly_the_live_model_value_rows() {
    let model = fixture();
    let names: BTreeSet<&str> = list(&model, &controls::MonitorOverlay::default(), RATE)
        .filter(|row| row.descriptor.automatable())
        .map(|row| row.descriptor.name)
        .collect();
    let expected: BTreeSet<&str> = [
        "fader_db",
        "mute",
        "pan",
        "matrix_ll",
        "matrix_lr",
        "matrix_rl",
        "matrix_rr",
        "matrix_mode",
    ]
    .into_iter()
    .collect();
    assert_eq!(names, expected);
}

/// Gate 3. Red if a row lacks a name, a reason's reopening condition, its choices, or a default
/// and committed value inside its own domain.
#[test]
fn tables_are_self_consistent() {
    let model = fixture();
    let rows: Vec<ControlRow> = list(&model, &controls::MonitorOverlay::default(), RATE).collect();
    let mut names = BTreeSet::new();
    for row in &rows {
        let d = &row.descriptor;
        let at = format!("{:?}", row.address);
        assert!(!d.name.is_empty() && !d.display_name.is_empty(), "{at}");
        assert!(d.minimum <= d.maximum, "{at}");
        assert!(d.contains(d.default), "default of {at}");
        assert!(d.contains(row.value), "value of {at}: {}", row.value);
        if let ControlClass::RebuildOnly { reopen, .. } = d.class {
            assert!(!reopen.is_empty(), "{at}");
        }
        assert_eq!(
            d.choices.is_empty(),
            d.kind != controls::ControlKind::Enumeration,
            "{at}"
        );
        // A name is unique within one owner's section and lane.
        if row.address.owner == Owner::Strip(id("vocal")) {
            assert!(
                names.insert((row.address.section.code(), d.name, row.address.lane as u8)),
                "{at}"
            );
        }
    }
}
