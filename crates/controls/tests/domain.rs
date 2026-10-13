//! One domain rule per value: the descriptor of a builtin row, of an effect parameter and of a
//! core-owned row agrees with the function the rest of the engine uses, at every launch rate.

mod common;

use builtins::{BUILTIN_PARAMETER_DESCRIPTORS, builtin_filter_cutoff_maximum_hz};
use common::{LAUNCH_EFFECTS, effects_fixture, insert_id, strip_own};
use common::{RATE, fixture, strip_builtin};
use controls::{ControlClass, Lane, MonitorOverlay, Section, list, set};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{ParameterUnit, parameter_value_valid};

fn away_from_zero(value: f32) -> f32 {
    f32::from_bits(value.to_bits() + 1)
}

fn toward_zero(value: f32) -> f32 {
    f32::from_bits(value.to_bits() - 1)
}

/// Red if a builtin row's descriptor is not decided by `BuiltinParameterDomain::contains` at the
/// session's rate: a probe at any rate (the cutoff bound is rate-keyed, so every rate is probed
/// with its own bound and neighbours) gets another answer, which is what a descriptor that carried
/// its own copy of the rate-resolved bounds would give after a bound moved in `builtins`.
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

/// Red if an effect parameter's descriptor is not decided by `effect_contract::parameter_value_valid`:
/// the probes cover each declared edge, its neighbours, the zeros, non-finite values and every
/// declared enumeration choice, for every parameter of every launch effect.
#[test]
fn effect_descriptor_domain_equals_the_contracts_domain() {
    let model = effects_fixture();
    let rows: Vec<_> = list(&model, &MonitorOverlay::default(), RATE).collect();
    let registry = launch_native_effect_registry().unwrap();
    for effect in LAUNCH_EFFECTS {
        let contract = registry.get_ascii(effect).unwrap().descriptor();
        for parameter in contract.parameters {
            let row = rows
                .iter()
                .find(|row| {
                    row.address.section == Section::Inserts(insert_id(effect))
                        && row.address.control == parameter.id.0
                })
                .unwrap();
            let mut probes = vec![
                0.0,
                -0.0,
                1.0,
                -1.0,
                2.0,
                1.0e9,
                f32::NAN,
                f32::INFINITY,
                f32::NEG_INFINITY,
                parameter.default_value,
            ];
            for edge in [parameter.minimum, parameter.maximum].into_iter().flatten() {
                probes.extend([edge, edge.next_up(), edge.next_down()]);
            }
            probes.extend(parameter.enum_choices.iter().map(|choice| choice.value));
            for probe in probes {
                assert_eq!(
                    row.descriptor.contains(probe),
                    parameter_value_valid(parameter, probe),
                    "{effect} {} value {probe}",
                    parameter.name
                );
            }
        }
    }
}

/// Red if a value a valid session holds is outside its row's domain: every listed model row
/// (derived rows compute their value and are not stored) must contain its own committed value,
/// and the largest pan window the grammar holds (`u32::MAX`, which preparation accepts) must be
/// writable and read back inside the row. A row whose maximum is a smaller invented bound turns
/// this red.
#[test]
fn a_value_a_valid_session_holds_is_inside_its_row() {
    let mut model = effects_fixture();
    let window = strip_own("vocal", 3, Lane::Both);
    set(&mut model, &window, u32::MAX as f32, None, RATE).unwrap();
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let rows: Vec<_> = list(&model, &MonitorOverlay::default(), rate).collect();
        let mut seen_window = false;
        for row in &rows {
            if row.descriptor.class == ControlClass::Derived {
                continue;
            }
            assert!(
                row.descriptor.contains(row.value),
                "{:?} holds {} outside {} at {rate}",
                row.address,
                row.value,
                row.descriptor.name
            );
            if row.address == window {
                seen_window = true;
                assert_eq!(row.value as u32, u32::MAX);
            }
        }
        assert!(seen_window);
    }
}
