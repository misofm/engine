//! Adversarial public metric and block semantics.

use conformance::{
    BlockError, ComparisonError, ComparisonTolerance, PlanarBlock, SampleLocation, SampleRateHz,
    SnrDb, compare_f32_to_f64,
};

const TOLERANCE: ComparisonTolerance = ComparisonTolerance {
    absolute: 0.25,
    relative: 0.0,
    relative_floor: 1e-12,
};

#[test]
fn tolerance_boundary_tie_and_snr_cases_are_explicit() {
    let actual_samples = [0.0_f32, 0.0];
    let reference_samples = [0.25_f64, -0.25];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 2, &actual_samples).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 2, &reference_samples).unwrap();
    let report = compare_f32_to_f64(actual, reference, TOLERANCE).unwrap();
    assert!(report.within_tolerance);
    assert_eq!(
        report.worst_sample,
        SampleLocation {
            channel: 0,
            frame: 0
        }
    );
    assert!(matches!(report.snr_db, SnrDb::Finite(value) if value.abs() < 1e-12));

    let zeros_f32 = [0.0_f32];
    let zeros_f64 = [0.0_f64];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &zeros_f32).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &zeros_f64).unwrap();
    assert!(matches!(
        compare_f32_to_f64(actual, reference, TOLERANCE)
            .unwrap()
            .snr_db,
        SnrDb::Undefined
    ));

    let nonzero_f32 = [0.25_f32];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &nonzero_f32).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &zeros_f64).unwrap();
    assert!(matches!(
        compare_f32_to_f64(actual, reference, TOLERANCE)
            .unwrap()
            .snr_db,
        SnrDb::NegativeInfinity
    ));

    let one_f32 = [1.0_f32];
    let one_f64 = [1.0_f64];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &one_f32).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &one_f64).unwrap();
    assert!(matches!(
        compare_f32_to_f64(actual, reference, TOLERANCE)
            .unwrap()
            .snr_db,
        SnrDb::PositiveInfinity
    ));
}

#[test]
fn invalid_rates_shapes_tolerances_and_nonfinite_values_are_typed() {
    assert!(matches!(
        PlanarBlock::try_new(SampleRateHz(1), 1, 1, &[0.0_f32]),
        Err(BlockError::InvalidRate)
    ));
    let actual_samples = [0.0_f32];
    let reference_samples = [0.0_f64];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &actual_samples).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(96_000), 1, 1, &reference_samples).unwrap();
    assert_eq!(
        compare_f32_to_f64(actual, reference, TOLERANCE),
        Err(ComparisonError::RateMismatch)
    );

    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &actual_samples).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &reference_samples).unwrap();
    assert_eq!(
        compare_f32_to_f64(
            actual,
            reference,
            ComparisonTolerance {
                absolute: f64::NAN,
                ..TOLERANCE
            }
        ),
        Err(ComparisonError::InvalidTolerance)
    );

    let bad = [f32::INFINITY];
    let actual = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &bad).unwrap();
    let reference = PlanarBlock::try_new(SampleRateHz(48_000), 1, 1, &reference_samples).unwrap();
    assert_eq!(
        compare_f32_to_f64(actual, reference, TOLERANCE),
        Err(ComparisonError::NonFiniteInput)
    );
}
