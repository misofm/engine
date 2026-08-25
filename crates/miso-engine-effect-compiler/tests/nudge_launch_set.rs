//! Issue #127: the launch effect set's nudge ladders, and the two ways they can rot.
//!
//! A ladder is authored data, so the gates here are authoring gates. One asserts that every
//! parameter that *can* carry a ladder does; one asserts that every ladder is either its
//! `(unit, mapping)` class default or a deliberate, listed override; and one asserts that the
//! class defaults are still anchored where the just-noticeable-difference research put them. Red
//! mutations are recorded in `tests/MUTATIONS.md`.

use miso_engine_effect_compiler::launch_native_effect_registry_v1;
use miso_engine_effect_contract::{
    EffectDescriptorV1, NudgeLadderV1, NudgeRatioClassV1, NudgeSizeV1, NudgeStepUnitV1,
    ParameterDomain, ParameterMapping, ParameterUnit, class_nudge_ladder_v1, nudge_neighbours_v1,
    nudge_parameter_value_v1, resolve_nudge_ladder_v1,
};

/// Every deliberate deviation from a `(unit, mapping)` class default, with the reason it exists.
///
/// This is the list issue #127's research says must exist: a unit class is a starting point, and
/// the previous engine's ratio-versus-downsample collision is what happens when a class is treated
/// as a ruling. A parameter that deviates without appearing here is a typo.
const OVERRIDES: &[(&str, &str, NudgeStepUnitV1, f32, &str)] = &[
    (
        "miso.delay",
        "delay time",
        NudgeStepUnitV1::Absolute,
        1.0,
        "the millisecond-on-linear class is anchored for lookahead windows; an echo's smallest \
         audible move is a whole millisecond",
    ),
    (
        "miso.gate-expander",
        "hold",
        NudgeStepUnitV1::Absolute,
        1.0,
        "a hold time runs to a full second, so the class default of 0.1 ms puts the whole ladder \
         below anything a listener can hear",
    ),
    (
        "miso.parametric-eq",
        "band-1-shelf-slope",
        NudgeStepUnitV1::Absolute,
        0.02,
        "shelf slope shares the ratio-on-linear class with a compression ratio, and 0.1 is three \
         times its whole 0.1..1.0 domain",
    ),
];

fn override_for(effect: &str, parameter: &str) -> Option<NudgeLadderV1> {
    // The EQ declares four identical bands, so the override is listed once and applies to the
    // matching field of every band.
    let field = parameter.strip_prefix("band-").map_or(parameter, |rest| {
        rest.split_once('-').map_or(rest, |(_, field)| field)
    });
    OVERRIDES.iter().find_map(|(id, name, unit, xs, _)| {
        let listed = name.strip_prefix("band-").map_or(*name, |rest| {
            rest.split_once('-').map_or(rest, |(_, field)| field)
        });
        // An override changes the magnitude and sometimes the unit; the ratio class is the shared
        // vocabulary and is never per-parameter.
        (*id == effect && listed == field).then_some(NudgeLadderV1 {
            xs: *xs,
            step_unit: *unit,
            ratio_class: NudgeRatioClassV1::Human,
        })
    })
}

fn descriptors() -> Vec<&'static EffectDescriptorV1> {
    launch_native_effect_registry_v1()
        .expect("launch registry")
        .descriptors()
        .collect()
}

/// A parameter carries a ladder unless the ABI cannot express one for it.
///
/// The two exemptions are the whole list: a `Boolean` domain has nothing between its two values,
/// and an `Exponential` mapping has no constant-unit step. Everything else -- including the
/// parameters the live command path cannot move, which an agent still wants described -- declares
/// one.
///
/// Red mutation: drop the `nudge` field from any one effect crate's parameter helper.
#[test]
fn every_launch_parameter_declares_a_ladder_unless_the_abi_cannot_express_one() {
    let mut with = 0;
    let mut without = 0;
    for descriptor in descriptors() {
        for parameter in descriptor.parameters {
            let expressible = !matches!(parameter.domain, ParameterDomain::Boolean)
                && !matches!(parameter.mapping, ParameterMapping::Exponential);
            match parameter.nudge {
                Some(_) => {
                    assert!(
                        expressible,
                        "{}/{} declares a ladder the ABI cannot express",
                        descriptor.id, parameter.display_name
                    );
                    assert!(
                        resolve_nudge_ladder_v1(parameter).is_some(),
                        "{}/{} declares a ladder that does not resolve",
                        descriptor.id,
                        parameter.display_name
                    );
                    with += 1;
                }
                None => {
                    assert!(
                        !expressible,
                        "{}/{} could declare a ladder and does not",
                        descriptor.id, parameter.display_name
                    );
                    without += 1;
                }
            }
        }
    }
    // The launch set is eight effects and 66 parameters; only the four band-enable booleans are
    // inexpressible. Pinning the counts makes "a new effect shipped without ladders" a failure
    // here rather than a silent gap.
    assert_eq!((with, without), (62, 4), "launch-set ladder coverage");
}

/// Every ladder is its class default, or a listed override.
///
/// Red mutation: change one launch parameter's declared `xs` without listing it in `OVERRIDES`.
#[test]
fn every_declared_ladder_is_its_class_default_or_a_listed_override() {
    let mut seen_overrides = 0;
    for descriptor in descriptors() {
        for parameter in descriptor.parameters {
            let Some(declared) = parameter.nudge else {
                continue;
            };
            if let Some(expected) = override_for(descriptor.id.as_str(), parameter.display_name) {
                assert_eq!(
                    (declared.xs, declared.step_unit),
                    (expected.xs, expected.step_unit),
                    "{}/{} does not match its listed override",
                    descriptor.id,
                    parameter.display_name
                );
                seen_overrides += 1;
                continue;
            }
            assert_eq!(
                Some(declared),
                class_nudge_ladder_v1(parameter),
                "{}/{} deviates from its class default without a listed override",
                descriptor.id,
                parameter.display_name
            );
        }
    }
    // Three listed overrides, and the EQ's applies to all four bands.
    assert_eq!(
        seen_overrides, 6,
        "every listed override is actually declared"
    );
}

/// The class defaults are still anchored where the JND research put them.
///
/// This table is a second, independent statement of `default_nudge_ladder_v1`'s anchors, written
/// in the terms a listener hears rather than in the terms the code stores. An `xs` rung measured
/// at mid-domain must land inside `[0.5x, 2x]` of its anchor -- wide enough that rounding to a
/// human-round number is fine, narrow enough that a decimal point in the wrong place is not.
///
/// Red mutation: change the dB class default from 0.5 to 5.0 in `default_nudge_ladder_v1`.
#[test]
fn the_class_defaults_are_jnd_anchored() {
    let anchor = |unit: ParameterUnit, mapping: ParameterMapping| match (unit, mapping) {
        (ParameterUnit::Db, ParameterMapping::Linear) => 0.5,
        (ParameterUnit::Hz, ParameterMapping::Logarithmic) => 20.0,
        (ParameterUnit::Milliseconds, ParameterMapping::Logarithmic) => 5.0,
        (ParameterUnit::Milliseconds, ParameterMapping::Linear) => 0.1,
        (ParameterUnit::Ratio, ParameterMapping::Logarithmic) => 2.5,
        (ParameterUnit::Ratio, ParameterMapping::Linear) => 0.1,
        (ParameterUnit::Linear, ParameterMapping::Linear) => 0.01,
        other => panic!("no anchor declared for {other:?}"),
    };
    let mut measured = 0;
    for descriptor in descriptors() {
        for parameter in descriptor.parameters {
            let Some(declared) = parameter.nudge else {
                continue;
            };
            if matches!(declared.step_unit, NudgeStepUnitV1::Steps) {
                assert_eq!(declared.xs, 1.0, "a stepped xs rung is one choice");
                continue;
            }
            let expected = override_for(descriptor.id.as_str(), parameter.display_name)
                .map_or_else(|| anchor(parameter.unit, parameter.mapping), |o| o.xs);
            // Measure the rung where the parameter actually lives, not where the table says it is.
            let (low, high) = (parameter.minimum.unwrap(), parameter.maximum.unwrap());
            let middle = match parameter.mapping {
                ParameterMapping::Logarithmic => (f64::from(low) * f64::from(high)).sqrt() as f32,
                _ => 0.5 * (low + high),
            };
            let (down, up) = nudge_neighbours_v1(parameter, middle, NudgeSizeV1::Xs)
                .expect("a declared ladder resolves at mid-domain");
            let step = match declared.step_unit {
                NudgeStepUnitV1::Absolute => f64::from(up - down) / 2.0,
                NudgeStepUnitV1::Cents => 1200.0 * (f64::from(up) / f64::from(down)).log2() / 2.0,
                NudgeStepUnitV1::Percent => {
                    100.0 * ((f64::from(up) / f64::from(down)).sqrt() - 1.0)
                }
                NudgeStepUnitV1::Steps => unreachable!("handled above"),
            };
            assert!(
                step >= 0.5 * f64::from(expected) && step <= 2.0 * f64::from(expected),
                "{}/{}: an xs rung measures {step} where {expected} is anchored",
                descriptor.id,
                parameter.display_name
            );
            measured += 1;
        }
    }
    assert_eq!(measured, 58, "every continuous ladder is measured");
}

/// The registry resolves every ladder once, at construction.
///
/// Red mutation: make `NativeEffectRegistry::nudge_ladders` re-derive on each call by returning a
/// freshly resolved vector -- the identity assertion below still passes, but the memo it proves is
/// gone, which is why the assertion is on the *stored* slice rather than on a fresh resolve.
#[test]
fn the_registry_memoizes_every_ladder() {
    let registry = launch_native_effect_registry_v1().expect("launch registry");
    for descriptor in registry.descriptors() {
        let memo = registry
            .nudge_ladders(descriptor.id)
            .expect("a registered effect has a memo");
        assert_eq!(memo.len(), descriptor.parameters.len());
        for (stored, parameter) in memo.iter().zip(descriptor.parameters) {
            assert_eq!(*stored, resolve_nudge_ladder_v1(parameter));
        }
        // The memo is borrowed, not rebuilt: two reads hand back the same address.
        let again = registry.nudge_ladders(descriptor.id).unwrap();
        assert!(std::ptr::eq(memo.as_ptr(), again.as_ptr()));
    }
}

/// A describe-style read of one effect lists five sizes with resolved values on both sides.
#[test]
fn a_describe_read_lists_five_rungs_for_a_compressor_threshold() {
    let registry = launch_native_effect_registry_v1().expect("launch registry");
    let descriptor = registry
        .descriptors()
        .find(|descriptor| descriptor.id.as_str() == "miso.compressor")
        .expect("the compressor is registered");
    let threshold = &descriptor.parameters[0];
    assert_eq!(threshold.display_name, "threshold");
    let mut rows = Vec::new();
    for size in NudgeSizeV1::ALL {
        let (down, up) = nudge_neighbours_v1(threshold, threshold.default_value, size).unwrap();
        rows.push((size.as_str(), down, up));
    }
    assert_eq!(rows.len(), 5);
    // -18 dB, an xs rung of 0.5 dB and the {1, 3, 5, 10, 30} multipliers.
    for (index, (name, down, up)) in rows.iter().enumerate() {
        let multiplier = f32::from([1u16, 3, 5, 10, 30][index]);
        assert!(
            (up - threshold.default_value - 0.5 * multiplier).abs() < 1e-3,
            "{name} up rung"
        );
        assert!(
            (threshold.default_value - down - 0.5 * multiplier).abs() < 1e-3,
            "{name} down rung"
        );
    }
    // And a parameter with no ladder answers with a typed refusal, not a fabricated row.
    let enabled = registry
        .descriptors()
        .find(|descriptor| descriptor.id.as_str() == "miso.parametric-eq")
        .unwrap()
        .parameters
        .iter()
        .find(|parameter| parameter.display_name == "band-1-enabled")
        .unwrap();
    assert!(nudge_parameter_value_v1(enabled, 0.0, NudgeSizeV1::Xs, 1).is_err());
}
