//! Independent frozen package-vector qualification.

use core::str::FromStr;
use miso_engine_effect_package::{
    ArtifactSelectionRequest, EffectArtifactAuthoring, EffectArtifactKind, EffectCid,
    EffectPackageAuthoring, EffectPackageLimits, effect_descriptor_identity,
    effect_package_cid, effect_package_required_size, encode_effect_package,
    select_effect_package_artifact, verify_effect_package,
};
use sha2::{Digest, Sha256};

struct Artifact<'a> {
    kind: EffectArtifactKind,
    path: &'a str,
    target: &'a str,
    features: &'a str,
    content_hex: &'a str,
    hash_hex: &'a str,
}

struct Selection<'a> {
    kind: EffectArtifactKind,
    target: &'a str,
    capabilities: &'a [&'a str],
    path: &'a str,
}

struct Vector<'a> {
    package_hex: &'a str,
    package_bytes: usize,
    package_sha: &'a str,
    cid_binary: &'a str,
    cid_text: &'a str,
    descriptor_identity: &'a str,
    artifacts: &'a [Artifact<'a>],
    selections: &'a [Selection<'a>],
}

fn hex(value: &str) -> Vec<u8> {
    let compact: Vec<_> = value
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    assert_eq!(compact.len() & 1, 0);
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("fixture must contain lowercase hexadecimal"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn hex_string(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(DIGITS[(byte >> 4) as usize]));
        output.push(char::from(DIGITS[(byte & 15) as usize]));
    }
    output
}

fn kind_value(kind: EffectArtifactKind) -> u32 {
    kind as u32
}

fn check_vector(vector: &Vector<'_>) -> Vec<u8> {
    let bytes = hex(vector.package_hex);
    assert_eq!(bytes.len(), vector.package_bytes);
    assert_eq!(hex_string(&Sha256::digest(&bytes)), vector.package_sha);
    let verified = verify_effect_package(&bytes, EffectPackageLimits::default()).unwrap();
    assert_eq!(
        hex(vector.descriptor_identity),
        effect_descriptor_identity(verified.descriptor(), 4_194_304)
            .unwrap()
            .as_bytes()
    );

    let actual: Vec<_> = verified.artifacts().collect();
    assert_eq!(actual.len(), vector.artifacts.len());
    for (index, (artifact, expected)) in actual.iter().zip(vector.artifacts).enumerate() {
        assert_eq!(artifact.artifact_index(), index as u32);
        assert_eq!(artifact.kind(), expected.kind);
        assert_eq!(artifact.path(), expected.path);
        assert_eq!(artifact.target(), expected.target);
        assert_eq!(artifact.features(), expected.features);
        assert_eq!(artifact.content(), hex(expected.content_hex));
        assert_eq!(artifact.sha2_256().as_slice(), hex(expected.hash_hex));
        let start = artifact.content().as_ptr() as usize;
        let package_start = bytes.as_ptr() as usize;
        assert!((package_start..package_start + bytes.len()).contains(&start));
    }

    let cid = effect_package_cid(&bytes, EffectPackageLimits::default()).unwrap();
    assert_eq!(cid.as_binary(), hex(vector.cid_binary).as_slice());
    assert_eq!(cid.to_string(), vector.cid_text);
    assert_eq!(EffectCid::from_str(vector.cid_text).unwrap(), cid);
    for selection in vector.selections {
        let selected = select_effect_package_artifact(
            &verified,
            ArtifactSelectionRequest {
                kind: selection.kind,
                target: selection.target,
                capabilities: selection.capabilities,
            },
        )
        .unwrap();
        assert_eq!(selected.path(), selection.path);
    }

    let mut authoring: Vec<_> = actual
        .iter()
        .map(|artifact| EffectArtifactAuthoring {
            kind: artifact.kind(),
            path: artifact.path(),
            target: artifact.target(),
            features: artifact.features(),
            content: artifact.content(),
        })
        .collect();
    authoring.reverse();
    let model = EffectPackageAuthoring {
        descriptor: verified.descriptor(),
        artifacts: &authoring,
    };
    let required =
        effect_package_required_size(&model, EffectPackageLimits::default()).unwrap();
    assert_eq!(required, bytes.len() as u64);
    let mut output = vec![0xa5; bytes.len() + 8];
    assert_eq!(
        encode_effect_package(&model, EffectPackageLimits::default(), &mut output).unwrap(),
        bytes.len()
    );
    assert_eq!(&output[..bytes.len()], bytes);
    assert_eq!(&output[bytes.len()..], &[0xa5; 8]);
    bytes
}

const A_ARTIFACTS: &[Artifact<'_>] = &[
    Artifact {
        kind: EffectArtifactKind::Source,
        path: "src/a.rs",
        target: "",
        features: "",
        content_hex: "412f736f757263652f610a",
        hash_hex: "ebcd1e1cf252671bac35015b5e68348cd9aa73f5c9461ea7d7807ab50e406d18",
    },
    Artifact {
        kind: EffectArtifactKind::Source,
        path: "src/z.rs",
        target: "",
        features: "",
        content_hex: "412f736f757263652f7a0a",
        hash_hex: "f891253b187296a1e51b8ea016d5c2b6f86dd194cbbe3cf6c5516b4bb32b368f",
    },
    Artifact {
        kind: EffectArtifactKind::CoreWasm,
        path: "wasm/base.wasm",
        target: "wasm32-unknown-unknown",
        features: "",
        content_hex: "0061736d412d62617365",
        hash_hex: "99ab3fd8f923338645c9ac751de2fc137d24c11a564aae2c88ca46e615dc209d",
    },
    Artifact {
        kind: EffectArtifactKind::CoreWasm,
        path: "wasm/bulk.wasm",
        target: "wasm32-unknown-unknown",
        features: "bulk-memory,simd128",
        content_hex: "0061736d412d62756c6b2b73696d64",
        hash_hex: "3fa3edf499363cea7820bed9045bc97b8025e777c1c2202290490e256100edff",
    },
    Artifact {
        kind: EffectArtifactKind::CoreWasm,
        path: "wasm/simd.wasm",
        target: "wasm32-unknown-unknown",
        features: "simd128",
        content_hex: "0061736d412d73696d64313238",
        hash_hex: "f42c959a633cab345e79e7612658530039d6cbd012032553bf14ce8b5cf4527b",
    },
    Artifact {
        kind: EffectArtifactKind::TargetNative,
        path: "native/x86-base.so",
        target: "x86_64-unknown-linux-gnu",
        features: "",
        content_hex: "412f6e61746976652f626173657f454c46",
        hash_hex: "e14273d6df76488c573627247d87348435305c9557a529bcf489fa5c6a7df7b2",
    },
    Artifact {
        kind: EffectArtifactKind::TargetNative,
        path: "native/x86-fma.so",
        target: "x86_64-unknown-linux-gnu",
        features: "avx2,fma",
        content_hex: "412f6e61746976652f617678322b666d610081",
        hash_hex: "4c604455c28633e8af6483312ff1801e3e68cfe1a5052b002ce77f19687a79ec",
    },
];

const B_ARTIFACTS: &[Artifact<'_>] = &[
    Artifact {
        kind: EffectArtifactKind::Source,
        path: "source/lib.rs",
        target: "",
        features: "",
        content_hex: "422f736f757263652f64697374696e63740a",
        hash_hex: "b92cf637f522c5a3cdf2864dc5a6deffafca01c0510be29790181a10db939060",
    },
    Artifact {
        kind: EffectArtifactKind::CoreWasm,
        path: "module/base.wasm",
        target: "wasm32-unknown-unknown",
        features: "",
        content_hex: "0061736d422d626173652d64697374696e6374",
        hash_hex: "a1f8cf0699f32ffd5d04af0667b9eaa8df1b2bab1e5effe01ea34c04697116e3",
    },
    Artifact {
        kind: EffectArtifactKind::CoreWasm,
        path: "module/core.wasm",
        target: "wasm32-unknown-unknown",
        features: "simd128",
        content_hex: "0061736d422d636f72652d64697374696e6374",
        hash_hex: "3ed7e6360fa4234557551a903077bc42d671e682c48cc4e52e613cea7203e246",
    },
    Artifact {
        kind: EffectArtifactKind::TargetNative,
        path: "module/arm64.dylib",
        target: "aarch64-apple-darwin",
        features: "neon",
        content_hex: "422f6e61746976652f61726d36342f64697374696e6374",
        hash_hex: "ab9293a9afd0a4aea575a8258695a8df5ba63e88b459e8e47f9fbf2e62119c22",
    },
];

const A_SELECTIONS: &[Selection<'_>] = &[
    Selection {
        kind: EffectArtifactKind::CoreWasm,
        target: "wasm32-unknown-unknown",
        capabilities: &[],
        path: "wasm/base.wasm",
    },
    Selection {
        kind: EffectArtifactKind::CoreWasm,
        target: "wasm32-unknown-unknown",
        capabilities: &["simd128"],
        path: "wasm/simd.wasm",
    },
    Selection {
        kind: EffectArtifactKind::CoreWasm,
        target: "wasm32-unknown-unknown",
        capabilities: &["bulk-memory", "simd128"],
        path: "wasm/bulk.wasm",
    },
    Selection {
        kind: EffectArtifactKind::TargetNative,
        target: "x86_64-unknown-linux-gnu",
        capabilities: &["avx2", "fma"],
        path: "native/x86-fma.so",
    },
];

const B_SELECTIONS: &[Selection<'_>] = &[
    Selection {
        kind: EffectArtifactKind::Source,
        target: "",
        capabilities: &[],
        path: "source/lib.rs",
    },
    Selection {
        kind: EffectArtifactKind::CoreWasm,
        target: "wasm32-unknown-unknown",
        capabilities: &[],
        path: "module/base.wasm",
    },
    Selection {
        kind: EffectArtifactKind::CoreWasm,
        target: "wasm32-unknown-unknown",
        capabilities: &["simd128"],
        path: "module/core.wasm",
    },
    Selection {
        kind: EffectArtifactKind::TargetNative,
        target: "aarch64-apple-darwin",
        capabilities: &["neon"],
        path: "module/arm64.dylib",
    },
];

#[test]
fn python_authored_vectors_match_every_frozen_identity_and_selection() {
    let a = check_vector(&Vector {
        package_hex: include_str!(
            "../../../fixtures/effect-package/v1/comprehensive-a.package.hex"
        ),
        package_bytes: 2547,
        package_sha: "af7b5d38afd3191c33d9d40d95d933ff9b83fe949cb95c3d80bd7bbf916daa52",
        cid_binary: "01551220af7b5d38afd3191c33d9d40d95d933ff9b83fe949cb95c3d80bd7bbf916daa52",
        cid_text: "bafkreifppnotrl6tdeodhwoubwk5sm77tob75fe4xfod3af5po7zc3nkki",
        descriptor_identity: "7d2f1ee79aa5833c546ea06548cb29e13b37f4ab690e9024f1480d2fdfade298",
        artifacts: A_ARTIFACTS,
        selections: A_SELECTIONS,
    });
    let b = check_vector(&Vector {
        package_hex: include_str!(
            "../../../fixtures/effect-package/v1/comprehensive-b.package.hex"
        ),
        package_bytes: 1327,
        package_sha: "6a5934e1222a8601c0aca2194da10f00cc5357596b6355f6c5d64baf748f532c",
        cid_binary: "015512206a5934e1222a8601c0aca2194da10f00cc5357596b6355f6c5d64baf748f532c",
        cid_text: "bafkreidkle2ocirkqya4blfcdfg2cdyazrjvowllmnk7nrowjoxxjd2tfq",
        descriptor_identity: "9bbf09878bca3228ad67687bc492bcc84894181884cf4e3ab387231fb318148f",
        artifacts: B_ARTIFACTS,
        selections: B_SELECTIONS,
    });
    assert_ne!(
        effect_package_cid(&a, EffectPackageLimits::default()).unwrap(),
        effect_package_cid(&b, EffectPackageLimits::default()).unwrap()
    );
}

#[test]
fn legal_content_and_descriptor_changes_accept_and_change_cid() {
    let bytes = hex(include_str!(
        "../../../fixtures/effect-package/v1/comprehensive-a.package.hex"
    ));
    let old_cid = effect_package_cid(&bytes, EffectPackageLimits::default()).unwrap();
    let descriptor_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;
    let table_len = u64::from_le_bytes(bytes[32..40].try_into().unwrap()) as usize;
    let table_start = 96 + descriptor_len;
    let content_start = table_start + table_len;
    let mut changed = bytes.clone();
    changed[content_start] ^= 1;
    let digest = Sha256::digest(
        &changed[content_start..content_start + A_ARTIFACTS[0].content_hex.len() / 2],
    );
    changed[table_start + 40..table_start + 72].copy_from_slice(&digest);
    verify_effect_package(&changed, EffectPackageLimits::default()).unwrap();
    assert_ne!(
        effect_package_cid(&changed, EffectPackageLimits::default()).unwrap(),
        old_cid
    );

    let verified = verify_effect_package(&bytes, EffectPackageLimits::default()).unwrap();
    let descriptor_b = hex(include_str!(
        "../../../fixtures/effect-descriptor/v1/comprehensive-b.wire.hex"
    ));
    let artifacts: Vec<_> = verified
        .artifacts()
        .map(|artifact| EffectArtifactAuthoring {
            kind: artifact.kind(),
            path: artifact.path(),
            target: artifact.target(),
            features: artifact.features(),
            content: artifact.content(),
        })
        .collect();
    let model = EffectPackageAuthoring {
        descriptor: &descriptor_b,
        artifacts: &artifacts,
    };
    let mut other = vec![
        0;
        effect_package_required_size(&model, EffectPackageLimits::default()).unwrap()
            as usize
    ];
    encode_effect_package(&model, EffectPackageLimits::default(), &mut other).unwrap();
    verify_effect_package(&other, EffectPackageLimits::default()).unwrap();
    assert_ne!(
        effect_package_cid(&other, EffectPackageLimits::default()).unwrap(),
        old_cid
    );
}

#[test]
fn enum_discriminants_used_by_the_manifest_are_frozen() {
    assert_eq!(kind_value(EffectArtifactKind::Source), 1);
    assert_eq!(kind_value(EffectArtifactKind::CoreWasm), 2);
    assert_eq!(kind_value(EffectArtifactKind::TargetNative), 3);
}
