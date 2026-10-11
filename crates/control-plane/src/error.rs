//! The typed failures the boundary reports. The adapter maps each one to its own bytes (#1309 D6).

use super::*;

/// A resource failure of the control plane's own preparation and bookkeeping.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceFault {
    /// A byte count or layout overflowed.
    Arithmetic,
    /// A byte count is not addressable on this platform.
    Platform,
    /// A configured limit refused the session, or a limit is zero where it may not be.
    Limit,
    /// An allocation failed.
    Allocation,
    /// The protocol queues could not be prepared.
    ProtocolQueue,
    /// The plan exchange could not be prepared.
    PlanExchange,
}

/// Why a compile or a structural command was refused.
#[derive(Debug)]
pub enum CompileFailure {
    /// The control plane's own resource failure.
    Resource(ResourceFault),
    /// Session, preparation and admission diagnostics, one `code\tpath\n` line each, exactly as
    /// the adapter reports them.
    Diagnostics(Vec<u8>),
}

pub(crate) fn failure(fault: ResourceFault) -> CompileFailure {
    CompileFailure::Resource(fault)
}

/// A whole-session diagnostic line, `code\t$\n`.
pub(crate) fn diagnostic(code: &str) -> CompileFailure {
    CompileFailure::Diagnostics(format!("{code}\t$\n").into_bytes())
}

pub(crate) fn session_diagnostics(diagnostics: &DiagnosticSet) -> CompileFailure {
    let mut bytes = Vec::new();
    for diagnostic in diagnostics.diagnostics() {
        bytes.extend_from_slice(diagnostic.code.as_str().as_bytes());
        bytes.push(b'\t');
        bytes.extend_from_slice(diagnostic.path.to_string().as_bytes());
        bytes.push(b'\n');
    }
    CompileFailure::Diagnostics(bytes)
}
