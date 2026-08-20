//! Public API integration coverage for checked-in fixtures.

use miso_engine_conformance::{FixtureLimits, PcmFixtureV1, parse_manifest};

#[test]
fn checked_in_manifest_lists_only_valid_exact_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/conformance");
    let entries = parse_manifest(&std::fs::read(root.join("MANIFEST.tsv")).expect("manifest"))
        .expect("valid manifest");
    assert_eq!(entries.len(), 11);
    for entry in entries {
        let bytes = std::fs::read(root.join(&entry.path)).expect("listed fixture");
        assert_eq!(bytes.len(), entry.length);
        let fixture = PcmFixtureV1::parse(&bytes, FixtureLimits::default()).expect("valid fixture");
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
    assert!(PcmFixtureV1::parse(&bytes, Default::default()).is_err());
    let bytes =
        include_bytes!("../../../fixtures/conformance/v1/rate-048000-impulse-dual-mono.mepcm");
    assert!(PcmFixtureV1::parse(&bytes[..bytes.len() - 1], Default::default()).is_err());
}
