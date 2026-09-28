//! Public API integration coverage for checked-in fixtures.

use conformance::{FixtureError, FixtureLimits, PcmFixture, PlanarBlock, parse_manifest};
use engine::{LAUNCH_SAMPLE_RATES, SampleRateHz};

#[test]
fn checked_in_manifest_lists_only_valid_exact_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/conformance");
    let entries = parse_manifest(&std::fs::read(root.join("MANIFEST.tsv")).expect("manifest"))
        .expect("valid manifest");
    assert_eq!(entries.len(), 7);
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

/// Owner ruling R5 (#1036): blocks and fixtures exist at the launch rates only. The former
/// extended research rates (176.4-384 kHz) refuse exactly like any other rate.
#[test]
fn planar_blocks_and_fixtures_accept_only_launch_rates() {
    let samples = [0.0_f32];
    for rate in LAUNCH_SAMPLE_RATES {
        assert!(PlanarBlock::try_new(rate, 1, 1, &samples).is_ok());
        let bytes = PcmFixture::encode(rate, 1, 1, &samples).expect("launch-rate fixture");
        assert_eq!(
            PcmFixture::parse(&bytes, FixtureLimits::default())
                .expect("launch-rate fixture parses")
                .rate(),
            rate
        );
    }
    let launch = PcmFixture::encode(SampleRateHz(48_000), 1, 1, &samples).expect("launch");
    for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001].map(SampleRateHz) {
        assert!(
            PlanarBlock::try_new(rate, 1, 1, &samples).is_err(),
            "{rate:?}"
        );
        assert_eq!(
            PcmFixture::encode(rate, 1, 1, &samples),
            Err(FixtureError::InvalidField),
            "{rate:?}"
        );
        // The same header at a non-launch rate is refused on parse too.
        let mut bytes = launch.clone();
        bytes[16..20].copy_from_slice(&rate.0.to_le_bytes());
        assert_eq!(
            PcmFixture::parse(&bytes, FixtureLimits::default()),
            Err(FixtureError::InvalidField),
            "{rate:?}"
        );
    }
}
