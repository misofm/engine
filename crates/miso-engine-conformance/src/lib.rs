//! Deterministic fixture, numerical comparison, and repeatability harnesses.

mod block;
mod compare;
mod determinism;
mod effect;
mod fixture;
mod manifest;
mod prng;

pub use block::{BlockError, PlanarBlock};
pub use compare::{
    ComparisonError, ComparisonReport, ComparisonTolerance, SampleLocation, SnrDb,
    compare_f32_to_f64,
};
pub use determinism::{DeterminismError, verify_bit_exact_repeat};
pub use effect::{
    ConformanceConfig, DUAL_ACCUMULATOR_DELAY_DESCRIPTOR, DualAccumulatorDelayFactory,
    EffectConformanceReport, FaultKind, run_effect_conformance,
};
pub use fixture::{FixtureError, FixtureLimits, PcmFixtureV1, crc32c};
pub use manifest::{ManifestEntry, ManifestError, parse_manifest};
pub use miso_engine_core::SampleRateHz;
pub use miso_engine_dsp_reference::{F64PlanarBuffer, IdentityProcessor, render_planar_f64};
pub use prng::SplitMix64;

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes() -> Vec<u8> {
        PcmFixtureV1::encode(
            SampleRateHz(48_000),
            2,
            2,
            &[1.0, -0.0, f32::from_bits(0x7fc0_0042), -0.25],
        )
        .expect("fixture")
    }

    #[test]
    fn crc32c_known_vector() {
        assert_eq!(crc32c(b"123456789"), 0xe306_9283);
    }

    #[test]
    fn fixture_preserves_bits_and_detects_every_bit_flip() {
        let original = bytes();
        let decoded = PcmFixtureV1::parse(&original, Default::default()).expect("decode");
        assert_eq!(decoded.samples()[1].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(decoded.samples()[2].to_bits(), 0x7fc0_0042);
        for index in 0..original.len() {
            for bit in 0..8 {
                let mut changed = original.clone();
                changed[index] ^= 1 << bit;
                assert!(
                    std::panic::catch_unwind(|| PcmFixtureV1::parse(&changed, Default::default()))
                        .is_ok()
                );
                assert!(PcmFixtureV1::parse(&changed, Default::default()).is_err());
            }
        }
    }

    #[test]
    fn fixture_header_limits_and_exact_length_reject() {
        let original = bytes();
        assert_eq!(
            PcmFixtureV1::parse(&original[..47], Default::default()),
            Err(FixtureError::TruncatedHeader)
        );
        let mut flags = original.clone();
        flags[12] = 1;
        assert_eq!(
            PcmFixtureV1::parse(&flags, Default::default()),
            Err(FixtureError::InvalidField)
        );
        let mut payload = original.clone();
        payload.push(0);
        assert_eq!(
            PcmFixtureV1::parse(&payload, Default::default()),
            Err(FixtureError::LengthMismatch)
        );
        assert_eq!(
            PcmFixtureV1::parse(
                &original,
                FixtureLimits {
                    max_frames: 1,
                    ..Default::default()
                }
            ),
            Err(FixtureError::LimitsExceeded)
        );
    }

    #[test]
    fn manifest_rejects_invalid_classes() {
        assert!(
            parse_manifest(b"miso-engine-fixture-manifest-v1\n00000000\t1\tv1/a.mepcm\n").is_ok()
        );
        for invalid in [
            b"miso-engine-fixture-manifest-v1\r\n".as_slice(),
            b"wrong\n".as_slice(),
            b"miso-engine-fixture-manifest-v1\n123\t1\tv1/a.mepcm\n".as_slice(),
            b"miso-engine-fixture-manifest-v1\n00000000\t1\t../a.mepcm\n".as_slice(),
            b"miso-engine-fixture-manifest-v1\n00000000\t1\tv1/b.mepcm\n00000000\t1\tv1/a.mepcm\n"
                .as_slice(),
        ] {
            assert!(parse_manifest(invalid).is_err());
        }
    }

    #[test]
    fn prng_vectors_and_range_are_frozen() {
        let mut generator = SplitMix64::default();
        for expected in [
            0x5b40_35d4_46c2_35e3,
            0x83ff_9551_1a9c_e0f5,
            0xb3aa_43fa_9b99_1609,
            0xc561_be39_7468_252c,
        ] {
            assert_eq!(generator.next_u64(), expected);
        }
        let mut generator = SplitMix64::default();
        for _ in 0..10_000 {
            let value = generator.next_bipolar_f32();
            assert!((-1.0..1.0).contains(&value));
        }
    }

    #[test]
    fn block_metrics_and_repeat_bits_cover_boundaries() {
        let actual = [1.0_f32, -0.0];
        let block = PlanarBlock::try_new(SampleRateHz(48_000), 1, 2, &actual).expect("block");
        let expected = [1.0_f64, 0.0];
        let reference =
            PlanarBlock::try_new(SampleRateHz(48_000), 1, 2, &expected).expect("reference");
        let report = compare_f32_to_f64(
            block,
            reference,
            ComparisonTolerance {
                absolute: 1e-6,
                relative: 2e-5,
                relative_floor: 1e-12,
            },
        )
        .expect("compare");
        assert!(report.within_tolerance);
        assert_eq!(report.peak_error, 0.0);
        assert!(
            verify_bit_exact_repeat(
                &[f32::from_bits(0x8000_0000), f32::from_bits(0x7fc0_0042)],
                &[f32::from_bits(0x8000_0000), f32::from_bits(0x7fc0_0042)]
            )
            .is_ok()
        );
        assert!(verify_bit_exact_repeat(&[0.0], &[-0.0]).is_err());
    }
}
