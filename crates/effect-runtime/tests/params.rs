//! Parameter domain validation, clamping and defaults.

use effect_runtime::params::{
    ParameterKind, ParameterMapping, ParameterSpec, clamp_to_domain, initial_defaults,
    is_negative_zero, normalize_zero, parameter_value_valid,
};

const THRESHOLD: ParameterSpec = ParameterSpec::continuous(-80.0, 0.0, -18.0);
const FREQUENCY: ParameterSpec = ParameterSpec::logarithmic(10.0, 20_000.0, 1_000.0);
const BYPASS: ParameterSpec = ParameterSpec::boolean(0.0);
const SLOPE: ParameterSpec = ParameterSpec {
    kind: ParameterKind::Enumeration(&[6.0, 12.0, 24.0, 48.0]),
    minimum: 6.0,
    maximum: 48.0,
    mapping: ParameterMapping::Linear,
    default: 12.0,
};

#[test]
fn continuous_domains_are_inclusive_and_reject_non_finite() {
    assert!(parameter_value_valid(&THRESHOLD, -80.0));
    assert!(parameter_value_valid(&THRESHOLD, 0.0));
    assert!(parameter_value_valid(&THRESHOLD, -0.0));
    assert!(parameter_value_valid(&THRESHOLD, -18.0));
    assert!(!parameter_value_valid(&THRESHOLD, -80.001));
    assert!(!parameter_value_valid(&THRESHOLD, 0.001));
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(!parameter_value_valid(&THRESHOLD, bad), "{bad}");
        assert!(!parameter_value_valid(&FREQUENCY, bad));
        assert!(!parameter_value_valid(&BYPASS, bad));
        assert!(!parameter_value_valid(&SLOPE, bad));
    }
}

#[test]
fn boolean_and_enumeration_domains_are_exact() {
    assert!(parameter_value_valid(&BYPASS, 0.0));
    assert!(parameter_value_valid(&BYPASS, -0.0));
    assert!(parameter_value_valid(&BYPASS, 1.0));
    assert!(!parameter_value_valid(&BYPASS, 0.5));
    assert!(!parameter_value_valid(&BYPASS, 2.0));

    for choice in [6.0f32, 12.0, 24.0, 48.0] {
        assert!(parameter_value_valid(&SLOPE, choice));
    }
    assert!(!parameter_value_valid(&SLOPE, 18.0));
    assert!(!parameter_value_valid(&SLOPE, 6.000_001));
}

/// `-0.0` is accepted as a way of writing zero and normalised away on the way in.
#[test]
fn negative_zero_is_accepted_and_normalised() {
    assert!(is_negative_zero(-0.0));
    assert!(!is_negative_zero(0.0));
    assert_eq!(normalize_zero(-0.0).to_bits(), 0.0f32.to_bits());
    assert_eq!(normalize_zero(0.0).to_bits(), 0.0f32.to_bits());
    assert_eq!(normalize_zero(-1.0).to_bits(), (-1.0f32).to_bits());
    assert_eq!(
        clamp_to_domain(&THRESHOLD, -0.0).to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(clamp_to_domain(&BYPASS, -0.0).to_bits(), 0.0f32.to_bits());
}

/// Clamping always lands inside the domain, and a NaN lands on the default rather than
/// propagating into a coefficient.
#[test]
fn clamping_always_lands_in_the_domain() {
    for value in [
        -1000.0f32,
        1000.0,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        -80.0,
        -0.0,
        -40.0,
    ] {
        let clamped = clamp_to_domain(&THRESHOLD, value);
        assert!(
            parameter_value_valid(&THRESHOLD, clamped),
            "{value} clamped to {clamped}"
        );
        assert!(!is_negative_zero(clamped));
    }
    assert_eq!(clamp_to_domain(&THRESHOLD, -1000.0), -80.0);
    assert_eq!(clamp_to_domain(&THRESHOLD, 1000.0), 0.0);
    assert_eq!(clamp_to_domain(&THRESHOLD, f32::NAN), -18.0);
    assert_eq!(clamp_to_domain(&BYPASS, 0.6), 1.0);
    assert_eq!(clamp_to_domain(&BYPASS, 0.4), 0.0);
    assert_eq!(
        clamp_to_domain(&SLOPE, 18.0),
        12.0,
        "enumeration falls back"
    );
    assert_eq!(clamp_to_domain(&SLOPE, 24.0), 24.0);
}

/// Defaults are written in order, clamped, and the write count is reported.
#[test]
fn defaults_are_clamped_and_counted() {
    let specs = [
        THRESHOLD,
        FREQUENCY,
        BYPASS,
        SLOPE,
        // A table with a typo in it: the default is outside its own domain.
        ParameterSpec::continuous(0.0, 1.0, 4.0),
    ];
    let mut out = [-1.0f32; 5];
    assert_eq!(initial_defaults(&specs, &mut out), 5);
    assert_eq!(out[0], -18.0);
    assert_eq!(out[1], 1_000.0);
    assert_eq!(out[2], 0.0);
    assert_eq!(out[3], 12.0);
    assert_eq!(
        out[4], 1.0,
        "an out-of-domain default is clamped, not trusted"
    );
    for (value, spec) in out.iter().zip(&specs) {
        assert!(parameter_value_valid(spec, *value));
    }

    let mut short = [0.0f32; 2];
    assert_eq!(initial_defaults(&specs, &mut short), 2);
    assert_eq!(short[0], -18.0);
}
