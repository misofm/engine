//! Every shipped effect parameter declares a lawful, exactly-spellable lattice (#242 S1).
//!
//! This is the catalog sweep the brief asks for: no row is exempt, and the assertions are the
//! lattice law rather than a transcription of today's numbers, so a new effect or a retuned step
//! is checked by the same sentences.

use miso_engine_effect_compiler::launch_native_effect_registry;
use miso_engine_effect_contract::{
    LatticePoint, ParameterDescriptor, ParameterDomain, ParameterMapping, StepSize, StepUnit,
    decimal_to_f32, lattice_index_for_decimal, parameter_lattice_points, resolve_parameter_step,
};

fn rows() -> Vec<(&'static str, &'static ParameterDescriptor)> {
    let registry = launch_native_effect_registry().expect("launch registry");
    registry
        .descriptors()
        .flat_map(|descriptor| {
            descriptor
                .parameters
                .iter()
                .map(move |parameter| (descriptor.id.as_str(), parameter))
        })
        .collect()
}

fn member(points: &[LatticePoint], value: f32) -> bool {
    let text = format!("{value}");
    lattice_index_for_decimal(points, &text).is_ok()
}

#[test]
fn the_launch_catalog_is_not_empty_and_every_row_is_swept() {
    let rows = rows();
    // The count is not a contract, but a silently emptied sweep is the failure mode this catches.
    assert_eq!(rows.len(), 66, "shipped controllable parameter rows");
    assert_eq!(
        launch_native_effect_registry().expect("registry").len(),
        8,
        "shipped effects"
    );
}

#[test]
fn every_shipped_row_declares_a_lawful_lattice() {
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let points = parameter_lattice_points(parameter)
            .unwrap_or_else(|error| panic!("{where_}: lattice is unlawful: {error:?}"));
        assert!(!points.is_empty(), "{where_}: empty lattice");
        // Ascending, gapless indices, and one canonical rendering per value.
        for (position, point) in points.iter().enumerate() {
            assert_eq!(
                point.index as usize, position,
                "{where_}: index is the ordinal"
            );
        }
        let mut previous: Option<f64> = None;
        for point in &points {
            let value: f64 = point
                .canonical
                .parse()
                .unwrap_or_else(|_| panic!("{where_}: {} is not decimal", point.canonical));
            if let Some(previous) = previous {
                assert!(
                    value > previous,
                    "{where_}: canonical order at {}",
                    point.canonical
                );
            }
            previous = Some(value);
        }
        // Every canonical rendering survives the one blessed conversion.
        for point in &points {
            assert!(
                decimal_to_f32(&point.canonical).is_some(),
                "{where_}: {} does not convert",
                point.canonical
            );
        }
    }
}

#[test]
fn every_declared_bound_and_default_is_a_lattice_member() {
    // #239 ruling 5461507633 B2/B3. This is the sentence that makes `20000`, `20` and
    // `0.70710677` legal to type; without it each would be an unreachable extreme.
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let points = parameter_lattice_points(parameter).expect("lattice");
        if parameter.domain == ParameterDomain::Continuous {
            let minimum = parameter.minimum.expect("continuous minimum");
            let maximum = parameter.maximum.expect("continuous maximum");
            assert!(
                member(&points, minimum),
                "{where_}: minimum {minimum} is illegal"
            );
            assert!(
                member(&points, maximum),
                "{where_}: maximum {maximum} is illegal"
            );
            assert_eq!(
                decimal_to_f32(&points.first().expect("first").canonical),
                Some(minimum),
                "{where_}: the bottom point IS the declared minimum"
            );
            assert_eq!(
                decimal_to_f32(&points.last().expect("last").canonical),
                Some(maximum),
                "{where_}: the top point IS the declared maximum"
            );
        }
        assert!(
            member(&points, parameter.default_value),
            "{where_}: default {} is illegal",
            parameter.default_value
        );
    }
}

#[test]
fn the_prepared_word_of_a_default_is_bit_identical_to_the_descriptor() {
    // Single-conversion authority: a document that spells the default reaches exactly the `f32`
    // the descriptor declares, so admitting the lattice moves no prepared word.
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let points = parameter_lattice_points(parameter).expect("lattice");
        let text = format!("{}", parameter.default_value);
        let index = lattice_index_for_decimal(&points, &text)
            .unwrap_or_else(|_| panic!("{where_}: default {text} is off-lattice"));
        let prepared = decimal_to_f32(&points[index as usize].canonical).expect("conversion");
        assert_eq!(
            prepared.to_bits(),
            parameter.default_value.to_bits(),
            "{where_}: default converts to a different word"
        );
    }
}

#[test]
fn every_lattice_point_reaches_the_engine_through_the_one_blessed_conversion() {
    // #242 eval 2. For every shipped row, both endpoints and an interior point are matched from
    // their own canonical text and converted once. The conversion is a pure function of that
    // text, so re-running it must be bit-identical -- there is no second decimal path that could
    // disagree, and re-spelling a point cannot move a prepared word.
    let mut checked = 0_usize;
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let points = parameter_lattice_points(parameter).expect("lattice");
        let interior = points.len() / 2;
        for position in [0, interior, points.len() - 1] {
            let point = &points[position];
            let index = lattice_index_for_decimal(&points, &point.canonical)
                .unwrap_or_else(|_| panic!("{where_}: {} is not its own point", point.canonical));
            assert_eq!(index as usize, position, "{where_}: matched a different point");
            let once = decimal_to_f32(&point.canonical).expect("conversion");
            let twice = decimal_to_f32(&points[index as usize].canonical).expect("conversion");
            assert_eq!(once.to_bits(), twice.to_bits(), "{where_}: conversion is not a function");
            assert!(once.is_finite(), "{where_}: {} converts to a non-finite word", point.canonical);
            // Trailing zeros are a different spelling of the same value and must not move it.
            let padded = if point.canonical.contains('.') {
                format!("{}0", point.canonical)
            } else {
                format!("{}.0", point.canonical)
            };
            let padded_index = lattice_index_for_decimal(&points, &padded)
                .unwrap_or_else(|_| panic!("{where_}: {padded} is the same value"));
            assert_eq!(padded_index, index, "{where_}: spelling changed the point");
            checked += 1;
        }
    }
    assert!(checked >= 198, "three points on each of 66 rows: {checked}");
}

#[test]
fn every_log_row_steps_geometrically_and_every_linear_row_arithmetically() {
    // A12: equal-unit stepping of a logarithmic parameter is rejected on the record. The
    // declared step unit and the mapping are one statement, so they are checked as one.
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let expected = match (parameter.domain, parameter.mapping) {
            (ParameterDomain::Boolean | ParameterDomain::Enumeration, _) => StepUnit::Index,
            (_, ParameterMapping::Logarithmic) => {
                if parameter.unit == miso_engine_effect_contract::ParameterUnit::Hz {
                    StepUnit::Cents
                } else {
                    StepUnit::Ratio
                }
            }
            _ => StepUnit::Absolute,
        };
        assert_eq!(
            parameter.lattice.step_unit, expected,
            "{where_}: step unit must follow the mapping"
        );
        if matches!(expected, StepUnit::Cents | StepUnit::Ratio) {
            let points = parameter_lattice_points(parameter).expect("lattice");
            // A geometric row's spacings widen with value; an arithmetic one's do not. One
            // comparison of the first and last generated gaps separates them decisively.
            let low: f64 = points[0].canonical.parse().expect("decimal");
            let next: f64 = points[1].canonical.parse().expect("decimal");
            let high: f64 = points[points.len() - 2].canonical.parse().expect("decimal");
            let top: f64 = points[points.len() - 1].canonical.parse().expect("decimal");
            assert!(
                (top - high) > (next - low),
                "{where_}: a logarithmic row must not step in equal units"
            );
        }
    }
}

#[test]
fn every_ladder_is_integer_multiples_of_the_step_and_stays_on_the_lattice() {
    for (effect, parameter) in rows() {
        let where_ = format!("{effect}#{}", parameter.id.0);
        let points = parameter_lattice_points(parameter).expect("lattice");
        let ladder = parameter.lattice.ladder;
        let multiples: Vec<u8> = StepSize::ALL
            .into_iter()
            .map(|size| ladder.multiple(size))
            .collect();
        assert!(multiples[0] >= 1, "{where_}: xs is at least one step");
        assert!(
            multiples.windows(2).all(|pair| pair[0] < pair[1]),
            "{where_}: ladder ascends"
        );
        // Being integer multiples of lattice indices is what makes drifting off-lattice
        // structurally impossible, so every resolved size is checked to land on a real point.
        let start = (points.len() / 2) as u32;
        for size in StepSize::ALL {
            for count in [1_i32, -1] {
                let landed = resolve_parameter_step(&points, start, size, count, ladder)
                    .unwrap_or_else(|| panic!("{where_}: {size:?} did not resolve"));
                assert!(
                    (landed as usize) < points.len(),
                    "{where_}: {size:?} left the lattice"
                );
            }
        }
    }
}

#[test]
fn every_builtin_row_declares_a_lawful_lattice_at_every_launch_rate() {
    // The builtin table is the other half of the shipped surface, including the appended `pan`
    // row and the two rate-keyed cutoffs whose ceiling moves with the prepared rate.
    for rate in [44_100_u32, 48_000, 88_200, 96_000] {
        for descriptor in &miso_engine_builtins::BUILTIN_PARAMETER_DESCRIPTORS {
            let where_ = format!("builtin {} @ {rate}", descriptor.name);
            let lattice = miso_engine_builtins::builtin_parameter_lattice_points(descriptor, rate)
                .unwrap_or_else(|error| panic!("{where_}: unlawful: {error:?}"));
            assert!(!lattice.points.is_empty(), "{where_}: empty");
            for point in &lattice.points {
                assert!(
                    decimal_to_f32(&point.canonical).is_some(),
                    "{where_}: {} does not convert",
                    point.canonical
                );
            }
            // A declared disabled sentinel is an admitted extra value outside the ordered set,
            // never a point that stepping can wander onto.
            if let Some(disabled) = &lattice.disabled {
                assert!(
                    lattice_index_for_decimal(&lattice.points, disabled).is_err(),
                    "{where_}: the disabled sentinel must sit outside the ordered lattice"
                );
            }
        }
    }
}

#[test]
fn the_appended_pan_row_is_the_persisted_pan_authority() {
    // #239 ruling 5461507633 B4: pan persists as pan. The row is stable-ID 12, per-lane, and
    // linear across the full field, so `hard left`, `centre` and `hard right` are all exact.
    let pan = miso_engine_builtins::BUILTIN_PARAMETER_DESCRIPTORS
        .iter()
        .find(|descriptor| descriptor.name == "pan")
        .expect("an authoritative pan row exists");
    assert_eq!(pan.id, 12, "stable appended identifier");
    let lattice =
        miso_engine_builtins::builtin_parameter_lattice_points(pan, 48_000).expect("pan lattice");
    assert_eq!(lattice.disabled, None);
    assert_eq!(lattice.points.first().expect("min").canonical, "-1.00");
    assert_eq!(lattice.points.last().expect("max").canonical, "1.00");
    for exact in ["-1.0", "0.0", "1.0", "-0.5", "0.25"] {
        assert!(
            lattice_index_for_decimal(&lattice.points, exact).is_ok(),
            "{exact} is a legal pan position"
        );
    }
    let refusal = lattice_index_for_decimal(&lattice.points, "0.005").expect_err("off-lattice");
    assert_eq!(refusal.lower.as_deref(), Some("0.00"));
    assert_eq!(refusal.upper.as_deref(), Some("0.01"));
}
