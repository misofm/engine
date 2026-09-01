//! Public API integration coverage for checked-in fixtures.

use conformance::{FixtureLimits, PcmFixture, PlanarBlock, parse_manifest};
use engine::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES, SampleRateHz};

#[test]
fn checked_in_manifest_lists_only_valid_exact_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/conformance");
    let entries = parse_manifest(&std::fs::read(root.join("MANIFEST.tsv")).expect("manifest"))
        .expect("valid manifest");
    assert_eq!(entries.len(), 11);
    for entry in entries {
        let bytes = std::fs::read(root.join(&entry.path)).expect("listed fixture");
        assert_eq!(bytes.len(), entry.length);
        let fixture = PcmFixture::parse(&bytes, FixtureLimits::default()).expect("valid fixture");
        assert_eq!(fixture.checksum(), entry.crc32c);
        assert!(fixture.samples().iter().all(|sample| sample.is_finite()));
    }
}

#[test]
fn fixture_trailing_and_truncated_bytes_fail_before_decode() {
    let mut bytes =
        include_bytes!("../../../fixtures/conformance/v1/rate-048000-impulse-dual-mono.mepcm")
            .to_vec();
    bytes.push(0);
    assert!(PcmFixture::parse(&bytes, Default::default()).is_err());
    let bytes =
        include_bytes!("../../../fixtures/conformance/v1/rate-048000-impulse-dual-mono.mepcm");
    assert!(PcmFixture::parse(&bytes[..bytes.len() - 1], Default::default()).is_err());
}

#[test]
fn planar_blocks_group_launch_gates_and_extended_compatibility_inputs() {
    let samples = [0.0_f32];
    for rate in LAUNCH_SAMPLE_RATES {
        assert!(PlanarBlock::try_new(rate, 1, 1, &samples).is_ok());
    }
    for rate in EXTENDED_COMPATIBILITY_SAMPLE_RATES {
        assert!(PlanarBlock::try_new(rate, 1, 1, &samples).is_ok());
    }
    for rate in [SampleRateHz(0), SampleRateHz(32_000), SampleRateHz(192_001)] {
        assert!(PlanarBlock::try_new(rate, 1, 1, &samples).is_err());
    }
}
