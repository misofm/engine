use crate::PackageError;
use sha2::{Digest, Sha256};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedEffectDescriptorV1 {
    pub bytes: Vec<u8>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectPackageV1 {
    pub descriptor: OwnedEffectDescriptorV1,
    pub artifacts: Vec<ArtifactV1>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactV1 {
    pub kind: ArtifactKind,
    pub path: String,
    pub target: String,
    pub features: String,
    pub content: Vec<u8>,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum ArtifactKind {
    Source = 1,
    CoreWasm = 2,
    TargetNative = 3,
}
#[derive(Clone, Copy, Debug)]
pub struct PackageLimits {
    pub maximum_manifest_bytes: u64,
    pub maximum_package_bytes: u64,
    pub maximum_artifacts: u32,
    pub maximum_artifact_bytes: u64,
    pub maximum_descriptor_bytes: u64,
}
impl Default for PackageLimits {
    fn default() -> Self {
        Self {
            maximum_manifest_bytes: 16 * 1024 * 1024,
            maximum_package_bytes: 256 * 1024 * 1024,
            maximum_artifacts: 4096,
            maximum_artifact_bytes: 128 * 1024 * 1024,
            maximum_descriptor_bytes: 4 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Debug)]
pub struct VerifiedEffectPackageV1<'a> {
    pub bytes: &'a [u8],
    pub descriptor: &'a [u8],
    pub artifacts: Vec<VerifiedArtifact<'a>>,
}
#[derive(Clone, Debug)]
pub struct VerifiedArtifact<'a> {
    pub kind: ArtifactKind,
    pub path: &'a str,
    pub target: &'a str,
    pub features: &'a str,
    pub content: &'a [u8],
    pub sha2_256: [u8; 32],
}
#[derive(Clone, Copy, Debug)]
pub struct ArtifactTarget<'a> {
    pub kind: ArtifactKind,
    pub target: &'a str,
    pub capabilities: &'a [&'a str],
}
pub fn encode_canonical_package_v1(
    package: &EffectPackageV1,
    limits: PackageLimits,
    output: &mut Vec<u8>,
) -> Result<(), PackageError> {
    validate_package(package, limits)?;
    let mut artifacts = package.artifacts.clone();
    artifacts.sort_by(|a, b| {
        (
            a.kind,
            a.target.as_bytes(),
            a.features.as_bytes(),
            a.path.as_bytes(),
        )
            .cmp(&(
                b.kind,
                b.target.as_bytes(),
                b.features.as_bytes(),
                b.path.as_bytes(),
            ))
    });
    if artifacts.windows(2).any(|w| same_key(&w[0], &w[1])) {
        return Err(PackageError::Canonical);
    }
    let mut table = Vec::new();
    let mut offset = 0u64;
    for artifact in &artifacts {
        let path = artifact.path.as_bytes();
        let target = artifact.target.as_bytes();
        let features = artifact.features.as_bytes();
        table.extend_from_slice(&(artifact.kind as u32).to_le_bytes());
        table.extend_from_slice(&0u32.to_le_bytes());
        table.extend_from_slice(&(path.len() as u32).to_le_bytes());
        table.extend_from_slice(&(target.len() as u32).to_le_bytes());
        table.extend_from_slice(&(features.len() as u32).to_le_bytes());
        table.extend_from_slice(&0u32.to_le_bytes());
        table.extend_from_slice(&offset.to_le_bytes());
        table.extend_from_slice(&(artifact.content.len() as u64).to_le_bytes());
        table.extend_from_slice(&Sha256::digest(&artifact.content));
        table.extend_from_slice(path);
        table.extend_from_slice(target);
        table.extend_from_slice(features);
        while table.len() % 8 != 0 {
            table.push(0)
        }
        offset = offset
            .checked_add(artifact.content.len() as u64)
            .ok_or(PackageError::Limit)?;
    }
    let total = 64usize
        .checked_add(package.descriptor.bytes.len())
        .and_then(|n| n.checked_add(table.len()))
        .and_then(|n| n.checked_add(offset as usize))
        .ok_or(PackageError::Limit)?;
    if total as u64 > limits.maximum_package_bytes {
        return Err(PackageError::Limit);
    }
    output.clear();
    output.extend_from_slice(b"MISOEPKG");
    output.extend_from_slice(&1u16.to_le_bytes());
    output.extend_from_slice(&64u16.to_le_bytes());
    output.extend_from_slice(&0u32.to_le_bytes());
    output.extend_from_slice(&(package.descriptor.bytes.len() as u64).to_le_bytes());
    output.extend_from_slice(&(table.len() as u64).to_le_bytes());
    output.extend_from_slice(&offset.to_le_bytes());
    output.extend_from_slice(&(artifacts.len() as u32).to_le_bytes());
    output.extend_from_slice(&[0; 20]);
    output.extend_from_slice(&package.descriptor.bytes);
    output.extend_from_slice(&table);
    for artifact in artifacts {
        output.extend_from_slice(&artifact.content)
    }
    Ok(())
}
pub fn verify_canonical_package_v1(
    bytes: &[u8],
    limits: PackageLimits,
) -> Result<VerifiedEffectPackageV1<'_>, PackageError> {
    if bytes.len() < 64
        || &bytes[..8] != b"MISOEPKG"
        || u16::from_le_bytes(bytes[8..10].try_into().unwrap()) != 1
        || u16::from_le_bytes(bytes[10..12].try_into().unwrap()) != 64
        || bytes[12..16] != [0; 4]
        || bytes[44..64] != [0; 20]
    {
        return Err(PackageError::Header);
    }
    let descriptor_len = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
    let table_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
    let content_len = u64::from_le_bytes(bytes[32..40].try_into().unwrap());
    let count = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
    if descriptor_len > limits.maximum_descriptor_bytes || count > limits.maximum_artifacts {
        return Err(PackageError::Limit);
    }
    let manifest = 64usize
        .checked_add(descriptor_len as usize)
        .and_then(|n| n.checked_add(table_len as usize))
        .ok_or(PackageError::Limit)?;
    let total = manifest
        .checked_add(content_len as usize)
        .ok_or(PackageError::Limit)?;
    if total != bytes.len()
        || total as u64 > limits.maximum_package_bytes
        || manifest as u64 > limits.maximum_manifest_bytes
    {
        return Err(PackageError::Length);
    }
    let descriptor = &bytes[64..64 + descriptor_len as usize];
    let table = &bytes[64 + descriptor_len as usize..manifest];
    let mut cursor = 0;
    let mut expected_offset = 0u64;
    let mut prior = None;
    let mut artifacts = Vec::new();
    while cursor < table.len() {
        if table.len() - cursor < 72 {
            return Err(PackageError::Length);
        }
        let entry = &table[cursor..];
        let kind = match u32::from_le_bytes(entry[0..4].try_into().unwrap()) {
            1 => ArtifactKind::Source,
            2 => ArtifactKind::CoreWasm,
            3 => ArtifactKind::TargetNative,
            _ => return Err(PackageError::Canonical),
        };
        if entry[4..8] != [0; 4] || entry[20..24] != [0; 4] {
            return Err(PackageError::Canonical);
        }
        let p = u32::from_le_bytes(entry[8..12].try_into().unwrap()) as usize;
        let t = u32::from_le_bytes(entry[12..16].try_into().unwrap()) as usize;
        let f = u32::from_le_bytes(entry[16..20].try_into().unwrap()) as usize;
        let offset = u64::from_le_bytes(entry[24..32].try_into().unwrap());
        let length = u64::from_le_bytes(entry[32..40].try_into().unwrap());
        let end = 72usize
            .checked_add(p)
            .and_then(|n| n.checked_add(t))
            .and_then(|n| n.checked_add(f))
            .ok_or(PackageError::Length)?;
        if end > table.len()
            || length == 0
            || length > limits.maximum_artifact_bytes
            || offset != expected_offset
        {
            return Err(PackageError::Length);
        }
        let path = core::str::from_utf8(&entry[72..72 + p]).map_err(|_| PackageError::Text)?;
        let target =
            core::str::from_utf8(&entry[72 + p..72 + p + t]).map_err(|_| PackageError::Text)?;
        let features =
            core::str::from_utf8(&entry[72 + p + t..end]).map_err(|_| PackageError::Text)?;
        let padded = (end + 7) & !7;
        if padded > table.len() || table[end..padded].iter().any(|x| *x != 0) {
            return Err(PackageError::Canonical);
        }
        if !valid_artifact_fields(kind, path, target, features) {
            return Err(PackageError::Canonical);
        }
        let content_end = (offset + length) as usize;
        if content_end > content_len as usize {
            return Err(PackageError::Length);
        }
        let content = &bytes[manifest + offset as usize..manifest + content_end];
        let hash = <[u8; 32]>::try_from(&entry[40..72]).unwrap();
        if Sha256::digest(content).as_slice() != hash {
            return Err(PackageError::Hash);
        }
        let key = (
            kind,
            target.as_bytes(),
            features.as_bytes(),
            path.as_bytes(),
        );
        if prior.is_some_and(|old| old >= key) {
            return Err(PackageError::Canonical);
        }
        prior = Some(key);
        expected_offset = offset + length;
        artifacts.push(VerifiedArtifact {
            kind,
            path,
            target,
            features,
            content,
            sha2_256: hash,
        });
        cursor = padded;
    }
    if cursor != table.len()
        || artifacts.len() != count as usize
        || expected_offset != content_len
        || !artifacts.iter().any(|a| a.kind == ArtifactKind::Source)
    {
        return Err(PackageError::Canonical);
    }
    Ok(VerifiedEffectPackageV1 {
        bytes,
        descriptor,
        artifacts,
    })
}
pub fn select_verified_artifact<'a>(
    package: &'a VerifiedEffectPackageV1<'a>,
    target: ArtifactTarget<'_>,
) -> Result<VerifiedArtifact<'a>, PackageError> {
    let mut matches: Vec<_> = package
        .artifacts
        .iter()
        .filter(|a| {
            a.kind == target.kind
                && a.target == target.target
                && a.features
                    .split(',')
                    .filter(|f| !f.is_empty())
                    .all(|f| target.capabilities.contains(&f))
        })
        .collect();
    matches.sort_by(|a, b| {
        b.features
            .split(',')
            .filter(|v| !v.is_empty())
            .count()
            .cmp(&a.features.split(',').filter(|v| !v.is_empty()).count())
            .then(a.features.cmp(b.features))
            .then(a.path.cmp(b.path))
    });
    matches
        .first()
        .map(|a| (*a).clone())
        .ok_or(PackageError::Unavailable)
}
fn same_key(a: &ArtifactV1, b: &ArtifactV1) -> bool {
    (a.kind, &a.target, &a.features, &a.path) == (b.kind, &b.target, &b.features, &b.path)
}
fn validate_package(p: &EffectPackageV1, limits: PackageLimits) -> Result<(), PackageError> {
    if p.descriptor.bytes.len() as u64 > limits.maximum_descriptor_bytes
        || p.artifacts.len() > limits.maximum_artifacts as usize
        || !p.artifacts.iter().any(|a| a.kind == ArtifactKind::Source)
        || p.artifacts.iter().any(|a| {
            a.content.is_empty()
                || a.content.len() as u64 > limits.maximum_artifact_bytes
                || !valid_artifact_fields(a.kind, &a.path, &a.target, &a.features)
        })
    {
        Err(PackageError::Canonical)
    } else {
        Ok(())
    }
}
fn valid_artifact_fields(kind: ArtifactKind, path: &str, target: &str, features: &str) -> bool {
    let path_ok = !path.is_empty()
        && !path.starts_with('/')
        && !path.ends_with('/')
        && path.split('/').all(|s| {
            !s.is_empty()
                && s != "."
                && s != ".."
                && s.bytes().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
                })
        });
    let target_ok = match kind {
        ArtifactKind::Source => target.is_empty(),
        ArtifactKind::CoreWasm => target == "wasm32-unknown-unknown",
        ArtifactKind::TargetNative => {
            !target.is_empty()
                && target
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        }
    };
    let tokens: Vec<_> = if features.is_empty() {
        Vec::new()
    } else {
        features.split(',').collect()
    };
    path_ok
        && target_ok
        && tokens.windows(2).all(|w| w[0] < w[1])
        && tokens.iter().all(|t| {
            !t.is_empty()
                && t.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
}
