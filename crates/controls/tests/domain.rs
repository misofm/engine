//! The descriptor domain of every builtin row agrees with the builtins' own domain at every
//! launch rate, so the validator cannot drift from `BuiltinParameterDomain::contains`.

mod common;

use builtins::{BUILTIN_PARAMETER_DESCRIPTORS, builtin_filter_cutoff_maximum_hz};
use common::{fixture, strip_builtin};
use controls::{Lane, list};
use effect_contract::ParameterUnit;

fn away_from_zero(value: f32) -> f32 {
    f32::from_bits(value.to_bits() + 1)
}

fn toward_zero(value: f32) -> f32 {
    f32::from_bits(value.to_bits() - 1)
}

/// Red if a builtin row's resolved minimum, maximum, disabled value or toggle encoding in the
/// control descriptor differs from `BuiltinParameterDomain::contains` for any probe at any rate
/// (the cutoff bound is rate-keyed, so every rate is probed with its own bound and neighbours).
#[test]
fn builtin_descriptor_domain_equals_the_builtins_domain() {
    let model = fixture();
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let bound = builtin_filter_cutoff_maximum_hz(rate).unwrap();
        let mut probes = vec![
            0.0,
            -0.0,
            0.5,
            1.0,
            -1.0,
            2.0,
            9.99,
            10.0,
            20.0,
            24.0,
            -144.0,
            48_000.0,
            48_001.0,
            1.0e9,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        for edge in [bound, 24.0, -144.0, 48_000.0, 10.0, 1.0] {
            probes.extend([edge, away_from_zero(edge), toward_zero(edge)]);
        }
        let rows: Vec<_> = list(&model, &controls::MonitorOverlay::default(), rate).collect();
        for builtin in &BUILTIN_PARAMETER_DESCRIPTORS {
            let lane = if builtin.scope == builtins::BuiltinParameterScope::PerLane {
                Lane::Left
            } else {
                Lane::Both
            };
            let address = strip_builtin("vocal", builtin.id, lane);
            let descriptor = rows
                .iter()
                .find(|row| row.address == address)
                .unwrap()
                .descriptor;
            for &probe in &probes {
                // A sample count is a whole number of frames; the builtins domain has no such rule.
                if descriptor.unit == ParameterUnit::Samples && probe != probe.trunc() {
                    continue;
                }
                assert_eq!(
                    descriptor.contains(probe),
                    builtin.domain.contains(probe, rate),
                    "{} at {rate} Hz, value {probe}",
                    builtin.name
                );
            }
        }
    }
}
