//! Strict deterministic fixture-manifest parsing.

/// A validated manifest entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestEntry {
    /// Canonical fixture CRC-32C from its header.
    pub crc32c: u32,
    /// Exact fixture byte length.
    pub length: usize,
    /// Relative slash-only path under the fixture root.
    pub path: String,
}

/// Manifest parse errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestError {
    /// Bytes were not UTF-8 or used CRLF.
    InvalidText,
    /// Header is absent or wrong.
    InvalidHeader,
    /// A row has malformed fields.
    InvalidRow,
    /// Rows are not lexicographically sorted/unique.
    UnsortedOrDuplicate,
    /// Path is unsafe.
    UnsafePath,
}

/// Parses the v1 tab-separated manifest with strict layout.
pub fn parse_manifest(bytes: &[u8]) -> Result<Vec<ManifestEntry>, ManifestError> {
    let text = core::str::from_utf8(bytes).map_err(|_| ManifestError::InvalidText)?;
    if text.contains('\r') || !text.ends_with('\n') {
        return Err(ManifestError::InvalidText);
    }
    let mut lines = text.lines();
    if lines.next() != Some("miso-engine-fixture-manifest-v1") {
        return Err(ManifestError::InvalidHeader);
    }
    let mut entries = Vec::new();
    let mut last = "";
    for line in lines {
        if line.is_empty() {
            return Err(ManifestError::InvalidRow);
        }
        let mut fields = line.split('\t');
        let (Some(crc), Some(length_text), Some(path), None) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            return Err(ManifestError::InvalidRow);
        };
        if crc.len() != 8
            || !crc
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(ManifestError::InvalidRow);
        }
        let crc8 = u32::from_str_radix(crc, 16).map_err(|_| ManifestError::InvalidRow)?;
        let length = length_text
            .parse::<usize>()
            .map_err(|_| ManifestError::InvalidRow)?;
        if length == 0 || length.to_string() != length_text {
            return Err(ManifestError::InvalidRow);
        }
        if path.is_empty()
            || path.starts_with('/')
            || path.contains('\\')
            || path.contains(':')
            || !path.starts_with("v1/")
            || !path.ends_with(".mepcm")
            || !path.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'-' | b'_')
            })
            || path
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err(ManifestError::UnsafePath);
        }
        if path <= last {
            return Err(ManifestError::UnsortedOrDuplicate);
        }
        last = path;
        entries.push(ManifestEntry {
            crc32c: crc8,
            length,
            path: path.to_owned(),
        });
    }
    if entries.is_empty() {
        return Err(ManifestError::InvalidRow);
    }
    Ok(entries)
}
