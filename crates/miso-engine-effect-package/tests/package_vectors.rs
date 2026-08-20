//! Package identity regression coverage.
use miso_engine_effect_package::*;
fn fixture() -> EffectPackageV1 {
    EffectPackageV1 {
        descriptor: OwnedEffectDescriptorV1 {
            bytes: b"d".to_vec(),
        },
        artifacts: vec![ArtifactV1 {
            kind: ArtifactKind::Source,
            path: "source/a.rs".to_owned(),
            target: String::new(),
            features: String::new(),
            content: b"hello".to_vec(),
        }],
    }
}
#[test]
fn package_round_trip_and_cid_mutation() {
    let mut bytes = Vec::new();
    encode_canonical_package_v1(&fixture(), PackageLimits::default(), &mut bytes).unwrap();
    let cid = EffectCid::from_package_bytes(&bytes);
    assert_eq!(cid.to_string().parse::<EffectCid>().unwrap(), cid);
    assert!(verify_canonical_package_v1(&bytes, PackageLimits::default()).is_ok());
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    assert_ne!(EffectCid::from_package_bytes(&bytes), cid);
}
#[test]
fn official_hello_cid_primitive_vector() {
    let cid = EffectCid::from_package_bytes(b"hello");
    assert_eq!(cid.as_binary()[..4], [1, 0x55, 0x12, 0x20]);
    assert_eq!(
        cid.to_string(),
        "bafkreibm6jg3ux5qumhcn2b3flc3tyu6dmlb4xa7u5bf44yegnrjhc4yeq"
    );
}
