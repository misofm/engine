//! Issue #143 P1 / R7: the derived identity re-pin, with its byte accounting asserted in-tree.
//!
//! Four descriptors moved. Nothing else did. Both halves of that sentence are checked here against
//! the encoder rather than against a recorded hex string, which is what makes this a *derived*
//! re-pin: the number that must hold is a formula over the declared menu, and a future tap that
//! changes it fails here instead of silently moving an identity.

#![allow(missing_docs)]

use effect_compiler::launch_native_effect_registry;
use effect_contract::{EffectDescriptor, ObservationCost, validate_descriptor};
use effect_package::{effect_descriptor_wire_required_size, encode_effect_descriptor_wire};

const LIMIT: u32 = 4 << 20;

/// The four effects that declare a tap. Every other launch effect must be bit-unmoved.
const DYNAMICS: [&str; 4] = [
    "miso.compressor",
    "miso.gate-expander",
    "miso.multiband-compressor",
    "miso.true-peak-limiter",
];

fn wire(descriptor: &'static EffectDescriptor) -> Vec<u8> {
    let size = effect_descriptor_wire_required_size(descriptor, LIMIT).unwrap();
    let mut bytes = vec![0_u8; size as usize];
    assert_eq!(
        encode_effect_descriptor_wire(descriptor, LIMIT, &mut bytes),
        Ok(size)
    );
    bytes
}

/// The same descriptor with its menu removed: the exact pre-#143 shape of these bytes.
///
/// `observations` is the last field and nothing else in the descriptor changed, so this is not an
/// approximation of "what it used to be" -- it is the same value the pre-#143 encoder saw, and its
/// encoding is the identity that was in the tree before this issue.
fn zero_tap_twin(descriptor: &'static EffectDescriptor) -> &'static EffectDescriptor {
    Box::leak(Box::new(EffectDescriptor {
        observations: &[],
        ..*descriptor
    }))
}

#[test]
fn every_declared_tap_costs_exactly_its_record_and_its_two_strings() {
    let registry = launch_native_effect_registry().unwrap();
    let mut moved = Vec::new();
    for descriptor in registry.descriptors() {
        validate_descriptor(descriptor).unwrap();
        let tapped = wire(descriptor);
        let twin = zero_tap_twin(descriptor);
        let untapped = wire(twin);
        let expected_delta: usize = descriptor
            .observations
            .iter()
            .map(|observation| 32 + observation.display_name.len() + observation.display_unit.len())
            .sum();
        assert_eq!(
            tapped.len() - untapped.len(),
            expected_delta,
            "{}: the section is exactly its records plus its strings",
            descriptor.id
        );
        if descriptor.observations.is_empty() {
            // The whole claim of a zero-tap descriptor: not one byte moved, and the two header
            // words a stale reader checks are still the zeros it demands.
            assert_eq!(
                tapped, untapped,
                "{}: zero taps move nothing",
                descriptor.id
            );
            assert_eq!(&tapped[88..96], &[0_u8; 8], "{}", descriptor.id);
        } else {
            moved.push(descriptor.id.as_str());
            assert_ne!(&tapped[88..96], &[0_u8; 8], "{}", descriptor.id);
        }
    }
    moved.sort_unstable();
    assert_eq!(moved, DYNAMICS, "exactly the four dynamics effects moved");
}

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
        // byte changed -- the tap reads state that was already there -- so `state_layout_version`
        // does not, and every persisted envelope of these effects still restores.
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
        // The same 48 bytes for all four, because the two strings are the same two strings.
        assert_eq!(32 + tap.display_name.len() + tap.display_unit.len(), 48);
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
