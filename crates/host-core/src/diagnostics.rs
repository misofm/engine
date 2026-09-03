//! The one place a host preparation failure becomes bytes.
//!
//! Every embedding (C ABI, browser Wasm, native, mobile) reports preparation failures as the same
//! `code\tpath\n` line format, so the same session rejected by two hosts produces the same text.
//! Before this module each host carried its own four copies of the same loop.

use core::fmt::Display;

/// Maximum number of diagnostic lines one preparation refusal retains.
///
/// A session may contain many independently invalid declarations. Retaining every diagnostic made
/// refusal time superlinear for dense automation documents and let an invalid document consume
/// memory after the parser had already decided to refuse it. Sixty-four lines preserve useful
/// field-local context while putting a fixed ceiling on accumulation for every host.
pub const MAXIMUM_PREPARE_DIAGNOSTIC_LINES: usize = 64;

/// Which stage of host preparation rejected the session.
///
/// Hosts map this onto their own result codes; the facade never defines result codes because the
/// C ABI and the browser ABI number them differently (issue 022 vs issue 024).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrepareRejection {
    /// The JSON failed to parse, or the session compiler rejected the model.
    Session,
    /// The session shape (sample rate, quantum, source ring or source channel count) is outside
    /// what this host declared it can drive.
    Shape,
    /// The native effect compiler rejected the session's effects.
    Effect,
    /// The builtin compiler rejected the session's builtins or metering.
    Builtin,
    /// The graph compiler, source binding or runtime binding rejected the session.
    Graph,
    /// A declared resource cap would be exceeded by the prepared session.
    Resource,
    /// A count, byte total or pointer-width conversion overflowed on this platform.
    Platform,
}

/// A bounded host-preparation failure: one classification plus the diagnostic text.
///
/// The bytes are `code\tpath\n` lines. A failure that has no session path (a cap, a shape rule)
/// uses the path `$`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrepareDiagnostics {
    kind: PrepareRejection,
    bytes: Vec<u8>,
}

impl PrepareDiagnostics {
    /// Build a failure from already-encoded `code\tpath\n` lines.
    #[must_use]
    pub const fn new(kind: PrepareRejection, bytes: Vec<u8>) -> Self {
        Self { kind, bytes }
    }

    /// Build the single-line failure `code\t$\n` for a rule that names no session path.
    #[must_use]
    pub fn fixed(kind: PrepareRejection, code: &str) -> Self {
        Self {
            kind,
            bytes: fixed_diagnostic_line(code),
        }
    }

    /// Which stage rejected the session.
    #[must_use]
    pub const fn kind(&self) -> PrepareRejection {
        self.kind
    }

    /// The diagnostic text, as `code\tpath\n` lines.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Take ownership of the diagnostic text.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Encode `(code, path)` pairs as `code\tpath\n` lines.
///
/// This is the only encoder in the workspace: the session, effect, builtin and graph compilers all
/// report `(code, path)` pairs whose types differ, so the encoder is generic over both halves.
pub fn diagnostic_lines<C: AsRef<str>, P: Display>(
    items: impl IntoIterator<Item = (C, P)>,
) -> Vec<u8> {
    use core::fmt::Write as _;

    let mut bytes = Vec::new();
    let mut path = String::new();
    for (code, item) in items.into_iter().take(MAXIMUM_PREPARE_DIAGNOSTIC_LINES) {
        bytes.extend_from_slice(code.as_ref().as_bytes());
        bytes.push(b'\t');
        path.clear();
        // Writing into a `String` cannot fail; a `Display` implementation that panics is the
        // caller's bug, not a formatting error.
        let _ = write!(&mut path, "{item}");
        bytes.extend_from_slice(path.as_bytes());
        bytes.push(b'\n');
    }
    bytes
}

/// Encode one fixed-code failure as the single line `code\t$\n`.
#[must_use]
pub fn fixed_diagnostic_line(code: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(code.len() + 3);
    bytes.extend_from_slice(code.as_bytes());
    bytes.extend_from_slice(b"\t$\n");
    bytes
}
