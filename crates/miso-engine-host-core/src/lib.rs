//! Shared host preparation: the one pipeline every embedding uses to turn session TOML into a
//! render plan, a source control set and a resource report.
//!
//! # Why this crate exists
//!
//! Issue #103 F1 found the C ABI host (`miso-engine-capi`) and the browser host
//! (`miso-engine-host-web`) carrying two ~300-line verbatim copies of the same compile
//! orchestration, two copies of the source control table with two different lookup strategies, two
//! copies of the diagnostic encoder, and two do-nothing `GraphRuntimeProcessor` shims. Divergence
//! had already started: only one copy rejected generation `0`, only one capped source channels.
//! This crate is the single definition; a host supplies [`HostPrepareCaps`] and nothing else.
//!
//! # What is *not* here
//!
//! * **Result codes.** The C ABI and the browser ABI number their results differently, so the
//!   facade returns typed values ([`PrepareRejection`], [`SourceControlError`]) and each host maps
//!   them onto its own frozen numbering.
//! * **The control protocol.** `miso-engine-protocol` is a host-specific transport; this crate
//!   never depends on it, so a host that does not speak it does not pay for it.
//! * **Unsafe code and FFI.** This crate is a plain `rlib` with no `#[unsafe(no_mangle)]` items: a
//!   `cdylib` re-exports every `no_mangle` symbol it links, so a facade that carried them would
//!   push the C ABI's exports into the browser artifact and break its frozen export gate.
//!
//! # Entry points
//!
//! | call | when |
//! |---|---|
//! | [`prepare_host_session`] | the common case: TOML in, [`PreparedHost`] and the compiled session out |
//! | [`compile_host_session`] | a host that wants the [`CompiledSession`] before preparing |
//! | [`parse_host_session`] + [`HostPrepareCaps::compile_caps`] | a host that builds its own `SessionStore` (the C ABI host does, for the control protocol) and then calls [`prepare_host_runtime`] |
//! | [`prepare_host_runtime`] | prepare from an already compiled session; also the plan-replacement path |
//! | [`HostPrepareCaps::validate_shape`] | check rate/quantum/ring before an expensive host-side pre-flight |
//!
//! # Threading
//!
//! Preparation allocates and must never run on a render thread. [`PreparedHost::plan`] moves to the
//! render thread and is rendered there exclusively; [`PreparedHost::sources`] stays on a control
//! thread and is fed from one thread at a time. Both are `Send` and neither is `Sync`.

pub mod diagnostics;
pub mod prepare;
pub mod source;

pub use diagnostics::{
    PrepareDiagnostics, PrepareRejection, diagnostic_lines, fixed_diagnostic_line,
};
pub use prepare::{
    HostPrepareCaps, HostPrepareReport, HostShapePolicy, LAUNCH_SAMPLE_RATES_HZ, PreparedHost,
    compile_host_session, count_effects, parse_host_session, prepare_host_runtime,
    prepare_host_session,
};
pub use source::{
    SourceControlError, SourceControlSet, SourceSubmission, control_table_bytes,
    source_id_arena_bytes,
};

#[doc(hidden)]
pub use miso_engine_session::CompiledSession;
