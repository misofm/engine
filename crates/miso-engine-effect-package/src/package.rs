use crate::{
    EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE, EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX,
    EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET, EffectPackageDiagnosticCodeV1 as Code,
    EffectPackageDiagnosticV1 as Diagnostic, effect_descriptor_identity_v1,
};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

const MAGIC: &[u8; 8] = b"MISOEPKG";
const VERSION: u16 = 1;
const HEADER_BYTES: u64 = 96;
const RECORD_BYTES: u64 = 72;
const HARD_DESCRIPTOR_CAP: u64 = 4_194_304;
const HARD_ARTIFACT_CAP: usize = 4_096;
type ArtifactOrder = [u16; HARD_ARTIFACT_CAP];

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum EffectArtifactKindV1 {
    Source = 1,
    CoreWasm = 2,
    TargetNative = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct EffectArtifactAuthoringV1<'a> {
    pub kind: EffectArtifactKindV1,
    pub path: &'a str,
    pub target: &'a str,
    pub features: &'a str,
    pub content: &'a [u8],
}

#[derive(Clone, Copy, Debug)]
pub struct EffectPackageAuthoringV1<'a> {
    pub descriptor: &'a [u8],
    pub artifacts: &'a [EffectArtifactAuthoringV1<'a>],
}

#[derive(Clone, Copy, Debug)]
pub struct EffectPackageLimitsV1 {
    pub maximum_descriptor_bytes: u64,
    pub maximum_manifest_bytes: u64,
    pub maximum_package_bytes: u64,
    pub maximum_artifacts: u32,
    pub maximum_artifact_bytes: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct ArtifactSelectionRequestV1<'a> {
    pub kind: EffectArtifactKindV1,
    pub target: &'a str,
    pub capabilities: &'a [&'a str],
}

impl Default for EffectPackageLimitsV1 {
    fn default() -> Self {
        Self {
            maximum_descriptor_bytes: HARD_DESCRIPTOR_CAP,
            maximum_manifest_bytes: 16_777_216,
            maximum_package_bytes: 268_435_456,
            maximum_artifacts: 4_096,
            maximum_artifact_bytes: 134_217_728,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedEffectPackageV1<'a> {
    bytes: &'a [u8],
    descriptor: &'a [u8],
    table: &'a [u8],
    contents: &'a [u8],
    count: u32,
}

impl<'a> VerifiedEffectPackageV1<'a> {
    pub const fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    pub const fn descriptor(self) -> &'a [u8] {
        self.descriptor
    }
    pub const fn artifact_count(self) -> u32 {
        self.count
    }
    pub const fn artifacts(self) -> VerifiedEffectArtifactIteratorV1<'a> {
        VerifiedEffectArtifactIteratorV1 {
            table: self.table,
            contents: self.contents,
            cursor: 0,
            index: 0,
            count: self.count,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedArtifactV1<'a> {
    artifact_index: u32,
    kind: EffectArtifactKindV1,
    path: &'a str,
    target: &'a str,
    features: &'a str,
    content: &'a [u8],
    sha2_256: [u8; 32],
}

impl<'a> VerifiedArtifactV1<'a> {
    pub const fn artifact_index(self) -> u32 {
        self.artifact_index
    }
    pub const fn kind(self) -> EffectArtifactKindV1 {
        self.kind
    }
    pub const fn path(self) -> &'a str {
        self.path
    }
    pub const fn target(self) -> &'a str {
        self.target
    }
    pub const fn features(self) -> &'a str {
        self.features
    }
    pub const fn content(self) -> &'a [u8] {
        self.content
    }
    pub const fn sha2_256(self) -> [u8; 32] {
        self.sha2_256
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifiedEffectArtifactIteratorV1<'a> {
    table: &'a [u8],
    contents: &'a [u8],
    cursor: usize,
    index: u32,
    count: u32,
}

impl<'a> Iterator for VerifiedEffectArtifactIteratorV1<'a> {
    type Item = VerifiedArtifactV1<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            return None;
        }
        let r = trusted_record(self.table, self.cursor);
        let item = VerifiedArtifactV1 {
            artifact_index: self.index,
            kind: r.kind,
            path: r.path,
            target: r.target,
            features: r.features,
            content: &self.contents[r.content_offset..r.content_offset + r.content_len],
            sha2_256: r.hash,
        };
        self.cursor = r.next;
        self.index += 1;
        Some(item)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.count - self.index) as usize;
        (n, Some(n))
    }
}
impl ExactSizeIterator for VerifiedEffectArtifactIteratorV1<'_> {}

#[derive(Clone, Copy)]
struct Layout {
    identity: [u8; 32],
    table: u64,
    content: u64,
    manifest: u64,
    total: u64,
}
#[derive(Clone, Copy)]
struct RawRecord<'a> {
    kind: u32,
    path: &'a [u8],
    target: &'a [u8],
    features: &'a [u8],
    hash: [u8; 32],
    content_offset: u64,
    content_len: u64,
    next: usize,
}
#[derive(Clone, Copy)]
struct TrustedRecord<'a> {
    kind: EffectArtifactKindV1,
    path: &'a str,
    target: &'a str,
    features: &'a str,
    hash: [u8; 32],
    content_offset: usize,
    content_len: usize,
    next: usize,
}
type RecordKey<'a> = (u32, &'a [u8], &'a [u8], &'a [u8]);

fn diag(code: Code, index: u32, offset: u64) -> Diagnostic {
    Diagnostic::new(code, 0, index, offset)
}
fn package_diag(code: Code, offset: u64) -> Diagnostic {
    diag(code, EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX, offset)
}
fn author_diag(code: Code, index: u32) -> Diagnostic {
    diag(code, index, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET)
}
fn add(a: u64, b: u64, offset: u64) -> Result<u64, Diagnostic> {
    a.checked_add(b)
        .ok_or_else(|| package_diag(Code::Overflow, offset))
}
fn host(value: u64, offset: u64) -> Result<usize, Diagnostic> {
    if value > isize::MAX as u64 {
        return Err(package_diag(Code::Overflow, offset));
    }
    usize::try_from(value).map_err(|_| package_diag(Code::Overflow, offset))
}
fn align8(value: u64, offset: u64) -> Result<u64, Diagnostic> {
    add(value, 7, offset).map(|v| v & !7)
}

fn descriptor_identity(bytes: &[u8], maximum: u64) -> Result<[u8; 32], Diagnostic> {
    let cap = maximum.min(HARD_DESCRIPTOR_CAP);
    if bytes.len() as u64 > cap {
        return Err(package_diag(Code::Limit, 24));
    }
    effect_descriptor_identity_v1(bytes, cap as u32)
        .map(|v| *v.as_bytes())
        .map_err(|e| {
            let offset = if e.byte_offset == EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE {
                EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET
            } else {
                HEADER_BYTES + u64::from(e.byte_offset)
            };
            Diagnostic::new(
                Code::Descriptor,
                e.code as u32,
                EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX,
                offset,
            )
        })
}

fn artifact_cap(limits: EffectPackageLimitsV1) -> u32 {
    limits.maximum_artifacts.min(HARD_ARTIFACT_CAP as u32)
}

/// Fills `order[..artifacts.len()]` with caller indices in canonical record order and returns the
/// caller index of the first record, in caller order, whose key repeats an earlier record's key.
///
/// Callers guarantee `artifacts.len() <= HARD_ARTIFACT_CAP`.
fn canonical_order(
    artifacts: &[EffectArtifactAuthoringV1<'_>],
    order: &mut ArtifactOrder,
) -> Option<u32> {
    let order = &mut order[..artifacts.len()];
    for (slot, index) in order.iter_mut().zip(0u16..) {
        *slot = index;
    }
    let at = |slot: u16| &artifacts[usize::from(slot)];
    order.sort_unstable_by(|a, b| key_cmp(at(*a), at(*b)).then(a.cmp(b)));
    order
        .windows(2)
        .filter(|pair| key_cmp(at(pair[0]), at(pair[1])) == Ordering::Equal)
        .map(|pair| u32::from(pair[1]))
        .min()
}

fn key_cmp(a: &EffectArtifactAuthoringV1<'_>, b: &EffectArtifactAuthoringV1<'_>) -> Ordering {
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
}
fn valid_path(path: &[u8]) -> bool {
    !path.is_empty()
        && path.len() <= 255
        && path.is_ascii()
        && path[0] != b'/'
        && path[path.len() - 1] != b'/'
        && path.split(|b| *b == b'/').all(|s| {
            !s.is_empty()
                && s != b"."
                && s != b".."
                && s.iter().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
                })
        })
}
fn valid_native_target(target: &[u8]) -> bool {
    if target.is_empty()
        || target.len() > 127
        || !target.is_ascii()
        || target.starts_with(b"wasm32-")
    {
        return false;
    }
    let mut count = 0;
    for c in target.split(|b| *b == b'-') {
        count += 1;
        if c.is_empty()
            || !c
                .iter()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'_')
        {
            return false;
        }
    }
    matches!(count, 3 | 4)
}
fn valid_features(features: &[u8]) -> bool {
    if features.len() > 255 || !features.is_ascii() {
        return false;
    }
    if features.is_empty() {
        return true;
    }
    let mut prior: Option<&[u8]> = None;
    for token in features.split(|b| *b == b',') {
        if !valid_feature_token(token) || prior.is_some_and(|p| p >= token) {
            return false;
        }
        prior = Some(token);
    }
    true
}

fn valid_feature_token(token: &[u8]) -> bool {
    !token.is_empty()
        && token.len() <= 32
        && token[0].is_ascii_lowercase()
        && token[1..]
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}
fn validate_artifact(
    a: &EffectArtifactAuthoringV1<'_>,
    index: u32,
    limits: EffectPackageLimitsV1,
) -> Result<(), Diagnostic> {
    if !valid_path(a.path.as_bytes()) {
        return Err(author_diag(Code::Path, index));
    }
    let target = match a.kind {
        EffectArtifactKindV1::Source => a.target.is_empty(),
        EffectArtifactKindV1::CoreWasm => a.target == "wasm32-unknown-unknown",
        EffectArtifactKindV1::TargetNative => valid_native_target(a.target.as_bytes()),
    };
    if !target {
        return Err(author_diag(Code::Target, index));
    }
    if (a.kind == EffectArtifactKindV1::Source && !a.features.is_empty())
        || !valid_features(a.features.as_bytes())
    {
        return Err(author_diag(Code::Features, index));
    }
    if a.content.is_empty() {
        return Err(author_diag(Code::Length, index));
    }
    if a.content.len() as u64 > limits.maximum_artifact_bytes {
        return Err(author_diag(Code::Limit, index));
    }
    Ok(())
}

fn preflight(
    package: &EffectPackageAuthoringV1<'_>,
    limits: EffectPackageLimitsV1,
    order: &mut ArtifactOrder,
) -> Result<Layout, Diagnostic> {
    let identity = descriptor_identity(package.descriptor, limits.maximum_descriptor_bytes)?;
    let count =
        u32::try_from(package.artifacts.len()).map_err(|_| package_diag(Code::Limit, 48))?;
    if count > artifact_cap(limits) {
        return Err(package_diag(Code::Limit, 48));
    }
    let duplicate = canonical_order(package.artifacts, order);
    let mut table = 0u64;
    let mut content = 0u64;
    let mut source = false;
    for (i, artifact) in package.artifacts.iter().enumerate() {
        validate_artifact(artifact, i as u32, limits)?;
        source |= artifact.kind == EffectArtifactKindV1::Source;
        if duplicate == Some(i as u32) {
            return Err(author_diag(Code::Order, i as u32));
        }
        let strings = add(
            add(
                artifact.path.len() as u64,
                artifact.target.len() as u64,
                EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
            )?,
            artifact.features.len() as u64,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        )?;
        table = add(
            table,
            align8(
                add(RECORD_BYTES, strings, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET)?,
                EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
            )?,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        )?;
        content = add(
            content,
            artifact.content.len() as u64,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        )?;
    }
    if !source {
        return Err(package_diag(
            Code::Unavailable,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        ));
    }
    let manifest = add(
        add(HEADER_BYTES, package.descriptor.len() as u64, 24)?,
        table,
        32,
    )?;
    if manifest > limits.maximum_manifest_bytes {
        return Err(package_diag(Code::Limit, 32));
    }
    let total = add(manifest, content, 40)?;
    if total > limits.maximum_package_bytes {
        return Err(package_diag(Code::Limit, 16));
    }
    host(total, 16)?;
    Ok(Layout {
        identity,
        table,
        content,
        manifest,
        total,
    })
}

pub fn effect_package_v1_required_size(
    package: &EffectPackageAuthoringV1<'_>,
    limits: EffectPackageLimitsV1,
) -> Result<u64, Diagnostic> {
    let mut order = [0; HARD_ARTIFACT_CAP];
    preflight(package, limits, &mut order).map(|v| v.total)
}
pub fn encode_effect_package_v1(
    package: &EffectPackageAuthoringV1<'_>,
    limits: EffectPackageLimitsV1,
    output: &mut [u8],
) -> Result<usize, Diagnostic> {
    let mut order = [0; HARD_ARTIFACT_CAP];
    let layout = preflight(package, limits, &mut order)?;
    let required = host(layout.total, 16)?;
    if output.len() < required {
        return Err(Diagnostic::buffer_too_small(layout.total));
    }
    let descriptor_len = package.descriptor.len();
    output[..required].fill(0);
    output[..8].copy_from_slice(MAGIC);
    output[8..10].copy_from_slice(&VERSION.to_le_bytes());
    output[10..12].copy_from_slice(&(HEADER_BYTES as u16).to_le_bytes());
    output[16..24].copy_from_slice(&layout.total.to_le_bytes());
    output[24..32].copy_from_slice(&(descriptor_len as u64).to_le_bytes());
    output[32..40].copy_from_slice(&layout.table.to_le_bytes());
    output[40..48].copy_from_slice(&layout.content.to_le_bytes());
    output[48..52].copy_from_slice(&(package.artifacts.len() as u32).to_le_bytes());
    output[56..88].copy_from_slice(&layout.identity);
    output[96..96 + descriptor_len].copy_from_slice(package.descriptor);
    let mut tc = 96 + descriptor_len;
    let mut cc = host(layout.manifest, 32)?;
    let content_start = cc;
    for &slot in &order[..package.artifacts.len()] {
        let a = &package.artifacts[usize::from(slot)];
        let p = a.path.as_bytes();
        let t = a.target.as_bytes();
        let f = a.features.as_bytes();
        output[tc..tc + 4].copy_from_slice(&(a.kind as u32).to_le_bytes());
        output[tc + 8..tc + 12].copy_from_slice(&(p.len() as u32).to_le_bytes());
        output[tc + 12..tc + 16].copy_from_slice(&(t.len() as u32).to_le_bytes());
        output[tc + 16..tc + 20].copy_from_slice(&(f.len() as u32).to_le_bytes());
        output[tc + 24..tc + 32].copy_from_slice(&((cc - content_start) as u64).to_le_bytes());
        output[tc + 32..tc + 40].copy_from_slice(&(a.content.len() as u64).to_le_bytes());
        output[tc + 40..tc + 72].copy_from_slice(&Sha256::digest(a.content));
        let s = tc + 72;
        output[s..s + p.len()].copy_from_slice(p);
        output[s + p.len()..s + p.len() + t.len()].copy_from_slice(t);
        output[s + p.len() + t.len()..s + p.len() + t.len() + f.len()].copy_from_slice(f);
        tc += (72 + p.len() + t.len() + f.len() + 7) & !7;
        output[cc..cc + a.content.len()].copy_from_slice(a.content);
        cc += a.content.len();
    }
    debug_assert_eq!(tc, content_start);
    debug_assert_eq!(cc, required);
    Ok(required)
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes(b[o..o + 2].try_into().expect("bounded"))
}
fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().expect("bounded"))
}
fn u64_at(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().expect("bounded"))
}
fn raw_record<'a>(
    table: &'a [u8],
    cursor: usize,
    index: u32,
    base: u64,
) -> Result<RawRecord<'a>, Diagnostic> {
    let absolute = base + cursor as u64;
    if table.len().saturating_sub(cursor) < 72 {
        return Err(diag(Code::Length, index, absolute));
    }
    let e = &table[cursor..];
    let pl = u32_at(e, 8) as u64;
    let tl = u32_at(e, 12) as u64;
    let fl = u32_at(e, 16) as u64;
    let available = (table.len() - cursor) as u64;
    let path_end = add(RECORD_BYTES, pl, absolute + 8)?;
    if path_end > available {
        return Err(diag(Code::Length, index, absolute + 8));
    }
    let target_end = add(path_end, tl, absolute + 12)?;
    if target_end > available {
        return Err(diag(Code::Length, index, absolute + 12));
    }
    let end = add(target_end, fl, absolute + 16)?;
    if end > available {
        return Err(diag(Code::Length, index, absolute + 16));
    }
    let padded = align8(end, absolute + 16)?;
    if padded > available {
        return Err(diag(Code::Length, index, absolute + 16));
    }
    let next = host(add(cursor as u64, padded, absolute)?, absolute)?;
    let ps = cursor + 72;
    let ts = ps + pl as usize;
    let fs = ts + tl as usize;
    let se = fs + fl as usize;
    Ok(RawRecord {
        kind: u32_at(e, 0),
        path: &table[ps..ts],
        target: &table[ts..fs],
        features: &table[fs..se],
        hash: table[cursor + 40..cursor + 72].try_into().expect("hash"),
        content_offset: u64_at(e, 24),
        content_len: u64_at(e, 32),
        next,
    })
}
fn kind(raw: u32, index: u32, offset: u64) -> Result<EffectArtifactKindV1, Diagnostic> {
    match raw {
        1 => Ok(EffectArtifactKindV1::Source),
        2 => Ok(EffectArtifactKindV1::CoreWasm),
        3 => Ok(EffectArtifactKindV1::TargetNative),
        _ => Err(diag(Code::Enum, index, offset)),
    }
}

pub fn verify_effect_package_v1(
    bytes: &[u8],
    limits: EffectPackageLimitsV1,
) -> Result<VerifiedEffectPackageV1<'_>, Diagnostic> {
    if bytes.len() as u64 > limits.maximum_package_bytes {
        return Err(package_diag(Code::Limit, 16));
    }
    if bytes.len() < 96 {
        return Err(package_diag(Code::Header, 0));
    }
    let total = u64_at(bytes, 16);
    let dl = u64_at(bytes, 24);
    let tl = u64_at(bytes, 32);
    let cl = u64_at(bytes, 40);
    let count = u32_at(bytes, 48);
    if total > limits.maximum_package_bytes {
        return Err(package_diag(Code::Limit, 16));
    }
    if dl > limits.maximum_descriptor_bytes.min(HARD_DESCRIPTOR_CAP) {
        return Err(package_diag(Code::Limit, 24));
    }
    let manifest = add(add(HEADER_BYTES, dl, 24)?, tl, 32)?;
    let computed = add(manifest, cl, 40)?;
    host(computed, 16)?;
    if manifest > limits.maximum_manifest_bytes {
        return Err(package_diag(Code::Limit, 32));
    }
    if count > artifact_cap(limits) {
        return Err(package_diag(Code::Limit, 48));
    }
    let de = host(add(HEADER_BYTES, dl, 24)?, 24)?;
    let me = host(manifest, 32)?;
    // Phase-one fixed-prefix checks apply to every declared record prefix that is physically
    // present. A later truncated table must not hide an earlier record's limit violation.
    if de <= bytes.len() {
        let table = &bytes[de..me.min(bytes.len())];
        let mut cursor = 0usize;
        let mut content_sum = 0u64;
        for i in 0..count {
            if table.len().saturating_sub(cursor) < 40 {
                break;
            }
            let at = de as u64 + cursor as u64;
            let content_len = u64_at(table, cursor + 32);
            if content_len > limits.maximum_artifact_bytes {
                return Err(diag(Code::Limit, i, at + 32));
            }
            content_sum = add(content_sum, content_len, at + 32)?;
            let Ok(r) = raw_record(table, cursor, i, de as u64) else {
                break;
            };
            cursor = r.next;
        }
    }
    if &bytes[..8] != MAGIC {
        return Err(package_diag(Code::Header, 0));
    }
    if u16_at(bytes, 8) != VERSION {
        return Err(package_diag(Code::Header, 8));
    }
    if u16_at(bytes, 10) != 96 {
        return Err(package_diag(Code::Header, 10));
    }
    if bytes[12..16].iter().any(|b| *b != 0) {
        return Err(package_diag(Code::Reserved, 12));
    }
    if bytes[52..56].iter().any(|b| *b != 0) {
        return Err(package_diag(Code::Reserved, 52));
    }
    if let Some(i) = bytes[88..96].iter().position(|b| *b != 0) {
        return Err(package_diag(Code::Reserved, 88 + i as u64));
    }
    if de <= bytes.len() {
        let table = &bytes[de..me.min(bytes.len())];
        let mut cursor = 0usize;
        for i in 0..count {
            let remaining = table.len().saturating_sub(cursor);
            if remaining < 8 {
                break;
            }
            let at = de as u64 + cursor as u64;
            if table[cursor + 4..cursor + 8].iter().any(|b| *b != 0) {
                return Err(diag(Code::Reserved, i, at + 4));
            }
            if remaining < 24 {
                break;
            }
            if table[cursor + 20..cursor + 24].iter().any(|b| *b != 0) {
                return Err(diag(Code::Reserved, i, at + 20));
            }
            if remaining < RECORD_BYTES as usize {
                break;
            }
            let Ok(r) = raw_record(table, cursor, i, de as u64) else {
                break;
            };
            cursor = r.next;
        }
    }
    if total != computed || total != bytes.len() as u64 {
        return Err(package_diag(Code::Length, 16));
    }
    if tl & 7 != 0 {
        return Err(package_diag(Code::Length, 32));
    }
    let descriptor = &bytes[96..de];
    let table = &bytes[de..me];
    let contents = &bytes[me..];
    let base = de as u64;
    let identity = descriptor_identity(descriptor, limits.maximum_descriptor_bytes)?;
    if bytes[56..88] != identity {
        return Err(package_diag(Code::Descriptor, 56));
    }
    let mut cursor = 0usize;
    let mut expected = 0u64;
    for i in 0..count {
        let r = raw_record(table, cursor, i, base)?;
        let at = base + cursor as u64;
        let unpadded = cursor + 72 + r.path.len() + r.target.len() + r.features.len();
        if r.content_offset != expected {
            return Err(diag(Code::Offset, i, at + 24));
        }
        if r.content_len == 0 {
            return Err(diag(Code::Length, i, at + 32));
        }
        expected = add(expected, r.content_len, at + 32)?;
        if expected > cl {
            return Err(diag(Code::Length, i, at + 32));
        }
        if let Some(p) = table[unpadded..r.next].iter().position(|b| *b != 0) {
            return Err(diag(Code::Reserved, i, base + unpadded as u64 + p as u64));
        }
        cursor = r.next;
    }
    if cursor != table.len() {
        return Err(package_diag(Code::Length, 32));
    }
    if expected != cl {
        return Err(package_diag(Code::Length, 40));
    }
    cursor = 0;
    for i in 0..count {
        let r = raw_record(table, cursor, i, base)?;
        let at = base + cursor as u64;
        let k = kind(r.kind, i, at)?;
        if !valid_path(r.path) {
            return Err(diag(Code::Path, i, at + 8));
        }
        let tv = match k {
            EffectArtifactKindV1::Source => r.target.is_empty(),
            EffectArtifactKindV1::CoreWasm => r.target == b"wasm32-unknown-unknown",
            EffectArtifactKindV1::TargetNative => valid_native_target(r.target),
        };
        if !tv {
            return Err(diag(Code::Target, i, at + 12));
        }
        if (k == EffectArtifactKindV1::Source && !r.features.is_empty())
            || !valid_features(r.features)
        {
            return Err(diag(Code::Features, i, at + 16));
        }
        cursor = r.next;
    }
    cursor = 0;
    let mut prior: Option<RecordKey<'_>> = None;
    for i in 0..count {
        let r = raw_record(table, cursor, i, base)?;
        let key = (r.kind, r.target, r.features, r.path);
        if prior.is_some_and(|p| p >= key) {
            return Err(diag(Code::Order, i, base + cursor as u64));
        }
        prior = Some(key);
        cursor = r.next;
    }
    cursor = 0;
    for i in 0..count {
        let r = raw_record(table, cursor, i, base)?;
        let s = host(r.content_offset, base + cursor as u64 + 24)?;
        let n = host(r.content_len, base + cursor as u64 + 32)?;
        if Sha256::digest(&contents[s..s + n]).as_slice() != r.hash {
            return Err(diag(Code::Hash, i, base + cursor as u64 + 40));
        }
        cursor = r.next;
    }
    cursor = 0;
    let mut source = false;
    for i in 0..count {
        let r = raw_record(table, cursor, i, base)?;
        source |= kind(r.kind, i, base + cursor as u64)? == EffectArtifactKindV1::Source;
        cursor = r.next;
    }
    if !source {
        return Err(package_diag(
            Code::Unavailable,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        ));
    }
    Ok(VerifiedEffectPackageV1 {
        bytes,
        descriptor,
        table,
        contents,
        count,
    })
}

fn trusted_record(table: &[u8], cursor: usize) -> TrustedRecord<'_> {
    let r = raw_record(table, cursor, 0, 0).expect("verified table");
    TrustedRecord {
        kind: kind(r.kind, 0, 0).expect("verified kind"),
        path: std::str::from_utf8(r.path).expect("verified path"),
        target: std::str::from_utf8(r.target).expect("verified target"),
        features: std::str::from_utf8(r.features).expect("verified features"),
        hash: r.hash,
        content_offset: r.content_offset as usize,
        content_len: r.content_len as usize,
        next: r.next,
    }
}

fn validate_selection_request(request: ArtifactSelectionRequestV1<'_>) -> Result<(), Diagnostic> {
    let target_valid = match request.kind {
        EffectArtifactKindV1::Source => request.target.is_empty(),
        EffectArtifactKindV1::CoreWasm => request.target == "wasm32-unknown-unknown",
        EffectArtifactKindV1::TargetNative => valid_native_target(request.target.as_bytes()),
    };
    if !target_valid {
        return Err(package_diag(
            Code::Target,
            EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
        ));
    }
    let mut prior: Option<&[u8]> = None;
    let mut joined_bytes = 0usize;
    for (index, capability) in request.capabilities.iter().enumerate() {
        let token = capability.as_bytes();
        if !valid_feature_token(token) || prior.is_some_and(|value| value >= token) {
            return Err(package_diag(
                Code::Features,
                EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
            ));
        }
        joined_bytes = joined_bytes
            .checked_add(token.len())
            .and_then(|value| value.checked_add(usize::from(index != 0)))
            .ok_or_else(|| package_diag(Code::Overflow, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET))?;
        if joined_bytes > 255 {
            return Err(package_diag(
                Code::Features,
                EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET,
            ));
        }
        prior = Some(token);
    }
    Ok(())
}

fn feature_subset(features: &str, capabilities: &[&str]) -> bool {
    if features.is_empty() {
        return true;
    }
    let mut capability_index = 0usize;
    for feature in features.split(',') {
        while capability_index < capabilities.len()
            && capabilities[capability_index].as_bytes() < feature.as_bytes()
        {
            capability_index += 1;
        }
        if capability_index == capabilities.len() || capabilities[capability_index] != feature {
            return false;
        }
    }
    true
}

fn feature_count(features: &str) -> usize {
    if features.is_empty() {
        0
    } else {
        features.split(',').count()
    }
}

fn preferred_candidate(
    candidate: VerifiedArtifactV1<'_>,
    selected: VerifiedArtifactV1<'_>,
) -> bool {
    let candidate_count = feature_count(candidate.features());
    let selected_count = feature_count(selected.features());
    candidate_count > selected_count
        || (candidate_count == selected_count
            && (candidate.features() < selected.features()
                || (candidate.features() == selected.features()
                    && candidate.path() < selected.path())))
}

fn artifact_hash_offset(package: VerifiedEffectPackageV1<'_>, artifact_index: u32) -> u64 {
    let mut cursor = 0usize;
    for index in 0..package.count {
        if index == artifact_index {
            return 96 + package.descriptor.len() as u64 + cursor as u64 + 40;
        }
        cursor = trusted_record(package.table, cursor).next;
    }
    EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET
}

pub fn select_effect_package_artifact_v1<'a>(
    package: &VerifiedEffectPackageV1<'a>,
    request: ArtifactSelectionRequestV1<'_>,
) -> Result<VerifiedArtifactV1<'a>, Diagnostic> {
    validate_selection_request(request)?;
    let mut selected = None;
    for candidate in package.artifacts() {
        if candidate.kind() != request.kind
            || candidate.target() != request.target
            || !feature_subset(candidate.features(), request.capabilities)
        {
            continue;
        }
        if selected.is_none_or(|current| preferred_candidate(candidate, current)) {
            selected = Some(candidate);
        }
    }
    let selected = selected
        .ok_or_else(|| package_diag(Code::Unavailable, EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET))?;
    if Sha256::digest(selected.content()).as_slice() != selected.sha2_256() {
        return Err(diag(
            Code::Hash,
            selected.artifact_index(),
            artifact_hash_offset(*package, selected.artifact_index()),
        ));
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn descriptor() -> Vec<u8> {
        let hex = include_str!("../../../fixtures/effect-descriptor/v1/comprehensive-a.wire.hex");
        let compact: Vec<_> = hex
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
    fn encoded() -> Vec<u8> {
        let d = descriptor();
        let aa = [
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::TargetNative,
                path: "bin/linux.so",
                target: "x86_64-unknown-linux-gnu",
                features: "avx2,fma",
                content: b"native",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "src/lib.rs",
                target: "",
                features: "",
                content: b"source",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "wasm/core.wasm",
                target: "wasm32-unknown-unknown",
                features: "simd128",
                content: b"wasm",
            },
        ];
        let p = EffectPackageAuthoringV1 {
            descriptor: &d,
            artifacts: &aa,
        };
        let mut b = vec![
            0;
            effect_package_v1_required_size(&p, EffectPackageLimitsV1::default()).unwrap()
                as usize
        ];
        encode_effect_package_v1(&p, EffectPackageLimitsV1::default(), &mut b).unwrap();
        b
    }

    fn selection_bytes(reverse: bool) -> Vec<u8> {
        let descriptor = descriptor();
        let mut artifacts = [
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "src/a.rs",
                target: "",
                features: "",
                content: b"source-a",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "src/z.rs",
                target: "",
                features: "",
                content: b"source-z",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/base-a.wasm",
                target: "wasm32-unknown-unknown",
                features: "",
                content: b"core-base-a",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/base-b.wasm",
                target: "wasm32-unknown-unknown",
                features: "",
                content: b"core-base-b",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/bulk.wasm",
                target: "wasm32-unknown-unknown",
                features: "bulk-memory,simd128",
                content: b"core-bulk",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/relaxed.wasm",
                target: "wasm32-unknown-unknown",
                features: "relaxed-simd,simd128",
                content: b"core-relaxed",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/simd-a.wasm",
                target: "wasm32-unknown-unknown",
                features: "simd128",
                content: b"core-simd-a",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/simd-b.wasm",
                target: "wasm32-unknown-unknown",
                features: "simd128",
                content: b"core-simd-b",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "core/threads.wasm",
                target: "wasm32-unknown-unknown",
                features: "threads",
                content: b"core-threads",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::TargetNative,
                path: "native/x86-base.so",
                target: "x86_64-unknown-linux-gnu",
                features: "",
                content: b"native-base",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::TargetNative,
                path: "native/x86-avx2.so",
                target: "x86_64-unknown-linux-gnu",
                features: "avx2",
                content: b"native-avx2",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::TargetNative,
                path: "native/x86-fma.so",
                target: "x86_64-unknown-linux-gnu",
                features: "avx2,fma",
                content: b"native-fma",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::TargetNative,
                path: "native/arm.so",
                target: "aarch64-unknown-linux-gnu",
                features: "",
                content: b"native-arm",
            },
        ];
        if reverse {
            artifacts.reverse();
        }
        let package = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &artifacts,
        };
        let mut bytes = vec![
            0;
            effect_package_v1_required_size(&package, EffectPackageLimitsV1::default()).unwrap()
                as usize
        ];
        encode_effect_package_v1(&package, EffectPackageLimitsV1::default(), &mut bytes).unwrap();
        bytes
    }

    fn select_path<'a>(
        package: &VerifiedEffectPackageV1<'a>,
        kind: EffectArtifactKindV1,
        target: &str,
        capabilities: &[&str],
    ) -> Result<&'a str, Diagnostic> {
        select_effect_package_artifact_v1(
            package,
            ArtifactSelectionRequestV1 {
                kind,
                target,
                capabilities,
            },
        )
        .map(VerifiedArtifactV1::path)
    }
    #[test]
    fn round_trip_layout_and_borrows() {
        let b = encoded();
        assert_eq!(&b[..8], MAGIC);
        assert_eq!(u16_at(&b, 10), 96);
        let v = verify_effect_package_v1(&b, EffectPackageLimitsV1::default()).unwrap();
        assert_eq!(v.as_bytes().as_ptr(), b.as_ptr());
        assert_eq!(v.artifact_count(), 3);
        let a: Vec<_> = v.artifacts().collect();
        assert_eq!(a[0].kind(), EffectArtifactKindV1::Source);
        assert_eq!(a[0].content(), b"source");
        assert_eq!(a[1].kind(), EffectArtifactKindV1::CoreWasm);
    }
    #[test]
    fn permutation_and_short_atomic() {
        let d = descriptor();
        let a = EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: "a",
            target: "",
            features: "",
            content: b"a",
        };
        let b = EffectArtifactAuthoringV1 {
            path: "b",
            content: b"b",
            ..a
        };
        let x = [a, b];
        let y = [b, a];
        let px = EffectPackageAuthoringV1 {
            descriptor: &d,
            artifacts: &x,
        };
        let py = EffectPackageAuthoringV1 {
            descriptor: &d,
            artifacts: &y,
        };
        let n = effect_package_v1_required_size(&px, EffectPackageLimitsV1::default()).unwrap()
            as usize;
        let (mut ox, mut oy) = (vec![0; n], vec![0; n]);
        encode_effect_package_v1(&px, EffectPackageLimitsV1::default(), &mut ox).unwrap();
        encode_effect_package_v1(&py, EffectPackageLimitsV1::default(), &mut oy).unwrap();
        assert_eq!(ox, oy);
        let mut short = vec![0xa5; n - 1];
        let old = short.clone();
        assert_eq!(
            encode_effect_package_v1(&px, EffectPackageLimitsV1::default(), &mut short),
            Err(Diagnostic::buffer_too_small(n as u64))
        );
        assert_eq!(short, old);
    }
    #[test]
    fn diagnostic_layout_and_mutations() {
        assert_eq!(std::mem::size_of::<Diagnostic>(), 32);
        assert_eq!(std::mem::align_of::<Diagnostic>(), 8);
        assert_eq!(std::mem::offset_of!(Diagnostic, code), 0);
        assert_eq!(std::mem::offset_of!(Diagnostic, detail), 4);
        assert_eq!(std::mem::offset_of!(Diagnostic, artifact_index), 8);
        assert_eq!(std::mem::offset_of!(Diagnostic, reserved), 12);
        assert_eq!(std::mem::offset_of!(Diagnostic, byte_offset), 16);
        assert_eq!(std::mem::offset_of!(Diagnostic, required_bytes), 24);
        let base = encoded();
        for &(o, c) in &[
            (0, Code::Header),
            (12, Code::Reserved),
            (52, Code::Reserved),
            (88, Code::Reserved),
            (56, Code::Descriptor),
        ] {
            let mut b = base.clone();
            b[o] ^= 1;
            assert_eq!(
                verify_effect_package_v1(&b, EffectPackageLimitsV1::default())
                    .unwrap_err()
                    .code,
                c,
                "{o}"
            );
        }
        let table = 96 + u64_at(&base, 24) as usize;
        for &(r, c) in &[(4, Code::Reserved), (24, Code::Offset), (40, Code::Hash)] {
            let mut b = base.clone();
            b[table + r] ^= 1;
            assert_eq!(
                verify_effect_package_v1(&b, EffectPackageLimitsV1::default())
                    .unwrap_err()
                    .code,
                c,
                "{r}"
            );
        }
    }
    #[test]
    fn authoring_grammar_and_limit() {
        let d = descriptor();
        let a = EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: "src/a.rs",
            target: "",
            features: "",
            content: b"x",
        };
        for (bad, c) in [
            (EffectArtifactAuthoringV1 { path: "../a", ..a }, Code::Path),
            (EffectArtifactAuthoringV1 { target: "x", ..a }, Code::Target),
            (
                EffectArtifactAuthoringV1 { features: "x", ..a },
                Code::Features,
            ),
            (
                EffectArtifactAuthoringV1 { content: b"", ..a },
                Code::Length,
            ),
        ] {
            let aa = [bad];
            let p = EffectPackageAuthoringV1 {
                descriptor: &d,
                artifacts: &aa,
            };
            assert_eq!(
                effect_package_v1_required_size(&p, EffectPackageLimitsV1::default())
                    .unwrap_err()
                    .code,
                c
            );
        }
        let aa = [a];
        let p = EffectPackageAuthoringV1 {
            descriptor: &d,
            artifacts: &aa,
        };
        let n = effect_package_v1_required_size(&p, EffectPackageLimitsV1::default()).unwrap();
        let l = EffectPackageLimitsV1 {
            maximum_package_bytes: n - 1,
            ..EffectPackageLimitsV1::default()
        };
        assert_eq!(
            effect_package_v1_required_size(&p, l).unwrap_err().code,
            Code::Limit
        );
    }

    fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
        bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }
    fn table_start(bytes: &[u8]) -> usize {
        96 + u64_at(bytes, 24) as usize
    }
    fn record_offsets(bytes: &[u8]) -> Vec<usize> {
        let start = table_start(bytes);
        let table_len = u64_at(bytes, 32) as usize;
        let table = &bytes[start..start + table_len];
        let mut offsets = Vec::new();
        let mut cursor = 0;
        for index in 0..u32_at(bytes, 48) {
            offsets.push(start + cursor);
            cursor = raw_record(table, cursor, index, start as u64).unwrap().next;
        }
        offsets
    }
    fn assert_code(bytes: &[u8], code: Code) -> Diagnostic {
        let error = verify_effect_package_v1(bytes, EffectPackageLimitsV1::default()).unwrap_err();
        assert_eq!(error.code, code);
        error
    }

    #[test]
    fn fixed_header_length_truncation_trailing_and_overflow_diagnostics() {
        let base = encoded();
        for &(offset, code) in &[(8, Code::Header), (10, Code::Header)] {
            let mut bytes = base.clone();
            bytes[offset] ^= 1;
            assert_eq!(assert_code(&bytes, code).byte_offset, offset as u64);
        }
        assert_eq!(assert_code(&base[..95], Code::Header).byte_offset, 0);
        let mut truncated = base.clone();
        truncated.pop();
        assert_eq!(assert_code(&truncated, Code::Length).byte_offset, 16);
        let mut trailing = base.clone();
        trailing.push(0);
        assert_eq!(assert_code(&trailing, Code::Length).byte_offset, 16);
        let mut overflow = vec![0; 96];
        overflow[..8].copy_from_slice(MAGIC);
        overflow[8..10].copy_from_slice(&1u16.to_le_bytes());
        overflow[10..12].copy_from_slice(&96u16.to_le_bytes());
        put_u64(&mut overflow, 16, u64::MAX);
        put_u64(&mut overflow, 32, u64::MAX);
        let limits = EffectPackageLimitsV1 {
            maximum_package_bytes: u64::MAX,
            ..EffectPackageLimitsV1::default()
        };
        assert_eq!(
            verify_effect_package_v1(&overflow, limits).unwrap_err(),
            package_diag(Code::Overflow, 32)
        );
    }

    #[test]
    fn every_table_grammar_padding_order_hash_and_source_class_rejects() {
        let base = encoded();
        let records = record_offsets(&base);
        let source = records[0];
        let core = records[1];
        let native = records[2];

        let mut kind_bad = base.clone();
        put_u32(&mut kind_bad, source, 99);
        assert_eq!(assert_code(&kind_bad, Code::Enum).artifact_index, 0);

        let mut path_bad = base.clone();
        path_bad[source + 72] = b'S';
        assert_eq!(assert_code(&path_bad, Code::Path).artifact_index, 0);

        let mut target_bad = base.clone();
        let core_path_len = u32_at(&base, core + 8) as usize;
        target_bad[core + 72 + core_path_len] = b'W';
        assert_eq!(assert_code(&target_bad, Code::Target).artifact_index, 1);

        let mut feature_bad = base.clone();
        let native_feature =
            native + 72 + u32_at(&base, native + 8) as usize + u32_at(&base, native + 12) as usize;
        feature_bad[native_feature] = b'A';
        assert_eq!(assert_code(&feature_bad, Code::Features).artifact_index, 2);

        let mut padding_bad = base.clone();
        let unpadded = source
            + 72
            + u32_at(&base, source + 8) as usize
            + u32_at(&base, source + 12) as usize
            + u32_at(&base, source + 16) as usize;
        assert!(records[1] > unpadded);
        padding_bad[unpadded] = 1;
        assert_eq!(assert_code(&padding_bad, Code::Reserved).artifact_index, 0);

        let descriptor = descriptor();
        let ordered_artifacts = [
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "a",
                target: "",
                features: "",
                content: b"a",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "b",
                target: "",
                features: "",
                content: b"b",
            },
        ];
        let ordered_package = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &ordered_artifacts,
        };
        let mut order_bad = vec![
            0;
            effect_package_v1_required_size(&ordered_package, EffectPackageLimitsV1::default())
                .unwrap() as usize
        ];
        encode_effect_package_v1(
            &ordered_package,
            EffectPackageLimitsV1::default(),
            &mut order_bad,
        )
        .unwrap();
        let order_first = record_offsets(&order_bad)[0];
        order_bad[order_first + 72] = b'z';
        assert_eq!(assert_code(&order_bad, Code::Order).artifact_index, 1);

        let mut hash_bad = base.clone();
        let content_start = table_start(&base) + u64_at(&base, 32) as usize;
        hash_bad[content_start] ^= 1;
        assert_eq!(assert_code(&hash_bad, Code::Hash).artifact_index, 0);

        let descriptor_end = table_start(&base);
        let mut no_source = base[..descriptor_end].to_vec();
        put_u64(&mut no_source, 16, descriptor_end as u64);
        put_u64(&mut no_source, 32, 0);
        put_u64(&mut no_source, 40, 0);
        put_u32(&mut no_source, 48, 0);
        assert_eq!(
            assert_code(&no_source, Code::Unavailable).artifact_index,
            u32::MAX
        );
    }

    #[test]
    fn structural_counts_offsets_lengths_and_padding_are_count_driven() {
        let base = encoded();
        let first = record_offsets(&base)[0];
        for field in [8usize, 12, 16] {
            let mut bytes = base.clone();
            put_u32(&mut bytes, first + field, u32::MAX);
            assert_eq!(
                assert_code(&bytes, Code::Length).byte_offset,
                (first + field) as u64
            );
        }
        for (relative, code) in [(24, Code::Offset), (32, Code::Length)] {
            let mut bytes = base.clone();
            if relative == 32 {
                put_u64(&mut bytes, first + relative, 0);
            } else {
                put_u64(&mut bytes, first + relative, 1);
            }
            assert_eq!(
                assert_code(&bytes, code).byte_offset,
                (first + relative) as u64
            );
        }
        let mut count_too_large = base.clone();
        put_u32(&mut count_too_large, 48, u32_at(&base, 48) + 1);
        assert_eq!(
            assert_code(&count_too_large, Code::Length).byte_offset,
            (table_start(&base) + u64_at(&base, 32) as usize) as u64
        );
        let mut table_unconsumed = base.clone();
        put_u32(&mut table_unconsumed, 48, u32_at(&base, 48) - 1);
        assert_eq!(assert_code(&table_unconsumed, Code::Length).byte_offset, 32);
        let mut bad_table_multiple = base.clone();
        put_u64(&mut bad_table_multiple, 32, u64_at(&base, 32) - 1);
        put_u64(&mut bad_table_multiple, 16, u64_at(&base, 16) - 1);
        bad_table_multiple.remove(table_start(&base) + u64_at(&base, 32) as usize - 1);
        assert_eq!(
            assert_code(&bad_table_multiple, Code::Length).byte_offset,
            32
        );
    }

    #[test]
    fn exact_and_one_below_limits_cover_all_five_caps() {
        let bytes = encoded();
        let verified = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
        let descriptor_len = verified.descriptor().len() as u64;
        let manifest = 96 + descriptor_len + u64_at(&bytes, 32);
        let maximum_artifact = verified
            .artifacts()
            .map(|a| a.content().len() as u64)
            .max()
            .unwrap();
        let exact = EffectPackageLimitsV1 {
            maximum_descriptor_bytes: descriptor_len,
            maximum_manifest_bytes: manifest,
            maximum_package_bytes: bytes.len() as u64,
            maximum_artifacts: verified.artifact_count(),
            maximum_artifact_bytes: maximum_artifact,
        };
        verify_effect_package_v1(&bytes, exact).unwrap();
        let mut descriptor_below = exact;
        descriptor_below.maximum_descriptor_bytes = descriptor_len - 1;
        let mut manifest_below = exact;
        manifest_below.maximum_manifest_bytes = manifest - 1;
        let mut package_below = exact;
        package_below.maximum_package_bytes = bytes.len() as u64 - 1;
        let mut count_below = exact;
        count_below.maximum_artifacts = verified.artifact_count() - 1;
        let mut artifact_below = exact;
        artifact_below.maximum_artifact_bytes = maximum_artifact - 1;
        for below in [
            descriptor_below,
            manifest_below,
            package_below,
            count_below,
            artifact_below,
        ] {
            assert_eq!(
                verify_effect_package_v1(&bytes, below).unwrap_err().code,
                Code::Limit
            );
        }

        let descriptor = descriptor();
        let artifacts = [
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::Source,
                path: "a",
                target: "",
                features: "",
                content: b"source",
            },
            EffectArtifactAuthoringV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                path: "b",
                target: "wasm32-unknown-unknown",
                features: "simd128",
                content: b"wasm",
            },
        ];
        let package = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &artifacts,
        };
        let required =
            effect_package_v1_required_size(&package, EffectPackageLimitsV1::default()).unwrap();
        let table = artifacts
            .iter()
            .map(|artifact| {
                (72 + artifact.path.len() + artifact.target.len() + artifact.features.len() + 7)
                    & !7
            })
            .sum::<usize>() as u64;
        let authoring_exact = EffectPackageLimitsV1 {
            maximum_descriptor_bytes: descriptor.len() as u64,
            maximum_manifest_bytes: 96 + descriptor.len() as u64 + table,
            maximum_package_bytes: required,
            maximum_artifacts: artifacts.len() as u32,
            maximum_artifact_bytes: 6,
        };
        assert_eq!(
            effect_package_v1_required_size(&package, authoring_exact),
            Ok(required)
        );
        let mut output = vec![0xa5; required as usize + 8];
        assert_eq!(
            encode_effect_package_v1(&package, authoring_exact, &mut output),
            Ok(required as usize)
        );
        assert!(output[required as usize..].iter().all(|byte| *byte == 0xa5));
        let mut below_limits = [authoring_exact; 5];
        below_limits[0].maximum_descriptor_bytes -= 1;
        below_limits[1].maximum_manifest_bytes -= 1;
        below_limits[2].maximum_package_bytes -= 1;
        below_limits[3].maximum_artifacts -= 1;
        below_limits[4].maximum_artifact_bytes -= 1;
        for below in below_limits {
            assert_eq!(
                effect_package_v1_required_size(&package, below)
                    .unwrap_err()
                    .code,
                Code::Limit
            );
            let mut canary = vec![0xa5; required as usize + 8];
            assert_eq!(
                encode_effect_package_v1(&package, below, &mut canary)
                    .unwrap_err()
                    .code,
                Code::Limit
            );
            assert!(canary.iter().all(|byte| *byte == 0xa5));
        }
    }

    #[test]
    fn descriptor_mapping_and_phase_priority_are_exact() {
        let mut invalid_descriptor = encoded();
        invalid_descriptor[96] ^= 1;
        let error = assert_code(&invalid_descriptor, Code::Descriptor);
        assert_eq!(error.detail, 4);
        assert_eq!(error.byte_offset, 96);

        let mut reserved_and_descriptor = invalid_descriptor;
        let first = record_offsets(&reserved_and_descriptor)[0];
        reserved_and_descriptor[first + 4] = 1;
        let error = assert_code(&reserved_and_descriptor, Code::Reserved);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 4) as u64);

        let mut limit_and_header = encoded();
        limit_and_header[0] ^= 1;
        let first = record_offsets(&limit_and_header)[0];
        put_u64(
            &mut limit_and_header,
            first + 32,
            EffectPackageLimitsV1::default().maximum_artifact_bytes + 1,
        );
        assert_eq!(
            assert_code(&limit_and_header, Code::Limit).artifact_index,
            0
        );

        let mut reserved_and_length = encoded();
        let first = record_offsets(&reserved_and_length)[0];
        reserved_and_length[first + 4] = 1;
        let wrong_total = u64_at(&reserved_and_length, 16) - 1;
        put_u64(&mut reserved_and_length, 16, wrong_total);
        assert_eq!(
            assert_code(&reserved_and_length, Code::Reserved).artifact_index,
            0
        );

        let mut offset_and_padding = encoded();
        let records = record_offsets(&offset_and_padding);
        let first = records[0];
        let padding = first
            + 72
            + u32_at(&offset_and_padding, first + 8) as usize
            + u32_at(&offset_and_padding, first + 12) as usize
            + u32_at(&offset_and_padding, first + 16) as usize;
        offset_and_padding[padding] = 1;
        put_u64(&mut offset_and_padding, first + 24, 1);
        assert_eq!(
            assert_code(&offset_and_padding, Code::Offset).artifact_index,
            0
        );

        let base = encoded();
        let first = record_offsets(&base)[0];
        let mut malformed_path_and_limit = base.clone();
        put_u32(&mut malformed_path_and_limit, first + 8, u32::MAX);
        put_u64(
            &mut malformed_path_and_limit,
            first + 32,
            EffectPackageLimitsV1::default().maximum_artifact_bytes + 1,
        );
        let error = assert_code(&malformed_path_and_limit, Code::Limit);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 32) as u64);

        let mut malformed_path_and_reserved = base.clone();
        put_u32(&mut malformed_path_and_reserved, first + 8, u32::MAX);
        malformed_path_and_reserved[first + 4] = 1;
        let error = assert_code(&malformed_path_and_reserved, Code::Reserved);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 4) as u64);

        let mut malformed_path = base;
        put_u32(&mut malformed_path, first + 8, u32::MAX);
        let error = assert_code(&malformed_path, Code::Length);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 8) as u64);

        let mut truncated_after_first_prefix = encoded();
        let first = record_offsets(&truncated_after_first_prefix)[0];
        put_u64(
            &mut truncated_after_first_prefix,
            first + 32,
            EffectPackageLimitsV1::default().maximum_artifact_bytes + 1,
        );
        truncated_after_first_prefix.truncate(first + 40);
        let error = assert_code(&truncated_after_first_prefix, Code::Limit);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 32) as u64);

        let mut truncated_after_first_reserved = encoded();
        let first = record_offsets(&truncated_after_first_reserved)[0];
        truncated_after_first_reserved[first + 20] = 1;
        truncated_after_first_reserved.truncate(first + 24);
        let error = assert_code(&truncated_after_first_reserved, Code::Reserved);
        assert_eq!(error.artifact_index, 0);
        assert_eq!(error.byte_offset, (first + 20) as u64);
    }

    #[test]
    fn authoring_duplicate_source_invariant_and_native_feature_grammars() {
        let descriptor = descriptor();
        let source = EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: "a",
            target: "",
            features: "",
            content: b"x",
        };
        let duplicates = [
            source,
            EffectArtifactAuthoringV1 {
                content: b"y",
                ..source
            },
        ];
        let package = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &duplicates,
        };
        let error = effect_package_v1_required_size(&package, EffectPackageLimitsV1::default())
            .unwrap_err();
        assert_eq!((error.code, error.artifact_index), (Code::Order, 1));

        let no_source = [EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::CoreWasm,
            path: "a",
            target: "wasm32-unknown-unknown",
            features: "",
            content: b"x",
        }];
        let package = EffectPackageAuthoringV1 {
            descriptor: &descriptor,
            artifacts: &no_source,
        };
        assert_eq!(
            effect_package_v1_required_size(&package, EffectPackageLimitsV1::default())
                .unwrap_err()
                .code,
            Code::Unavailable
        );

        for target in [
            "wasm32-unknown-linux-gnu",
            "x86_64-linux",
            "x86_64--linux-gnu",
            "X86_64-unknown-linux-gnu",
        ] {
            let artifacts = [
                source,
                EffectArtifactAuthoringV1 {
                    kind: EffectArtifactKindV1::TargetNative,
                    path: "b",
                    target,
                    features: "",
                    content: b"y",
                },
            ];
            let package = EffectPackageAuthoringV1 {
                descriptor: &descriptor,
                artifacts: &artifacts,
            };
            assert_eq!(
                effect_package_v1_required_size(&package, EffectPackageLimitsV1::default())
                    .unwrap_err()
                    .code,
                Code::Target
            );
        }
        for features in ["a,a", "b,a", "a,,b", "1a", "a_b", "a+", "A"] {
            let artifacts = [
                source,
                EffectArtifactAuthoringV1 {
                    kind: EffectArtifactKindV1::CoreWasm,
                    path: "b",
                    target: "wasm32-unknown-unknown",
                    features,
                    content: b"y",
                },
            ];
            let package = EffectPackageAuthoringV1 {
                descriptor: &descriptor,
                artifacts: &artifacts,
            };
            assert_eq!(
                effect_package_v1_required_size(&package, EffectPackageLimitsV1::default())
                    .unwrap_err()
                    .code,
                Code::Features,
                "{features}"
            );
        }
    }

    #[test]
    fn selection_baselines_supersets_and_total_ties_are_exact() {
        let bytes = selection_bytes(false);
        let package = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
        assert_eq!(
            select_path(&package, EffectArtifactKindV1::Source, "", &[]),
            Ok("src/a.rs")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &[]
            ),
            Ok("core/base-a.wasm")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &["simd128"]
            ),
            Ok("core/simd-a.wasm")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &["simd128", "threads"]
            ),
            Ok("core/simd-a.wasm")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &["bulk-memory", "relaxed-simd", "simd128"],
            ),
            Ok("core/bulk.wasm")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::TargetNative,
                "x86_64-unknown-linux-gnu",
                &["avx2", "fma"],
            ),
            Ok("native/x86-fma.so")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::TargetNative,
                "x86_64-unknown-linux-gnu",
                &["fma"],
            ),
            Ok("native/x86-base.so")
        );
    }

    #[test]
    fn selection_has_no_feature_kind_or_target_implication() {
        let bytes = selection_bytes(false);
        let package = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &["relaxed-simd"],
            ),
            Ok("core/base-a.wasm")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::TargetNative,
                "riscv64-unknown-linux-gnu",
                &[],
            )
            .unwrap_err()
            .code,
            Code::Unavailable
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::TargetNative,
                "aarch64-unknown-linux-gnu",
                &["avx2", "fma"],
            ),
            Ok("native/arm.so")
        );
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "x86_64-unknown-linux-gnu",
                &[],
            )
            .unwrap_err()
            .code,
            Code::Target
        );
    }

    #[test]
    fn selection_request_capabilities_are_canonical_and_strict() {
        let bytes = selection_bytes(false);
        let package = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
        for capabilities in [
            &["simd128", "avx2"][..],
            &["simd128", "simd128"][..],
            &[""][..],
            &["1simd"][..],
            &["simd_128"][..],
            &["simd+128"][..],
            &["SIMD128"][..],
        ] {
            let error = select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                capabilities,
            )
            .unwrap_err();
            assert_eq!(error.code, Code::Features, "{capabilities:?}");
            assert_eq!(error.artifact_index, u32::MAX);
            assert_eq!(error.byte_offset, u64::MAX);
        }
        let too_long = "a23456789012345678901234567890123";
        assert_eq!(too_long.len(), 33);
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &[too_long],
            )
            .unwrap_err()
            .code,
            Code::Features
        );

        let capabilities: Vec<String> = (b'a'..=b'h')
            .map(|first| format!("{}{}", char::from(first), "0".repeat(30)))
            .collect();
        let references: Vec<&str> = capabilities.iter().map(String::as_str).collect();
        assert_eq!(references.join(",").len(), 255);
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &references,
            )
            .unwrap(),
            "core/base-a.wasm"
        );
        let mut too_many_bytes = capabilities;
        too_many_bytes[7].push('0');
        let references: Vec<&str> = too_many_bytes.iter().map(String::as_str).collect();
        assert_eq!(references.join(",").len(), 256);
        assert_eq!(
            select_path(
                &package,
                EffectArtifactKindV1::CoreWasm,
                "wasm32-unknown-unknown",
                &references,
            )
            .unwrap_err()
            .code,
            Code::Features
        );
    }

    #[test]
    fn selection_is_permutation_stable_and_returns_borrowed_content() {
        let forward = selection_bytes(false);
        let reverse = selection_bytes(true);
        assert_eq!(forward, reverse);
        let package = verify_effect_package_v1(&forward, EffectPackageLimitsV1::default()).unwrap();
        let selected = select_effect_package_artifact_v1(
            &package,
            ArtifactSelectionRequestV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                target: "wasm32-unknown-unknown",
                capabilities: &["bulk-memory", "relaxed-simd", "simd128"],
            },
        )
        .unwrap();
        assert_eq!(selected.path(), "core/bulk.wasm");
        assert_eq!(selected.content(), b"core-bulk");
        let package_start = forward.as_ptr() as usize;
        let content_start = selected.content().as_ptr() as usize;
        assert!(content_start >= package_start);
        assert!(content_start + selected.content().len() <= package_start + forward.len());
        assert_eq!(
            selected.sha2_256(),
            Sha256::digest(selected.content()).as_slice()
        );
    }

    #[test]
    fn selection_rehash_detects_post_verification_content_tamper() {
        let mut bytes = selection_bytes(false);
        let request = ArtifactSelectionRequestV1 {
            kind: EffectArtifactKindV1::CoreWasm,
            target: "wasm32-unknown-unknown",
            capabilities: &["simd128"],
        };
        let (selected_index, content_offset) = {
            let package =
                verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
            let selected = select_effect_package_artifact_v1(&package, request).unwrap();
            (
                selected.artifact_index(),
                selected.content().as_ptr() as usize - bytes.as_ptr() as usize,
            )
        };
        bytes[content_offset] ^= 1;
        let descriptor_end = table_start(&bytes);
        let manifest = descriptor_end + u64_at(&bytes, 32) as usize;
        let forged = VerifiedEffectPackageV1 {
            bytes: &bytes,
            descriptor: &bytes[96..descriptor_end],
            table: &bytes[descriptor_end..manifest],
            contents: &bytes[manifest..],
            count: u32_at(&bytes, 48),
        };
        let error = select_effect_package_artifact_v1(&forged, request).unwrap_err();
        assert_eq!(
            (error.code, error.artifact_index),
            (Code::Hash, selected_index)
        );
        assert_eq!(
            error.byte_offset,
            (record_offsets(&bytes)[selected_index as usize] + 40) as u64
        );
    }
}
