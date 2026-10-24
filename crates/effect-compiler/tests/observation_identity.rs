//! Issue #143 P1 / R7: which launch descriptors declare an observation tap, and what that moved.
//!
//! Six descriptors declare a tap. Nothing else does. Both claims are read from the static
//! `effect-contract` descriptors the launch registry carries. The byte accounting of the
//! descriptor wire that once sat beside them went with the wire (#1037, owner ruling R6).

#![allow(missing_docs)]

use effect_compiler::launch_native_effect_registry;
use effect_contract::{ObservationCost, ObservationFold, ObservationKind};

/// The four effects that gained their tap in #143, a `contract_minor` bump to 1.
const TAP_ADDED_BY_143: [&str; 4] = [
    "miso.compressor",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
];

/// The five effects that declare one gain-reduction tap.
const DYNAMICS: [&str; 5] = [
    "miso.compressor",
    "miso.de-esser",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
];

/// The dynamic EQ (#17) declares one signed gain-change tap per band (#1918).
const DYNAMIC_EQ: &str = "miso.dynamic-eq";

#[test]
fn a_declared_tap_moves_contract_minor_and_leaves_the_state_layout_alone() {
    let registry = launch_native_effect_registry().unwrap();
    for descriptor in registry.descriptors() {
        let dynamics = DYNAMICS.contains(&descriptor.id.as_str());
        let band_taps = descriptor.id.as_str() == DYNAMIC_EQ;
        assert_eq!(
            descriptor.observations.is_empty(),
            !(dynamics || band_taps),
            "{}: only the dynamics effects declare a tap",
            descriptor.id
        );
        // A tap added to a shipped descriptor is a *semantic* addition, so #143 moved its
        // `contract_minor`. No state word changed -- the tap reads state that was already there --
        // so `state_layout_version` did not (`launch_native_state_layouts_are_v1`). Every other
        // effect, the de-esser (#15) and the dynamic EQ (#17), born with their taps, included, is at
        // 1.0.
        let bumped = TAP_ADDED_BY_143.contains(&descriptor.id.as_str());
        assert_eq!(
            descriptor.contract_minor,
            u16::from(bumped),
            "{}: only #143's added taps are a minor bump",
            descriptor.id
        );
        if band_taps {
            assert_eq!(descriptor.observations.len(), 4, "{}", descriptor.id);
            for (band, tap) in descriptor.observations.iter().enumerate() {
                assert_eq!(tap.id.0 as usize, band + 1, "{}", descriptor.id);
                assert_eq!(tap.kind, ObservationKind::GainChangeDb, "{}", descriptor.id);
                assert_eq!(tap.fold, ObservationFold::PeakSigned, "{}", descriptor.id);
                assert_eq!(tap.cost, ObservationCost::Resident, "{}", descriptor.id);
            }
            continue;
        }
        if !dynamics {
            continue;
        }
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
