//! Bounded diagnostic storage, and the failure vocabularies the boundary reports.

use super::*;

pub(crate) struct FixedBytes {
    pub(crate) bytes: Box<[u8]>,
    pub(crate) len: usize,
}

impl FixedBytes {
    pub(crate) fn try_new(capacity: u64) -> Result<Self, CompileFailure> {
        let capacity = usize::try_from(capacity).map_err(|_| failure("capi.resource.platform"))?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| failure("capi.resource.allocation"))?;
        bytes.resize(capacity, 0);
        Ok(Self {
            bytes: bytes.into_boxed_slice(),
            len: 0,
        })
    }

    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }

    pub(crate) fn set(&mut self, value: &[u8]) {
        let value = core::str::from_utf8(value).unwrap_or("capi.internal.utf8");
        self.len = value.len().min(self.bytes.len());
        while !value.is_char_boundary(self.len) {
            self.len -= 1;
        }
        self.bytes[..self.len].copy_from_slice(&value.as_bytes()[..self.len]);
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// Fixed render diagnostics for a plan handle.
///
/// The render thread stores one of these codes into `Plan::last_error`; `miso_engine_v1_last_error`
/// loads it from any thread and returns the matching `'static` text. A plan diagnostic is therefore
/// a single relaxed atomic word rather than shared mutable string storage: the render thread never
/// takes a borrow that a concurrent query could invalidate.
pub(crate) mod plan_error {
    /// The most recent render call succeeded.
    pub(crate) const NONE: u32 = 0;
    /// `output.samples` is not aligned for `f32`.
    pub(crate) const OUTPUT_UNALIGNED: u32 = 1;
    /// The declared sample capacity is not addressable as one slice on this platform.
    pub(crate) const OUTPUT_PLATFORM: u32 = 2;
    /// The two-plane layout does not fit the declared capacity, or the stride is short.
    pub(crate) const OUTPUT_LAYOUT: u32 = 3;
    /// The frame count is not the prepared quantum.
    pub(crate) const OUTPUT_SHAPE: u32 = 4;
    /// The requested absolute sample is not the one the plan is waiting for.
    pub(crate) const TIME_DISCONTINUITY: u32 = 5;
    /// Advancing the absolute sample clock would overflow `u64`.
    pub(crate) const TIME_OVERFLOW: u32 = 6;
    /// The prepared plan itself rejected the render call.
    pub(crate) const PLAN_REJECTED: u32 = 7;
    /// The canonical floating-point environment did not take on this render thread (issue #146).
    pub(crate) const FP_ENVIRONMENT: u32 = 8;

    /// Returns the frozen diagnostic text for `code`.
    ///
    /// One code per rule, so a rejected render names the single check it failed. Before W4-5 five
    /// distinct rules were folded into one `render.contract.rejected` string.
    pub(crate) const fn text(code: u32) -> &'static [u8] {
        match code {
            NONE => b"",
            OUTPUT_UNALIGNED => b"render.output.unaligned",
            OUTPUT_PLATFORM => b"render.output.platform",
            OUTPUT_LAYOUT => b"render.output.layout",
            OUTPUT_SHAPE => b"render.output.shape",
            TIME_DISCONTINUITY => b"render.time.discontinuity",
            TIME_OVERFLOW => b"render.time.overflow",
            PLAN_REJECTED => b"render.plan.rejected",
            FP_ENVIRONMENT => b"render.fp_environment.invalid",
            _ => b"render.internal",
        }
    }
}

#[derive(Debug)]
pub(crate) struct CompileFailure {
    pub(crate) diagnostics: Vec<u8>,
}

pub(crate) fn failure(code: &str) -> CompileFailure {
    CompileFailure {
        diagnostics: format!("{code}\t$\n").into_bytes(),
    }
}

pub(crate) fn session_diagnostics(diagnostics: &DiagnosticSet) -> CompileFailure {
    let mut bytes = Vec::new();
    for diagnostic in diagnostics.diagnostics() {
        bytes.extend_from_slice(diagnostic.code.as_str().as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(diagnostic.path.to_string().as_bytes());
        bytes.push(b'\n');
    }
    CompileFailure { diagnostics: bytes }
}
