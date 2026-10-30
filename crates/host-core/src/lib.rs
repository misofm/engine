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
//!   host-specific. This crate supplies only
#![cfg_attr(feature = "control-provider", doc = "  [`SessionControlProvider`],")]
#![cfg_attr(
    not(feature = "control-provider"),
    doc = "  `SessionControlProvider` (feature `control-provider`),"
)]
//!   the shared typed adapter from a compiled session and live plan sample projection to the
//!   transport-neutral provider trait; a host still owns whether and how that provider is
//!   transported.
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
//! | [`prepare_host_runtime_successor`] | prepare a replacement plan whose unchanged sources keep their rings ([`SuccessorBase`], [`SourceControlSet::adopt_persisting`]) |
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
//! | [`StartedRenderSession::render_planar`] | render, exclusively | the guarded render entry: pins the canonical floating-point environment for the block and restores the caller's exact control word |
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
//!   `tools/realtime-policy` enforces the source-level half of that;
//!   `tools/audit` and the browser call-graph gate enforce the binary half.
//! * **A native render entry borrows the caller's floating-point environment; it never adopts it**
//!   (issue #146). A native audio thread may run with hardware FTZ and DAZ set (see
//!   `crates/lane/src/fpenv.rs`), and issue #144 measured what that does: 69-70 of the 331 corpus
//!   comparisons render off-pin, because a transient intra-block denormal is not a recursive state
//!   word the master-plan D7 flush law can reach. So the entry saves the caller's control word,
//!   installs the canonical one, renders, and restores the caller's exact word -- on the success
//!   path, on every rejection path and while unwinding. A host does not have to configure its
//!   thread, and gets its thread back unchanged. [`StartedRenderSession`] is that entry for an
//!   embedding host; `miso_engine_v1_render_f32_planar` is it for the C ABI. Browser Wasm needs
//!   neither: the core specification fixes round-to-nearest-even and full subnormal arithmetic, so
//!   the guard is a zero-sized value there and the shipped artifact is unchanged.
//! * **A started session is neither `Send` nor `Sync`.** The attestation is a statement about one
//!   thread; a handle that could move would let a host launder it onto a thread that was never
//!   attested. [`PreparedHost`] stays `Send`, because moving *preparation* to the render thread is
//!   the supported hand-off.

pub mod control_preparation;
#[cfg(feature = "control-provider")]
pub mod control_provider;
pub mod diagnostics;
pub mod duck;
#[cfg(feature = "control-provider")]
pub mod live_delta;
pub mod meters;
pub mod observation;
pub use control_preparation::{
    EQ_EDIT_CAPACITY, EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer,
    EqTargetPreparerError, INPUT_FILTER_EDIT_CAPACITY, INPUT_FILTER_TARGET_CAPACITY,
    INPUT_FILTER_VALUE_COUNT, InputFilterEdit, InputFilterEditErrorKind, InputFilterPreparer,
    InputFilterPreparerError, apply_input_filter_edit,
};
pub mod prepare;
pub mod render_session;
pub mod response;
pub mod response_capture;
pub mod route_controls;
pub mod shape;
pub mod source;
pub mod spectrum;
pub mod spectrum_service;
#[cfg(feature = "control-provider")]
pub mod transition;
#[cfg(feature = "control-provider")]
pub mod warm;
// Issues #996 and #997: the true-peak limiter's linked-pair session test. It predates the
// `test-support` width seam (`test_only_prepare_host_runtime_with_live_controls_on`) that
// `tests/` files now use, and stays a unit test on the `#[cfg(test)]` backend seam in `prepare`.
#[cfg(test)]
mod limiter_linked_session;

/// The meter tap and metric set a plan's meter rows name (#1588 D3).
pub use builtins::{MeterMetricSet, MeterTap};
#[cfg(feature = "control-provider")]
pub use control_provider::{
    CatalogResources, ControlCatalog, MAXIMUM_METER_BATCH_RECORDS, PlanSampleSource,
    ResponseFrameError, SessionControlProvider, SessionControlProviderError,
    SessionControlProviderResources, StagedCatalog, StagedValues, response_ready_event,
    serve_response_frame,
};
#[cfg(all(feature = "control-provider", feature = "test-support"))]
pub use control_provider::{
    fail_next_stages, take_response_counts, take_stage_counts, verify_stages,
};
pub use diagnostics::{
    PrepareDiagnostics, PrepareRejection, diagnostic_lines, fixed_diagnostic_line,
};
pub use duck::DuckOverlay;
pub use effect_compiler::EffectStateProjection;
pub use engine::LAUNCH_SAMPLE_RATES;
pub use engine::realtime::{
    ObserverCarry, ObserverKind, ResponseSnapshotAvailability, ResponseSnapshotCapture,
    ResponseSnapshotError, ResponseSnapshotOwnerInfo, ResponseSnapshotSection,
    ResponseSnapshotSink,
};
/// A following route's seven-word cell (#1652 D1): what a host writes a [`SendRecord::Following`]
/// with (#1653).
pub use graph::{FollowRouteControlRecord, GraphFollowRouteControlProducer, RouteFollowFlags};
/// The bank dispatch a preparation or a projection is planned for ([`HostProjectionRequest::backend`]):
/// re-exported so a host names it without a `lane` dependency of its own.
pub use graph_compiler::Backend;
#[cfg(feature = "control-provider")]
pub use live_delta::{
    LiveDelta, LiveRebuild, LiveRouteRecord, LiveStripRecords, automation_jump_row,
    classify_live_delta, classify_live_delta_with_monitor, classify_live_edit,
    classify_live_staged,
};
#[cfg(all(feature = "control-provider", feature = "test-support"))]
pub use live_delta::{take_fast_classification_count, take_lowered_owner_count};
pub use meters::{
    GainReductionTaps, METER_FRAME_RING_DEPTH, METER_QUEUE_DEPTH, MeterCarry, MeterDelivery,
    MeterFrame, MeterFrameConsumer, MeterFrameProducer, MeterFrameRing, MeterFrameRingSetupError,
    MeterLevel, MeterTables, MeterValidity, OutputPeakDrops, OutputPeakGate, OutputPeakObserver,
    OutputPeakReader, OutputPeakWindow, TapView, default_meter_window_blocks, fold_gain_reduction,
    meter_selection, output_peak_meter, output_peak_queue_payload,
};
pub use prepare::{
    GraphJumpLengthProducer, GraphJumpLengths, HostLiveControlHandles, HostLiveControlRequest,
    HostLiveLanes, HostMeterRequest, HostPrepareCaps, HostPrepareReport, HostProjection,
    HostProjectionRequest, HostShapePolicy, LiveRampRow, LiveRamps, MAXIMUM_VCA_REACH_PAIRS,
    MAXIMUM_VCAS_BOUNDED, MonitorBase, NO_OBSERVATION, PlanStateInventory, PreparedHost,
    SOURCE_STALL_TOLERANCE_MS, SendRecord, SourceInventoryRow, SuccessorBase, WarmConfig,
    WarmDecision, WarmLead, WarmUnavailable, check_counts, check_effect_projection,
    compile_host_model, compile_host_session, count_effects, default_source_ring_frames,
    effect_producer_position, follow_route_producer_for, follow_route_producer_position,
    jump_length_retained_bytes, launch_maximum_observation_taps, p_max_samples,
    pair_carried_meters, pair_carried_observations, parse_host_session, prepare_host_runtime,
    prepare_host_runtime_successor, prepare_host_runtime_with_live_controls,
    prepare_host_runtime_with_live_controls_successor, prepare_host_runtime_with_live_lanes,
    prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_live_lanes_successor, prepare_host_runtime_with_selected_meters,
    prepare_host_runtime_with_selected_meters_successor, prepare_host_runtime_with_spectrum,
    prepare_host_runtime_with_spectrum_collection, prepare_host_session,
    prepare_host_session_with_live_controls, project_effects, project_host_session,
    session_jump_lengths, source_seek_runway_blocks, stall_ring_frames, strip_control_table_bytes,
    strip_position, warm_lead,
};
pub use render_session::StartedRenderSession;
pub use response::{
    PreparedResponsePreview, PreparedResponseQuery, RESPONSE_MAXIMUM_OVERRIDES,
    RESPONSE_MAXIMUM_POINTS, ResolvedResponseTarget, ResponseAddressSection, ResponseChannels,
    ResponseCurves, ResponseMember, ResponseMemberKind, ResponseMemberSection,
    ResponseParameterOverride, ResponsePreviewError, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewOutput, ResponsePreviewRequest, ResponsePreviewTarget, ResponseQuery,
    ResponseQueryError, ResponseRefusal, ResponseSnapshot, ResponseSnapshotCollector,
    ResponseSnapshotMode, ResponseSnapshotOutput, ResponseSnapshotOwner,
    ResponseSnapshotQueryError, ResponseSnapshotQuerySummary, TrackResponse,
    compose_track_response, generate_response_grid, input_filter_target, prepare_response_preview,
    prepare_response_query, query_response_snapshot_into, resolve_response_target,
};
pub use response_capture::{
    ResponseCaptureCeiling, ResponseCaptureClient, ResponseCaptureFailure, ResponseCaptureJob,
    ResponseCaptureRefusal, ResponseCaptureServer, ResponseCaptureSetupError, ResponseReady,
    response_capture_ceiling, response_capture_channel, response_capture_owners,
    response_capture_retained_bytes,
};
pub use route_controls::{
    RouteControlError, RouteControlProducer, RouteControlRecord, RouteControlResources,
    route_producer_position, route_record,
};
/// The most channels one source can declare at launch (mono or stereo): the one limit the session
/// validator, the C ABI's source submission and the browser's staging all read.
pub use session::SOURCE_CHANNELS_LAUNCH_MAXIMUM;
pub use shape::{HostSessionShape, compiled_session_shape};
pub use source::{
    ConsumerDonation, DonationError, DonationPlan, DonationSide, DonationSideMut, ProducerDonation,
    SourceControlError, SourceControlSet, SourceSubmission, apply_donation, check_donation,
    control_table_bytes, drops_acked_fresh_source, source_id_arena_bytes,
};
pub use spectrum::{
    SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB, SPECTRUM_WINDOW_FRAMES, SpectrumAnalysisError,
    SpectrumAnalysisHistory, SpectrumAnalysisHistoryResources, SpectrumAnalysisMetadata,
    SpectrumAnalyzer, SpectrumCadence, SpectrumCadenceError, SpectrumCapture,
    SpectrumCaptureCollection, SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionReadError,
    SpectrumCaptureCollectionRequest, SpectrumCaptureCollectionSelectionError,
    SpectrumCaptureError, SpectrumCaptureReadError, SpectrumCaptureRequest,
    SpectrumCaptureResources, SpectrumChannels, SpectrumContinuousCaptureError,
    SpectrumContinuousReadError, SpectrumContinuousWindow, SpectrumHop, SpectrumOutput,
    SpectrumPair, SpectrumPrepareError, SpectrumSmoothingConfig, SpectrumSmoothingConfigError,
    SpectrumTarget, SpectrumTargetRef, SpectrumWindow, spectrum_analysis_history_resources,
    spectrum_capture_collection_resources, spectrum_capture_resources,
    spectrum_capture_resources_for,
};
pub use spectrum_service::{
    PreparedSpectrumCaptures, SPECTRUM_CHANNEL_BOTH, SPECTRUM_CHANNEL_LEFT, SPECTRUM_CHANNEL_RIGHT,
    SPECTRUM_MAXIMUM_CAPTURE_BYTES, SPECTRUM_MAXIMUM_ID_BYTES, SPECTRUM_TARGET_OUTPUT,
    SPECTRUM_TARGET_TRACK_POST_INPUT, SPECTRUM_TARGET_TRACK_POST_PAN, SpectrumAnalysis,
    SpectrumAnalysisResult, SpectrumConfiguration, SpectrumPreparation, SpectrumRequestError,
    SpectrumService, SpectrumServiceError, SpectrumSnapshot, SpectrumStreamMetadata,
    SpectrumStreamStatus, spectrum_channels_from_raw, spectrum_smoothing, spectrum_target_kind,
    spectrum_target_ref,
};
#[cfg(feature = "control-provider")]
pub use transition::{
    DuckRouteRecord, DuckStripRecords, StripTransition, StripTransitionError, grown_strips,
    plan_strip_transition,
};
#[cfg(feature = "control-provider")]
pub use warm::{PrimedCandidate, fall_back_to_transition, publish_primed};

/// The control-side half of one prepared effect's live-control channel (issue #140 A).
///
/// Re-exported here so a host does not have to depend on `effect-compiler` -- the
/// compile pipeline stays in this crate (#106 F1) and a host names only what its own ABI names.
pub use effect_compiler::{
    EffectControlOwnerError, EffectControlProducer, EffectControlResources,
    EffectObservationHandle, EffectRack, LiveEffectAddress, LiveEffectRack,
    launch_registry_owns_factory, parametric_eq_target_preparation_factory,
};
#[cfg(feature = "test-support")]
pub use effect_compiler::{
    test_only_parametric_eq_design_call_count, test_only_reset_parametric_eq_design_calls,
};
/// The session timeline's sample in a prepared plan (#1641 D8), for tests of hosts that do not
/// name `graph` (host-web).
#[cfg(feature = "test-support")]
pub use graph::test_only_timeline_at;
#[cfg(feature = "test-support")]
pub use prepare::{
    test_only_prepare_host_runtime_on, test_only_prepare_host_runtime_successor_on,
    test_only_prepare_host_runtime_with_live_controls_on,
    test_only_prepare_host_runtime_with_live_controls_successor_on,
    test_only_prepare_host_runtime_with_selected_meters_on, test_only_set_source_read_offset,
};
#[cfg(feature = "test-support")]
pub use spectrum::{
    SpectrumOperationCounts, test_only_reset_spectrum_operation_counts,
    test_only_spectrum_operation_counts,
};

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
/// thread, by `PreparedRenderPlan::arm_mono_collapse`, which since issue #970 arms only the chains
/// that gather their tracks' inputs; a plan nobody joins never collapses, which is the safe default
/// and the reason the join is an explicit call rather than an inference.
pub use builtins_compiler::{session_structural_symmetry, track_mono_source};

/// The live fader/mute and matrix/pan records and their lane selector (issue #1255 D6), re-exported
/// so a host that pushes `classify_live_delta`'s records names them without depending on
/// `builtins-compiler` or `builtins`.
pub use builtins::BuiltinLaneSelector;
/// One meter's consumer (`HostLiveControlHandles::meters`), re-exported so a host that keeps them,
/// as the C ABI does per provider epoch (#1581 D6), names the type.
pub use builtins_compiler::MeterConsumer;
/// One strip's live-control producers (`HostLiveControlHandles::strip_controls`), re-exported so a
/// host that keeps them, as the C ABI does per provider epoch (#1256 D3), names the type without
/// depending on `builtins-compiler`.
pub use builtins_compiler::{FaderCells, LiveValuesSuperseded, MatrixCells, TrackControlProducer};
pub use builtins_compiler::{FaderOffsetsRecord, TrackControlRecord, TrackFaderRecord};
pub use effect_contract::{ChannelSymmetryWitness, LiveControlRecord, SeamSide, SymmetryEvent};

#[doc(hidden)]
pub use session::CompiledSession;
