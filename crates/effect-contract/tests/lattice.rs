//! Parameter-lattice law (#242 S1/S2, #239 rulings 5461507633 B and 5462028562 B).
//!
//! These prove the LAW on synthetic descriptors; the shipped catalog is swept separately in
//! `effect-compiler`, which is the crate that can see every launch effect at once.

use effect_contract::{
    AutomationRate, EnumChoice, LatticePoint, ParameterChannelPolicy, ParameterDescriptor,
    ParameterDomain, ParameterId, ParameterLattice, ParameterMapping, ParameterUnit, SmoothingRule,
    StepSize, canonical_descriptor_decimal, decimal_to_f32, lattice_index_for_decimal,
    parameter_lattice_points, resolve_parameter_step,
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
fn declared_bounds_and_default_are_always_members() {
    // The parametric EQ's frequency row: 20 kHz is the round user-facing extreme, and a pure
    // greatest-generated-point-below-max rule would have made it unreachable.
    let points = hertz(10.0, 20_000.0, 80.0, 20.0);
    assert_eq!(points.first().expect("minimum").canonical, "10.000");
    assert_eq!(points.last().expect("maximum").canonical, "20000.000");
    assert!(points.first().expect("minimum").intrinsic);
    assert!(points.last().expect("maximum").intrinsic);
    assert!(contains(&points, "80.000"));
    // 20 kHz is exactly spellable, which is the whole point of the amendment.
    assert!(lattice_index_for_decimal(&points, "20000.0").is_ok());
    assert!(lattice_index_for_decimal(&points, "20000").is_ok());
}

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
    assert!(lattice_index_for_decimal(&ratio, "20").is_ok());
    assert!(lattice_index_for_decimal(&ratio, "4.0").is_ok());

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
    let index = lattice_index_for_decimal(&q, "0.70710677").expect("butterworth default");
    let point = &q[index as usize];
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
    // 0.25 dB is off a 0.1 dB grid: the refusal must name the two values that bracket it.
    let refusal = lattice_index_for_decimal(&points, "0.25").expect_err("off-lattice");
    assert_eq!(refusal.lower.as_deref(), Some("0.2"));
    assert_eq!(refusal.upper.as_deref(), Some("0.3"));
}

// ---------------------------------------------------------------------------
// S2: exact decimal, never an `f32` round trip.
// ---------------------------------------------------------------------------

#[test]
fn equivalent_spellings_of_one_lattice_point_are_one_value() {
    let points = parameter_lattice_points(&descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
        Some(0.0),
        Some(1.0),
        0.5,
        ParameterLattice::arithmetic(0.01, 2),
    ))
    .expect("linear lattice");
    // The document's accepted spelling is the author's exact utterance and is preserved as
    // written; every equivalent spelling names the same point and reaches the same `f32`.
    let spellings = ["0.3", "0.30", "0.300000", "3e-1", "0.3e0", "+0.30"];
    let mut prepared = Vec::new();
    for spelling in spellings {
        let index = lattice_index_for_decimal(&points, spelling)
            .unwrap_or_else(|_| panic!("{spelling} is the same lattice point"));
        prepared.push(decimal_to_f32(&points[index as usize].canonical).expect("conversion"));
    }
    assert!(
        prepared
            .windows(2)
            .all(|pair| pair[0].to_bits() == pair[1].to_bits()),
        "spelling variance is provably inert in the prepared word: {prepared:?}"
    );
    assert_eq!(prepared[0], 0.3_f32);
}

#[test]
fn decimal_matching_refuses_what_an_f32_comparison_would_admit() {
    // The red mutation this eval exists for: validating by comparing `f32` words instead of
    // decimals. `0.3` is not `f32`-exact, so a whole neighbourhood of DIFFERENT decimals rounds
    // to the same word. Every one of them is off-lattice and must be refused by name.
    let points = parameter_lattice_points(&descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Continuous,
        ParameterMapping::Linear,
        Some(0.0),
        Some(1.0),
        0.5,
        ParameterLattice::arithmetic(0.1, 1),
    ))
    .expect("linear lattice");
    let legal = decimal_to_f32("0.3").expect("blessed conversion");
    let impostors = [
        "0.30000001",
        "0.300000011920928955078125",
        "0.3000000119209290",
        "0.30000000596046447753906250",
    ];
    for impostor in impostors {
        // Each impostor is a DIFFERENT decimal that rounds to the SAME `f32` word, which is
        // exactly what an `f32`-comparison validator cannot tell apart.
        assert_eq!(
            impostor.parse::<f32>().expect("parses").to_bits(),
            legal.to_bits(),
            "{impostor} must be the same f32 word as the legal 0.3"
        );
        let refusal = lattice_index_for_decimal(&points, impostor)
            .expect_err("a different decimal is a different value");
        assert_eq!(refusal.lower.as_deref(), Some("0.3"));
        assert_eq!(refusal.upper.as_deref(), Some("0.4"));
    }
    // And the legal spelling itself still matches, so the rule discriminates rather than refusing
    // everything in the neighbourhood.
    let index = lattice_index_for_decimal(&points, "0.3").expect("the lattice point itself");
    assert_eq!(
        decimal_to_f32(&points[index as usize].canonical).map(f32::to_bits),
        Some(legal.to_bits())
    );
}

#[test]
fn refusal_names_the_two_nearest_legal_values() {
    let points = hertz(10.0, 20_000.0, 80.0, 20.0);
    // The shipped fixture value that #242 makes illegal, and what it would have to become.
    let refusal = lattice_index_for_decimal(&points, "120.0").expect_err("off-lattice");
    assert_eq!(refusal.lower.as_deref(), Some("119.865"));
    assert_eq!(refusal.upper.as_deref(), Some("121.257"));
    // Below the minimum there is no lower neighbour, and above the maximum no upper one: that is
    // out-of-domain, which is a different diagnosis from off-lattice.
    let under = lattice_index_for_decimal(&points, "1.0").expect_err("under range");
    assert_eq!(under.lower, None);
    assert_eq!(under.upper.as_deref(), Some("10.000"));
    let over = lattice_index_for_decimal(&points, "44100.0").expect_err("over range");
    assert_eq!(over.lower.as_deref(), Some("20000.000"));
    assert_eq!(over.upper, None);
    // Text that is not a decimal literal at all brackets nothing.
    let nonsense = lattice_index_for_decimal(&points, "twelve").expect_err("not a decimal");
    assert_eq!(nonsense.lower, None);
    assert_eq!(nonsense.upper, None);
}

// ---------------------------------------------------------------------------
// Enumerations and booleans.
// ---------------------------------------------------------------------------

#[test]
fn an_enumeration_lattice_is_spelled_in_choice_values_and_indexed_by_ordinal() {
    static CHOICES: [EnumChoice; 6] = [
        EnumChoice {
            value: 1.0,
            label: "bell",
        },
        EnumChoice {
            value: 2.0,
            label: "low-shelf",
        },
        EnumChoice {
            value: 3.0,
            label: "high-shelf",
        },
        EnumChoice {
            value: 4.0,
            label: "low-pass",
        },
        EnumChoice {
            value: 5.0,
            label: "high-pass",
        },
        EnumChoice {
            value: 6.0,
            label: "notch",
        },
    ];
    let mut parameter = descriptor(
        ParameterUnit::Linear,
        ParameterDomain::Enumeration,
        ParameterMapping::Stepped,
        None,
        None,
        1.0,
        ParameterLattice::indices(),
    );
    parameter.enum_choices = &CHOICES;
    let points = parameter_lattice_points(&parameter).expect("index lattice");
    // The document spells the CHOICE VALUE; the persist plane carries the ORDINAL. Conflating
    // the two would refuse the last choice and silently relabel every other one.
    assert_eq!(
        points
            .iter()
            .map(|p| p.canonical.as_str())
            .collect::<Vec<_>>(),
        ["1", "2", "3", "4", "5", "6"]
    );
    assert_eq!(lattice_index_for_decimal(&points, "6.0"), Ok(5));
    assert_eq!(lattice_index_for_decimal(&points, "1"), Ok(0));
    assert!(lattice_index_for_decimal(&points, "7").is_err());
    assert!(lattice_index_for_decimal(&points, "1.5").is_err());
}

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
    assert_eq!(lattice_index_for_decimal(&points, "1.0"), Ok(1));
    assert!(lattice_index_for_decimal(&points, "2").is_err());
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

#[test]
fn a_negative_zero_spelling_names_the_zero_lattice_point() {
    // A document may spell zero with a sign. `-0.0` and `0.0` are the same NUMBER, and the
    // lattice is a set of numbers, so both name the one zero point and both prepare the one
    // word. The descriptor surface has no signed zero at all -- `canonical_descriptor_decimal`
    // refuses `-0.0` -- so there is no second point for a signed spelling to land on, and the
    // matcher must not invent one. Pinned because a normalizer that let the sign survive would
    // sort `-0.0` below every negative point and silently refuse a lawful document.
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

    let zero = lattice_index_for_decimal(&points, "0.0").expect("the zero point");
    assert_eq!(points[zero as usize].canonical, "0.0");
    for spelling in ["-0.0", "-0", "0", "+0.0", "-0.00", "0.000"] {
        assert_eq!(
            lattice_index_for_decimal(&points, spelling),
            Ok(zero),
            "{spelling} names the zero lattice point"
        );
    }
    // And the prepared word carries no sign either: every spelling reaches positive zero through
    // the one blessed conversion.
    let prepared = decimal_to_f32(&points[zero as usize].canonical).expect("conversion");
    assert_eq!(prepared.to_bits(), 0.0_f32.to_bits());
    assert!(!prepared.is_sign_negative());

    // A signed zero is still a distinct value from its neighbours, so the rule is about the sign
    // of zero and not about collapsing small magnitudes.
    assert_ne!(lattice_index_for_decimal(&points, "-0.1"), Ok(zero));
    assert_ne!(lattice_index_for_decimal(&points, "0.1"), Ok(zero));
    // The descriptor surface refuses to render a signed zero in the first place.
    assert_eq!(canonical_descriptor_decimal(-0.0, 1), None);
}
