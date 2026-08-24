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
//!
//! # Host callback contract (V1)
//!
//! Normative for every embedding: the C ABI host, the browser host, and the native and mobile hosts
//! that issue 023 will grow real platform audio callbacks for. It lives here because this is the
//! crate every host already depends on, and because #106 found the two existing hosts had each
//! transcribed a different subset of it into their own comments.
//!
//! | call | thread | rule |
//! |---|---|---|
//! | [`prepare_host_session`] / [`prepare_host_runtime`] | control | allocates, parses and compiles; never inside an audio callback |
//! | [`SourceControlSet::submit`] / [`SourceControlSet::seek`] | control, one thread at a time | copies once into the ring, returns typed backpressure, never blocks and never allocates |
//! | `PreparedRenderPlan::render(io, RenderTime { absolute_sample })` | render, exclusively | exactly once per quantum; `absolute_sample` must equal the previous report's `next_absolute_sample`, and `0` on the first call; no other call touches the plan from any other thread |
//! | `drop(PreparedHost)` / `PlanRetirer::try_reclaim` | control | only after the render thread has quiesced; never from the callback |
//! | `NativeGraphWorkerPoolV1` creation (issue #100) | control, before the first render | the pool is control-owned and plan-independent; its lease travels with the plan; `wasm32` has no pool |
//! | render-thread and worker priority, affinity, platform workgroups | host platform code | outside the engine crates entirely; the issue-023 successor owns it (hand-off recorded from #100 §5) |
//!
//! Further rules that are not per-call:
//!
//! * [`PreparedHost`], `PreparedRenderPlan` and [`SourceControlSet`] are `Send` and **not** `Sync`.
//!   Move each to its thread once. A host that wants plan replacement does not swap fields; it
//!   wraps the plan in `miso_engine_core::realtime::plan_exchange` and calls
//!   `RealtimePlanOwner::render`, which is the only retirement mechanism the engine offers.
//! * Output is planar `f32`, one quantum of frames, caller-owned, with the channel count the
//!   session's output profile declares.
//! * **A `RenderError` is sticky and must not free anything.** The host keeps the plan, the session
//!   and the source rings exactly where they are, fills the output block with positive zero,
//!   records the failure, and reclaims on teardown from a control thread. This is not advisory:
//!   the browser host proves it with a call-graph gate over its shipped artifact
//!   (`scripts/check-web-audioworklet-callgraph.py`), which fails if anything reachable from the
//!   render export can call an allocator, a deallocator or drop glue.
//! * `render` performs no allocation, no lock, no syscall, no logging and no structural mutation.
//!   `scripts/check-realtime-policy.sh` enforces the source-level half of that;
//!   `tools/miso-engine-graph-audit` and the browser call-graph gate enforce the binary half.

pub mod diagnostics;
pub mod prepare;
pub mod source;

pub use diagnostics::{
    PrepareDiagnostics, PrepareRejection, diagnostic_lines, fixed_diagnostic_line,
};
pub use prepare::{
    HostConsoleHandlesV1, HostConsoleRequestV1, HostPrepareCaps, HostPrepareReport,
    HostShapePolicy, LAUNCH_SAMPLE_RATES_HZ, PreparedHost, compile_host_session, count_effects,
    parse_host_session, prepare_host_runtime, prepare_host_runtime_with_console,
    prepare_host_session, prepare_host_session_with_console,
};
pub use source::{
    SourceControlError, SourceControlSet, SourceSubmission, control_table_bytes,
    source_id_arena_bytes,
};

#[doc(hidden)]
pub use miso_engine_session::CompiledSession;
