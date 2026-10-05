//! Bounded diagnostic storage, and capi's bytes for the failures the control plane reports.

use super::*;

pub(crate) struct FixedBytes {
    pub(crate) bytes: Box<[u8]>,
    pub(crate) len: usize,
}

impl FixedBytes {
    pub(crate) fn try_new(capacity: u64) -> Result<Self, CompileFailure> {
        let capacity = usize::try_from(capacity)
            .map_err(|_| CompileFailure::Resource(ResourceFault::Platform))?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| CompileFailure::Resource(ResourceFault::Allocation))?;
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

/// The exact bytes capi reports for a control-plane compile failure (#1309 D6).
///
/// A resource fault is one whole-session line with capi's own code; diagnostics are the session,
/// preparation and admission lines as the control plane built them.
pub(crate) fn failure_bytes(failure: CompileFailure) -> Vec<u8> {
    match failure {
        CompileFailure::Resource(fault) => {
            let code = match fault {
                ResourceFault::Arithmetic => "capi.resource.arithmetic",
                ResourceFault::Platform => "capi.resource.platform",
                ResourceFault::Limit => "capi.resource.limit",
                ResourceFault::Allocation => "capi.resource.allocation",
                ResourceFault::ProtocolQueue => "capi.protocol.queue",
                ResourceFault::PlanExchange => "capi.plan.exchange",
            };
            format!("{code}\t$\n").into_bytes()
        }
        CompileFailure::Diagnostics(bytes) => bytes,
    }
}

/// A refused compile, as `miso_engine_v1_compile_session` reports it.
#[derive(Debug)]
pub(crate) struct CompileRejection {
    pub(crate) diagnostics: Vec<u8>,
}

impl From<CompileFailure> for CompileRejection {
    fn from(failure: CompileFailure) -> Self {
        Self {
            diagnostics: failure_bytes(failure),
        }
    }
}

/// capi's render diagnostic for a typed render refusal: one code per rule.
pub(crate) fn render_error_code(error: RenderError) -> u32 {
    match error {
        RenderError::OutputShape => plan_error::OUTPUT_SHAPE,
        RenderError::TimeDiscontinuity { .. } => plan_error::TIME_DISCONTINUITY,
        RenderError::TimeOverflow => plan_error::TIME_OVERFLOW,
        _ => plan_error::PLAN_REJECTED,
    }
}

/// The result code and diagnostic text a source failure reports across the C boundary.
pub(crate) trait SourceFailureReport {
    /// The result code and diagnostic text this failure reports across the C boundary.
    fn report(self) -> (u32, &'static [u8]);
}

impl SourceFailureReport for SourceFailure {
    fn report(self) -> (u32, &'static [u8]) {
        match self {
            Self::Internal => (RESULT_INTERNAL, b"capi.source.epoch"),
            Self::Control(error) => {
                let code = if error.is_backpressure() {
                    RESULT_BACKPRESSURE
                } else if error.is_internal() {
                    RESULT_INTERNAL
                } else {
                    RESULT_INVALID_ARGUMENT
                };
                (code, error.diagnostic().as_bytes())
            }
        }
    }
}

#[cfg(test)]
mod failure_bytes_tests {
    //! #1309 D6 follow-up: every failure kind's exact C ABI diagnostic bytes.
    //!
    //! These bytes are C ABI output (`miso_engine_v1_compile_session`'s diagnostics and a source
    //! call's last error), so AGENTS.md permits pinning them exactly. Each pin is the literal the
    //! capi runtime emitted before #1309 moved the control plane out (`d6fa539a1`).

    use super::*;

    /// The pinned line for each fault. The match is exhaustive, so a new fault cannot ship unpinned.
    const fn pinned(fault: ResourceFault) -> &'static [u8] {
        match fault {
            ResourceFault::Arithmetic => b"capi.resource.arithmetic\t$\n",
            ResourceFault::Platform => b"capi.resource.platform\t$\n",
            ResourceFault::Limit => b"capi.resource.limit\t$\n",
            ResourceFault::Allocation => b"capi.resource.allocation\t$\n",
            ResourceFault::ProtocolQueue => b"capi.protocol.queue\t$\n",
            ResourceFault::PlanExchange => b"capi.plan.exchange\t$\n",
        }
    }

    #[test]
    fn every_failure_kind_reports_its_frozen_c_abi_bytes() {
        for fault in [
            ResourceFault::Arithmetic,
            ResourceFault::Platform,
            ResourceFault::Limit,
            ResourceFault::Allocation,
            ResourceFault::ProtocolQueue,
            ResourceFault::PlanExchange,
        ] {
            assert_eq!(
                failure_bytes(CompileFailure::Resource(fault)),
                pinned(fault),
                "{fault:?}"
            );
            assert_eq!(
                CompileRejection::from(CompileFailure::Resource(fault)).diagnostics,
                pinned(fault),
                "{fault:?} through CompileRejection"
            );
        }

        let lines = b"session.json.invalid\t$.tracks\ngraph.cycle\t$.routes[0]\n".to_vec();
        assert_eq!(
            failure_bytes(CompileFailure::Diagnostics(lines.clone())),
            lines
        );

        assert_eq!(
            SourceFailure::Internal.report(),
            (RESULT_INTERNAL, &b"capi.source.epoch"[..])
        );
    }
}
