//! Deterministic corruption matrix for the strict fixture/manifest parsers.

use std::panic::catch_unwind;

use conformance::{
    FixtureError, FixtureLimits, PcmFixture, SampleRateHz, SplitMix64, crc32c, parse_manifest,
};

fn fixture() -> Vec<u8> {
    PcmFixture::encode(SampleRateHz(48_000), 2, 2, &[1.0, -0.0, 0.25, -0.25]).expect("fixture")
}

fn set_crc(bytes: &mut [u8]) {
    bytes[40..44].fill(0);
    let checksum = crc32c(bytes);
    bytes[40..44].copy_from_slice(&checksum.to_le_bytes());
}

#[test]
fn every_header_field_limit_overflow_truncation_and_eof_is_rejected() {
    let original = fixture();
    for range in [0..8, 8..10, 10..12] {
        let mut changed = original.clone();
        changed[range.start] ^= 1;
        assert_eq!(
            PcmFixture::parse(&changed, Default::default()),
            Err(FixtureError::InvalidHeader)
        );
    }
    for offset in [12, 16, 20, 22, 24, 44] {
        let mut changed = original.clone();
        changed[offset] ^= 1;
        assert!(PcmFixture::parse(&changed, Default::default()).is_err());
    }
    for length in 0..original.len() {
        assert!(PcmFixture::parse(&original[..length], Default::default()).is_err());
    }
    let mut trailing = original.clone();
    trailing.extend_from_slice(&[0, 1, 2]);
    assert_eq!(
        PcmFixture::parse(&trailing, Default::default()),
        Err(FixtureError::LengthMismatch)
    );
    for limits in [
        FixtureLimits {
            max_channels: 1,
            ..Default::default()
        },
        FixtureLimits {
            max_frames: 1,
            ..Default::default()
        },
        FixtureLimits {
            max_payload_bytes: 15,
            ..Default::default()
        },
    ] {
        assert_eq!(
            PcmFixture::parse(&original, limits),
            Err(FixtureError::LimitsExceeded)
        );
    }
    let mut overflow = original.clone();
    overflow[20..22].copy_from_slice(&u16::MAX.to_le_bytes());
    overflow[24..32].copy_from_slice(&u64::MAX.to_le_bytes());
    overflow[32..40].copy_from_slice(&u64::MAX.to_le_bytes());
    set_crc(&mut overflow);
    assert_eq!(
        PcmFixture::parse(
            &overflow,
            FixtureLimits {
                max_channels: u16::MAX,
                max_frames: u64::MAX,
                max_payload_bytes: u64::MAX,
            }
        ),
        Err(FixtureError::LimitsExceeded)
    );
}

#[test]
fn every_bit_and_4096_seeded_mutations_are_panic_free_and_detected() {
    let original = fixture();
    for byte in 0..original.len() {
        for bit in 0..8 {
            let mut changed = original.clone();
            changed[byte] ^= 1 << bit;
            let result = catch_unwind(|| PcmFixture::parse(&changed, Default::default()));
            assert!(result.is_ok());
            assert!(result.unwrap().is_err());
        }
    }
    let mut generator = SplitMix64::default();
    for _ in 0..4_096 {
        let mut changed = original.clone();
        let index = generator.next_u64() as usize % changed.len();
        let bit = (generator.next_u64() & 7) as u8;
        changed[index] ^= 1_u8 << bit;
        let result = catch_unwind(|| PcmFixture::parse(&changed, Default::default()));
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }
}

#[test]
fn manifest_rejects_all_noncanonical_text_and_path_classes() {
    let valid = b"miso-engine-fixture-manifest-v1\n00000000\t48\tv1/a.mepcm\n";
    assert!(parse_manifest(valid).is_ok());
    for invalid in [
        b"miso-engine-fixture-manifest-v1\r\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\nABCDEF00\t48\tv1/a.mepcm\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t048\tv1/a.mepcm\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\t/a.mepcm\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\t../a.mepcm\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\tC:/a.mepcm\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\tv1/a.mepcm\n00000000\t48\tv1/a.mepcm\n"
            .as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\tv1/b.mepcm\n00000000\t48\tv1/a.mepcm\n"
            .as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\tv1/a.txt\n".as_slice(),
        b"miso-engine-fixture-manifest-v1\n00000000\t48\tv1/a.mepcm".as_slice(),
        b"\xff\n".as_slice(),
    ] {
        assert!(parse_manifest(invalid).is_err(), "accepted {invalid:?}");
    }
}
