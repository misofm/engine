#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Parameter-lattice law (#242 S1/S2, #239 rulings 5461507633 B and 5462028562 B).
//!
//! These prove the LAW on synthetic descriptors; the shipped catalog is swept separately in
//! `effect-compiler`, which is the crate that can see every launch effect at once.

use effect_contract::{
    AutomationRate, LatticePoint, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain,
    ParameterId, ParameterLattice, ParameterMapping, ParameterUnit, SmoothingRule, StepSize,
    canonical_descriptor_decimal, decimal_to_f32, parameter_lattice_points, resolve_parameter_step,
};

fn descriptor(
    unit: ParameterUnit,
    domain: ParameterDomain,
    mapping: ParameterMapping,
    minimum: Option<f32>,
    maximum: Option<f32>,
    default_value: f32,
    lattice: ParameterLattice,
) -> ParameterDescriptor {
    ParameterDescriptor {
        id: ParameterId(1),
        display_name: "probe",
        name: "probe",
        display_unit: "probe",
        unit,
        domain,
        minimum,
        maximum,
        default_value,
        mapping,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: 16,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice,
        prepared_reason: None,
    }
}

fn hertz(minimum: f32, maximum: f32, default_value: f32, cents: f32) -> Vec<LatticePoint> {
    parameter_lattice_points(&descriptor(
        ParameterUnit::Hz,
        ParameterDomain::Continuous,
        ParameterMapping::Logarithmic,
        Some(minimum),
        Some(maximum),
        default_value,
        ParameterLattice::cents(cents, 3),
    ))
    .expect("cents lattice")
}

fn contains(points: &[LatticePoint], canonical: &str) -> bool {
    points.iter().any(|point| point.canonical == canonical)
}

// ---------------------------------------------------------------------------
// Endpoints and defaults are lattice members by declaration (ruling B2/B3).
// ---------------------------------------------------------------------------

#[test]
fn geometric_ratio_row_keeps_its_round_maximum_and_meaningful_default() {
    // The compressor ratio row: 20:1 is the declared maximum, so `set ratio 20` is legal.
    let ratio = parameter_lattice_points(&descriptor(
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        ParameterMapping::Logarithmic,
        Some(1.0),
        Some(20.0),
        4.0,
        ParameterLattice::ratio(1.02, 8),
    ))
    .expect("ratio lattice");
    assert_eq!(ratio.last().expect("maximum").canonical, "20.00000000");
    assert!(contains(&ratio, "4.00000000"));

    // The EQ Q row: 0.70710677 IS Butterworth 1/sqrt(2). Quantizing it onto the percent grid
    // would be a real quality loss, so ruling B3 admits it as an intrinsic member instead.
    let q = parameter_lattice_points(&descriptor(
        ParameterUnit::Ratio,
        ParameterDomain::Continuous,
        ParameterMapping::Logarithmic,
        Some(0.1),
        Some(18.0),
        core::f32::consts::FRAC_1_SQRT_2,
        ParameterLattice::ratio(1.02, 8),
    ))
    .expect("q lattice");
    assert_eq!(q.last().expect("maximum").canonical, "18.00000000");
    let point = q
        .iter()
        .find(|point| point.canonical == "0.70710677")
        .expect("butterworth default");
    assert!(point.intrinsic);
    assert_eq!(
        decimal_to_f32(&point.canonical).expect("blessed conversion"),
        core::f32::consts::FRAC_1_SQRT_2,
        "the declared default survives the lattice exactly"
    );
}

// ---------------------------------------------------------------------------
// A12: log rows step in cents/ratio, never in equal raw units.
// ---------------------------------------------------------------------------

#[test]
fn cents_rows_are_geometric_and_never_equal_hertz() {
    let points = hertz(10.0, 20_000.0, 80.0, 20.0);
    // Adjacent GENERATED points hold a constant ratio; the two intrinsic detents (the declared
    // default and the declared maximum) are deliberately allowed to make an irregular adjacency,
    // which is the hardware-detent precedent the ruling cites.
    // The rendering precision itself limits how exactly a spacing can be observed: at the bottom
    // of the range one unit in the last rendered place is worth a measurable fraction of a cent,
    // so the tolerance is derived from the row's own precision rather than guessed.
    let quantum = 0.0005_f64;
    let mut irregular = Vec::new();
    let mut equal_hertz_spacings = std::collections::BTreeSet::new();
    for window in points.windows(2) {
        let low: f64 = window[0].canonical.parse().expect("decimal");
        let high: f64 = window[1].canonical.parse().expect("decimal");
        let cents = 1200.0 * (high / low).log2();
        let tolerance = 1200.0 * ((1.0 + quantum / low).log2() + (1.0 + quantum / high).log2());
        if (cents - 20.0).abs() > tolerance {
            irregular.push((
                window[0].canonical.clone(),
                window[1].canonical.clone(),
                cents,
            ));
        }
        equal_hertz_spacings.insert(((high - low) * 1000.0).round() as i64);
    }
    // Only the intrinsic detents -- the declared default and the declared maximum -- may sit off
    // the regular geometric grid, and each of them disturbs at most the two spacings it touches.
    assert!(
        irregular.len() <= 4,
        "only the intrinsic detents break the 20-cent grid: {irregular:?}"
    );
    for (low, high, _) in &irregular {
        let touches_detent = points
            .iter()
            .filter(|point| point.intrinsic)
            .any(|point| &point.canonical == low || &point.canonical == high);
        assert!(
            touches_detent,
            "irregular spacing {low}..{high} is not at a detent"
        );
    }
    // The red-mutation guard for this eval: an equal-Hertz ladder would collapse every spacing
    // to one value. #127's "1005.79 Hz" lesson, as a gate.
    assert!(
        equal_hertz_spacings.len() > 100,
        "a geometric row must NOT have constant Hertz spacing"
    );
    // Musical evenness: exactly 60 steps of 20 cents make an octave, so an octave above the
    // minimum is a generated point, not an approximation.
    assert!(contains(&points, "20.000"));
    assert!(contains(&points, "40.000"));
    assert!(contains(&points, "80.000"));
}

#[test]
fn arithmetic_rows_step_by_exactly_one_declared_unit() {
    let points = parameter_lattice_points(&descriptor(
        ParameterUnit::Db,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
        Some(-24.0),
        Some(24.0),
        0.0,
        ParameterLattice::arithmetic(0.1, 1),
    ))
    .expect("decibel lattice");
    assert_eq!(points.len(), 481);
    assert_eq!(points.first().expect("minimum").canonical, "-24.0");
    assert_eq!(points.last().expect("maximum").canonical, "24.0");
    assert!(contains(&points, "0.0"));
}

// ---------------------------------------------------------------------------
// Enumerations and booleans.
// ---------------------------------------------------------------------------

#[test]
fn a_boolean_lattice_admits_exactly_its_two_encodings() {
    let points = parameter_lattice_points(&descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Boolean,
        ParameterMapping::Stepped,
        None,
        None,
        0.0,
        ParameterLattice::indices(),
    ))
    .expect("boolean lattice");
    assert_eq!(
        points
            .iter()
            .map(|p| p.canonical.as_str())
            .collect::<Vec<_>>(),
        ["0", "1"]
    );
}

// ---------------------------------------------------------------------------
// A declaration whose own bound it cannot spell is refused, not rounded.
// ---------------------------------------------------------------------------

#[test]
fn a_bound_the_pinned_precision_cannot_spell_is_a_declaration_error() {
    // `0.995` at two decimals renders `1.00`, which is outside its own domain. Before this rule
    // the row would have silently declared a maximum it does not have.
    let two_decimals = parameter_lattice_points(&descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
        Some(0.0),
        Some(0.995),
        0.25,
        ParameterLattice::arithmetic(0.01, 2),
    ));
    assert!(
        two_decimals.is_err(),
        "a bound that cannot be spelled is refused"
    );
    assert_eq!(
        canonical_descriptor_decimal(0.995, 2).as_deref(),
        Some("1.00")
    );

    // The shipped delay row's per-parameter override: three decimals spell the bound exactly and
    // leave one irregular top detent, which is the ruled precedent.
    let three_decimals = parameter_lattice_points(&descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
        Some(0.0),
        Some(0.995),
        0.25,
        ParameterLattice::arithmetic(0.01, 3),
    ))
    .expect("three-decimal damping lattice");
    assert_eq!(three_decimals.last().expect("maximum").canonical, "0.995");
    assert_eq!(
        decimal_to_f32(&three_decimals.last().expect("maximum").canonical),
        Some(0.995)
    );
}

// ---------------------------------------------------------------------------
// The ladder can never leave the lattice.
// ---------------------------------------------------------------------------

#[test]
fn every_ladder_size_lands_on_a_lattice_point_and_clamps_at_the_ends() {
    let points = hertz(10.0, 20_000.0, 80.0, 20.0);
    let ladder = ParameterLattice::cents(20.0, 3).ladder;
    let start = points.len() / 2;
    for size in StepSize::ALL {
        for count in [1_i32, -1, 4, -4] {
            let landed = resolve_parameter_step(&points, start as u32, size, count, ladder)
                .expect("a resolved step stays on the lattice");
            assert!((landed as usize) < points.len());
            let expected = (start as i64 + i64::from(ladder.multiple(size)) * i64::from(count))
                .clamp(0, points.len() as i64 - 1);
            assert_eq!(i64::from(landed), expected, "{size:?} x {count}");
        }
    }
    // Clamping is exact at both ends, never one past.
    assert_eq!(
        resolve_parameter_step(&points, 0, StepSize::Xl, -100, ladder),
        Some(0)
    );
    assert_eq!(
        resolve_parameter_step(&points, points.len() as u32 - 1, StepSize::Xl, 100, ladder),
        Some(points.len() as u32 - 1)
    );
}
