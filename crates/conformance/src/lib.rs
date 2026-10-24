//! Deterministic fixture, effect-conformance, and protocol-corpus harnesses.

mod effect;
mod fixture;
mod manifest;
mod prng;
mod protocol_corpus;
pub mod ramp_endpoint;
mod randomized;
pub mod rest_probe;
pub mod swap_probe;

pub use dsp_reference::{F64PlanarBuffer, IdentityProcessor, render_planar_f64};
pub use effect::{
    ConformanceConfig, DUAL_ACCUMULATOR_DELAY_DESCRIPTOR, DualAccumulatorDelayFactory,
    EffectConformanceReport, EffectConformanceTierReport, FaultKind, run_effect_conformance,
};
pub use engine::SampleRateHz;
pub use fixture::{FixtureError, FixtureLimits, PcmFixture, crc32c};
pub use manifest::{ManifestEntry, ManifestError, parse_manifest};
pub use prng::SplitMix64;
pub use protocol_corpus::{
    COMPLETE_SCHEMA_HASH, ConformanceDecoder, ConformanceFrame, RetiredCodeRow,
    complete_all_opcode_fixture, complete_all_opcode_fixture_bytes, complete_schema_corpus,
    console_session_fixture, retired_code_rows,
};
pub use randomized::{
    Craft, DifferentialCoverage, EffectDifferential, Payload, admit, assert_d7_reports,
    assert_reached, d7_report_violations, run_effect_differential, tail_bound_for_request,
    tail_bound_of,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes() -> Vec<u8> {
        PcmFixture::encode(
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
        let decoded = PcmFixture::parse(&original, Default::default()).expect("decode");
        assert_eq!(decoded.samples()[1].to_bits(), (-0.0_f32).to_bits());
        assert_eq!(decoded.samples()[2].to_bits(), 0x7fc0_0042);
        for index in 0..original.len() {
            for bit in 0..8 {
                let mut changed = original.clone();
                changed[index] ^= 1 << bit;
                let result =
                    std::panic::catch_unwind(|| PcmFixture::parse(&changed, Default::default()));
                assert!(result.expect("fixture parsing must not panic").is_err());
            }
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
}
