//! Package format and retained raw-CID primitive regression coverage.
use effect_package::*;

fn descriptor() -> Vec<u8> {
    let compact: Vec<_> =
        include_str!("../../../fixtures/effect-descriptor/v1/comprehensive-a.wire.hex")
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("fixture is lowercase hexadecimal"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

#[test]
fn accepted_descriptor_package_round_trip_and_raw_cid_mutation() {
    let descriptor = descriptor();
    let artifacts = [EffectArtifactAuthoring {
        kind: EffectArtifactKind::Source,
        path: "source/a.rs",
        target: "",
        features: "",
        content: b"hello",
    }];
    let package = EffectPackageAuthoring {
        descriptor: &descriptor,
        artifacts: &artifacts,
    };
    let required = effect_package_required_size(&package, EffectPackageLimits::default()).unwrap();
    let mut bytes = vec![0; required as usize];
    encode_effect_package(&package, EffectPackageLimits::default(), &mut bytes).unwrap();
    let cid = effect_package_cid(&bytes, EffectPackageLimits::default()).unwrap();
    assert_eq!(cid.to_string().parse::<EffectCid>().unwrap(), cid);
    assert!(verify_effect_package(&bytes, EffectPackageLimits::default()).is_ok());
    assert!(verify_effect_package_cid(&bytes, EffectPackageLimits::default(), &cid).is_ok());
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    assert_ne!(EffectCid::from_raw_bytes(&bytes), cid);
    assert_eq!(
        effect_package_cid(&bytes, EffectPackageLimits::default())
            .unwrap_err()
            .code,
        EffectPackageDiagnosticCode::Hash
    );
}

#[test]
fn official_hello_cid_primitive_vector() {
    let cid = EffectCid::from_raw_bytes(b"hello");
    assert_eq!(cid.as_binary()[..4], [1, 0x55, 0x12, 0x20]);
    assert_eq!(
        cid.to_string(),
        "bafkreibm6jg3ux5qumhcn2b3flc3tyu6dmlb4xa7u5bf44yegnrjhc4yeq"
    );
}
