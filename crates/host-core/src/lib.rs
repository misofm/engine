//! Shared host preparation: the one pipeline every embedding uses to turn session JSON into a
//! render plan, a source control set and a resource report.
//!
//! # Why this crate exists
//!
//! Issue #103 F1 found the C ABI host (`capi`) and the browser host
//! (`host-web`) carrying two ~300-line verbatim copies of the same compile
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
//! * **Control transport orchestration.** Protocol queues, replay and command dispatch remain
//!   host-specific. This crate supplies only [`SessionControlProvider`], the shared typed adapter
//!   from a compiled session and live plan sample projection to the transport-neutral provider
//!   trait; a host still owns whether and how that provider is transported.
//! * **Unsafe code and FFI.** This crate is a plain `rlib` with no `#[unsafe(no_mangle)]` items: a
//!   `cdylib` re-exports every `no_mangle` symbol it links, so a facade that carried them would
//!   push the C ABI's exports into the browser artifact and break its frozen export gate.
//!
//! # Entry points
//!
//! | call | when |
//! |---|---|
//! | [`prepare_host_session`] | the common case: JSON in, [`PreparedHost`] and the compiled session out |
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
//! | [`PreparedHost::start_render_session`] / [`StartedRenderSession::start`] | render, once | attests this thread's floating-point environment before the first block and returns the plan on refusal |
//! | [`StartedRenderSession::render_contiguous`] | render, exclusively | the guarded render entry: pins the canonical floating-point environment for the block and restores the caller's exact control word |
//! | [`SourceControlSet::submit`] / [`SourceControlSet::seek`] | control, one thread at a time | copies once into the ring, returns typed backpressure, never blocks and never allocates |
//! | `PreparedRenderPlan::render(io, RenderTime { absolute_sample })` | render, exclusively | exactly once per quantum; `absolute_sample` must equal the previous report's `next_absolute_sample`, and `0` on the first call; no other call touches the plan from any other thread |
//! | `drop(PreparedHost)` / `PlanRetirer::try_reclaim` | control | only after the render thread has quiesced; never from the callback |
//! | render-thread and worker priority, affinity, platform workgroups | host platform code | outside the engine crates entirely; the issue-023 successor owns it (hand-off recorded from #100 §5) |
//!
//! Further rules that are not per-call:
//!
//! * [`PreparedHost`], `PreparedRenderPlan` and [`SourceControlSet`] are `Send` and **not** `Sync`.
//!   Move each to its thread once. A host that wants plan replacement does not swap fields; it
//!   wraps the plan in `engine::realtime::plan_exchange` and calls
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
//!   `tools/audit` and the browser call-graph gate enforce the binary half.
//! * **A native render entry borrows the caller's floating-point environment; it never adopts it**
//!   (issue #146). Every DAW audio callback arrives with hardware FTZ and DAZ set, and issue #144
//!   measured what that does: 69-70 of the 331 cross-target corpus comparisons render off-pin,
//!   because a transient intra-block denormal is not a recursive state word the master-plan D7
//!   flush law can reach. So the entry saves the caller's control word, installs the canonical one,
//!   renders, and restores the caller's exact word -- on the success path, on every rejection path
//!   and while unwinding. A host does not have to configure its thread, and gets its thread back
//!   unchanged. [`StartedRenderSession`] is that entry for an embedding host;
//!   `miso_engine_v1_render_f32_planar` is it for the C ABI. Browser Wasm needs neither: the core
//!   specification fixes round-to-nearest-even and full subnormal arithmetic, so the guard is a
//!   zero-sized value there and the shipped artifact is unchanged.
//! * **A started session is neither `Send` nor `Sync`.** The attestation is a statement about one
//!   thread; a handle that could move would let a host launder it onto a thread that was never
//!   attested. [`PreparedHost`] stays `Send`, because moving *preparation* to the render thread is
//!   the supported hand-off.

#[cfg(feature = "control-provider")]
pub mod control_provider;
pub mod diagnostics;
pub mod prepare;
pub mod render_session;
pub mod shape;
pub mod solo;
pub mod source;

#[cfg(feature = "control-provider")]
pub use control_provider::{
    PlanSampleSource, PreparedSessionControlCatalog, SessionControlProvider,
    SessionControlProviderError, SessionControlProviderResources,
};
pub use diagnostics::{
    PrepareDiagnostics, PrepareRejection, diagnostic_lines, fixed_diagnostic_line,
};
pub use prepare::{
    HostConsoleHandles, HostConsoleRequest, HostPrepareCaps, HostPrepareReport, HostShapePolicy,
    LAUNCH_SAMPLE_RATES_HZ, PreparedHost, SOURCE_STALL_TOLERANCE_MS, compile_host_model,
    compile_host_session, count_effects, default_source_ring_frames, parse_host_session,
    prepare_host_runtime, prepare_host_runtime_with_console, prepare_host_session,
    prepare_host_session_with_console,
};
pub use render_session::StartedRenderSession;
pub use shape::{HostSessionShape, compiled_session_shape};
pub use solo::{ConsoleMuteDelta, ConsoleSoloState};
pub use source::{
    SourceControlError, SourceControlSet, SourceSubmission, control_table_bytes,
    source_id_arena_bytes,
};

/// The control-side half of one prepared effect's live-console channel (issue #140 A).
///
/// Re-exported here so a host does not have to depend on `effect-compiler` -- the
/// compile pipeline stays in this crate (#106 F1) and a host names only what its own ABI names.
pub use effect_compiler::{EffectControlProducer, EffectObservationHandle, EffectRack};

/// The channel-symmetry witness vocabulary, re-exported for hosts and planners.
///
/// # What a caller does with it
///
/// Two halves answer the question "may this track's two channels be collapsed into one plane?",
/// and they are deliberately separate because they are decided at different times by different
/// owners:
///
/// * [`session_structural_symmetry`] answers from the **compiled session**, before any plan
///   exists: the `SOURCE` term, which is the planner's pooling class.
/// * `PreparedRenderPlan::symmetry_counters` answers from the **built runtime**: the census of the
///   four terms preparation, restore and the live drains maintain.
///
/// The conjunction is [`ChannelSymmetryWitness::and`], and since mono-collapse M2 it is what the
/// bank chains dispatch on: a cohort whose every active lane holds both halves renders one plane
/// and duplicates it at the fader/matrix seam. The two halves are joined once, off the render
/// thread, by `PreparedRenderPlan::arm_mono_collapse`; a plan nobody joins never collapses, which
/// is the safe default and the reason the join is an explicit call rather than an inference.
pub use builtins_compiler::{session_structural_symmetry, track_mono_source};
pub use effect_contract::{ChannelSymmetryWitness, LiveConsoleRecord, SeamSide, SymmetryEvent};

#[doc(hidden)]
pub use session::CompiledSession;
