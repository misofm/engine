//! Issue #143 P1 / R7: which launch descriptors declare an observation tap, and what that moved.
//!
//! Four descriptors declare a tap. Nothing else does. Both claims are read from the static
//! `effect-contract` descriptors the launch registry carries. The byte accounting of the
//! descriptor wire that once sat beside them went with the wire (#1037, owner ruling R6).

#![allow(missing_docs)]

use effect_compiler::launch_native_effect_registry;
use effect_contract::ObservationCost;

/// The four effects that declare a tap.
const DYNAMICS: [&str; 4] = [
    "miso.compressor",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
];

#[test]
fn a_declared_tap_moves_contract_minor_and_leaves_the_state_layout_alone() {
    let registry = launch_native_effect_registry().unwrap();
    for descriptor in registry.descriptors() {
        let dynamics = DYNAMICS.contains(&descriptor.id.as_str());
        assert_eq!(
            descriptor.observations.is_empty(),
            !dynamics,
            "{}: only the dynamics effects declare a tap",
            descriptor.id
        );
        if !dynamics {
            continue;
        }
        // The menu is a *semantic* addition to the descriptor, so `contract_minor` moves. No state
        // word changed -- the tap reads state that was already there -- so `state_layout_version`
        // does not (`launch_native_state_layouts_are_v1`).
        assert_eq!(
            descriptor.contract_minor, 1,
            "{}: declaring the first tap is a minor bump",
            descriptor.id
        );
        assert_eq!(descriptor.observations.len(), 1, "{}", descriptor.id);
        let tap = descriptor.observations[0];
        assert_eq!(tap.id.0, 1, "{}", descriptor.id);
        assert_eq!(tap.display_name, "Gain Reduction", "{}", descriptor.id);
        assert_eq!(tap.display_unit, "dB", "{}", descriptor.id);
        assert_eq!(tap.cost, ObservationCost::Resident, "{}", descriptor.id);
    }
}

/// Every launch-native state layout has its sole prelaunch V1 identity, stated as its own assertion
/// so a future edit that bumps one has to argue with this test rather than with a comment.
#[test]
fn launch_native_state_layouts_are_v1() {
    let registry = launch_native_effect_registry().unwrap();
    for descriptor in registry.descriptors() {
        assert_eq!(
            descriptor.state_layout_version, 1,
            "{}: launch-native state layouts are born at V1",
            descriptor.id
        );
    }
}
